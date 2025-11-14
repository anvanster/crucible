//! Crucible Core - Architecture validation engine
//!
//! This library implements the Crucible specification for
//! AI-native application architecture.

pub mod error;
pub mod parser;
pub mod types;
pub mod validator;
pub mod graph;
pub mod generator;

pub use error::{CrucibleError, Result};
pub use parser::Parser;
pub use types::{Manifest, Module, Project};
pub use validator::{ValidationResult, Validator};
pub use generator::Generator;

/// Version of the Crucible specification this library implements
pub const SPEC_VERSION: &str = "0.1.0";
