//! Error types for Crucible

use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CrucibleError>;

#[derive(Error, Debug)]
pub enum CrucibleError {
    #[error("Failed to read file {path}: {source}")]
    FileRead {
        path: String,
        #[source]
        source: io::Error,
    },

    #[error("Failed to parse {file}: {message}")]
    ParseError { file: String, message: String },

    #[error("Module not found: {name}")]
    ModuleNotFound { name: String },

    #[error("Export not found: {module}.{export}")]
    ExportNotFound { module: String, export: String },

    #[error("Circular dependency detected: {cycle}")]
    CircularDependency { cycle: String },

    #[error("Layer boundary violation: {from} -> {to}")]
    LayerViolation { from: String, to: String },

    #[error("Type not found: {type_name}")]
    TypeNotFound { type_name: String },

    #[error("Function call target not found: {call}")]
    CallTargetNotFound { call: String },

    #[error("Validation failed: {message}")]
    ValidationFailed { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_file_read_error_display() {
        let err = CrucibleError::FileRead {
            path: "/path/to/file.json".to_string(),
            source: io::Error::new(io::ErrorKind::NotFound, "file not found"),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("/path/to/file.json"));
        assert!(msg.contains("Failed to read file"));
    }

    #[test]
    fn test_parse_error_display() {
        let err = CrucibleError::ParseError {
            file: "manifest.json".to_string(),
            message: "invalid JSON".to_string(),
        };
        assert_eq!(
            format!("{}", err),
            "Failed to parse manifest.json: invalid JSON"
        );
    }

    #[test]
    fn test_module_not_found_error() {
        let err = CrucibleError::ModuleNotFound {
            name: "test-module".to_string(),
        };
        assert_eq!(format!("{}", err), "Module not found: test-module");
    }

    #[test]
    fn test_export_not_found_error() {
        let err = CrucibleError::ExportNotFound {
            module: "mymodule".to_string(),
            export: "MyClass".to_string(),
        };
        assert_eq!(format!("{}", err), "Export not found: mymodule.MyClass");
    }

    #[test]
    fn test_circular_dependency_error() {
        let err = CrucibleError::CircularDependency {
            cycle: "A -> B -> C -> A".to_string(),
        };
        assert_eq!(
            format!("{}", err),
            "Circular dependency detected: A -> B -> C -> A"
        );
    }

    #[test]
    fn test_layer_violation_error() {
        let err = CrucibleError::LayerViolation {
            from: "domain".to_string(),
            to: "presentation".to_string(),
        };
        assert_eq!(
            format!("{}", err),
            "Layer boundary violation: domain -> presentation"
        );
    }

    #[test]
    fn test_type_not_found_error() {
        let err = CrucibleError::TypeNotFound {
            type_name: "UnknownType".to_string(),
        };
        assert_eq!(format!("{}", err), "Type not found: UnknownType");
    }

    #[test]
    fn test_call_target_not_found_error() {
        let err = CrucibleError::CallTargetNotFound {
            call: "someFunction".to_string(),
        };
        assert_eq!(
            format!("{}", err),
            "Function call target not found: someFunction"
        );
    }

    #[test]
    fn test_validation_failed_error() {
        let err = CrucibleError::ValidationFailed {
            message: "Multiple rules violated".to_string(),
        };
        assert_eq!(
            format!("{}", err),
            "Validation failed: Multiple rules violated"
        );
    }

    #[test]
    fn test_error_debug() {
        let err = CrucibleError::ModuleNotFound {
            name: "test".to_string(),
        };
        let debug = format!("{:?}", err);
        assert!(debug.contains("ModuleNotFound"));
    }
}
