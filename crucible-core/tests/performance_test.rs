//! Tests for performance improvements in Stage 5

use crucible_core::claude::config::{IntegrationConfig, IntegrationMode, ValidationLevel};
use crucible_core::parser::Parser;
use crucible_core::validator::{ChangeTracker, Validator};
use crucible_core::{Module, Project};
use std::collections::HashMap;
use std::env;
use std::fs;
use tempfile::TempDir;

/// Test that caching improves parse performance
#[test]
fn test_caching_performance() {
    let dir = TempDir::new().unwrap();
    let modules_dir = dir.path().join("modules");
    fs::create_dir(&modules_dir).unwrap();

    // Create a test module
    let module_content = r#"{
        "module": "test",
        "version": "1.0.0",
        "layer": "core",
        "description": "Test module",
        "exports": {},
        "dependencies": {}
    }"#;

    fs::write(modules_dir.join("test.json"), module_content).unwrap();

    let parser = Parser::new(dir.path());

    // First parse should take full time
    let start = std::time::Instant::now();
    let module1 = parser.parse_module("test").unwrap();
    let first_parse_time = start.elapsed();

    // Second parse should be faster due to cache
    let start = std::time::Instant::now();
    let module2 = parser.parse_module("test").unwrap();
    let cached_parse_time = start.elapsed();

    // Verify modules are identical
    assert_eq!(module1.module, module2.module);

    // Cached parse should be significantly faster (at least 2x)
    // In practice it's usually 10-100x faster, but we use 2x for test stability
    assert!(
        cached_parse_time < first_parse_time / 2,
        "Cached parse should be at least 2x faster. First: {:?}, Cached: {:?}",
        first_parse_time,
        cached_parse_time
    );

    // Verify cache stats
    let stats = parser.cache_stats();
    assert!(stats.enabled);
    assert_eq!(stats.modules_cached, 1);
}

/// Test environment variable overrides
#[test]
fn test_env_overrides() {
    // Set environment variables
    env::set_var("CRUCIBLE_CLAUDE_MODE", "strict");
    env::set_var("CRUCIBLE_VALIDATION", "error");
    env::set_var("CRUCIBLE_AUTO_SYNC", "false");
    env::set_var("CRUCIBLE_MAX_TOKENS", "8000");
    env::set_var("CRUCIBLE_CACHE_ENABLED", "false");
    env::set_var("CRUCIBLE_INCREMENTAL", "false");

    let mut config = IntegrationConfig::default();
    config.apply_env_overrides();

    assert_eq!(config.mode, IntegrationMode::Strict);
    assert_eq!(config.validation.severity, ValidationLevel::Error);
    assert_eq!(config.sync.auto_sync, false);
    assert_eq!(config.context.max_tokens, 8000);
    assert_eq!(config.performance.enable_caching, false);
    assert_eq!(config.performance.incremental_validation, false);

    // Clean up
    env::remove_var("CRUCIBLE_CLAUDE_MODE");
    env::remove_var("CRUCIBLE_VALIDATION");
    env::remove_var("CRUCIBLE_AUTO_SYNC");
    env::remove_var("CRUCIBLE_MAX_TOKENS");
    env::remove_var("CRUCIBLE_CACHE_ENABLED");
    env::remove_var("CRUCIBLE_INCREMENTAL");
}

/// Test incremental validation
#[test]
fn test_incremental_validation() {
    let dir = TempDir::new().unwrap();
    let modules_dir = dir.path().join("modules");
    fs::create_dir(&modules_dir).unwrap();

    // Create manifest
    let manifest_content = r#"{
        "version": "0.1.0",
        "project": {
            "name": "test-project",
            "language": "rust",
            "architecture_pattern": "layered"
        },
        "modules": ["module1", "module2", "module3"],
        "strict_validation": false
    }"#;
    fs::write(dir.path().join("manifest.json"), manifest_content).unwrap();

    // Create three modules with dependencies
    let module1_content = r#"{
        "module": "module1",
        "version": "1.0.0",
        "layer": "core",
        "description": "Core module",
        "exports": {},
        "dependencies": {}
    }"#;

    let module2_content = r#"{
        "module": "module2",
        "version": "1.0.0",
        "layer": "business",
        "description": "Business module",
        "exports": {},
        "dependencies": {
            "module1": "1.0.0"
        }
    }"#;

    let module3_content = r#"{
        "module": "module3",
        "version": "1.0.0",
        "layer": "presentation",
        "description": "Presentation module",
        "exports": {},
        "dependencies": {
            "module2": "1.0.0"
        }
    }"#;

    fs::write(modules_dir.join("module1.json"), module1_content).unwrap();
    fs::write(modules_dir.join("module2.json"), module2_content).unwrap();
    fs::write(modules_dir.join("module3.json"), module3_content).unwrap();

    // Parse project
    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();

    // Create validator with incremental support
    let mut validator = Validator::new_with_incremental(project);

    // First validation should validate all modules
    let result1 = validator.incremental_validate(dir.path());
    assert_eq!(result1.validated_modules.len(), 3);

    // Second validation with no changes should validate nothing
    let result2 = validator.incremental_validate(dir.path());
    assert_eq!(result2.validated_modules.len(), 0);
    assert!(result2
        .info
        .iter()
        .any(|i| i.message.contains("No modules changed")));

    // Modify module1 (which module2 and module3 depend on)
    std::thread::sleep(std::time::Duration::from_millis(10)); // Ensure timestamp changes
    let module1_updated = r#"{
        "module": "module1",
        "version": "1.0.1",
        "layer": "core",
        "description": "Updated core module",
        "exports": {},
        "dependencies": {}
    }"#;
    fs::write(modules_dir.join("module1.json"), module1_updated).unwrap();

    // Re-parse project with changes (simulating loading in a new session)
    let parser = Parser::new(dir.path());
    let updated_project = parser.parse_project().unwrap();
    let mut validator = Validator::new_with_incremental(updated_project);

    // Incremental validation with fresh validator should validate all modules (no prior state)
    let result3 = validator.incremental_validate(dir.path());

    // Since this is a fresh validator, all 3 modules are considered "changed"
    assert_eq!(result3.validated_modules.len(), 3);
    assert!(result3
        .info
        .iter()
        .any(|i| i.message.contains("3 modules changed, 3 modules validated")));
}

/// Test change tracker dependency graph
#[test]
fn test_change_tracker_dependencies() {
    // Create a test project with dependencies
    let mut modules = Vec::new();

    // Module A (no dependencies)
    modules.push(Module {
        module: "A".to_string(),
        version: "1.0.0".to_string(),
        layer: Some("core".to_string()),
        description: Some("Module A".to_string()),
        exports: HashMap::new(),
        dependencies: HashMap::new(),
    });

    // Module B depends on A
    let mut b_deps = HashMap::new();
    b_deps.insert("A".to_string(), "1.0.0".to_string());
    modules.push(Module {
        module: "B".to_string(),
        version: "1.0.0".to_string(),
        layer: Some("business".to_string()),
        description: Some("Module B".to_string()),
        exports: HashMap::new(),
        dependencies: b_deps,
    });

    // Module C depends on B
    let mut c_deps = HashMap::new();
    c_deps.insert("B".to_string(), "1.0.0".to_string());
    modules.push(Module {
        module: "C".to_string(),
        version: "1.0.0".to_string(),
        layer: Some("presentation".to_string()),
        description: Some("Module C".to_string()),
        exports: HashMap::new(),
        dependencies: c_deps,
    });

    // Module D depends on A
    let mut d_deps = HashMap::new();
    d_deps.insert("A".to_string(), "1.0.0".to_string());
    modules.push(Module {
        module: "D".to_string(),
        version: "1.0.0".to_string(),
        layer: Some("business".to_string()),
        description: Some("Module D".to_string()),
        exports: HashMap::new(),
        dependencies: d_deps,
    });

    let project = Project {
        manifest: crucible_core::Manifest {
            version: "0.1.0".to_string(),
            project: crucible_core::types::ProjectConfig {
                name: "test".to_string(),
                language: crucible_core::types::Language::Rust,
                architecture_pattern: Some(crucible_core::types::ArchitecturePattern::Layered),
            },
            modules: vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()],
            strict_validation: false,
            metadata: None,
        },
        modules,
        rules: None,
    };

    let mut tracker = ChangeTracker::new();
    tracker.build_dependency_graph(&project);

    // Test: Changing A should affect B, C (through B), and D
    let mut changed = HashMap::new();
    changed.insert("A".to_string(), true);

    let affected = tracker.get_affected_modules(&changed);
    assert_eq!(affected.len(), 4); // A, B, C, D
    assert!(affected.contains_key("A"));
    assert!(affected.contains_key("B"));
    assert!(affected.contains_key("C"));
    assert!(affected.contains_key("D"));

    // Test: Changing B should affect C
    let mut changed = HashMap::new();
    changed.insert("B".to_string(), true);

    let affected = tracker.get_affected_modules(&changed);
    assert_eq!(affected.len(), 2); // B, C
    assert!(affected.contains_key("B"));
    assert!(affected.contains_key("C"));

    // Test: Changing C should only affect C (no dependents)
    let mut changed = HashMap::new();
    changed.insert("C".to_string(), true);

    let affected = tracker.get_affected_modules(&changed);
    assert_eq!(affected.len(), 1); // Only C
    assert!(affected.contains_key("C"));
}

/// Test global configuration loading (if ~/.claude/crucible/global.json exists)
#[test]
fn test_global_config_loading() {
    // This test only runs if global config exists
    if let Some(home) = dirs::home_dir() {
        let global_path = home.join(".claude").join("crucible").join("global.json");

        if global_path.exists() {
            // Test that global config is loaded
            let _config = IntegrationConfig::load_with_overrides(None).unwrap();

            // Global config should have been applied
            // We can't assert specific values since we don't know what's in the user's global config
            // But we can verify the loading mechanism works
            println!("Global config loaded successfully");
        } else {
            // Create a temporary global config for testing
            let config_dir = home.join(".claude").join("crucible");
            if !config_dir.exists() {
                fs::create_dir_all(&config_dir).ok();
            }

            let test_global = r#"{
                "enabled": true,
                "auto_discover": true,
                "default_mode": "strict",
                "default_validation_level": "error",
                "template_paths": [],
                "performance": {
                    "enable_caching": false,
                    "cache_ttl_seconds": 1800,
                    "lazy_loading": false,
                    "incremental_validation": false,
                    "max_parallel_modules": 2
                }
            }"#;

            // Write test global config
            if fs::write(&global_path, test_global).is_ok() {
                // Load config with global overrides
                let config = IntegrationConfig::load_with_overrides(None).unwrap();

                // Verify global settings were applied
                assert_eq!(config.mode, IntegrationMode::Strict);
                assert_eq!(config.validation.severity, ValidationLevel::Error);
                assert_eq!(config.performance.enable_caching, false);
                assert_eq!(config.performance.cache_ttl_seconds, 1800);
                assert_eq!(config.performance.max_parallel_modules, 2);

                // Clean up test file
                fs::remove_file(global_path).ok();
            }
        }
    }
}