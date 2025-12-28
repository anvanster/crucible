//! Error types for compliance validation

use thiserror::Error;

/// Result type for compliance operations
pub type ComplianceResult<T> = Result<T, ComplianceError>;

/// Errors that can occur during compliance validation
#[derive(Error, Debug)]
pub enum ComplianceError {
    #[error("Failed to load compliance framework: {0}")]
    FrameworkLoadError(String),

    #[error("Invalid compliance framework definition: {0}")]
    InvalidFramework(String),

    #[error("Compliance framework not found: {0}")]
    FrameworkNotFound(String),

    #[error("Failed to parse compliance file {path}: {message}")]
    ParseError { path: String, message: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Crucible core error: {0}")]
    CoreError(#[from] crucible_core::error::CrucibleError),

    #[error("Validation rule error in {rule_id}: {message}")]
    RuleError { rule_id: String, message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ComplianceError::FrameworkNotFound("HIPAA".to_string());
        assert_eq!(err.to_string(), "Compliance framework not found: HIPAA");
    }

    #[test]
    fn test_parse_error_display() {
        let err = ComplianceError::ParseError {
            path: "hipaa.json".to_string(),
            message: "invalid JSON".to_string(),
        };
        assert!(err.to_string().contains("hipaa.json"));
        assert!(err.to_string().contains("invalid JSON"));
    }
}
