use crucible_core::{Parser, Validator};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_parse_valid_manifest() {
    let dir = tempdir().unwrap();
    let manifest_content = r#"{
        "version": "0.1.0",
        "project": {
            "name": "test",
            "language": "rust"
        },
        "modules": []
    }"#;

    fs::write(dir.path().join("manifest.json"), manifest_content).unwrap();

    let parser = Parser::new(dir.path());
    let manifest = parser.parse_manifest().unwrap();

    assert_eq!(manifest.project.name, "test");
    assert_eq!(manifest.version, "0.1.0");
}

#[test]
fn test_validate_no_circular_deps() {
    // Create a test project with no circular dependencies
    let dir = tempdir().unwrap();

    // Create manifest
    let manifest = r#"{
        "version": "0.1.0",
        "project": {"name": "test", "language": "rust"},
        "modules": ["a", "b"]
    }"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();

    // Create modules directory
    fs::create_dir(dir.path().join("modules")).unwrap();

    // Module A depends on nothing
    let module_a = r#"{
        "module": "a",
        "version": "1.0.0",
        "exports": {},
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/a.json"), module_a).unwrap();

    // Module B depends on A
    let module_b = r#"{
        "module": "b",
        "version": "1.0.0",
        "exports": {},
        "dependencies": {"a": "^1.0.0"}
    }"#;
    fs::write(dir.path().join("modules/b.json"), module_b).unwrap();

    // Parse and validate
    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    assert!(result.valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_validate_circular_deps() {
    // Create a test project with circular dependencies
    let dir = tempdir().unwrap();

    // Create manifest
    let manifest = r#"{
        "version": "0.1.0",
        "project": {"name": "test", "language": "rust"},
        "modules": ["a", "b"]
    }"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();

    // Create modules directory
    fs::create_dir(dir.path().join("modules")).unwrap();

    // Module A depends on B
    let module_a = r#"{
        "module": "a",
        "version": "1.0.0",
        "exports": {},
        "dependencies": {"b": "^1.0.0"}
    }"#;
    fs::write(dir.path().join("modules/a.json"), module_a).unwrap();

    // Module B depends on A (circular!)
    let module_b = r#"{
        "module": "b",
        "version": "1.0.0",
        "exports": {},
        "dependencies": {"a": "^1.0.0"}
    }"#;
    fs::write(dir.path().join("modules/b.json"), module_b).unwrap();

    // Parse and validate
    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    assert!(!result.valid);
    assert!(!result.errors.is_empty());
    assert!(result
        .errors
        .iter()
        .any(|e| e.rule == "no-circular-dependencies"));
}

#[test]
fn test_layer_boundary_validation() {
    let dir = tempdir().unwrap();

    // Create manifest
    let manifest = r#"{
        "version": "0.1.0",
        "project": {"name": "test", "language": "rust"},
        "modules": ["domain", "app"]
    }"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();

    // Create rules with layered architecture
    let rules = r#"{
        "architecture": {
            "pattern": "layered",
            "layers": [
                {"name": "application", "can_depend_on": ["domain"]},
                {"name": "domain", "can_depend_on": []}
            ]
        },
        "rules": [
            {"id": "respect-layer-boundaries", "enabled": true, "severity": "error"}
        ]
    }"#;
    fs::write(dir.path().join("rules.json"), rules).unwrap();

    // Create modules directory
    fs::create_dir(dir.path().join("modules")).unwrap();

    // Domain module
    let module_domain = r#"{
        "module": "domain",
        "version": "1.0.0",
        "layer": "domain",
        "exports": {},
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/domain.json"), module_domain).unwrap();

    // Application module depends on domain (valid)
    let module_app = r#"{
        "module": "app",
        "version": "1.0.0",
        "layer": "application",
        "exports": {},
        "dependencies": {"domain": "^1.0.0"}
    }"#;
    fs::write(dir.path().join("modules/app.json"), module_app).unwrap();

    // Parse and validate
    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    assert!(result.valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_layer_boundary_violation() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "rust"}, "modules": ["domain", "app"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    let rules = r#"{"architecture": {"pattern": "layered", "layers": [{"name": "application", "can_depend_on": ["domain"]}, {"name": "domain", "can_depend_on": []}]}, "rules": [{"id": "respect-layer-boundaries", "enabled": true, "severity": "error"}]}"#;
    fs::write(dir.path().join("rules.json"), rules).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();
    let module_domain = r#"{"module": "domain", "version": "1.0.0", "layer": "domain", "exports": {}, "dependencies": {"app": "^1.0.0"}}"#;
    fs::write(dir.path().join("modules/domain.json"), module_domain).unwrap();
    let module_app = r#"{"module": "app", "version": "1.0.0", "layer": "application", "exports": {}, "dependencies": {}}"#;
    fs::write(dir.path().join("modules/app.json"), module_app).unwrap();
    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();
    assert!(!result.valid);
    assert!(!result.errors.is_empty());
    assert!(result
        .errors
        .iter()
        .any(|e| e.rule == "respect-layer-boundaries"));
}

#[test]
fn test_type_existence_validation() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "rust"}, "modules": ["user"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();
    let module_user = r#"{"module": "user", "version": "1.0.0", "exports": {"UserService": {"type": "class", "methods": {"getUser": {"inputs": [{"name": "id", "type": "NonExistentType"}], "returns": {"type": "string"}}}}}, "dependencies": {}}"#;
    fs::write(dir.path().join("modules/user.json"), module_user).unwrap();
    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();
    assert!(!result.valid);
    assert!(!result.errors.is_empty());
    assert!(result
        .errors
        .iter()
        .any(|e| e.rule == "all-types-must-exist"));
}

#[test]
fn test_validation_with_generic_types() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "rust"}, "modules": ["repo"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();
    let module_repo = r#"{"module": "repo", "version": "1.0.0", "exports": {"Repository": {"type": "class", "methods": {"findAll": {"inputs": [], "returns": {"type": "Promise<Vec<string>>"}}}}}, "dependencies": {}}"#;
    fs::write(dir.path().join("modules/repo.json"), module_repo).unwrap();
    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();
    assert!(result.valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_validation_with_cross_module_types() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "rust"}, "modules": ["types", "service"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();
    let module_types = r#"{"module": "types", "version": "1.0.0", "exports": {"User": {"type": "interface", "properties": {"id": {"type": "string", "required": true}, "name": {"type": "string", "required": true}}}}, "dependencies": {}}"#;
    fs::write(dir.path().join("modules/types.json"), module_types).unwrap();
    let module_service = r#"{"module": "service", "version": "1.0.0", "exports": {"UserService": {"type": "class", "methods": {"getUser": {"inputs": [{"name": "id", "type": "string"}], "returns": {"type": "User"}}}}}, "dependencies": {"types": "^1.0.0"}}"#;
    fs::write(dir.path().join("modules/service.json"), module_service).unwrap();
    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();
    assert!(result.valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_empty_project_validation() {
    let dir = tempdir().unwrap();
    let manifest =
        r#"{"version": "0.1.0", "project": {"name": "empty", "language": "rust"}, "modules": []}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();
    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();
    assert!(result.valid);
    assert!(result.errors.is_empty());
}

// =============================================================================
// Event Type Validation Tests
// =============================================================================

#[test]
fn test_event_type_valid() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "typescript"}, "modules": ["events"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    // Valid event with payload
    let events_module = r#"{
        "module": "events",
        "version": "1.0.0",
        "exports": {
            "UserCreated": {
                "type": "event",
                "payload": {
                    "userId": {"type": "string", "required": true},
                    "email": {"type": "string", "required": true},
                    "createdAt": {"type": "Date", "required": true}
                }
            }
        },
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/events.json"), events_module).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    assert!(result.valid, "Event type should be valid: {:?}", result.errors);
}

#[test]
fn test_event_with_methods_warns() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "typescript"}, "modules": ["events"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    // Event with methods (should warn)
    let events_module = r#"{
        "module": "events",
        "version": "1.0.0",
        "exports": {
            "BadEvent": {
                "type": "event",
                "payload": {
                    "data": {"type": "string", "required": true}
                },
                "methods": {
                    "doSomething": {
                        "inputs": [],
                        "returns": {"type": "void"}
                    }
                }
            }
        },
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/events.json"), events_module).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    // Should have a warning about event having methods
    assert!(!result.warnings.is_empty(), "Should warn about event with methods");
    assert!(result.warnings.iter().any(|w| w.rule == "event-structure"));
}

#[test]
fn test_event_payload_type_validation() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "typescript"}, "modules": ["events"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    // Event with invalid payload type
    let events_module = r#"{
        "module": "events",
        "version": "1.0.0",
        "exports": {
            "BadEvent": {
                "type": "event",
                "payload": {
                    "data": {"type": "NonExistentType", "required": true}
                }
            }
        },
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/events.json"), events_module).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    assert!(!result.valid, "Event with non-existent payload type should fail");
    assert!(result.errors.iter().any(|e| e.rule == "all-types-must-exist"));
}

// =============================================================================
// Trait Type Validation Tests
// =============================================================================

#[test]
fn test_trait_type_valid() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "typescript"}, "modules": ["traits"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    // Valid trait with async methods
    let traits_module = r#"{
        "module": "traits",
        "version": "1.0.0",
        "exports": {
            "Repository": {
                "type": "trait",
                "methods": {
                    "findById": {
                        "inputs": [{"name": "id", "type": "string"}],
                        "returns": {"type": "object"},
                        "async": true
                    },
                    "save": {
                        "inputs": [{"name": "entity", "type": "object"}],
                        "returns": {"type": "void"},
                        "async": true
                    }
                }
            }
        },
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/traits.json"), traits_module).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    assert!(result.valid, "Trait type should be valid: {:?}", result.errors);
}

#[test]
fn test_trait_without_methods_warns() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "typescript"}, "modules": ["traits"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    // Trait without methods (should warn)
    let traits_module = r#"{
        "module": "traits",
        "version": "1.0.0",
        "exports": {
            "EmptyTrait": {
                "type": "trait",
                "methods": {}
            }
        },
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/traits.json"), traits_module).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    // Should have a warning about trait without methods
    assert!(!result.warnings.is_empty(), "Should warn about trait without methods");
    assert!(result.warnings.iter().any(|w| w.rule == "trait-structure"));
}

#[test]
fn test_trait_with_properties_warns() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "typescript"}, "modules": ["traits"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    // Trait with properties (should warn)
    let traits_module = r#"{
        "module": "traits",
        "version": "1.0.0",
        "exports": {
            "BadTrait": {
                "type": "trait",
                "methods": {
                    "doSomething": {
                        "inputs": [],
                        "returns": {"type": "void"}
                    }
                },
                "properties": {
                    "name": {"type": "string", "required": true}
                }
            }
        },
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/traits.json"), traits_module).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    // Should have a warning about trait with properties
    assert!(!result.warnings.is_empty(), "Should warn about trait with properties");
    assert!(result.warnings.iter().any(|w| w.rule == "trait-structure"));
}

#[test]
fn test_trait_with_payload_errors() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "typescript"}, "modules": ["traits"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    // Trait with payload (should error - payload is for events only)
    let traits_module = r#"{
        "module": "traits",
        "version": "1.0.0",
        "exports": {
            "BadTrait": {
                "type": "trait",
                "methods": {
                    "doSomething": {
                        "inputs": [],
                        "returns": {"type": "void"}
                    }
                },
                "payload": {
                    "data": {"type": "string", "required": true}
                }
            }
        },
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/traits.json"), traits_module).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    // Should have an error about trait with payload
    assert!(!result.valid, "Trait with payload should fail validation");
    assert!(result.errors.iter().any(|e| e.rule == "trait-structure"));
}

#[test]
fn test_non_event_with_payload_errors() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "typescript"}, "modules": ["types"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    // Interface with payload (should error - payload is for events only)
    let types_module = r#"{
        "module": "types",
        "version": "1.0.0",
        "exports": {
            "BadInterface": {
                "type": "interface",
                "properties": {
                    "name": {"type": "string", "required": true}
                },
                "payload": {
                    "data": {"type": "string", "required": true}
                }
            }
        },
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/types.json"), types_module).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    // Should have an error about non-event with payload
    assert!(!result.valid, "Non-event with payload should fail validation");
    assert!(result.errors.iter().any(|e| e.rule == "export-structure"));
}
