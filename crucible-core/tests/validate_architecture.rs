//! Test to validate the architecture definitions for Stage 5 performance features

use crucible_core::{Parser, Validator};

#[test]
fn validate_performance_architecture() {
    // Parse the architecture from .crucible directory
    let parser = Parser::new(".crucible");

    let project = parser.parse_project()
        .expect("Failed to parse architecture definitions");

    // Validate the architecture
    let validator = Validator::new(project);
    let result = validator.validate();

    // Print any issues for debugging
    if !result.errors.is_empty() {
        eprintln!("Validation errors:");
        for error in &result.errors {
            eprintln!("  - [{}] {}: {}",
                     error.rule,
                     error.location.as_ref().unwrap_or(&"global".to_string()),
                     error.message);
        }
    }

    if !result.warnings.is_empty() {
        eprintln!("Validation warnings:");
        for warning in &result.warnings {
            eprintln!("  - [{}] {}: {}",
                     warning.rule,
                     warning.location.as_ref().unwrap_or(&"global".to_string()),
                     warning.message);
        }
    }

    if !result.info.is_empty() {
        eprintln!("Validation info:");
        for info in &result.info {
            eprintln!("  - [{}] {}: {}",
                     info.rule,
                     info.location.as_ref().unwrap_or(&"global".to_string()),
                     info.message);
        }
    }

    // Assert validation passes
    assert!(result.valid, "Architecture validation failed");

    println!("✅ Architecture validation passed!");
    println!("   Validated {} modules", result.validated_modules.len());
}