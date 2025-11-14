//! Architecture validation engine

use crate::types::{Project, Severity};
use petgraph::algo::is_cyclic_directed;
use petgraph::graph::DiGraph;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
    pub info: Vec<ValidationIssue>,
}

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub rule: String,
    pub severity: Severity,
    pub message: String,
    pub location: Option<String>,
}

pub struct Validator {
    project: Project,
}

impl Validator {
    pub fn new(project: Project) -> Self {
        Self { project }
    }

    /// Run all validation rules
    pub fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
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
                available_types.insert(
                    format!("{}.{}", module.module, export_name),
                    true,
                );
                available_types.insert(export_name.clone(), true);
            }
        }

        // Add primitive types
        for primitive in &["string", "number", "boolean", "void", "null"] {
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
}
