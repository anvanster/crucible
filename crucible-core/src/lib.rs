//! Crucible Core - Architecture validation engine
//!
//! This library implements the Crucible specification for
//! AI-native application architecture.

pub mod cache;
pub mod claude;
pub mod error;
pub mod generator;
pub mod graph;
pub mod parser;
pub mod types;
pub mod validator;

pub use error::{CrucibleError, Result};
pub use generator::Generator;
pub use parser::Parser;
pub use types::{Manifest, Module, Project};
pub use validator::{ChangeTracker, ValidationResult, Validator};

/// Version of the Crucible specification this library implements
pub const SPEC_VERSION: &str = "0.1.0";
