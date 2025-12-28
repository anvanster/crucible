//! Crucible Compliance - HIPAA/PCI-DSS/SOC2 validation for architecture definitions
//!
//! This crate provides compliance validation capabilities for Crucible projects,
//! enabling automated verification of architectural definitions against regulatory
//! frameworks like HIPAA, PCI-DSS, and SOC2.

pub mod error;
pub mod framework;
pub mod loader;
pub mod reporter;
pub mod validator;

pub use error::{ComplianceError, ComplianceResult};
pub use framework::Framework;
pub use loader::FrameworkLoader;
pub use reporter::{OutputFormat, ReportConfig, Reporter};
pub use validator::{ComplianceValidator, ValidationReport, Violation};

// Re-export types from crucible-core for convenience
pub use crucible_core::types::{
    ComplianceExamples, ComplianceFramework, ComplianceRequirement, ComplianceRule, Severity,
    ValidationCheck, ValidationCheckType,
};
