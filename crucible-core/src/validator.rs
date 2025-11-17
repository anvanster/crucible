//! Architecture validation engine

use crate::types::{Project, Severity};
use petgraph::algo::is_cyclic_directed;
use petgraph::graph::DiGraph;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
    pub info: Vec<ValidationIssue>,
    /// Modules that were actually validated (for incremental validation)
    pub validated_modules: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub rule: String,
    pub severity: Severity,
    pub message: String,
    pub location: Option<String>,
}

/// Tracks module changes for incremental validation
#[derive(Debug, Clone)]
pub struct ChangeTracker {
    /// Last validation time for each module
    pub module_timestamps: HashMap<String, SystemTime>,
    /// Modules that have changed since last validation
    pub changed_modules: HashMap<String, bool>,
    /// Dependency graph for impact analysis
    pub dependency_graph: HashMap<String, HashMap<String, bool>>,
}

impl ChangeTracker {
    pub fn new() -> Self {
        Self {
            module_timestamps: HashMap::new(),
            changed_modules: HashMap::new(),
            dependency_graph: HashMap::new(),
        }
    }

    /// Build dependency graph from project
    pub fn build_dependency_graph(&mut self, project: &Project) {
        self.dependency_graph.clear();

        // Build forward dependencies (who depends on me)
        for module in &project.modules {
            for (dep_name, _) in &module.dependencies {
                self.dependency_graph
                    .entry(dep_name.clone())
                    .or_insert_with(HashMap::new)
                    .insert(module.module.clone(), true);
            }
        }
    }

    /// Detect changed modules based on file modification times
    pub fn detect_changes(&mut self, project: &Project, root_path: &Path) -> HashMap<String, bool> {
        let mut changed = HashMap::new();

        for module in &project.modules {
            let module_path = root_path
                .join("modules")
                .join(format!("{}.json", module.module));

            if let Ok(metadata) = fs::metadata(&module_path) {
                if let Ok(modified) = metadata.modified() {
                    // Check if this is a new module or has been modified
                    if let Some(last_validated) = self.module_timestamps.get(&module.module) {
                        if modified > *last_validated {
                            changed.insert(module.module.clone(), true);
                        }
                    } else {
                        // New module, needs validation
                        changed.insert(module.module.clone(), true);
                    }
                }
            }
        }

        self.changed_modules = changed.clone();
        changed
    }

    /// Get all modules affected by changes (changed + dependents)
    pub fn get_affected_modules(
        &self,
        changed_modules: &HashMap<String, bool>,
    ) -> HashMap<String, bool> {
        let mut affected = changed_modules.clone();
        let mut to_process: Vec<String> = changed_modules.keys().cloned().collect();

        while let Some(module) = to_process.pop() {
            // Find all modules that depend on this one
            if let Some(dependents) = self.dependency_graph.get(&module) {
                for (dependent, _) in dependents {
                    if !affected.contains_key(dependent) {
                        affected.insert(dependent.clone(), true);
                        to_process.push(dependent.clone());
                    }
                }
            }
        }

        affected
    }

    /// Update timestamps after successful validation
    pub fn update_timestamps(&mut self, validated_modules: &[String]) {
        let now = SystemTime::now();
        for module in validated_modules {
            self.module_timestamps.insert(module.clone(), now);
        }
    }
}

pub struct Validator {
    project: Project,
    /// Optional change tracker for incremental validation
    change_tracker: Option<ChangeTracker>,
}

impl Validator {
    pub fn new(project: Project) -> Self {
        Self {
            project,
            change_tracker: None,
        }
    }

    /// Create a new validator with incremental validation support
    pub fn new_with_incremental(project: Project) -> Self {
        let mut change_tracker = ChangeTracker::new();
        change_tracker.build_dependency_graph(&project);

        Self {
            project,
            change_tracker: Some(change_tracker),
        }
    }

    /// Perform incremental validation - only validate changed modules and their dependents
    pub fn incremental_validate(&mut self, root_path: &Path) -> ValidationResult {
        // Extract data from tracker to avoid borrow issues
        let (changed_modules, affected_modules) = if let Some(ref mut tracker) = self.change_tracker
        {
            let changed = tracker.detect_changes(&self.project, root_path);

            if changed.is_empty() {
                // No changes, return successful result
                return ValidationResult {
                    valid: true,
                    errors: Vec::new(),
                    warnings: Vec::new(),
                    info: vec![ValidationIssue {
                        rule: "incremental-validation".to_string(),
                        severity: Severity::Info,
                        message: "No modules changed since last validation".to_string(),
                        location: None,
                    }],
                    validated_modules: Vec::new(),
                };
            }

            let affected = tracker.get_affected_modules(&changed);
            (changed, affected)
        } else {
            // No change tracker, fall back to full validation
            return self.validate();
        };

        // Filter project to only validate affected modules
        let filtered_project = self.filter_project_modules(&affected_modules);

        // Run validation on filtered project
        let mut result = self.validate_filtered(&filtered_project);

        // Add info about what was validated
        result.info.insert(
            0,
            ValidationIssue {
                rule: "incremental-validation".to_string(),
                severity: Severity::Info,
                message: format!(
                    "Incremental validation: {} modules changed, {} modules validated",
                    changed_modules.len(),
                    affected_modules.len()
                ),
                location: None,
            },
        );

        // Update timestamps for successfully validated modules
        if result.valid {
            if let Some(ref mut tracker) = self.change_tracker {
                tracker.update_timestamps(&result.validated_modules);
            }
        }

        result
    }

    /// Filter project to only include specified modules
    fn filter_project_modules(&self, module_names: &HashMap<String, bool>) -> Project {
        let filtered_modules: Vec<_> = self
            .project
            .modules
            .iter()
            .filter(|m| module_names.contains_key(&m.module))
            .cloned()
            .collect();

        Project {
            manifest: self.project.manifest.clone(),
            modules: filtered_modules,
            rules: self.project.rules.clone(),
        }
    }

    /// Validate a filtered project (internal use for incremental validation)
    fn validate_filtered(&self, project: &Project) -> ValidationResult {
        let temp_validator = Validator::new(project.clone());
        temp_validator.validate()
    }

    /// Run all validation rules
    pub fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
            validated_modules: self
                .project
                .modules
                .iter()
                .map(|m| m.module.clone())
                .collect(),
        };

        // Check for circular dependencies
        if let Some(issues) = self.check_circular_dependencies() {
            for issue in issues {
                match issue.severity {
                    Severity::Error => {
                        result.valid = false;
                        result.errors.push(issue);
                    }
                    Severity::Warning => result.warnings.push(issue),
                    Severity::Info => result.info.push(issue),
                }
            }
        }

        // Check layer boundaries if architecture is defined
        if let Some(rules) = &self.project.rules {
            if let Some(arch) = &rules.architecture {
                if let Some(issues) = self.check_layer_boundaries(arch) {
                    for issue in issues {
                        match issue.severity {
                            Severity::Error => {
                                result.valid = false;
                                result.errors.push(issue);
                            }
                            Severity::Warning => result.warnings.push(issue),
                            Severity::Info => result.info.push(issue),
                        }
                    }
                }
            }
        }

        // Check that all types exist
        if let Some(issues) = self.check_type_existence() {
            for issue in issues {
                match issue.severity {
                    Severity::Error => {
                        result.valid = false;
                        result.errors.push(issue);
                    }
                    Severity::Warning => result.warnings.push(issue),
                    Severity::Info => result.info.push(issue),
                }
            }
        }

        // Check that all function calls reference existing exports
        if let Some(issues) = self.check_call_targets() {
            for issue in issues {
                match issue.severity {
                    Severity::Error => {
                        result.valid = false;
                        result.errors.push(issue);
                    }
                    Severity::Warning => result.warnings.push(issue),
                    Severity::Info => result.info.push(issue),
                }
            }
        }

        // Check that all used dependencies are declared
        if let Some(issues) = self.check_used_dependencies() {
            for issue in issues {
                match issue.severity {
                    Severity::Error => {
                        result.valid = false;
                        result.errors.push(issue);
                    }
                    Severity::Warning => result.warnings.push(issue),
                    Severity::Info => result.info.push(issue),
                }
            }
        }

        // Check that all declared dependencies are used (warning only)
        if let Some(issues) = self.check_declared_dependencies() {
            for issue in issues {
                match issue.severity {
                    Severity::Error => {
                        result.valid = false;
                        result.errors.push(issue);
                    }
                    Severity::Warning => result.warnings.push(issue),
                    Severity::Info => result.info.push(issue),
                }
            }
        }

        result
    }

    /// Check for circular dependencies between modules
    fn check_circular_dependencies(&self) -> Option<Vec<ValidationIssue>> {
        let mut graph = DiGraph::new();
        let mut node_map = HashMap::new();
        let mut issues = Vec::new();

        // Add nodes for each module
        for module in &self.project.modules {
            let node = graph.add_node(module.module.clone());
            node_map.insert(module.module.clone(), node);
        }

        // Add edges for dependencies
        for module in &self.project.modules {
            if let Some(from_node) = node_map.get(&module.module) {
                for (dep_name, _) in &module.dependencies {
                    if let Some(to_node) = node_map.get(dep_name) {
                        graph.add_edge(*from_node, *to_node, ());
                    }
                }
            }
        }

        // Check for cycles
        if is_cyclic_directed(&graph) {
            issues.push(ValidationIssue {
                rule: "no-circular-dependencies".to_string(),
                severity: Severity::Error,
                message: "Circular dependency detected in module graph".to_string(),
                location: None,
            });
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    /// Check that modules respect layer boundaries
    fn check_layer_boundaries(
        &self,
        architecture: &crate::types::Architecture,
    ) -> Option<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Create a map of module to layer
        let mut module_layers = HashMap::new();
        for module in &self.project.modules {
            if let Some(layer) = &module.layer {
                module_layers.insert(module.module.clone(), layer.clone());
            }
        }

        // Check each module's dependencies
        for module in &self.project.modules {
            if let Some(from_layer) = module.layer.as_ref() {
                // Find the layer definition
                let layer_def = architecture.layers.iter().find(|l| &l.name == from_layer);

                if let Some(layer) = layer_def {
                    // Check each dependency
                    for (dep_name, _) in &module.dependencies {
                        if let Some(to_layer) = module_layers.get(dep_name) {
                            // Check if this dependency is allowed
                            if !layer.can_depend_on.contains(to_layer) {
                                issues.push(ValidationIssue {
                                    rule: "respect-layer-boundaries".to_string(),
                                    severity: Severity::Error,
                                    message: format!(
                                        "Layer '{}' cannot depend on layer '{}'",
                                        from_layer, to_layer
                                    ),
                                    location: Some(format!("{} -> {}", module.module, dep_name)),
                                });
                            }
                        }
                    }
                }
            }
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    /// Check that all referenced types exist
    fn check_type_existence(&self) -> Option<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Collect all available types
        let mut available_types = HashMap::new();
        for module in &self.project.modules {
            for (export_name, _) in &module.exports {
                available_types.insert(format!("{}.{}", module.module, export_name), true);
                available_types.insert(export_name.clone(), true);
            }
        }

        // Add primitive types
        for primitive in &["string", "number", "boolean", "void", "null", "Date"] {
            available_types.insert(primitive.to_string(), true);
        }

        // Check all type references
        for module in &self.project.modules {
            for (export_name, export) in &module.exports {
                // Check method parameter and return types
                if let Some(methods) = &export.methods {
                    for (method_name, method) in methods {
                        // Check input types
                        for param in &method.inputs {
                            if !self.is_type_available(&param.param_type, &available_types) {
                                issues.push(ValidationIssue {
                                    rule: "all-types-must-exist".to_string(),
                                    severity: Severity::Error,
                                    message: format!("Type '{}' not found", param.param_type),
                                    location: Some(format!(
                                        "{}.{}.{}",
                                        module.module, export_name, method_name
                                    )),
                                });
                            }
                        }

                        // Check return type
                        if !self.is_type_available(&method.returns.return_type, &available_types) {
                            issues.push(ValidationIssue {
                                rule: "all-types-must-exist".to_string(),
                                severity: Severity::Error,
                                message: format!("Type '{}' not found", method.returns.return_type),
                                location: Some(format!(
                                    "{}.{}.{}",
                                    module.module, export_name, method_name
                                )),
                            });
                        }
                    }
                }
            }
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    /// Check if a type is available (handles generics)
    fn is_type_available(&self, type_name: &str, available_types: &HashMap<String, bool>) -> bool {
        // Handle generic types like Array<T>, Map<K,V>, etc.
        if type_name.contains('<') {
            let base_type = type_name.split('<').next().unwrap();
            return matches!(
                base_type,
                "Array" | "Vec" | "Map" | "HashMap" | "Promise" | "Result" | "Optional" | "Option"
            );
        }

        available_types.contains_key(type_name)
    }

    /// Check that all function calls reference existing exports
    /// Call format: "module.Export.method" or "module.function"
    fn check_call_targets(&self) -> Option<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Build a map of all available exports and their methods
        let mut available_exports: HashMap<String, HashMap<String, bool>> = HashMap::new();

        for module in &self.project.modules {
            for (export_name, export) in &module.exports {
                let full_export_name = format!("{}.{}", module.module, export_name);
                let mut methods = HashMap::new();

                // Add methods if they exist
                if let Some(export_methods) = &export.methods {
                    for method_name in export_methods.keys() {
                        methods.insert(method_name.clone(), true);
                    }
                }

                available_exports.insert(full_export_name, methods);
            }
        }

        // Check all calls in the project
        for module in &self.project.modules {
            for (export_name, export) in &module.exports {
                if let Some(methods) = &export.methods {
                    for (method_name, method) in methods {
                        for call in &method.calls {
                            // Parse call: "module.Export.method" or "module.function"
                            let parts: Vec<&str> = call.split('.').collect();

                            if parts.len() < 2 {
                                issues.push(ValidationIssue {
                                    rule: "all-calls-must-exist".to_string(),
                                    severity: Severity::Error,
                                    message: format!(
                                        "Invalid call format '{}' (expected 'module.Export.method' or 'module.function')",
                                        call
                                    ),
                                    location: Some(format!(
                                        "{}.{}.{}",
                                        module.module, export_name, method_name
                                    )),
                                });
                                continue;
                            }

                            let target_module = parts[0];
                            let target_export = parts[1];

                            // Check if this is a self-call (calling own export's methods)
                            // Format: "module.method" where module == current && method is on same export
                            if target_module == module.module && parts.len() == 2 {
                                // Check if this is calling a method on the SAME export (self-call)
                                if let Some(self_methods) = &export.methods {
                                    if self_methods.contains_key(target_export) {
                                        // This is a self-call - validate it exists (already checked above)
                                        continue;
                                    }
                                }
                                // Not a self-call, fall through to normal validation
                            }

                            // Check if calling own export's method with full format
                            if target_module == module.module
                                && parts.len() == 3
                                && target_export == export_name
                            {
                                // Full format: "module.Export.method" calling same export
                                let target_method = parts[2];
                                if let Some(self_methods) = &export.methods {
                                    if !self_methods.contains_key(target_method) {
                                        issues.push(ValidationIssue {
                                            rule: "all-calls-must-exist".to_string(),
                                            severity: Severity::Error,
                                            message: format!(
                                                "Method '{}' not found on '{}'",
                                                target_method, export_name
                                            ),
                                            location: Some(format!(
                                                "{}.{}.{}",
                                                module.module, export_name, method_name
                                            )),
                                        });
                                    }
                                }
                                continue;
                            }

                            // Check if it's a function call (2 parts) or method call (3 parts)
                            if parts.len() == 2 {
                                // Function call: module.function
                                let full_name = format!("{}.{}", target_module, target_export);
                                if !available_exports.contains_key(&full_name) {
                                    issues.push(ValidationIssue {
                                        rule: "all-calls-must-exist".to_string(),
                                        severity: Severity::Error,
                                        message: format!("Call target '{}' not found", call),
                                        location: Some(format!(
                                            "{}.{}.{}",
                                            module.module, export_name, method_name
                                        )),
                                    });
                                }
                            } else if parts.len() == 3 {
                                // Method call: module.Export.method
                                let target_method = parts[2];
                                let full_export = format!("{}.{}", target_module, target_export);

                                if let Some(export_methods) = available_exports.get(&full_export) {
                                    if !export_methods.contains_key(target_method) {
                                        issues.push(ValidationIssue {
                                            rule: "all-calls-must-exist".to_string(),
                                            severity: Severity::Error,
                                            message: format!(
                                                "Method '{}' not found on '{}.{}'",
                                                target_method, target_module, target_export
                                            ),
                                            location: Some(format!(
                                                "{}.{}.{}",
                                                module.module, export_name, method_name
                                            )),
                                        });
                                    }
                                } else {
                                    issues.push(ValidationIssue {
                                        rule: "all-calls-must-exist".to_string(),
                                        severity: Severity::Error,
                                        message: format!(
                                            "Export '{}.{}' not found",
                                            target_module, target_export
                                        ),
                                        location: Some(format!(
                                            "{}.{}.{}",
                                            module.module, export_name, method_name
                                        )),
                                    });
                                }
                            } else {
                                issues.push(ValidationIssue {
                                    rule: "all-calls-must-exist".to_string(),
                                    severity: Severity::Error,
                                    message: format!("Invalid call format '{}'", call),
                                    location: Some(format!(
                                        "{}.{}.{}",
                                        module.module, export_name, method_name
                                    )),
                                });
                            }
                        }
                    }
                }
            }
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    /// Check that all modules referenced in calls are declared as dependencies
    fn check_used_dependencies(&self) -> Option<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        for module in &self.project.modules {
            let mut used_modules = std::collections::HashSet::new();

            // Collect all modules referenced in calls
            for (_, export) in &module.exports {
                if let Some(methods) = &export.methods {
                    for (_, method) in methods {
                        for call in &method.calls {
                            // Extract module name from call (first part before '.')
                            if let Some(target_module) = call.split('.').next() {
                                // Skip if calling own module
                                if target_module != module.module {
                                    used_modules.insert(target_module.to_string());
                                }
                            }
                        }
                    }
                }
            }

            // Check that all used modules are in dependencies
            for used_module in used_modules {
                if !module.dependencies.contains_key(&used_module) {
                    issues.push(ValidationIssue {
                        rule: "used-dependencies-declared".to_string(),
                        severity: Severity::Error,
                        message: format!(
                            "Module '{}' is used but not declared in dependencies",
                            used_module
                        ),
                        location: Some(module.module.clone()),
                    });
                }
            }
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    /// Check that all declared dependencies are actually used (warning only)
    fn check_declared_dependencies(&self) -> Option<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        for module in &self.project.modules {
            let mut used_modules = std::collections::HashSet::new();

            // Collect all modules referenced in calls
            for (_, export) in &module.exports {
                if let Some(methods) = &export.methods {
                    for (_, method) in methods {
                        for call in &method.calls {
                            if let Some(target_module) = call.split('.').next() {
                                used_modules.insert(target_module.to_string());
                            }
                        }
                    }
                }
            }

            // Check for unused dependencies
            for (dep_name, _) in &module.dependencies {
                if !used_modules.contains(dep_name) {
                    issues.push(ValidationIssue {
                        rule: "declared-dependencies-must-be-used".to_string(),
                        severity: Severity::Warning,
                        message: format!("Dependency '{}' is declared but not used", dep_name),
                        location: Some(module.module.clone()),
                    });
                }
            }
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }
}
