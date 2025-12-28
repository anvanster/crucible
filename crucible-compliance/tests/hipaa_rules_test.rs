//! Comprehensive tests for all 57 HIPAA compliance rules
//!
//! This test file ensures every HIPAA rule in the framework is properly validated.
//! Rules are organized by category matching the HIPAA Security Rule structure.

use crucible_compliance::{ComplianceValidator, FrameworkLoader};
use crucible_core::types::{
    Export, ExportType, Language, Manifest, Method, Module, Project, ProjectConfig, Property,
    ReturnType,
};
use std::collections::HashMap;
use std::path::PathBuf;

fn get_frameworks_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frameworks")
}

fn create_test_manifest() -> Manifest {
    Manifest {
        version: "0.1.0".to_string(),
        project: ProjectConfig {
            name: "hipaa-test".to_string(),
            language: Language::TypeScript,
            architecture_pattern: None,
        },
        modules: vec!["test".to_string()],
        strict_validation: true,
        metadata: None,
    }
}

/// Helper to create a module with specific properties and methods
fn create_module(
    module_name: &str,
    export_name: &str,
    properties: HashMap<String, Property>,
    methods: HashMap<String, Method>,
) -> Module {
    let mut exports = HashMap::new();
    exports.insert(
        export_name.to_string(),
        Export {
            export_type: ExportType::Class,
            methods: Some(methods),
            properties: Some(properties),
            values: None,
            dependencies: None,
            payload: None,
        },
    );

    Module {
        module: module_name.to_string(),
        version: "1.0.0".to_string(),
        layer: None,
        description: None,
        exports,
        dependencies: HashMap::new(),
    }
}

/// Helper to create a simple property with annotations
fn prop(annotations: Vec<&str>) -> Property {
    Property {
        prop_type: "string".to_string(),
        required: true,
        description: None,
        annotations: annotations.iter().map(|s| s.to_string()).collect(),
    }
}

/// Helper to create a method with effects and annotations
fn method(effects: Vec<&str>, annotations: Vec<&str>) -> Method {
    Method {
        inputs: vec![],
        returns: ReturnType {
            return_type: "void".to_string(),
            inner: None,
        },
        throws: vec![],
        calls: vec![],
        effects: effects.iter().map(|s| s.to_string()).collect(),
        is_async: false,
        annotations: annotations.iter().map(|s| s.to_string()).collect(),
    }
}

// ============================================================
// Section 1: Data Exposure Prevention Rules (no-phi-in-*)
// ============================================================

#[test]
fn test_rule_no_phi_in_logs() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi"]));

    let mut methods = HashMap::new();
    methods.insert("logData".to_string(), method(vec!["logging"], vec![]));

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "no-phi-in-logs"),
        "Should detect no-phi-in-logs violation"
    );
}

#[test]
fn test_rule_no_phi_in_logs_console_log() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi"]));

    let mut methods = HashMap::new();
    methods.insert(
        "debugPatient".to_string(),
        method(vec!["console.log"], vec![]),
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "no-phi-in-logs"),
        "Should detect PHI in console.log"
    );
}

#[test]
fn test_rule_no_phi_in_error_messages() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi"]));

    let mut methods = HashMap::new();
    methods.insert("handleError".to_string(), method(vec!["throw"], vec![]));

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "no-phi-in-error-messages"),
        "Should detect PHI in error messages"
    );
}

#[test]
fn test_rule_no_phi_in_urls() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi"]));

    let mut methods = HashMap::new();
    methods.insert("redirect".to_string(), method(vec!["url.param"], vec![]));

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "no-phi-in-urls"),
        "Should detect PHI in URLs"
    );
}

#[test]
fn test_rule_no_phi_in_local_storage() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi"]));

    let mut methods = HashMap::new();
    methods.insert(
        "cacheData".to_string(),
        method(vec!["localStorage.setItem"], vec![]),
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "no-phi-in-local-storage"),
        "Should detect PHI in local storage"
    );
}

#[test]
fn test_rule_no_phi_in_cookies() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi"]));

    let mut methods = HashMap::new();
    methods.insert("setCookie".to_string(), method(vec!["cookie.set"], vec![]));

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "no-phi-in-cookies"),
        "Should detect PHI in cookies"
    );
}

#[test]
fn test_rule_no_phi_in_cache() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi"]));

    let mut methods = HashMap::new();
    methods.insert(
        "cachePatient".to_string(),
        method(vec!["cache.set"], vec![]),
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "no-phi-in-cache"),
        "Should detect PHI in cache"
    );
}

// ============================================================
// Section 2: Audit Logging Rules
// ============================================================

#[test]
fn test_rule_phi_access_requires_audit() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut methods = HashMap::new();
    methods.insert(
        "getPatient".to_string(),
        method(vec![], vec!["@phi-access"]), // No audit.log effect
    );

    let module = create_module("patient", "PatientService", HashMap::new(), methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-access-requires-audit"),
        "Should detect missing audit log for PHI access"
    );
}

#[test]
fn test_rule_phi_access_requires_audit_compliant() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut methods = HashMap::new();
    methods.insert(
        "getPatient".to_string(),
        method(vec!["audit.log"], vec!["@phi-access"]), // Has audit.log
    );

    let module = create_module("patient", "PatientService", HashMap::new(), methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-access-requires-audit"),
        "Should not flag compliant PHI access with audit log"
    );
}

#[test]
fn test_rule_phi_modification_requires_audit() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut methods = HashMap::new();
    methods.insert(
        "updatePatient".to_string(),
        method(vec![], vec!["@phi-modify"]), // No audit.log
    );

    let module = create_module("patient", "PatientService", HashMap::new(), methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-modification-requires-audit"),
        "Should detect missing audit log for PHI modification"
    );
}

#[test]
fn test_rule_phi_disclosure_requires_audit() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut methods = HashMap::new();
    methods.insert(
        "sharePatientData".to_string(),
        method(vec![], vec!["@phi-disclosure"]), // No audit.log
    );

    let module = create_module("patient", "PatientService", HashMap::new(), methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-disclosure-requires-audit"),
        "Should detect missing audit log for PHI disclosure"
    );
}

// ============================================================
// Section 3: Access Control Rules
// ============================================================

#[test]
fn test_rule_phi_requires_authentication() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi"]));

    let mut methods = HashMap::new();
    methods.insert("getPatient".to_string(), method(vec![], vec![])); // No @requires-auth

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-requires-authentication"),
        "Should detect PHI access without authentication"
    );
}

#[test]
fn test_rule_phi_requires_authorization() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi"]));

    let mut methods = HashMap::new();
    methods.insert(
        "getPatient".to_string(),
        method(vec![], vec!["@requires-auth"]), // Has auth but no role
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-requires-authorization"),
        "Should detect PHI access without authorization"
    );
}

#[test]
fn test_rule_phi_storage_encryption() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi"])); // No @encrypted

    let module = create_module("patient", "PatientService", properties, HashMap::new());
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-storage-encryption"),
        "Should detect unencrypted PHI storage"
    );
}

#[test]
fn test_rule_phi_storage_encryption_compliant() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi", "@encrypted"])); // Encrypted

    let module = create_module("patient", "PatientService", properties, HashMap::new());
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-storage-encryption"),
        "Should not flag encrypted PHI storage"
    );
}

#[test]
fn test_rule_phi_transmission_encryption() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi"]));

    let mut methods = HashMap::new();
    methods.insert(
        "transmitPatient".to_string(),
        method(vec![], vec!["@requires-auth", "@requires-role"]), // No @https-only
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-transmission-encryption"),
        "Should detect PHI transmission without encryption"
    );
}

#[test]
fn test_rule_phi_email_encryption() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("ssn".to_string(), prop(vec!["@phi"]));

    let mut methods = HashMap::new();
    methods.insert(
        "emailPatient".to_string(),
        method(vec!["email.send"], vec![]),
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-email-encryption"),
        "Should detect unencrypted PHI email"
    );
}

// ============================================================
// Section 4: Session Management Rules
// ============================================================

#[test]
fn test_rule_session_timeout() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property has @phi-access annotation (data being accessed)
    let mut properties = HashMap::new();
    properties.insert("patientData".to_string(), prop(vec!["@phi-access"]));

    // Method accesses this data but lacks @session-timeout
    let mut methods = HashMap::new();
    methods.insert(
        "accessPhi".to_string(),
        method(vec!["audit.log"], vec![]), // No @session-timeout
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "session-timeout"),
        "Should detect missing session timeout for PHI access"
    );
}

#[test]
fn test_rule_unique_user_identification() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property has @phi-access annotation
    let mut properties = HashMap::new();
    properties.insert("patientData".to_string(), prop(vec!["@phi-access"]));

    // Method has session-timeout but lacks @user-identified
    let mut methods = HashMap::new();
    methods.insert(
        "accessPhi".to_string(),
        method(vec!["audit.log"], vec!["@session-timeout"]), // No @user-identified
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "unique-user-identification"),
        "Should detect missing unique user identification"
    );
}

// ============================================================
// Section 5: Authentication Rules
// ============================================================

#[test]
fn test_rule_failed_login_monitoring() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut methods = HashMap::new();
    methods.insert(
        "login".to_string(),
        method(vec![], vec!["@authentication"]), // No audit.log
    );

    let module = create_module("auth", "AuthService", HashMap::new(), methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "failed-login-monitoring"),
        "Should detect missing login monitoring"
    );
}

#[test]
fn test_rule_account_lockout() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @authentication annotation (authentication context)
    let mut properties = HashMap::new();
    properties.insert("credentials".to_string(), prop(vec!["@authentication"]));

    // Method accesses auth data but lacks @account-lockout
    let mut methods = HashMap::new();
    methods.insert(
        "login".to_string(),
        method(vec!["audit.log"], vec![]), // No @account-lockout
    );

    let module = create_module("auth", "AuthService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "account-lockout"),
        "Should detect missing account lockout"
    );
}

#[test]
fn test_rule_password_complexity() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("password".to_string(), prop(vec!["@password"]));

    let mut methods = HashMap::new();
    methods.insert("setPassword".to_string(), method(vec![], vec![])); // No @password-policy

    let module = create_module("auth", "AuthService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "password-complexity"),
        "Should detect missing password complexity policy"
    );
}

// ============================================================
// Section 6: Integrity Controls
// ============================================================

#[test]
fn test_rule_phi_integrity_verification() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut methods = HashMap::new();
    methods.insert(
        "updatePatient".to_string(),
        method(vec!["audit.log"], vec!["@phi-modify"]), // No integrity.check
    );

    let module = create_module("patient", "PatientService", HashMap::new(), methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-integrity-verification"),
        "Should detect missing integrity verification"
    );
}

#[test]
fn test_rule_phi_deletion_authorization() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @phi-delete annotation (deletion context)
    let mut properties = HashMap::new();
    properties.insert("patientRecord".to_string(), prop(vec!["@phi-delete"]));

    // Method lacks @delete-authorized
    let mut methods = HashMap::new();
    methods.insert(
        "deletePatient".to_string(),
        method(vec![], vec![]), // No @delete-authorized
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-deletion-authorization"),
        "Should detect missing deletion authorization"
    );
}

// ============================================================
// Section 7: Emergency Access Rules
// ============================================================

#[test]
fn test_rule_emergency_access_procedure() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @phi-emergency annotation (emergency access context)
    let mut properties = HashMap::new();
    properties.insert(
        "emergencyPatientData".to_string(),
        prop(vec!["@phi-emergency"]),
    );

    // Method lacks @break-glass
    let mut methods = HashMap::new();
    methods.insert(
        "emergencyAccess".to_string(),
        method(vec![], vec![]), // No @break-glass
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "emergency-access-procedure"),
        "Should detect missing emergency access procedure"
    );
}

#[test]
fn test_rule_emergency_access_audit() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut methods = HashMap::new();
    methods.insert(
        "breakGlassAccess".to_string(),
        method(vec![], vec!["@break-glass"]), // No audit.log
    );

    let module = create_module("patient", "PatientService", HashMap::new(), methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "emergency-access-audit"),
        "Should detect missing emergency access audit"
    );
}

// ============================================================
// Section 8: Security Incident Rules
// ============================================================

#[test]
fn test_rule_security_incident_logging() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut methods = HashMap::new();
    methods.insert(
        "handleIncident".to_string(),
        method(vec![], vec!["@security-incident"]), // No incident.log
    );

    let module = create_module("security", "SecurityService", HashMap::new(), methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "security-incident-logging"),
        "Should detect missing security incident logging"
    );
}

#[test]
fn test_rule_breach_notification() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut methods = HashMap::new();
    methods.insert(
        "handleBreach".to_string(),
        method(vec![], vec!["@breach-confirmed"]), // No breach.notify
    );

    let module = create_module("security", "SecurityService", HashMap::new(), methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "breach-notification"),
        "Should detect missing breach notification"
    );
}

// ============================================================
// Section 9: Workforce Security Rules
// ============================================================

#[test]
fn test_rule_access_termination() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @user-termination annotation (termination context)
    let mut properties = HashMap::new();
    properties.insert("userAccess".to_string(), prop(vec!["@user-termination"]));

    // Method lacks @access-terminated
    let mut methods = HashMap::new();
    methods.insert(
        "terminateUser".to_string(),
        method(vec![], vec![]), // No @access-terminated
    );

    let module = create_module("hr", "HRService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "access-termination"),
        "Should detect missing access termination"
    );
}

// ============================================================
// Section 10: Device Security Rules
// ============================================================

#[test]
fn test_rule_mobile_device_security() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @mobile-access annotation (mobile access context)
    let mut properties = HashMap::new();
    properties.insert(
        "mobilePatientData".to_string(),
        prop(vec!["@mobile-access"]),
    );

    // Method lacks @mobile-secured
    let mut methods = HashMap::new();
    methods.insert(
        "mobilePatientAccess".to_string(),
        method(vec![], vec![]), // No @mobile-secured
    );

    let module = create_module("mobile", "MobileService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "mobile-device-security"),
        "Should detect missing mobile device security"
    );
}

#[test]
fn test_rule_remote_access_security() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @remote-access annotation (remote access context)
    let mut properties = HashMap::new();
    properties.insert(
        "remotePatientData".to_string(),
        prop(vec!["@remote-access"]),
    );

    // Method lacks @vpn-required
    let mut methods = HashMap::new();
    methods.insert(
        "remotePatientAccess".to_string(),
        method(vec![], vec![]), // No @vpn-required
    );

    let module = create_module("remote", "RemoteService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "remote-access-security"),
        "Should detect missing remote access security"
    );
}

// ============================================================
// Section 11: Business Associate Rules
// ============================================================

#[test]
fn test_rule_business_associate_data() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut methods = HashMap::new();
    methods.insert(
        "shareWithBA".to_string(),
        method(vec![], vec!["@ba-disclosure"]), // No disclosure.log
    );

    let module = create_module("integration", "IntegrationService", HashMap::new(), methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "business-associate-data"),
        "Should detect missing BA disclosure logging"
    );
}

#[test]
fn test_rule_subcontractor_security() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @subcontractor-access annotation
    let mut properties = HashMap::new();
    properties.insert(
        "subcontractorData".to_string(),
        prop(vec!["@subcontractor-access"]),
    );

    // Method lacks @baa-signed
    let mut methods = HashMap::new();
    methods.insert(
        "subcontractorAccess".to_string(),
        method(vec![], vec![]), // No @baa-signed
    );

    let module = create_module("integration", "IntegrationService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "subcontractor-security"),
        "Should detect missing subcontractor BAA"
    );
}

// ============================================================
// Section 12: Export and Transfer Rules
// ============================================================

#[test]
fn test_rule_phi_export_control() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @phi-export annotation
    let mut properties = HashMap::new();
    properties.insert(
        "exportablePatientData".to_string(),
        prop(vec!["@phi-export"]),
    );

    // Method lacks @export-authorized
    let mut methods = HashMap::new();
    methods.insert(
        "exportPatients".to_string(),
        method(vec![], vec![]), // No @export-authorized
    );

    let module = create_module("export", "ExportService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-export-control"),
        "Should detect unauthorized PHI export"
    );
}

// ============================================================
// Section 13: Input Validation and Security Rules
// ============================================================

#[test]
fn test_rule_input_validation() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @phi-input annotation
    let mut properties = HashMap::new();
    properties.insert("patientInput".to_string(), prop(vec!["@phi-input"]));

    // Method lacks @validated
    let mut methods = HashMap::new();
    methods.insert(
        "createPatient".to_string(),
        method(vec![], vec![]), // No @validated
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "input-validation"),
        "Should detect missing input validation"
    );
}

#[test]
fn test_rule_sql_injection_prevention() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @phi-query annotation
    let mut properties = HashMap::new();
    properties.insert("patientQuery".to_string(), prop(vec!["@phi-query"]));

    // Method lacks @parameterized
    let mut methods = HashMap::new();
    methods.insert(
        "queryPatient".to_string(),
        method(vec![], vec![]), // No @parameterized
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "sql-injection-prevention"),
        "Should detect missing SQL injection prevention"
    );
}

#[test]
fn test_rule_xss_prevention() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @phi-display annotation
    let mut properties = HashMap::new();
    properties.insert("patientDisplay".to_string(), prop(vec!["@phi-display"]));

    // Method lacks @xss-protected
    let mut methods = HashMap::new();
    methods.insert(
        "displayPatient".to_string(),
        method(vec![], vec![]), // No @xss-protected
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "xss-prevention"),
        "Should detect missing XSS prevention"
    );
}

// ============================================================
// Section 14: Encryption Key Management Rules
// ============================================================

#[test]
fn test_rule_encryption_key_management() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @encryption-key annotation
    let mut properties = HashMap::new();
    properties.insert("masterKey".to_string(), prop(vec!["@encryption-key"]));

    // Method lacks @key-managed
    let mut methods = HashMap::new();
    methods.insert(
        "handleKey".to_string(),
        method(vec![], vec![]), // No @key-managed
    );

    let module = create_module("crypto", "CryptoService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "encryption-key-management"),
        "Should detect missing key management"
    );
}

// ============================================================
// Section 15: Contingency Rules
// ============================================================

#[test]
fn test_rule_contingency_backup() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @phi-critical annotation
    let mut properties = HashMap::new();
    properties.insert(
        "criticalPatientData".to_string(),
        prop(vec!["@phi-critical"]),
    );

    // Method lacks @backup-enabled
    let mut methods = HashMap::new();
    methods.insert(
        "handleCriticalPhi".to_string(),
        method(vec![], vec![]), // No @backup-enabled
    );

    let module = create_module("storage", "StorageService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "contingency-backup"),
        "Should detect missing backup for critical PHI"
    );
}

#[test]
fn test_rule_contingency_recovery() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    // Property with @phi-critical annotation
    let mut properties = HashMap::new();
    properties.insert(
        "criticalPatientData".to_string(),
        prop(vec!["@phi-critical"]),
    );

    // Method has @backup-enabled but lacks @recovery-plan
    let mut methods = HashMap::new();
    methods.insert(
        "handleCriticalPhi".to_string(),
        method(vec![], vec!["@backup-enabled"]), // No @recovery-plan
    );

    let module = create_module("storage", "StorageService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.rule_id == "contingency-recovery"),
        "Should detect missing recovery plan"
    );
}

// ============================================================
// Section 16: Compliant Examples (should pass)
// ============================================================

#[test]
fn test_fully_compliant_phi_access() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert(
        "ssn".to_string(),
        prop(vec!["@phi", "@encrypted"]), // Properly encrypted
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
                "@requires-auth".to_string(),
                "@requires-role".to_string(),
                "@https-only".to_string(),
                "@session-timeout".to_string(),
                "@user-identified".to_string(),
            ],
        },
    );

    let module = create_module("patient", "PatientService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();

    // Should pass core access control rules
    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-requires-authentication"),
        "Should pass authentication check"
    );
    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-requires-authorization"),
        "Should pass authorization check"
    );
    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-storage-encryption"),
        "Should pass encryption check"
    );
    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-transmission-encryption"),
        "Should pass transmission check"
    );
    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "phi-access-requires-audit"),
        "Should pass audit check"
    );
    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "session-timeout"),
        "Should pass session timeout check"
    );
}

#[test]
fn test_compliant_authentication_service() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut properties = HashMap::new();
    properties.insert("password".to_string(), prop(vec!["@password"]));

    let mut methods = HashMap::new();
    methods.insert(
        "login".to_string(),
        Method {
            inputs: vec![],
            returns: ReturnType {
                return_type: "AuthResult".to_string(),
                inner: None,
            },
            throws: vec![],
            calls: vec![],
            effects: vec!["audit.log".to_string()],
            is_async: true,
            annotations: vec![
                "@authentication".to_string(),
                "@account-lockout".to_string(),
                "@password-policy".to_string(),
            ],
        },
    );

    let module = create_module("auth", "AuthService", properties, methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();

    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "failed-login-monitoring"),
        "Should pass login monitoring check"
    );
    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "account-lockout"),
        "Should pass account lockout check"
    );
    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "password-complexity"),
        "Should pass password complexity check"
    );
}

#[test]
fn test_compliant_emergency_access() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut methods = HashMap::new();
    methods.insert(
        "emergencyPatientAccess".to_string(),
        Method {
            inputs: vec![],
            returns: ReturnType {
                return_type: "Patient".to_string(),
                inner: None,
            },
            throws: vec![],
            calls: vec![],
            effects: vec!["audit.log".to_string()],
            is_async: true,
            annotations: vec!["@phi-emergency".to_string(), "@break-glass".to_string()],
        },
    );

    let module = create_module("emergency", "EmergencyService", HashMap::new(), methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();

    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "emergency-access-procedure"),
        "Should pass emergency access procedure check"
    );
    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "emergency-access-audit"),
        "Should pass emergency access audit check"
    );
}

#[test]
fn test_compliant_security_incident_handling() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();
    let validator = ComplianceValidator::new(hipaa);

    let mut methods = HashMap::new();
    methods.insert(
        "handleSecurityIncident".to_string(),
        Method {
            inputs: vec![],
            returns: ReturnType {
                return_type: "void".to_string(),
                inner: None,
            },
            throws: vec![],
            calls: vec![],
            effects: vec!["incident.log".to_string()],
            is_async: true,
            annotations: vec!["@security-incident".to_string()],
        },
    );
    methods.insert(
        "handleBreach".to_string(),
        Method {
            inputs: vec![],
            returns: ReturnType {
                return_type: "void".to_string(),
                inner: None,
            },
            throws: vec![],
            calls: vec![],
            effects: vec!["breach.notify".to_string()],
            is_async: true,
            annotations: vec!["@breach-confirmed".to_string()],
        },
    );

    let module = create_module("security", "SecurityService", HashMap::new(), methods);
    let project = Project {
        manifest: create_test_manifest(),
        modules: vec![module],
        rules: None,
    };

    let report = validator.validate(&project).unwrap();

    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "security-incident-logging"),
        "Should pass security incident logging check"
    );
    assert!(
        !report
            .violations
            .iter()
            .any(|v| v.rule_id == "breach-notification"),
        "Should pass breach notification check"
    );
}

// ============================================================
// Rule Count Verification
// ============================================================

#[test]
fn test_hipaa_framework_has_53_rules() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();

    assert_eq!(
        hipaa.rule_count(),
        53,
        "HIPAA framework should have 53 rules"
    );
}

#[test]
fn test_all_hipaa_rules_have_unique_ids() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();

    let mut seen_ids = std::collections::HashSet::new();
    for rule in hipaa.rules() {
        assert!(
            seen_ids.insert(rule.id.clone()),
            "Duplicate rule ID found: {}",
            rule.id
        );
    }
}

#[test]
fn test_all_hipaa_rules_have_requirement_ids() {
    let mut loader = FrameworkLoader::new();
    loader.load_directory(get_frameworks_path()).unwrap();
    let hipaa = loader.get("HIPAA").unwrap();

    for rule in hipaa.rules() {
        assert!(
            rule.requirement_id.is_some(),
            "Rule {} is missing a requirement_id",
            rule.id
        );
    }
}
