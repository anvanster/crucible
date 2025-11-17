use crucible_core::{Parser, Validator};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_all_calls_must_exist_valid() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "rust"}, "modules": ["math"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    let module_math = r#"{
        "module": "math",
        "version": "1.0.0",
        "exports": {
            "add": {
                "type": "function",
                "methods": {
                    "add": {
                        "inputs": [{"name": "a", "type": "number"}, {"name": "b", "type": "number"}],
                        "returns": {"type": "number"},
                        "throws": [],
                        "calls": [],
                        "effects": []
                    }
                }
            },
            "Calculator": {
                "type": "class",
                "methods": {
                    "calculate": {
                        "inputs": [{"name": "a", "type": "number"}, {"name": "b", "type": "number"}],
                        "returns": {"type": "number"},
                        "throws": [],
                        "calls": ["math.add"],
                        "effects": []
                    }
                }
            }
        },
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/math.json"), module_math).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    // Print errors for debugging
    for error in &result.errors {
        eprintln!("✗ {}: {}", error.rule, error.message);
        if let Some(loc) = &error.location {
            eprintln!("  at {loc}");
        }
    }

    assert!(result.valid, "Expected validation to pass");
    assert!(result.errors.is_empty());
}

#[test]
fn test_all_calls_must_exist_invalid() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "rust"}, "modules": ["math"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    let module_math = r#"{
        "module": "math",
        "version": "1.0.0",
        "exports": {
            "Calculator": {
                "type": "class",
                "methods": {
                    "calculate": {
                        "inputs": [],
                        "returns": {"type": "number"},
                        "throws": [],
                        "calls": ["math.nonexistent"],
                        "effects": []
                    }
                }
            }
        },
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/math.json"), module_math).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    assert!(!result.valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.rule == "all-calls-must-exist"));
}

#[test]
fn test_used_dependencies_declared_valid() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "rust"}, "modules": ["a", "b"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    let module_a = r#"{"module": "a", "version": "1.0.0", "exports": {"foo": {"type": "function", "methods": {"foo": {"inputs": [], "returns": {"type": "void"}, "calls": [], "effects": []}}}}, "dependencies": {}}"#;
    let module_b = r#"{"module": "b", "version": "1.0.0", "exports": {"Bar": {"type": "class", "methods": {"doIt": {"inputs": [], "returns": {"type": "void"}, "calls": ["a.foo"], "effects": []}}}}, "dependencies": {"a": "^1.0.0"}}"#;

    fs::write(dir.path().join("modules/a.json"), module_a).unwrap();
    fs::write(dir.path().join("modules/b.json"), module_b).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    assert!(result.valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_used_dependencies_declared_invalid() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "rust"}, "modules": ["a", "b"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    let module_a = r#"{"module": "a", "version": "1.0.0", "exports": {"foo": {"type": "function", "methods": {"foo": {"inputs": [], "returns": {"type": "void"}, "calls": [], "effects": []}}}}, "dependencies": {}}"#;
    let module_b = r#"{"module": "b", "version": "1.0.0", "exports": {"Bar": {"type": "class", "methods": {"doIt": {"inputs": [], "returns": {"type": "void"}, "calls": ["a.foo"], "effects": []}}}}, "dependencies": {}}"#;

    fs::write(dir.path().join("modules/a.json"), module_a).unwrap();
    fs::write(dir.path().join("modules/b.json"), module_b).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    assert!(!result.valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.rule == "used-dependencies-declared"));
}

#[test]
fn test_declared_dependencies_must_be_used_warning() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "rust"}, "modules": ["a", "b"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    let module_a = r#"{"module": "a", "version": "1.0.0", "exports": {"foo": {"type": "function", "methods": {"foo": {"inputs": [], "returns": {"type": "void"}, "calls": [], "effects": []}}}}, "dependencies": {}}"#;
    let module_b = r#"{"module": "b", "version": "1.0.0", "exports": {"Bar": {"type": "class", "methods": {"doIt": {"inputs": [], "returns": {"type": "void"}, "calls": [], "effects": []}}}}, "dependencies": {"a": "^1.0.0"}}"#;

    fs::write(dir.path().join("modules/a.json"), module_a).unwrap();
    fs::write(dir.path().join("modules/b.json"), module_b).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    assert!(result.valid); // Should be valid (only warning)
    assert!(result
        .warnings
        .iter()
        .any(|w| w.rule == "declared-dependencies-must-be-used"));
}

#[test]
fn test_self_calls_allowed() {
    let dir = tempdir().unwrap();
    let manifest = r#"{"version": "0.1.0", "project": {"name": "test", "language": "rust"}, "modules": ["parser"]}"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    fs::create_dir(dir.path().join("modules")).unwrap();

    let module_parser = r#"{
        "module": "parser",
        "version": "1.0.0",
        "exports": {
            "Parser": {
                "type": "class",
                "methods": {
                    "parse_project": {
                        "inputs": [],
                        "returns": {"type": "void"},
                        "throws": [],
                        "calls": ["parser.parse_manifest", "parser.parse_modules"],
                        "effects": []
                    },
                    "parse_manifest": {
                        "inputs": [],
                        "returns": {"type": "void"},
                        "throws": [],
                        "calls": [],
                        "effects": []
                    },
                    "parse_modules": {
                        "inputs": [],
                        "returns": {"type": "void"},
                        "throws": [],
                        "calls": [],
                        "effects": []
                    }
                }
            }
        },
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/parser.json"), module_parser).unwrap();

    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();

    assert!(result.valid);
    assert!(result.errors.is_empty());
}
