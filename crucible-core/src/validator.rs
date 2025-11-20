//! Architecture validation engine

use crate::types::{Project, ReturnType, Severity};
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
    /// What was actually found (for comparison errors)
    pub found: Option<String>,
    /// What was expected (for comparison errors)
    pub expected: Option<String>,
    /// Suggestion for how to fix the issue
    pub suggestion: Option<String>,
    /// Link to relevant documentation
    pub doc_link: Option<String>,
}

impl ValidationIssue {
    /// Create a new validation issue with basic information
    pub fn new(rule: String, severity: Severity, message: String, location: Option<String>) -> Self {
        Self {
            rule,
            severity,
            message,
            location,
            found: None,
            expected: None,
            suggestion: None,
            doc_link: None,
        }
    }

    /// Create a new validation issue with comparison information
    pub fn with_comparison(
        rule: String,
        severity: Severity,
        message: String,
        location: Option<String>,
        found: String,
        expected: String,
    ) -> Self {
        Self {
            rule,
            severity,
            message,
            location,
            found: Some(found),
            expected: Some(expected),
            suggestion: None,
            doc_link: None,
        }
    }

    /// Add a suggestion to the issue
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    /// Add a documentation link to the issue
    pub fn with_doc_link(mut self, doc_link: String) -> Self {
        self.doc_link = Some(doc_link);
        self
    }
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

impl Default for ChangeTracker {
    fn default() -> Self {
        Self::new()
    }
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
            for dep_name in module.dependencies.keys() {
                self.dependency_graph
                    .entry(dep_name.clone())
                    .or_default()
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
                for dependent in dependents.keys() {
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
                    info: vec![ValidationIssue::new(
                        "incremental-validation".to_string(),
                        Severity::Info,
                        "No modules changed since last validation".to_string(),
                        None,
                    )],
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
            ValidationIssue::new(
                "incremental-validation".to_string(),
                Severity::Info,
                format!(
                    "Incremental validation: {} modules changed, {} modules validated",
                    changed_modules.len(),
                    affected_modules.len()
                ),
                None,
            ),
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
                for dep_name in module.dependencies.keys() {
                    if let Some(to_node) = node_map.get(dep_name) {
                        graph.add_edge(*from_node, *to_node, ());
                    }
                }
            }
        }

        // Check for cycles
        if is_cyclic_directed(&graph) {
            issues.push(
                ValidationIssue::new(
                    "no-circular-dependencies".to_string(),
                    Severity::Error,
                    "Circular dependency detected in module graph".to_string(),
                    None,
                )
                .with_suggestion(
                    "Remove one of the dependencies creating the cycle. \
                     Use 'crucible graph' to visualize the dependency structure."
                        .to_string(),
                )
                .with_doc_link("https://github.com/anvanster/crucible#circular-dependencies".to_string()),
            );
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
                    for dep_name in module.dependencies.keys() {
                        if let Some(to_layer) = module_layers.get(dep_name) {
                            // Check if this dependency is allowed
                            if !layer.can_depend_on.contains(to_layer) {
                                let allowed_layers = layer.can_depend_on.join(", ");
                                issues.push(
                                    ValidationIssue::with_comparison(
                                        "respect-layer-boundaries".to_string(),
                                        Severity::Error,
                                        format!(
                                            "Layer boundary violation: '{from_layer}' cannot depend on '{to_layer}'"
                                        ),
                                        Some(format!("{} -> {}", module.module, dep_name)),
                                        format!("dependency on '{to_layer}' layer"),
                                        format!("dependency on one of: {allowed_layers}"),
                                    )
                                    .with_suggestion(format!(
                                        "Remove the dependency on '{dep_name}' from module '{}', \
                                         or restructure your architecture to allow '{from_layer}' → '{to_layer}' dependencies.",
                                        module.module
                                    ))
                                    .with_doc_link("https://github.com/anvanster/crucible/blob/main/docs/common-mistakes.md#layer-dependency-issues".to_string()),
                                );
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
            for export_name in module.exports.keys() {
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
                                // Try to find similar type names for suggestion
                                let similar_types = self.find_similar_types(&param.param_type, &available_types);
                                let mut issue = ValidationIssue::new(
                                    "all-types-must-exist".to_string(),
                                    Severity::Error,
                                    format!("Type '{}' not found in parameter '{}'", param.param_type, param.name),
                                    Some(format!(
                                        "{}.{}.{} (parameter: {})",
                                        module.module, export_name, method_name, param.name
                                    )),
                                )
                                .with_doc_link("https://github.com/anvanster/crucible/blob/main/docs/type-system.md".to_string());

                                if !similar_types.is_empty() {
                                    issue = issue.with_suggestion(format!(
                                        "Did you mean one of: {}? If using a type from another module, \
                                         ensure it's listed in the dependencies field.",
                                        similar_types.join(", ")
                                    ));
                                } else {
                                    issue = issue.with_suggestion(
                                        "Ensure the type is exported from a module listed in dependencies, \
                                         or use a built-in type (string, number, boolean, void, Date)."
                                            .to_string(),
                                    );
                                }

                                issues.push(issue);
                            }
                        }

                        // Check return type (including array items if present)
                        if !self.is_return_type_available(&method.returns, &available_types) {
                            let type_desc = if method.returns.return_type == "array"
                                && method.returns.inner.is_some()
                            {
                                format!("array<{}>", method.returns.inner.as_ref().unwrap())
                            } else {
                                method.returns.return_type.clone()
                            };

                            let similar_types = self.find_similar_types(&type_desc, &available_types);
                            let mut issue = ValidationIssue::new(
                                "all-types-must-exist".to_string(),
                                Severity::Error,
                                format!("Return type '{type_desc}' not found"),
                                Some(format!(
                                    "{}.{}.{} (returns)",
                                    module.module, export_name, method_name
                                )),
                            )
                            .with_doc_link("https://github.com/anvanster/crucible/blob/main/docs/type-system.md".to_string());

                            if !similar_types.is_empty() {
                                issue = issue.with_suggestion(format!(
                                    "Did you mean one of: {}? If using a type from another module, \
                                     ensure it's listed in the dependencies field.",
                                    similar_types.join(", ")
                                ));
                            } else {
                                issue = issue.with_suggestion(
                                    "Ensure the type is exported from a module listed in dependencies, \
                                     or use a built-in type (string, number, boolean, void, Date, Promise<T>)."
                                        .to_string(),
                                );
                            }

                            issues.push(issue);
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
    fn is_type_available(&self, type_name: &str, _available_types: &HashMap<String, bool>) -> bool {
        // Use the new TypeValidator for enhanced type checking
        use crate::type_system::TypeValidator;

        let validator = TypeValidator::new();
        validator
            .validate_type_string(type_name, None, &self.project.modules)
            .is_ok()
    }

    /// Check if a return type is available (handles array items)
    fn is_return_type_available(
        &self,
        return_type: &ReturnType,
        _available_types: &HashMap<String, bool>,
    ) -> bool {
        use crate::type_system::{TypeParser, TypeValidator};

        let parser = TypeParser::new();
        let validator = TypeValidator::new();

        // Parse the return type with items if present
        let type_ref = if return_type.return_type == "array" && return_type.inner.is_some() {
            // Array with items
            parser
                .parse_from_json("array", None, return_type.inner.as_deref(), None)
                .ok()
        } else {
            // Regular type
            parser.parse(&return_type.return_type).ok()
        };

        if let Some(type_ref) = type_ref {
            validator
                .validate_type_exists(&type_ref, &self.project.modules)
                .is_ok()
        } else {
            false
        }
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
                                issues.push(
                                    ValidationIssue::with_comparison(
                                        "all-calls-must-exist".to_string(),
                                        Severity::Error,
                                        format!("Invalid call format: '{call}'"),
                                        Some(format!(
                                            "{}.{}.{}",
                                            module.module, export_name, method_name
                                        )),
                                        format!("'{call}'"),
                                        "'module.Export.method' or 'module.function'".to_string(),
                                    )
                                    .with_suggestion(
                                        "Use format 'module.function' for function calls or \
                                         'module.Export.method' for method calls."
                                            .to_string(),
                                    )
                                    .with_doc_link("https://github.com/anvanster/crucible/blob/main/docs/schema-reference.md#method-calls".to_string()),
                                );
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
                                        issues.push(
                                            ValidationIssue::new(
                                                "all-calls-must-exist".to_string(),
                                                Severity::Error,
                                                format!(
                                                    "Method '{target_method}' not found on '{export_name}'"
                                                ),
                                                Some(format!(
                                                    "{}.{}.{}",
                                                    module.module, export_name, method_name
                                                )),
                                            )
                                            .with_suggestion(format!(
                                                "Ensure '{target_method}' is defined as a method in the '{}' export.",
                                                export_name
                                            ))
                                            .with_doc_link("https://github.com/anvanster/crucible/blob/main/docs/schema-reference.md#method-calls".to_string()),
                                        );
                                    }
                                }
                                continue;
                            }

                            // Check if it's a function call (2 parts) or method call (3 parts)
                            if parts.len() == 2 {
                                // Function call: module.function
                                let full_name = format!("{target_module}.{target_export}");
                                if !available_exports.contains_key(&full_name) {
                                    issues.push(
                                        ValidationIssue::new(
                                            "all-calls-must-exist".to_string(),
                                            Severity::Error,
                                            format!("Call target '{call}' not found"),
                                            Some(format!(
                                                "{}.{}.{}",
                                                module.module, export_name, method_name
                                            )),
                                        )
                                        .with_suggestion(format!(
                                            "Ensure '{target_export}' is exported from module '{target_module}' \
                                             and '{target_module}' is listed in dependencies."
                                        ))
                                        .with_doc_link("https://github.com/anvanster/crucible/blob/main/docs/schema-reference.md#dependencies".to_string()),
                                    );
                                }
                            } else if parts.len() == 3 {
                                // Method call: module.Export.method
                                let target_method = parts[2];
                                let full_export = format!("{target_module}.{target_export}");

                                if let Some(export_methods) = available_exports.get(&full_export) {
                                    if !export_methods.contains_key(target_method) {
                                        issues.push(
                                            ValidationIssue::new(
                                                "all-calls-must-exist".to_string(),
                                                Severity::Error,
                                                format!(
                                                    "Method '{target_method}' not found on '{target_module}.{target_export}'"
                                                ),
                                                Some(format!(
                                                    "{}.{}.{}",
                                                    module.module, export_name, method_name
                                                )),
                                            )
                                            .with_suggestion(format!(
                                                "Ensure '{target_method}' is defined as a method in export '{}' of module '{}'.",
                                                target_export, target_module
                                            ))
                                            .with_doc_link("https://github.com/anvanster/crucible/blob/main/docs/schema-reference.md#methods".to_string()),
                                        );
                                    }
                                } else {
                                    issues.push(
                                        ValidationIssue::new(
                                            "all-calls-must-exist".to_string(),
                                            Severity::Error,
                                            format!(
                                                "Export '{target_module}.{target_export}' not found"
                                            ),
                                            Some(format!(
                                                "{}.{}.{}",
                                                module.module, export_name, method_name
                                            )),
                                        )
                                        .with_suggestion(format!(
                                            "Ensure '{target_export}' is exported from module '{target_module}' \
                                             and '{target_module}' is listed in dependencies."
                                        ))
                                        .with_doc_link("https://github.com/anvanster/crucible/blob/main/docs/schema-reference.md#dependencies".to_string()),
                                    );
                                }
                            } else {
                                issues.push(
                                    ValidationIssue::with_comparison(
                                        "all-calls-must-exist".to_string(),
                                        Severity::Error,
                                        format!("Invalid call format: '{call}'"),
                                        Some(format!(
                                            "{}.{}.{}",
                                            module.module, export_name, method_name
                                        )),
                                        format!("'{call}'"),
                                        "'module.function' or 'module.Export.method'".to_string(),
                                    )
                                    .with_suggestion(
                                        "Use format 'module.function' for function calls or \
                                         'module.Export.method' for method calls."
                                            .to_string(),
                                    )
                                    .with_doc_link("https://github.com/anvanster/crucible/blob/main/docs/schema-reference.md#method-calls".to_string()),
                                );
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
            for export in module.exports.values() {
                if let Some(methods) = &export.methods {
                    for method in methods.values() {
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
                    issues.push(
                        ValidationIssue::new(
                            "used-dependencies-declared".to_string(),
                            Severity::Error,
                            format!(
                                "Module '{used_module}' is used but not declared in dependencies"
                            ),
                            Some(module.module.clone()),
                        )
                        .with_suggestion(format!(
                            "Add '{used_module}' to the dependencies field in module '{}'.\n\
                             Example: \"dependencies\": {{\"{}user\": \"ExportName\", ... }}",
                            module.module, used_module
                        ))
                        .with_doc_link("https://github.com/anvanster/crucible/blob/main/docs/schema-reference.md#dependencies".to_string()),
                    );
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
            for export in module.exports.values() {
                if let Some(methods) = &export.methods {
                    for method in methods.values() {
                        for call in &method.calls {
                            if let Some(target_module) = call.split('.').next() {
                                used_modules.insert(target_module.to_string());
                            }
                        }
                    }
                }
            }

            // Check for unused dependencies
            for dep_name in module.dependencies.keys() {
                if !used_modules.contains(dep_name) {
                    issues.push(
                        ValidationIssue::new(
                            "declared-dependencies-must-be-used".to_string(),
                            Severity::Warning,
                            format!("Dependency '{dep_name}' is declared but not used"),
                            Some(module.module.clone()),
                        )
                        .with_suggestion(format!(
                            "Remove '{dep_name}' from the dependencies field in module '{}', \
                             or add a method call that uses it.",
                            module.module
                        ))
                        .with_doc_link("https://github.com/anvanster/crucible/blob/main/docs/schema-reference.md#dependencies".to_string()),
                    );
                }
            }
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    /// Find type names similar to the given type (for suggestions)
    /// Uses simple Levenshtein distance for fuzzy matching
    fn find_similar_types(
        &self,
        target_type: &str,
        available_types: &HashMap<String, bool>,
    ) -> Vec<String> {
        let mut candidates: Vec<(String, usize)> = available_types
            .keys()
            .filter_map(|type_name| {
                let distance = levenshtein_distance(target_type, type_name);
                // Only suggest if distance is small relative to type name length
                if distance <= 3 && distance < target_type.len() / 2 {
                    Some((type_name.clone(), distance))
                } else {
                    None
                }
            })
            .collect();

        // Sort by distance (closest first) and take top 3
        candidates.sort_by_key(|(_, dist)| *dist);
        candidates.into_iter().take(3).map(|(name, _)| name).collect()
    }
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost,
            );
        }
    }

    matrix[len1][len2]
}
