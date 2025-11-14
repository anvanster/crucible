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
