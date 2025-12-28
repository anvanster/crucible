//! Compliance validation engine

use crate::error::ComplianceResult;
use crate::framework::Framework;
use crucible_core::types::{
    ComplianceRule, Export, Method, Module, Project, Property, Severity, ValidationCheckType,
};
use std::collections::HashMap;

/// A compliance violation found during validation
#[derive(Debug, Clone)]
pub struct Violation {
    /// The rule that was violated
    pub rule_id: String,
    /// Severity of the violation
    pub severity: Severity,
    /// Description of the violation
    pub description: String,
    /// Location in the architecture (module.export.method or module.export.property)
    pub location: String,
    /// The specific issue found
    pub issue: String,
    /// Suggestion for fixing the violation
    pub suggestion: Option<String>,
}

/// Report containing all validation results
#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
    /// Framework that was validated against
    pub framework_name: String,
    /// Framework version
    pub framework_version: String,
    /// All violations found
    pub violations: Vec<Violation>,
    /// Number of rules checked
    pub rules_checked: usize,
    /// Number of modules validated
    pub modules_validated: usize,
}

impl ValidationReport {
    /// Create a new empty report
    pub fn new(framework_name: String, framework_version: String) -> Self {
        Self {
            framework_name,
            framework_version,
            ..Default::default()
        }
    }

    /// Check if the validation passed (no errors)
    pub fn passed(&self) -> bool {
        !self
            .violations
            .iter()
            .any(|v| v.severity == Severity::Error)
    }

    /// Get error-level violations
    pub fn errors(&self) -> Vec<&Violation> {
        self.violations
            .iter()
            .filter(|v| v.severity == Severity::Error)
            .collect()
    }

    /// Get warning-level violations
    pub fn warnings(&self) -> Vec<&Violation> {
        self.violations
            .iter()
            .filter(|v| v.severity == Severity::Warning)
            .collect()
    }

    /// Get the total number of violations
    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }

    /// Get the number of errors
    pub fn error_count(&self) -> usize {
        self.errors().len()
    }

    /// Get the number of warnings
    pub fn warning_count(&self) -> usize {
        self.warnings().len()
    }
}

/// The compliance validator engine
#[derive(Debug)]
pub struct ComplianceValidator<'a> {
    /// The framework to validate against
    framework: &'a Framework,
}

impl<'a> ComplianceValidator<'a> {
    /// Create a new validator for a framework
    pub fn new(framework: &'a Framework) -> Self {
        Self { framework }
    }

    /// Validate a project against the compliance framework
    pub fn validate(&self, project: &Project) -> ComplianceResult<ValidationReport> {
        let mut report = ValidationReport::new(
            self.framework.name().to_string(),
            self.framework.version().to_string(),
        );

        report.rules_checked = self.framework.rule_count();
        report.modules_validated = project.modules.len();

        for module in &project.modules {
            self.validate_module(module, &mut report)?;
        }

        Ok(report)
    }

    /// Validate a single module
    fn validate_module(
        &self,
        module: &Module,
        report: &mut ValidationReport,
    ) -> ComplianceResult<()> {
        for (export_name, export) in &module.exports {
            let location_base = format!("{}.{}", module.module, export_name);
            self.validate_export(export, &location_base, report)?;
        }
        Ok(())
    }

    /// Validate an export
    fn validate_export(
        &self,
        export: &Export,
        location: &str,
        report: &mut ValidationReport,
    ) -> ComplianceResult<()> {
        // Validate properties
        if let Some(properties) = &export.properties {
            for (prop_name, property) in properties {
                let prop_location = format!("{location}.{prop_name}");
                self.validate_property(property, &prop_location, report)?;
            }
        }

        // Validate payload (for events)
        if let Some(payload) = &export.payload {
            for (prop_name, property) in payload {
                let prop_location = format!("{location}.payload.{prop_name}");
                self.validate_property(property, &prop_location, report)?;
            }
        }

        // Validate methods
        if let Some(methods) = &export.methods {
            // Collect property annotations for context
            let property_annotations = self.collect_property_annotations(export);

            for (method_name, method) in methods {
                let method_location = format!("{location}.{method_name}");
                self.validate_method(method, &method_location, &property_annotations, report)?;
            }
        }

        Ok(())
    }

    /// Collect all property annotations from an export for validation context
    fn collect_property_annotations(&self, export: &Export) -> HashMap<String, Vec<String>> {
        let mut annotations = HashMap::new();

        if let Some(properties) = &export.properties {
            for (name, prop) in properties {
                if !prop.annotations.is_empty() {
                    annotations.insert(name.clone(), prop.annotations.clone());
                }
            }
        }

        if let Some(payload) = &export.payload {
            for (name, prop) in payload {
                if !prop.annotations.is_empty() {
                    annotations.insert(name.clone(), prop.annotations.clone());
                }
            }
        }

        annotations
    }

    /// Validate a property against storage and annotation rules
    fn validate_property(
        &self,
        property: &Property,
        location: &str,
        report: &mut ValidationReport,
    ) -> ComplianceResult<()> {
        for rule in self.framework.rules() {
            if let ValidationCheckType::StorageCheck = rule.validates.check_type {
                self.check_storage_rule(rule, property, location, report)?;
            }
        }
        Ok(())
    }

    /// Validate a method against effect and access rules
    fn validate_method(
        &self,
        method: &Method,
        location: &str,
        property_annotations: &HashMap<String, Vec<String>>,
        report: &mut ValidationReport,
    ) -> ComplianceResult<()> {
        for rule in self.framework.rules() {
            match rule.validates.check_type {
                ValidationCheckType::EffectCheck => {
                    self.check_effect_rule(rule, method, location, property_annotations, report)?;
                }
                ValidationCheckType::EffectRequirement => {
                    self.check_effect_requirement(rule, method, location, report)?;
                }
                ValidationCheckType::DataAccessCheck => {
                    self.check_data_access_rule(
                        rule,
                        method,
                        location,
                        property_annotations,
                        report,
                    )?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Check storage requirements for properties
    fn check_storage_rule(
        &self,
        rule: &ComplianceRule,
        property: &Property,
        location: &str,
        report: &mut ValidationReport,
    ) -> ComplianceResult<()> {
        let check = &rule.validates;

        // Check if property has any annotations that require specific storage
        if !check.when_accessing.is_empty() {
            let has_sensitive_annotation = property
                .annotations
                .iter()
                .any(|a| check.when_accessing.contains(a));

            if has_sensitive_annotation {
                // Check if required annotations are present
                let has_required = check
                    .required_annotations
                    .iter()
                    .all(|req| property.annotations.contains(req));

                if !has_required {
                    report.violations.push(Violation {
                        rule_id: rule.id.clone(),
                        severity: rule.severity.clone(),
                        description: rule.description.clone(),
                        location: location.to_string(),
                        issue: format!(
                            "Property with {} annotations requires {} annotations",
                            check.when_accessing.join(", "),
                            check.required_annotations.join(", ")
                        ),
                        suggestion: Some(format!(
                            "Add {} to the property annotations",
                            check.required_annotations.join(", ")
                        )),
                    });
                }
            }
        }

        Ok(())
    }

    /// Check effect-based rules (e.g., no PHI in logs)
    fn check_effect_rule(
        &self,
        rule: &ComplianceRule,
        method: &Method,
        location: &str,
        property_annotations: &HashMap<String, Vec<String>>,
        report: &mut ValidationReport,
    ) -> ComplianceResult<()> {
        let check = &rule.validates;

        // Check if method has triggering effects
        let has_trigger_effect = method.effects.iter().any(|e| check.when_effect.contains(e));

        if has_trigger_effect {
            // Check if the class/module has forbidden data annotations
            let has_forbidden_data = property_annotations
                .values()
                .any(|annotations| annotations.iter().any(|a| check.forbidden_data.contains(a)));

            if has_forbidden_data {
                // Check if method has mitigating annotations
                let has_mitigation = check
                    .required_annotations
                    .iter()
                    .all(|req| method.annotations.contains(req));

                if !has_mitigation && !check.required_annotations.is_empty() {
                    report.violations.push(Violation {
                        rule_id: rule.id.clone(),
                        severity: rule.severity.clone(),
                        description: rule.description.clone(),
                        location: location.to_string(),
                        issue: format!(
                            "Method with {} effect accesses data with {} annotations without proper safeguards",
                            check.when_effect.join(", "),
                            check.forbidden_data.join(", ")
                        ),
                        suggestion: rule.examples.as_ref().and_then(|ex| ex.compliant.clone()),
                    });
                } else if check.required_annotations.is_empty() {
                    // If no mitigation possible, it's always a violation
                    report.violations.push(Violation {
                        rule_id: rule.id.clone(),
                        severity: rule.severity.clone(),
                        description: rule.description.clone(),
                        location: location.to_string(),
                        issue: format!(
                            "Method with {} effect must not access data with {} annotations",
                            check.when_effect.join(", "),
                            check.forbidden_data.join(", ")
                        ),
                        suggestion: Some(
                            "Remove sensitive data from the operation or use a different approach"
                                .to_string(),
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    /// Check effect requirements (e.g., PHI access requires audit.log)
    fn check_effect_requirement(
        &self,
        rule: &ComplianceRule,
        method: &Method,
        location: &str,
        report: &mut ValidationReport,
    ) -> ComplianceResult<()> {
        let check = &rule.validates;

        // Check if method has annotations that require certain effects
        let has_trigger_annotation = method
            .annotations
            .iter()
            .any(|a| check.when_accessing.contains(a));

        if has_trigger_annotation {
            let has_required_effects = check
                .required_effects
                .iter()
                .all(|req| method.effects.contains(req));

            if !has_required_effects {
                report.violations.push(Violation {
                    rule_id: rule.id.clone(),
                    severity: rule.severity.clone(),
                    description: rule.description.clone(),
                    location: location.to_string(),
                    issue: format!(
                        "Method with {} annotations requires {} effects",
                        check.when_accessing.join(", "),
                        check.required_effects.join(", ")
                    ),
                    suggestion: Some(format!(
                        "Add {} to the method's effects",
                        check.required_effects.join(", ")
                    )),
                });
            }
        }

        Ok(())
    }

    /// Check data access patterns
    fn check_data_access_rule(
        &self,
        rule: &ComplianceRule,
        method: &Method,
        location: &str,
        property_annotations: &HashMap<String, Vec<String>>,
        report: &mut ValidationReport,
    ) -> ComplianceResult<()> {
        let check = &rule.validates;

        // Check if accessing sensitive data
        let accesses_sensitive = property_annotations
            .values()
            .any(|annotations| annotations.iter().any(|a| check.when_accessing.contains(a)));

        if accesses_sensitive {
            // Check for required method annotations
            let has_required_annotations = check
                .required_annotations
                .iter()
                .all(|req| method.annotations.contains(req));

            if !has_required_annotations {
                report.violations.push(Violation {
                    rule_id: rule.id.clone(),
                    severity: rule.severity.clone(),
                    description: rule.description.clone(),
                    location: location.to_string(),
                    issue: format!(
                        "Accessing data with {} annotations requires {} method annotations",
                        check.when_accessing.join(", "),
                        check.required_annotations.join(", ")
                    ),
                    suggestion: Some(format!(
                        "Add {} to the method annotations",
                        check.required_annotations.join(", ")
                    )),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crucible_core::types::{
        ComplianceFramework, ComplianceRule, Export, ExportType, Language, Manifest, Method,
        Module, Project, ProjectConfig, Property, ReturnType, ValidationCheck, ValidationCheckType,
    };
    use std::collections::HashMap;

    fn create_test_manifest() -> Manifest {
        Manifest {
            version: "0.1.0".to_string(),
            project: ProjectConfig {
                name: "test".to_string(),
                language: Language::TypeScript,
                architecture_pattern: None,
            },
            modules: vec!["test".to_string()],
            strict_validation: true,
            metadata: None,
        }
    }

    fn create_hipaa_framework() -> Framework {
        let definition = ComplianceFramework {
            compliance_framework: "HIPAA".to_string(),
            version: "1.0.0".to_string(),
            description: Some("HIPAA compliance rules".to_string()),
            requirements: vec![],
            rules: vec![
                ComplianceRule {
                    id: "no-phi-in-logs".to_string(),
                    requirement_id: Some("164.312(a)(1)".to_string()),
                    severity: Severity::Error,
                    description: "PHI must not be logged".to_string(),
                    rationale: Some("HIPAA requires PHI protection".to_string()),
                    violation_cost: None,
                    validates: ValidationCheck {
                        check_type: ValidationCheckType::EffectCheck,
                        when_effect: vec!["logging".to_string()],
                        when_accessing: vec![],
                        forbidden_data: vec!["@phi".to_string()],
                        required_annotations: vec![],
                        required_effects: vec![],
                        recommend_fields: None,
                        warn_if_all_fields: None,
                    },
                    examples: None,
                },
                ComplianceRule {
                    id: "phi-requires-audit".to_string(),
                    requirement_id: Some("164.312(b)".to_string()),
                    severity: Severity::Error,
                    description: "PHI access requires audit logging".to_string(),
                    rationale: None,
                    violation_cost: None,
                    validates: ValidationCheck {
                        check_type: ValidationCheckType::EffectRequirement,
                        when_effect: vec![],
                        when_accessing: vec!["@phi-access".to_string()],
                        forbidden_data: vec![],
                        required_annotations: vec![],
                        required_effects: vec!["audit.log".to_string()],
                        recommend_fields: None,
                        warn_if_all_fields: None,
                    },
                    examples: None,
                },
                ComplianceRule {
                    id: "phi-requires-auth".to_string(),
                    requirement_id: None,
                    severity: Severity::Warning,
                    description: "PHI access should require authentication".to_string(),
                    rationale: None,
                    violation_cost: None,
                    validates: ValidationCheck {
                        check_type: ValidationCheckType::DataAccessCheck,
                        when_effect: vec![],
                        when_accessing: vec!["@phi".to_string()],
                        forbidden_data: vec![],
                        required_annotations: vec!["@requires-auth".to_string()],
                        required_effects: vec![],
                        recommend_fields: None,
                        warn_if_all_fields: None,
                    },
                    examples: None,
                },
            ],
        };
        Framework::new(definition)
    }

    #[test]
    fn test_validation_passes_compliant_code() {
        let framework = create_hipaa_framework();
        let validator = ComplianceValidator::new(&framework);

        // Create a compliant module (no logging with PHI)
        let mut properties = HashMap::new();
        properties.insert(
            "name".to_string(),
            Property {
                prop_type: "string".to_string(),
                required: true,
                description: None,
                annotations: vec![], // No PHI
            },
        );

        let mut methods = HashMap::new();
        methods.insert(
            "getName".to_string(),
            Method {
                inputs: vec![],
                returns: ReturnType {
                    return_type: "string".to_string(),
                    inner: None,
                },
                throws: vec![],
                calls: vec![],
                effects: vec!["logging".to_string()],
                is_async: false,
                annotations: vec![],
            },
        );

        let mut exports = HashMap::new();
        exports.insert(
            "User".to_string(),
            Export {
                export_type: ExportType::Class,
                methods: Some(methods),
                properties: Some(properties),
                values: None,
                dependencies: None,
                payload: None,
            },
        );

        let module = Module {
            module: "user".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: None,
            exports,
            dependencies: HashMap::new(),
        };

        let project = Project {
            manifest: create_test_manifest(),
            modules: vec![module],
            rules: None,
        };

        let report = validator.validate(&project).unwrap();
        assert!(report.passed());
        assert_eq!(report.error_count(), 0);
    }

    #[test]
    fn test_validation_detects_phi_in_logs() {
        let framework = create_hipaa_framework();
        let validator = ComplianceValidator::new(&framework);

        // Create a non-compliant module (logging with PHI)
        let mut properties = HashMap::new();
        properties.insert(
            "ssn".to_string(),
            Property {
                prop_type: "string".to_string(),
                required: true,
                description: None,
                annotations: vec!["@phi".to_string()], // PHI data
            },
        );

        let mut methods = HashMap::new();
        methods.insert(
            "logPatient".to_string(),
            Method {
                inputs: vec![],
                returns: ReturnType {
                    return_type: "void".to_string(),
                    inner: None,
                },
                throws: vec![],
                calls: vec![],
                effects: vec!["logging".to_string()], // Logging effect
                is_async: false,
                annotations: vec![],
            },
        );

        let mut exports = HashMap::new();
        exports.insert(
            "PatientService".to_string(),
            Export {
                export_type: ExportType::Class,
                methods: Some(methods),
                properties: Some(properties),
                values: None,
                dependencies: None,
                payload: None,
            },
        );

        let module = Module {
            module: "patient".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: None,
            exports,
            dependencies: HashMap::new(),
        };

        let project = Project {
            manifest: create_test_manifest(),
            modules: vec![module],
            rules: None,
        };

        let report = validator.validate(&project).unwrap();
        assert!(!report.passed());
        assert_eq!(report.error_count(), 1);
        assert_eq!(report.violations[0].rule_id, "no-phi-in-logs");
    }

    #[test]
    fn test_validation_detects_missing_audit_log() {
        let framework = create_hipaa_framework();
        let validator = ComplianceValidator::new(&framework);

        let mut methods = HashMap::new();
        methods.insert(
            "getPatientData".to_string(),
            Method {
                inputs: vec![],
                returns: ReturnType {
                    return_type: "PatientData".to_string(),
                    inner: None,
                },
                throws: vec![],
                calls: vec![],
                effects: vec![], // Missing audit.log
                is_async: false,
                annotations: vec!["@phi-access".to_string()], // PHI access annotation
            },
        );

        let mut exports = HashMap::new();
        exports.insert(
            "PatientRepository".to_string(),
            Export {
                export_type: ExportType::Class,
                methods: Some(methods),
                properties: None,
                values: None,
                dependencies: None,
                payload: None,
            },
        );

        let module = Module {
            module: "patient".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: None,
            exports,
            dependencies: HashMap::new(),
        };

        let project = Project {
            manifest: create_test_manifest(),
            modules: vec![module],
            rules: None,
        };

        let report = validator.validate(&project).unwrap();
        assert!(!report.passed());
        assert!(report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-requires-audit"));
    }

    #[test]
    fn test_report_statistics() {
        let framework = create_hipaa_framework();
        let validator = ComplianceValidator::new(&framework);

        let project = Project {
            manifest: create_test_manifest(),
            modules: vec![],
            rules: None,
        };

        let report = validator.validate(&project).unwrap();
        assert_eq!(report.framework_name, "HIPAA");
        assert_eq!(report.framework_version, "1.0.0");
        assert_eq!(report.rules_checked, 3);
        assert!(report.passed());
    }
}
