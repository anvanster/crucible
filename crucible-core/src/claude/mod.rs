//! Claude Code integration for Crucible
//!
//! This module provides seamless integration between Crucible architecture
//! definitions and Claude Code, enabling architecture-aware AI-assisted development.

pub mod config;
pub mod context;
pub mod discovery;
pub mod rust_parser;
pub mod sync;
pub mod templates;
pub mod validation;

pub use config::{IntegrationConfig, IntegrationMode, ValidationLevel};
pub use context::ContextGenerator;
pub use discovery::ArchitectureDiscovery;
pub use rust_parser::{DiscoveredModule, RustParser};
pub use sync::{SyncManager, SyncReport};
pub use templates::TemplateEngine;
pub use validation::{SuggestionType, ValidationHooks, ValidationSuggestion};
