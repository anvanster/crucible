//! Core type definitions matching the Crucible specification

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub version: String,
    pub project: ProjectConfig,
    pub modules: Vec<String>,
    #[serde(default = "default_strict")]
    pub strict_validation: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub language: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_pattern: Option<ArchitecturePattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    TypeScript,
    Rust,
    Python,
    Go,
    Java,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArchitecturePattern {
    Layered,
    Hexagonal,
    Microservices,
    Modular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub module: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub exports: HashMap<String, Export>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    #[serde(rename = "type")]
    pub export_type: ExportType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub methods: Option<HashMap<String, Method>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Property>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<Dependency>>,
    /// Payload for event types - defines the data carried by the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<HashMap<String, Property>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExportType {
    Class,
    Function,
    Interface,
    Type,
    Enum,
    /// Domain events with typed payloads
    Event,
    /// Rust-style traits with async method support
    Trait,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub inputs: Vec<Parameter>,
    pub returns: ReturnType,
    #[serde(default)]
    pub throws: Vec<String>,
    #[serde(default)]
    pub calls: Vec<String>,
    #[serde(default)]
    pub effects: Vec<String>,
    /// Whether the method is async (for Rust traits, TypeScript async functions, etc.)
    #[serde(default, rename = "async")]
    pub is_async: bool,
    /// Compliance and metadata annotations (e.g., @requires-auth, @audit-log, @rate-limited)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    #[serde(default)]
    pub optional: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnType {
    #[serde(rename = "type")]
    pub return_type: String,
    #[serde(skip_serializing_if = "Option::is_none", alias = "items")]
    pub inner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    pub prop_type: String,
    #[serde(default = "default_required")]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Compliance and metadata annotations (e.g., @phi, @pii, @encrypted)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub module: String,
    pub imports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

/// A complete Crucible project
#[derive(Debug, Clone)]
pub struct Project {
    pub manifest: Manifest,
    pub modules: Vec<Module>,
    pub rules: Option<Rules>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Architecture>,
    pub rules: Vec<Rule>,
    #[serde(default)]
    pub custom_rules: Vec<CustomRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Architecture {
    pub pattern: ArchitecturePattern,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub can_depend_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub enabled: bool,
    pub severity: Severity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRule {
    pub id: String,
    #[serde(rename = "type")]
    pub rule_type: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    pub severity: Severity,
}

// ============================================================================
// Compliance Framework Types
// ============================================================================

/// A compliance framework definition (e.g., HIPAA, PCI-DSS, SOC2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFramework {
    pub compliance_framework: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub requirements: Vec<ComplianceRequirement>,
    pub rules: Vec<ComplianceRule>,
}

/// A compliance requirement reference (e.g., HIPAA 164.312(a)(1))
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    pub id: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
}

/// A compliance rule that validates architecture against requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRule {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement_id: Option<String>,
    pub severity: Severity,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_cost: Option<String>,
    pub validates: ValidationCheck,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<ComplianceExamples>,
}

/// Examples of compliant and non-compliant code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceExamples {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant: Option<String>,
}

/// A validation check that defines how to validate compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCheck {
    #[serde(rename = "type")]
    pub check_type: ValidationCheckType,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub when_effect: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub when_accessing: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forbidden_data: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_annotations: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_effects: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommend_fields: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warn_if_all_fields: Option<bool>,
}

/// Types of validation checks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationCheckType {
    /// Check that effects don't expose sensitive data
    EffectCheck,
    /// Check storage requirements for data types
    StorageCheck,
    /// Require specific effects for certain operations
    EffectRequirement,
    /// Check data access patterns
    DataAccessCheck,
}

fn default_strict() -> bool {
    true
}

fn default_required() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_serialization() {
        let manifest = Manifest {
            version: "0.1.0".to_string(),
            project: ProjectConfig {
                name: "test-project".to_string(),
                language: Language::Rust,
                architecture_pattern: Some(ArchitecturePattern::Layered),
            },
            modules: vec!["module1".to_string(), "module2".to_string()],
            strict_validation: true,
            metadata: Some(Metadata {
                author: Some("Test Author".to_string()),
                repository: Some("https://github.com/test/repo".to_string()),
                created: Some("2025-01-01T00:00:00Z".to_string()),
            }),
        };

        let json = serde_json::to_string(&manifest).unwrap();
        let deserialized: Manifest = serde_json::from_str(&json).unwrap();

        assert_eq!(manifest.version, deserialized.version);
        assert_eq!(manifest.project.name, deserialized.project.name);
        assert_eq!(manifest.modules.len(), deserialized.modules.len());
    }

    #[test]
    fn test_manifest_default_strict_validation() {
        let json = r#"{
            "version": "0.1.0",
            "project": {"name": "test", "language": "rust"},
            "modules": []
        }"#;

        let manifest: Manifest = serde_json::from_str(json).unwrap();
        assert!(manifest.strict_validation);
    }

    #[test]
    fn test_language_serialization() {
        assert_eq!(
            serde_json::to_string(&Language::TypeScript).unwrap(),
            r#""typescript""#
        );
        assert_eq!(serde_json::to_string(&Language::Rust).unwrap(), r#""rust""#);
        assert_eq!(
            serde_json::to_string(&Language::Python).unwrap(),
            r#""python""#
        );
        assert_eq!(serde_json::to_string(&Language::Go).unwrap(), r#""go""#);
        assert_eq!(serde_json::to_string(&Language::Java).unwrap(), r#""java""#);
    }

    #[test]
    fn test_architecture_pattern_serialization() {
        assert_eq!(
            serde_json::to_string(&ArchitecturePattern::Layered).unwrap(),
            r#""layered""#
        );
        assert_eq!(
            serde_json::to_string(&ArchitecturePattern::Hexagonal).unwrap(),
            r#""hexagonal""#
        );
        assert_eq!(
            serde_json::to_string(&ArchitecturePattern::Microservices).unwrap(),
            r#""microservices""#
        );
        assert_eq!(
            serde_json::to_string(&ArchitecturePattern::Modular).unwrap(),
            r#""modular""#
        );
    }

    #[test]
    fn test_module_serialization() {
        let mut exports = HashMap::new();
        exports.insert(
            "TestClass".to_string(),
            Export {
                export_type: ExportType::Class,
                methods: None,
                properties: None,
                values: None,
                dependencies: None,
                payload: None,
            },
        );

        let mut dependencies = HashMap::new();
        dependencies.insert("other-module".to_string(), "^1.0.0".to_string());

        let module = Module {
            module: "test-module".to_string(),
            version: "1.0.0".to_string(),
            layer: Some("core".to_string()),
            description: Some("Test module".to_string()),
            exports,
            dependencies,
        };

        let json = serde_json::to_string(&module).unwrap();
        let deserialized: Module = serde_json::from_str(&json).unwrap();

        assert_eq!(module.module, deserialized.module);
        assert_eq!(module.version, deserialized.version);
        assert_eq!(module.layer, deserialized.layer);
    }

    #[test]
    fn test_export_type_serialization() {
        assert_eq!(
            serde_json::to_string(&ExportType::Class).unwrap(),
            r#""class""#
        );
        assert_eq!(
            serde_json::to_string(&ExportType::Interface).unwrap(),
            r#""interface""#
        );
        assert_eq!(
            serde_json::to_string(&ExportType::Function).unwrap(),
            r#""function""#
        );
        assert_eq!(
            serde_json::to_string(&ExportType::Type).unwrap(),
            r#""type""#
        );
        assert_eq!(
            serde_json::to_string(&ExportType::Enum).unwrap(),
            r#""enum""#
        );
    }

    #[test]
    fn test_method_with_defaults() {
        let json = r#"{
            "inputs": [],
            "returns": {"type": "void"}
        }"#;

        let method: Method = serde_json::from_str(json).unwrap();
        assert!(method.throws.is_empty());
        assert!(method.calls.is_empty());
        assert!(method.effects.is_empty());
    }

    #[test]
    fn test_parameter_optional_default() {
        let json = r#"{
            "name": "param1",
            "type": "string"
        }"#;

        let param: Parameter = serde_json::from_str(json).unwrap();
        assert!(!param.optional);
    }

    #[test]
    fn test_property_required_default() {
        let json = r#"{
            "type": "string"
        }"#;

        let prop: Property = serde_json::from_str(json).unwrap();
        assert!(prop.required);
    }

    #[test]
    fn test_severity_serialization() {
        assert_eq!(
            serde_json::to_string(&Severity::Error).unwrap(),
            r#""error""#
        );
        assert_eq!(
            serde_json::to_string(&Severity::Warning).unwrap(),
            r#""warning""#
        );
        assert_eq!(serde_json::to_string(&Severity::Info).unwrap(), r#""info""#);
    }

    #[test]
    fn test_rules_deserialization() {
        let json = r#"{
            "rules": [
                {
                    "id": "test-rule",
                    "enabled": true,
                    "severity": "error",
                    "description": "Test rule"
                }
            ]
        }"#;

        let rules: Rules = serde_json::from_str(json).unwrap();
        assert_eq!(rules.rules.len(), 1);
        assert_eq!(rules.rules[0].id, "test-rule");
        assert!(rules.custom_rules.is_empty());
    }

    #[test]
    fn test_layer_deserialization() {
        let json = r#"{
            "name": "core",
            "can_depend_on": ["utils"]
        }"#;

        let layer: Layer = serde_json::from_str(json).unwrap();
        assert_eq!(layer.name, "core");
        assert_eq!(layer.can_depend_on.len(), 1);
        assert_eq!(layer.can_depend_on[0], "utils");
    }

    #[test]
    fn test_complete_export_with_methods() {
        let json = r#"{
            "type": "class",
            "methods": {
                "testMethod": {
                    "inputs": [{"name": "arg", "type": "string"}],
                    "returns": {"type": "boolean"},
                    "throws": ["Error"],
                    "calls": ["someFunction"],
                    "effects": ["io.write"]
                }
            }
        }"#;

        let export: Export = serde_json::from_str(json).unwrap();
        assert!(matches!(export.export_type, ExportType::Class));
        assert!(export.methods.is_some());
        let methods = export.methods.unwrap();
        assert!(methods.contains_key("testMethod"));
    }

    #[test]
    fn test_module_without_layer() {
        let json = r#"{
            "module": "test",
            "version": "1.0.0",
            "exports": {}
        }"#;

        let module: Module = serde_json::from_str(json).unwrap();
        assert!(module.layer.is_none());
        assert!(module.dependencies.is_empty());
    }

    #[test]
    fn test_event_export_type_serialization() {
        assert_eq!(
            serde_json::to_string(&ExportType::Event).unwrap(),
            r#""event""#
        );
        assert_eq!(
            serde_json::to_string(&ExportType::Trait).unwrap(),
            r#""trait""#
        );
    }

    #[test]
    fn test_event_with_payload() {
        let json = r#"{
            "type": "event",
            "payload": {
                "imageId": {"type": "ImageId", "required": true},
                "timestamp": {"type": "DateTime", "required": false}
            }
        }"#;

        let export: Export = serde_json::from_str(json).unwrap();
        assert!(matches!(export.export_type, ExportType::Event));
        assert!(export.payload.is_some());
        let payload = export.payload.unwrap();
        assert!(payload.contains_key("imageId"));
        assert!(payload.contains_key("timestamp"));
        assert!(payload.get("imageId").unwrap().required);
        assert!(!payload.get("timestamp").unwrap().required);
    }

    #[test]
    fn test_trait_with_async_methods() {
        let json = r#"{
            "type": "trait",
            "methods": {
                "plan": {
                    "inputs": [{"name": "request", "type": "BuildRequest"}],
                    "returns": {"type": "Result<BuildPlan, OrchestratorError>"},
                    "async": true
                },
                "validate": {
                    "inputs": [],
                    "returns": {"type": "bool"},
                    "async": false
                }
            }
        }"#;

        let export: Export = serde_json::from_str(json).unwrap();
        assert!(matches!(export.export_type, ExportType::Trait));
        assert!(export.methods.is_some());
        let methods = export.methods.unwrap();
        assert!(methods.get("plan").unwrap().is_async);
        assert!(!methods.get("validate").unwrap().is_async);
    }

    #[test]
    fn test_method_async_default() {
        let json = r#"{
            "inputs": [],
            "returns": {"type": "void"}
        }"#;

        let method: Method = serde_json::from_str(json).unwrap();
        assert!(!method.is_async);
    }

    #[test]
    fn test_property_with_annotations() {
        let json = r#"{
            "type": "string",
            "annotations": ["@phi", "@encrypted"]
        }"#;

        let prop: Property = serde_json::from_str(json).unwrap();
        assert_eq!(prop.annotations.len(), 2);
        assert!(prop.annotations.contains(&"@phi".to_string()));
        assert!(prop.annotations.contains(&"@encrypted".to_string()));
    }

    #[test]
    fn test_method_with_annotations() {
        let json = r#"{
            "inputs": [],
            "returns": {"type": "void"},
            "annotations": ["@requires-auth", "@audit-log"]
        }"#;

        let method: Method = serde_json::from_str(json).unwrap();
        assert_eq!(method.annotations.len(), 2);
        assert!(method.annotations.contains(&"@requires-auth".to_string()));
        assert!(method.annotations.contains(&"@audit-log".to_string()));
    }

    #[test]
    fn test_compliance_framework_deserialization() {
        let json = r#"{
            "compliance_framework": "HIPAA",
            "version": "1.0.0",
            "description": "HIPAA compliance rules",
            "requirements": [
                {
                    "id": "164.312(a)(1)",
                    "category": "Access Control"
                }
            ],
            "rules": [
                {
                    "id": "no-phi-in-logs",
                    "requirement_id": "164.312(a)(1)",
                    "severity": "error",
                    "description": "PHI must not be logged",
                    "validates": {
                        "type": "effect_check",
                        "when_effect": ["logging"],
                        "forbidden_data": ["@phi"]
                    }
                }
            ]
        }"#;

        let framework: ComplianceFramework = serde_json::from_str(json).unwrap();
        assert_eq!(framework.compliance_framework, "HIPAA");
        assert_eq!(framework.version, "1.0.0");
        assert_eq!(framework.requirements.len(), 1);
        assert_eq!(framework.rules.len(), 1);
        assert_eq!(framework.rules[0].id, "no-phi-in-logs");
        assert_eq!(framework.rules[0].severity, Severity::Error);
    }

    #[test]
    fn test_validation_check_types() {
        assert_eq!(
            serde_json::to_string(&ValidationCheckType::EffectCheck).unwrap(),
            r#""effect_check""#
        );
        assert_eq!(
            serde_json::to_string(&ValidationCheckType::StorageCheck).unwrap(),
            r#""storage_check""#
        );
        assert_eq!(
            serde_json::to_string(&ValidationCheckType::EffectRequirement).unwrap(),
            r#""effect_requirement""#
        );
        assert_eq!(
            serde_json::to_string(&ValidationCheckType::DataAccessCheck).unwrap(),
            r#""data_access_check""#
        );
    }

    #[test]
    fn test_compliance_rule_with_examples() {
        let json = r#"{
            "id": "test-rule",
            "severity": "warning",
            "description": "Test rule",
            "validates": {
                "type": "storage_check",
                "required_annotations": ["@encrypted"]
            },
            "examples": {
                "violation": "unencrypted PHI storage",
                "compliant": "encrypted PHI storage"
            }
        }"#;

        let rule: ComplianceRule = serde_json::from_str(json).unwrap();
        assert!(rule.examples.is_some());
        let examples = rule.examples.unwrap();
        assert_eq!(examples.violation.unwrap(), "unencrypted PHI storage");
        assert_eq!(examples.compliant.unwrap(), "encrypted PHI storage");
    }
}
