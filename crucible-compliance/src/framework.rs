//! Compliance framework wrapper with validation logic

use crucible_core::types::{ComplianceFramework, ComplianceRule, Severity};
use std::collections::HashMap;

/// A loaded and validated compliance framework
#[derive(Debug, Clone)]
pub struct Framework {
    /// The underlying framework definition
    pub definition: ComplianceFramework,
    /// Rules indexed by ID for fast lookup
    rules_by_id: HashMap<String, usize>,
    /// Rules grouped by severity
    rules_by_severity: HashMap<Severity, Vec<usize>>,
}

impl Framework {
    /// Create a new Framework from a ComplianceFramework definition
    pub fn new(definition: ComplianceFramework) -> Self {
        let mut rules_by_id = HashMap::new();
        let mut rules_by_severity: HashMap<Severity, Vec<usize>> = HashMap::new();

        for (idx, rule) in definition.rules.iter().enumerate() {
            rules_by_id.insert(rule.id.clone(), idx);
            rules_by_severity
                .entry(rule.severity.clone())
                .or_default()
                .push(idx);
        }

        Self {
            definition,
            rules_by_id,
            rules_by_severity,
        }
    }

    /// Get the framework name
    pub fn name(&self) -> &str {
        &self.definition.compliance_framework
    }

    /// Get the framework version
    pub fn version(&self) -> &str {
        &self.definition.version
    }

    /// Get a rule by ID
    pub fn get_rule(&self, rule_id: &str) -> Option<&ComplianceRule> {
        self.rules_by_id
            .get(rule_id)
            .map(|&idx| &self.definition.rules[idx])
    }

    /// Get all rules
    pub fn rules(&self) -> &[ComplianceRule] {
        &self.definition.rules
    }

    /// Get rules by severity
    pub fn rules_by_severity(&self, severity: &Severity) -> Vec<&ComplianceRule> {
        self.rules_by_severity
            .get(severity)
            .map(|indices| {
                indices
                    .iter()
                    .map(|&idx| &self.definition.rules[idx])
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get error-level rules
    pub fn error_rules(&self) -> Vec<&ComplianceRule> {
        self.rules_by_severity(&Severity::Error)
    }

    /// Get warning-level rules
    pub fn warning_rules(&self) -> Vec<&ComplianceRule> {
        self.rules_by_severity(&Severity::Warning)
    }

    /// Get the total number of rules
    pub fn rule_count(&self) -> usize {
        self.definition.rules.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crucible_core::types::{ValidationCheck, ValidationCheckType};

    fn create_test_framework() -> ComplianceFramework {
        ComplianceFramework {
            compliance_framework: "TestFramework".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test compliance framework".to_string()),
            requirements: vec![],
            rules: vec![
                ComplianceRule {
                    id: "error-rule".to_string(),
                    requirement_id: None,
                    severity: Severity::Error,
                    description: "Error level rule".to_string(),
                    rationale: None,
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
                    id: "warning-rule".to_string(),
                    requirement_id: None,
                    severity: Severity::Warning,
                    description: "Warning level rule".to_string(),
                    rationale: None,
                    violation_cost: None,
                    validates: ValidationCheck {
                        check_type: ValidationCheckType::StorageCheck,
                        when_effect: vec![],
                        when_accessing: vec![],
                        forbidden_data: vec![],
                        required_annotations: vec!["@encrypted".to_string()],
                        required_effects: vec![],
                        recommend_fields: None,
                        warn_if_all_fields: None,
                    },
                    examples: None,
                },
            ],
        }
    }

    #[test]
    fn test_framework_creation() {
        let def = create_test_framework();
        let framework = Framework::new(def);

        assert_eq!(framework.name(), "TestFramework");
        assert_eq!(framework.version(), "1.0.0");
        assert_eq!(framework.rule_count(), 2);
    }

    #[test]
    fn test_get_rule_by_id() {
        let def = create_test_framework();
        let framework = Framework::new(def);

        let rule = framework.get_rule("error-rule");
        assert!(rule.is_some());
        assert_eq!(rule.unwrap().description, "Error level rule");

        let missing = framework.get_rule("nonexistent");
        assert!(missing.is_none());
    }

    #[test]
    fn test_rules_by_severity() {
        let def = create_test_framework();
        let framework = Framework::new(def);

        let errors = framework.error_rules();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].id, "error-rule");

        let warnings = framework.warning_rules();
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].id, "warning-rule");
    }
}
