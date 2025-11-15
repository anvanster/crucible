//! Configuration management for Claude Code integration

use crate::error::{CrucibleError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Integration mode determines the level of Claude Code engagement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntegrationMode {
    /// Read-only architecture awareness
    Basic,
    /// Active validation and sync
    Enhanced,
    /// Enforce all architectural rules
    Strict,
}

impl Default for IntegrationMode {
    fn default() -> Self {
        Self::Enhanced
    }
}

/// Validation severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValidationLevel {
    Error,
    Warning,
    Info,
}

impl Default for ValidationLevel {
    fn default() -> Self {
        Self::Warning
    }
}

/// Output format for generated files
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OutputFormat {
    Json,
    Yaml,
    Markdown,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Json
    }
}

/// Features configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    pub auto_validation: bool,
    pub sync_on_change: bool,
    pub suggest_improvements: bool,
    pub track_violations: bool,
    pub generate_types: bool,
}

impl Default for Features {
    fn default() -> Self {
        Self {
            auto_validation: true,
            sync_on_change: true,
            suggest_improvements: true,
            track_violations: true,
            generate_types: true,
        }
    }
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub pre_commit: bool,
    pub on_save: bool,
    pub on_generate: bool,
    pub severity: ValidationLevel,
    pub rules: HashMap<String, ValidationLevel>,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        let mut rules = HashMap::new();
        rules.insert("layer_boundaries".to_string(), ValidationLevel::Error);
        rules.insert("circular_dependencies".to_string(), ValidationLevel::Error);
        rules.insert(
            "undeclared_dependencies".to_string(),
            ValidationLevel::Warning,
        );
        rules.insert("missing_exports".to_string(), ValidationLevel::Warning);
        rules.insert("type_mismatches".to_string(), ValidationLevel::Error);

        Self {
            pre_commit: true,
            on_save: false,
            on_generate: true,
            severity: ValidationLevel::Warning,
            rules,
        }
    }
}

/// Sync configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub strategy: String,
    pub auto_sync: bool,
    pub conflict_resolution: String,
    pub ignore_patterns: Vec<String>,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            strategy: "bidirectional".to_string(),
            auto_sync: true,
            conflict_resolution: "prompt".to_string(),
            ignore_patterns: vec![
                "*.test.ts".to_string(),
                "*.spec.ts".to_string(),
                "__tests__/**".to_string(),
            ],
        }
    }
}

/// Context optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    pub max_tokens: usize,
    pub include_descriptions: bool,
    pub include_examples: bool,
    pub focus_mode: String,
    pub optimization: String,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            max_tokens: 4000,
            include_descriptions: true,
            include_examples: false,
            focus_mode: "auto".to_string(),
            optimization: "aggressive".to_string(),
        }
    }
}

/// Main integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub version: String,
    pub mode: IntegrationMode,
    pub project: ProjectInfo,
    pub features: Features,
    pub validation: ValidationConfig,
    pub sync: SyncConfig,
    pub templates: HashMap<String, String>,
    pub context: ContextConfig,
    pub commands: HashMap<String, String>,
    pub ui: UiConfig,
}

/// Project information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub root: String,
    pub crucible_dir: String,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub show_hints: bool,
    pub inline_validation: bool,
    pub highlight_violations: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_hints: true,
            inline_validation: true,
            highlight_violations: true,
        }
    }
}

impl IntegrationConfig {
    /// Create a new configuration with the given mode
    pub fn new(mode: IntegrationMode, project_name: &str, project_root: &Path) -> Self {
        let mut templates = HashMap::new();
        templates.insert("service".to_string(), "templates/service.hbs".to_string());
        templates.insert(
            "repository".to_string(),
            "templates/repository.hbs".to_string(),
        );
        templates.insert(
            "controller".to_string(),
            "templates/controller.hbs".to_string(),
        );
        templates.insert("module".to_string(), "templates/module.hbs".to_string());

        let mut commands = HashMap::new();
        commands.insert("validate".to_string(), "crucible validate".to_string());
        commands.insert("sync".to_string(), "crucible claude sync".to_string());
        commands.insert("generate".to_string(), "crucible generate".to_string());

        Self {
            version: "0.1.0".to_string(),
            mode,
            project: ProjectInfo {
                name: project_name.to_string(),
                root: project_root.display().to_string(),
                crucible_dir: project_root.join(".crucible").display().to_string(),
            },
            features: Features::default(),
            validation: ValidationConfig::default(),
            sync: SyncConfig::default(),
            templates,
            context: ContextConfig::default(),
            commands,
            ui: UiConfig::default(),
        }
    }

    /// Load configuration from a file
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path).map_err(|e| CrucibleError::FileRead {
            path: path.display().to_string(),
            source: e,
        })?;

        serde_json::from_str(&content).map_err(|e| CrucibleError::ParseError {
            file: path.display().to_string(),
            message: e.to_string(),
        })
    }

    /// Save configuration to a file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content =
            serde_json::to_string_pretty(self).map_err(|e| CrucibleError::ParseError {
                file: path.display().to_string(),
                message: e.to_string(),
            })?;

        fs::write(path, content).map_err(|e| CrucibleError::FileRead {
            path: path.display().to_string(),
            source: e,
        })?;

        Ok(())
    }

    /// Write all Claude integration files
    pub fn write_claude_files(&self, project_root: &Path) -> Result<()> {
        let claude_dir = project_root.join(".claude");
        let crucible_dir = claude_dir.join("crucible");

        // Create directories
        fs::create_dir_all(&crucible_dir).map_err(|e| CrucibleError::FileRead {
            path: crucible_dir.display().to_string(),
            source: e,
        })?;

        // Write config.json
        let config_path = crucible_dir.join("config.json");
        self.save(&config_path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_integration_config_new() {
        let dir = tempdir().unwrap();
        let config = IntegrationConfig::new(IntegrationMode::Enhanced, "test-project", dir.path());

        assert_eq!(config.mode, IntegrationMode::Enhanced);
        assert_eq!(config.project.name, "test-project");
        assert!(config.features.auto_validation);
    }

    #[test]
    fn test_integration_config_save_load() {
        let dir = tempdir().unwrap();
        let config = IntegrationConfig::new(IntegrationMode::Strict, "test-project", dir.path());

        let config_path = dir.path().join("config.json");
        config.save(&config_path).unwrap();

        let loaded = IntegrationConfig::load(&config_path).unwrap();
        assert_eq!(loaded.mode, IntegrationMode::Strict);
        assert_eq!(loaded.project.name, "test-project");
    }

    #[test]
    fn test_default_validation_rules() {
        let config = ValidationConfig::default();
        assert_eq!(
            config.rules.get("layer_boundaries"),
            Some(&ValidationLevel::Error)
        );
        assert_eq!(
            config.rules.get("circular_dependencies"),
            Some(&ValidationLevel::Error)
        );
    }
}
