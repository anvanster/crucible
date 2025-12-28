//! Integration tests using a sample healthcare project architecture

use crucible_compliance::{
    ComplianceValidator, FrameworkLoader, OutputFormat, ReportConfig, Reporter,
};
use crucible_core::types::{
    ArchitecturePattern, Export, ExportType, Language, Manifest, Method, Module, Project,
    ProjectConfig, Property, ReturnType,
};
use std::collections::HashMap;
use std::path::PathBuf;

fn create_healthcare_manifest() -> Manifest {
    Manifest {
        version: "0.1.0".to_string(),
        project: ProjectConfig {
            name: "healthcare-app".to_string(),
            language: Language::TypeScript,
            architecture_pattern: Some(ArchitecturePattern::Layered),
        },
        modules: vec![
            "patient".to_string(),
            "medical-records".to_string(),
            "billing".to_string(),
            "auth".to_string(),
        ],
        strict_validation: true,
        metadata: None,
    }
}

fn get_frameworks_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frameworks")
}

// ============================================================
// HIPAA Compliance Tests
// ============================================================

#[test]
fn test_hipaa_compliant_patient_service() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();

    let hipaa = loader.get("HIPAA").expect("HIPAA framework not found");
    let validator = ComplianceValidator::new(hipaa);

    // Create a fully compliant patient service with all required annotations
    let mut properties = HashMap::new();
    properties.insert(
        "patientId".to_string(),
        Property {
            prop_type: "string".to_string(),
            required: true,
            description: Some("Anonymized patient identifier".to_string()),
            annotations: vec![], // No PHI annotation on ID
        },
    );
    properties.insert(
        "ssn".to_string(),
        Property {
            prop_type: "string".to_string(),
            required: true,
            description: Some("Social Security Number".to_string()),
            annotations: vec![
                "@phi".to_string(),
                "@encrypted".to_string(),         // Storage encryption
                "@encrypted-at-rest".to_string(), // Explicit at-rest encryption
                "@field-encrypted".to_string(),   // Database field encryption
            ],
        },
    );

    let mut methods = HashMap::new();
    methods.insert(
        "getPatient".to_string(),
        Method {
            inputs: vec![],
            returns: ReturnType {
                return_type: "Patient".to_string(),
                inner: None,
            },
            throws: vec![],
            calls: vec![],
            effects: vec!["audit.log".to_string()], // Audit logging
            is_async: true,
            annotations: vec![
                "@phi-access".to_string(),
                "@requires-auth".to_string(), // Authentication required
                "@requires-role".to_string(), // Authorization required
                "@https-only".to_string(),    // Transport encryption
                "@session-timeout".to_string(), // Auto logoff
            ],
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
        layer: Some("service".to_string()),
        description: Some("Patient management service".to_string()),
        exports,
        dependencies: HashMap::new(),
    };

    let project = Project {
        manifest: create_healthcare_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();

    assert!(
        report.passed(),
        "Compliant patient service should pass HIPAA validation. Violations: {:?}",
        report.violations
    );
    assert_eq!(report.error_count(), 0);
}

#[test]
fn test_hipaa_violation_phi_in_logs() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();

    let hipaa = loader.get("HIPAA").expect("HIPAA framework not found");
    let validator = ComplianceValidator::new(hipaa);

    // Create a non-compliant service that logs PHI
    let mut properties = HashMap::new();
    properties.insert(
        "ssn".to_string(),
        Property {
            prop_type: "string".to_string(),
            required: true,
            description: None,
            annotations: vec!["@phi".to_string()], // Has PHI
        },
    );

    let mut methods = HashMap::new();
    methods.insert(
        "logPatientInfo".to_string(),
        Method {
            inputs: vec![],
            returns: ReturnType {
                return_type: "void".to_string(),
                inner: None,
            },
            throws: vec![],
            calls: vec![],
            effects: vec!["logging".to_string()], // Logging effect with PHI = violation
            is_async: false,
            annotations: vec![],
        },
    );

    let mut exports = HashMap::new();
    exports.insert(
        "PatientLogger".to_string(),
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
        manifest: create_healthcare_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();

    assert!(
        !report.passed(),
        "Service logging PHI should fail HIPAA validation"
    );
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "no-phi-in-logs"),
        "Should detect no-phi-in-logs violation"
    );
}

#[test]
fn test_hipaa_violation_missing_audit_log() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();

    let hipaa = loader.get("HIPAA").expect("HIPAA framework not found");
    let validator = ComplianceValidator::new(hipaa);

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
            effects: vec![], // Missing audit.log effect
            is_async: true,
            annotations: vec!["@phi-access".to_string()], // PHI access without audit
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
        module: "medical-records".to_string(),
        version: "1.0.0".to_string(),
        layer: None,
        description: None,
        exports,
        dependencies: HashMap::new(),
    };

    let project = Project {
        manifest: create_healthcare_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();

    assert!(
        !report.passed(),
        "PHI access without audit should fail HIPAA validation"
    );
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-access-requires-audit"),
        "Should detect phi-access-requires-audit violation"
    );
}

// ============================================================
// PCI-DSS Compliance Tests
// ============================================================

#[test]
fn test_pci_compliant_payment_service() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();

    let pci = loader.get("PCI-DSS").expect("PCI-DSS framework not found");
    let validator = ComplianceValidator::new(pci);

    // Create a compliant payment service
    let mut properties = HashMap::new();
    properties.insert(
        "cardNumber".to_string(),
        Property {
            prop_type: "string".to_string(),
            required: true,
            description: Some("Encrypted card number".to_string()),
            annotations: vec![
                "@pan".to_string(),
                "@encrypted".to_string(), // Compliant: encrypted
            ],
        },
    );

    let mut methods = HashMap::new();
    methods.insert(
        "processPayment".to_string(),
        Method {
            inputs: vec![],
            returns: ReturnType {
                return_type: "PaymentResult".to_string(),
                inner: None,
            },
            throws: vec![],
            calls: vec![],
            effects: vec!["audit.log".to_string()],
            is_async: true,
            annotations: vec![
                "@cardholder-data-access".to_string(),
                "@requires-auth".to_string(),
            ],
        },
    );

    let mut exports = HashMap::new();
    exports.insert(
        "PaymentService".to_string(),
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
        module: "billing".to_string(),
        version: "1.0.0".to_string(),
        layer: Some("service".to_string()),
        description: None,
        exports,
        dependencies: HashMap::new(),
    };

    let project = Project {
        manifest: create_healthcare_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();

    assert!(
        report.passed(),
        "Compliant payment service should pass PCI-DSS. Violations: {:?}",
        report.violations
    );
}

#[test]
fn test_pci_violation_pan_in_logs() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();

    let pci = loader.get("PCI-DSS").expect("PCI-DSS framework not found");
    let validator = ComplianceValidator::new(pci);

    let mut properties = HashMap::new();
    properties.insert(
        "cardNumber".to_string(),
        Property {
            prop_type: "string".to_string(),
            required: true,
            description: None,
            annotations: vec!["@pan".to_string()],
        },
    );

    let mut methods = HashMap::new();
    methods.insert(
        "logTransaction".to_string(),
        Method {
            inputs: vec![],
            returns: ReturnType {
                return_type: "void".to_string(),
                inner: None,
            },
            throws: vec![],
            calls: vec![],
            effects: vec!["logging".to_string()], // Logging PAN = violation
            is_async: false,
            annotations: vec![],
        },
    );

    let mut exports = HashMap::new();
    exports.insert(
        "TransactionLogger".to_string(),
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
        module: "billing".to_string(),
        version: "1.0.0".to_string(),
        layer: None,
        description: None,
        exports,
        dependencies: HashMap::new(),
    };

    let project = Project {
        manifest: create_healthcare_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();

    assert!(
        !report.passed(),
        "Logging PAN should fail PCI-DSS validation"
    );
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "no-pan-in-logs"),
        "Should detect no-pan-in-logs violation"
    );
}

// ============================================================
// SOC2 Compliance Tests
// ============================================================

#[test]
fn test_soc2_compliant_user_service() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();

    let soc2 = loader.get("SOC2").expect("SOC2 framework not found");
    let validator = ComplianceValidator::new(soc2);

    // Create a compliant user service
    let mut properties = HashMap::new();
    properties.insert(
        "email".to_string(),
        Property {
            prop_type: "string".to_string(),
            required: true,
            description: Some("User email".to_string()),
            annotations: vec![
                "@pii".to_string(),
                "@encrypted".to_string(), // Compliant: encrypted
            ],
        },
    );

    let mut methods = HashMap::new();
    methods.insert(
        "getUser".to_string(),
        Method {
            inputs: vec![],
            returns: ReturnType {
                return_type: "User".to_string(),
                inner: None,
            },
            throws: vec![],
            calls: vec![],
            effects: vec!["audit.log".to_string()],
            is_async: true,
            annotations: vec![
                "@sensitive-data-access".to_string(),
                "@requires-auth".to_string(),
            ],
        },
    );

    let mut exports = HashMap::new();
    exports.insert(
        "UserService".to_string(),
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
        module: "auth".to_string(),
        version: "1.0.0".to_string(),
        layer: Some("service".to_string()),
        description: None,
        exports,
        dependencies: HashMap::new(),
    };

    let project = Project {
        manifest: create_healthcare_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();

    assert!(
        report.passed(),
        "Compliant user service should pass SOC2. Violations: {:?}",
        report.violations
    );
}

#[test]
fn test_soc2_violation_pii_in_logs() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();

    let soc2 = loader.get("SOC2").expect("SOC2 framework not found");
    let validator = ComplianceValidator::new(soc2);

    let mut properties = HashMap::new();
    properties.insert(
        "email".to_string(),
        Property {
            prop_type: "string".to_string(),
            required: true,
            description: None,
            annotations: vec!["@pii".to_string()],
        },
    );

    let mut methods = HashMap::new();
    methods.insert(
        "logUserActivity".to_string(),
        Method {
            inputs: vec![],
            returns: ReturnType {
                return_type: "void".to_string(),
                inner: None,
            },
            throws: vec![],
            calls: vec![],
            effects: vec!["logging".to_string()], // Logging PII = violation
            is_async: false,
            annotations: vec![],
        },
    );

    let mut exports = HashMap::new();
    exports.insert(
        "ActivityLogger".to_string(),
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
        module: "auth".to_string(),
        version: "1.0.0".to_string(),
        layer: None,
        description: None,
        exports,
        dependencies: HashMap::new(),
    };

    let project = Project {
        manifest: create_healthcare_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();

    assert!(!report.passed(), "Logging PII should fail SOC2 validation");
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "no-pii-in-logs"),
        "Should detect no-pii-in-logs violation"
    );
}

// ============================================================
// Multi-Framework Validation Tests
// ============================================================

#[test]
fn test_multi_framework_healthcare_project() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();

    // Healthcare projects often need HIPAA + SOC2 compliance
    let hipaa = loader.get("HIPAA").expect("HIPAA framework not found");
    let soc2 = loader.get("SOC2").expect("SOC2 framework not found");

    // Create a fully compliant module that passes both frameworks
    let mut properties = HashMap::new();
    properties.insert(
        "patientData".to_string(),
        Property {
            prop_type: "object".to_string(),
            required: true,
            description: None,
            annotations: vec![
                "@phi".to_string(),
                "@pii".to_string(),
                "@encrypted".to_string(),
                "@encrypted-at-rest".to_string(), // HIPAA storage encryption
                "@field-encrypted".to_string(),   // HIPAA database encryption
            ],
        },
    );

    let mut methods = HashMap::new();
    methods.insert(
        "getPatientRecord".to_string(),
        Method {
            inputs: vec![],
            returns: ReturnType {
                return_type: "PatientRecord".to_string(),
                inner: None,
            },
            throws: vec![],
            calls: vec![],
            effects: vec!["audit.log".to_string()],
            is_async: true,
            annotations: vec![
                "@phi-access".to_string(),
                "@sensitive-data-access".to_string(),
                "@requires-auth".to_string(),
                "@requires-role".to_string(),   // HIPAA authorization
                "@https-only".to_string(),      // HIPAA transport encryption
                "@session-timeout".to_string(), // HIPAA auto logoff
            ],
        },
    );

    let mut exports = HashMap::new();
    exports.insert(
        "HealthRecordService".to_string(),
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
        module: "medical-records".to_string(),
        version: "1.0.0".to_string(),
        layer: Some("service".to_string()),
        description: None,
        exports,
        dependencies: HashMap::new(),
    };

    let project = Project {
        manifest: create_healthcare_manifest(),
        modules: vec![module],
        rules: None,
    };

    // Validate against both frameworks
    let hipaa_validator = ComplianceValidator::new(hipaa);
    let hipaa_report = hipaa_validator.validate(&project).unwrap();

    let soc2_validator = ComplianceValidator::new(soc2);
    let soc2_report = soc2_validator.validate(&project).unwrap();

    assert!(
        hipaa_report.passed(),
        "Should pass HIPAA. Violations: {:?}",
        hipaa_report.violations
    );
    assert!(
        soc2_report.passed(),
        "Should pass SOC2. Violations: {:?}",
        soc2_report.violations
    );
}

// ============================================================
// Reporter Tests
// ============================================================

#[test]
fn test_reporter_formats() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();

    let hipaa = loader.get("HIPAA").expect("HIPAA framework not found");
    let validator = ComplianceValidator::new(hipaa);

    // Create a module with a violation to have something to report
    let mut properties = HashMap::new();
    properties.insert(
        "ssn".to_string(),
        Property {
            prop_type: "string".to_string(),
            required: true,
            description: None,
            annotations: vec!["@phi".to_string()],
        },
    );

    let mut methods = HashMap::new();
    methods.insert(
        "logData".to_string(),
        Method {
            inputs: vec![],
            returns: ReturnType {
                return_type: "void".to_string(),
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
        "TestService".to_string(),
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
        module: "test".to_string(),
        version: "1.0.0".to_string(),
        layer: None,
        description: None,
        exports,
        dependencies: HashMap::new(),
    };

    let project = Project {
        manifest: create_healthcare_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();

    // Test all output formats
    let text_reporter = Reporter::new(ReportConfig::new(OutputFormat::Text).with_color(false));
    let text_output = text_reporter.format(&report);
    assert!(text_output.contains("HIPAA"));
    assert!(text_output.contains("no-phi-in-logs"));

    let json_reporter = Reporter::new(ReportConfig::new(OutputFormat::Json));
    let json_output = json_reporter.format(&report);
    let parsed: serde_json::Value = serde_json::from_str(&json_output).unwrap();
    assert_eq!(parsed["framework"], "HIPAA");
    assert!(!parsed["passed"].as_bool().unwrap());

    let sarif_reporter = Reporter::new(ReportConfig::new(OutputFormat::Sarif));
    let sarif_output = sarif_reporter.format(&report);
    let parsed: serde_json::Value = serde_json::from_str(&sarif_output).unwrap();
    assert!(parsed["$schema"].as_str().unwrap().contains("sarif"));

    let md_reporter = Reporter::new(ReportConfig::new(OutputFormat::Markdown));
    let md_output = md_reporter.format(&report);
    assert!(md_output.contains("# Compliance Validation Report"));
    assert!(md_output.contains("`no-phi-in-logs`"));

    let html_reporter = Reporter::new(ReportConfig::new(OutputFormat::Html));
    let html_output = html_reporter.format(&report);
    assert!(html_output.contains("<!DOCTYPE html>"));
    assert!(html_output.contains("HIPAA"));
    assert!(html_output.contains("no-phi-in-logs"));
    assert!(html_output.contains("status-failed"));
}
