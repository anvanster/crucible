//! Type system enhancement tests (TDD)
//!
//! These tests define the expected behavior for TypeScript type system support.
//! They should FAIL initially, then PASS after implementation.

use crucible_core::type_system::*;
use crucible_core::types::Module;

// =============================================================================
// Phase 1: Built-in Type Support
// =============================================================================

#[test]
fn test_builtin_primitive_types_recognized() {
    // Given: Common primitive types
    let primitives = vec!["string", "number", "boolean", "void"];

    // When: Checking if they are built-in types
    // Then: All should be recognized as built-in
    for primitive in primitives {
        assert!(
            is_builtin_type(primitive),
            "{primitive} should be recognized as built-in"
        );
    }
}

#[test]
fn test_builtin_object_types_recognized() {
    // Given: Common built-in object types
    let objects = vec!["Date", "Buffer", "Error", "RegExp"];

    // When: Checking if they are built-in types
    // Then: All should be recognized as built-in
    for obj in objects {
        assert!(
            is_builtin_type(obj),
            "{obj} should be recognized as built-in"
        );
    }
}

#[test]
fn test_builtin_special_types_recognized() {
    // Given: Special TypeScript types
    let special = vec!["object", "any", "unknown", "null", "undefined"];

    // When: Checking if they are built-in types
    // Then: All should be recognized as built-in
    for s in special {
        assert!(is_builtin_type(s), "{s} should be recognized as built-in");
    }
}

#[test]
fn test_custom_types_not_builtin() {
    // Given: Custom type names
    let custom = vec!["Patient", "User", "CustomType"];

    // When: Checking if they are built-in types
    // Then: None should be recognized as built-in
    for c in custom {
        assert!(
            !is_builtin_type(c),
            "{c} should NOT be recognized as built-in"
        );
    }
}

// =============================================================================
// Phase 2: Nullable Type Support
// =============================================================================

#[test]
fn test_parse_nullable_type() {
    // Given: TypeParser with JSON-like params
    let parser = TypeParser::new();

    // When: Parsing with nullable flag
    let type_ref = parser
        .parse_from_json("Patient", Some(true), None, None)
        .unwrap();

    // Then: Should parse correctly with nullable flag
    assert_eq!(type_ref.base_type, "Patient");
    assert!(type_ref.nullable);
}

#[test]
fn test_parse_non_nullable_type() {
    // Given: TypeParser with JSON-like params
    let parser = TypeParser::new();

    // When: Parsing without nullable flag
    let type_ref = parser.parse_from_json("Patient", None, None, None).unwrap();

    // Then: Should default to non-nullable
    assert_eq!(type_ref.base_type, "Patient");
    assert!(!type_ref.nullable);
}

#[test]
fn test_validate_nullable_type_exists() {
    // Given: Modules with a Patient type
    let modules = vec![create_patient_module()];

    // When: Validating a nullable Patient type (module-qualified)
    let result = validate_type_string("patient.Patient", Some(true), &modules);

    // Then: Should validate successfully
    assert!(
        result.is_ok(),
        "Nullable Patient type should validate: {result:?}"
    );
}

// =============================================================================
// Phase 3: Array Syntax Support
// =============================================================================

#[test]
fn test_parse_array_shorthand_syntax() {
    // Given: Array shorthand syntax
    let type_str = "Patient[]";

    // When: Parsing the type
    let type_ref = parse_type_string(type_str).unwrap();

    // Then: Should parse as array with Patient items
    assert_eq!(type_ref.base_type, "array");
    assert!(type_ref.items.is_some());
    let items = type_ref.items.unwrap();
    assert_eq!(items.base_type, "Patient");
}

#[test]
fn test_parse_nested_array_syntax() {
    // Given: Nested array syntax
    let type_str = "Patient[][]";

    // When: Parsing the type
    let type_ref = parse_type_string(type_str).unwrap();

    // Then: Should parse as nested arrays
    assert_eq!(type_ref.base_type, "array");
    let items = type_ref.items.unwrap();
    assert_eq!(items.base_type, "array");
    let nested_items = items.items.unwrap();
    assert_eq!(nested_items.base_type, "Patient");
}

#[test]
fn test_parse_array_long_form() {
    // Given: TypeParser with array items
    let parser = TypeParser::new();

    // When: Parsing with items field
    let type_ref = parser
        .parse_from_json("array", None, Some("Patient"), None)
        .unwrap();

    // Then: Should parse as array with Patient items
    assert_eq!(type_ref.base_type, "array");
    assert!(type_ref.items.is_some());
    let items = type_ref.items.unwrap();
    assert_eq!(items.base_type, "Patient");
}

#[test]
fn test_validate_array_type() {
    // Given: Modules with a Patient type
    let modules = vec![create_patient_module()];

    // When: Validating patient.Patient[] type
    let result = validate_type_string("patient.Patient[]", None, &modules);

    // Then: Should validate successfully
    assert!(
        result.is_ok(),
        "patient.Patient[] should validate: {result:?}"
    );
}

// =============================================================================
// Phase 4: Generic Type Support
// =============================================================================

#[test]
fn test_parse_partial_generic() {
    // Given: TypeParser with type args
    let parser = TypeParser::new();
    let args = vec!["Patient".to_string()];

    // When: Parsing with type args
    let type_ref = parser
        .parse_from_json("Partial", None, None, Some(&args))
        .unwrap();

    // Then: Should parse as generic with type args
    assert_eq!(type_ref.base_type, "Partial");
    assert_eq!(type_ref.type_args.len(), 1);
    assert_eq!(type_ref.type_args[0].base_type, "Patient");
}

#[test]
fn test_generic_types_recognized() {
    // Given: Common generic utility types
    let generics = vec!["Partial", "Omit", "Pick", "Record", "Promise"];

    // When: Checking if they are generic types
    // Then: All should be recognized as generics
    for g in generics {
        assert!(is_generic_type(g), "{g} should be recognized as generic");
    }
}

#[test]
fn test_validate_partial_type() {
    // Given: Modules with a Patient type
    let modules = vec![create_patient_module()];

    // When: Validating Partial<patient.Patient> (module-qualified)
    let type_ref = TypeReference {
        base_type: "Partial".to_string(),
        nullable: false,
        items: None,
        type_args: vec![TypeReference {
            base_type: "patient.Patient".to_string(),
            nullable: false,
            items: None,
            type_args: vec![],
        }],
    };

    let result = validate_type_reference(&type_ref, &modules);

    // Then: Should validate successfully
    assert!(
        result.is_ok(),
        "Partial<patient.Patient> should validate: {result:?}"
    );
}

#[test]
fn test_validate_promise_type() {
    // Given: Modules with a Patient type
    let modules = vec![create_patient_module()];

    // When: Validating Promise<Patient>
    let type_ref = TypeReference {
        base_type: "Promise".to_string(),
        nullable: false,
        items: None,
        type_args: vec![TypeReference {
            base_type: "Patient".to_string(),
            nullable: false,
            items: None,
            type_args: vec![],
        }],
    };

    let result = validate_type_reference(&type_ref, &modules);

    // Then: Should validate successfully
    assert!(
        result.is_ok(),
        "Promise<Patient> should validate: {result:?}"
    );
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_validate_complex_type_combinations() {
    // Given: Modules with various types
    let modules = vec![create_patient_module()];

    // When/Then: All complex combinations should validate
    let cases = vec![
        ("patient.Patient", false, "Basic type"),
        ("patient.Patient", true, "Nullable type"),
        ("patient.Patient[]", false, "Array type"),
        ("Date", false, "Built-in type"),
        ("Buffer", false, "Node built-in"),
    ];

    for (type_str, nullable, desc) in cases {
        let result = validate_type_string(type_str, Some(nullable), &modules);
        assert!(result.is_ok(), "{desc} should validate: {result:?}");
    }
}

// =============================================================================
// Test Helpers
// =============================================================================

fn create_patient_module() -> Module {
    use crucible_core::types::{Export, ExportType, Property};
    use std::collections::HashMap;

    let mut exports = HashMap::new();
    let mut properties = HashMap::new();

    properties.insert(
        "id".to_string(),
        Property {
            prop_type: "string".to_string(),
            required: true,
            description: None,
            annotations: vec![],
        },
    );

    properties.insert(
        "name".to_string(),
        Property {
            prop_type: "string".to_string(),
            required: true,
            description: None,
            annotations: vec![],
        },
    );

    exports.insert(
        "Patient".to_string(),
        Export {
            export_type: ExportType::Type,
            methods: None,
            properties: Some(properties),
            values: None,
            dependencies: None,
            payload: None,
        },
    );

    Module {
        module: "patient".to_string(),
        version: "1.0.0".to_string(),
        layer: Some("domain".to_string()),
        description: Some("Patient domain entity".to_string()),
        exports,
        dependencies: HashMap::new(),
    }
}
