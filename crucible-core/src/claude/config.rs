//! Configuration management for Claude Code integration with global config and environment variables

use crate::error::{CrucibleError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
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

impl IntegrationMode {
    /// Parse from environment variable value
    pub fn from_env_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "basic" => Some(Self::Basic),
            "enhanced" => Some(Self::Enhanced),
            "strict" => Some(Self::Strict),
            _ => None,
        }
    }
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

impl ValidationLevel {
    /// Parse from environment variable value
    pub fn from_env_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "error" => Some(Self::Error),
            "warning" => Some(Self::Warning),
            "info" => Some(Self::Info),
            _ => None,
        }
    }
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
    /// Enable incremental validation (only validate changed modules and dependents)
    pub incremental: bool,
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
            incremental: true,
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
            focus_mode: "relevant".to_string(),
            optimization: "balanced".to_string(),
        }
    }
}

/// Templates configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub service: Option<String>,
    pub repository: Option<String>,
    pub controller: Option<String>,
    pub component: Option<String>,
    pub test: Option<String>,
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            service: Some(".crucible/templates/service.hbs".to_string()),
            repository: Some(".crucible/templates/repository.hbs".to_string()),
            controller: Some(".crucible/templates/controller.hbs".to_string()),
            component: Some(".crucible/templates/component.hbs".to_string()),
            test: Some(".crucible/templates/test.hbs".to_string()),
        }
    }
}

/// Global configuration for user-wide settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub enabled: bool,
    pub auto_discover: bool,
    pub default_mode: IntegrationMode,
    pub default_validation_level: ValidationLevel,
    pub template_paths: Vec<String>,
    pub performance: PerformanceConfig,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_discover: true,
            default_mode: IntegrationMode::Enhanced,
            default_validation_level: ValidationLevel::Warning,
            template_paths: vec![],
            performance: PerformanceConfig::default(),
        }
    }
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub enable_caching: bool,
    pub cache_ttl_seconds: u64,
    pub lazy_loading: bool,
    pub incremental_validation: bool,
    pub max_parallel_modules: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            cache_ttl_seconds: 3600,
            lazy_loading: true,
            incremental_validation: true,
            max_parallel_modules: 4,
        }
    }
}

/// Main integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub version: String,
    pub mode: IntegrationMode,
    pub features: Features,
    pub validation: ValidationConfig,
    pub sync: SyncConfig,
    pub context: ContextConfig,
    pub templates: TemplateConfig,
    pub performance: PerformanceConfig,
}

impl IntegrationConfig {
    /// Load configuration from a file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path).map_err(|e| CrucibleError::FileRead {
            path: path.display().to_string(),
            source: e,
        })?;

        serde_json::from_str(&content).map_err(|e| CrucibleError::ParseError {
            file: path.display().to_string(),
            message: format!("Invalid configuration: {}", e),
        })
    }

    /// Save configuration to a file
    pub fn to_file(&self, path: &Path) -> Result<()> {
        let content =
            serde_json::to_string_pretty(self).map_err(|e| CrucibleError::ParseError {
                file: path.display().to_string(),
                message: format!("Failed to serialize config: {}", e),
            })?;

        fs::write(path, content).map_err(|e| CrucibleError::FileRead {
            path: path.display().to_string(),
            source: e,
        })?;

        Ok(())
    }

    /// Load global configuration from ~/.claude/crucible/global.json
    pub fn load_global() -> Option<GlobalConfig> {
        let home = dirs::home_dir()?;
        let global_path = home.join(".claude").join("crucible").join("global.json");

        if global_path.exists() {
            let content = fs::read_to_string(&global_path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    /// Merge with global configuration
    pub fn merge_with_global(&mut self, global: &GlobalConfig) {
        // Only override if not explicitly set
        if self.mode == IntegrationMode::default() {
            self.mode = global.default_mode;
        }

        if self.validation.severity == ValidationLevel::default() {
            self.validation.severity = global.default_validation_level;
        }

        // Merge performance settings
        self.performance.enable_caching = global.performance.enable_caching;
        self.performance.cache_ttl_seconds = global.performance.cache_ttl_seconds;
        self.performance.lazy_loading = global.performance.lazy_loading;
        self.performance.incremental_validation = global.performance.incremental_validation;
        self.performance.max_parallel_modules = global.performance.max_parallel_modules;
    }

    /// Apply environment variable overrides
    pub fn apply_env_overrides(&mut self) {
        // CRUCIBLE_CLAUDE_MODE
        if let Ok(mode) = env::var("CRUCIBLE_CLAUDE_MODE") {
            if let Some(m) = IntegrationMode::from_env_str(&mode) {
                self.mode = m;
            }
        }

        // CRUCIBLE_VALIDATION
        if let Ok(level) = env::var("CRUCIBLE_VALIDATION") {
            if let Some(l) = ValidationLevel::from_env_str(&level) {
                self.validation.severity = l;
            }
        }

        // CRUCIBLE_AUTO_SYNC
        if let Ok(auto_sync) = env::var("CRUCIBLE_AUTO_SYNC") {
            self.sync.auto_sync = auto_sync.to_lowercase() == "true" || auto_sync == "1";
        }

        // CRUCIBLE_CACHE_ENABLED
        if let Ok(cache) = env::var("CRUCIBLE_CACHE_ENABLED") {
            self.performance.enable_caching = cache.to_lowercase() == "true" || cache == "1";
        }

        // CRUCIBLE_INCREMENTAL
        if let Ok(incremental) = env::var("CRUCIBLE_INCREMENTAL") {
            self.performance.incremental_validation =
                incremental.to_lowercase() == "true" || incremental == "1";
        }

        // CRUCIBLE_MAX_TOKENS
        if let Ok(tokens) = env::var("CRUCIBLE_MAX_TOKENS") {
            if let Ok(max_tokens) = tokens.parse::<usize>() {
                self.context.max_tokens = max_tokens;
            }
        }
    }

    /// Create configuration with all overrides applied
    pub fn load_with_overrides(path: Option<&Path>) -> Result<Self> {
        let mut config = if let Some(p) = path {
            Self::from_file(p)?
        } else {
            Self::default()
        };

        // Apply global config if it exists
        if let Some(global) = Self::load_global() {
            config.merge_with_global(&global);
        }

        // Apply environment variable overrides (highest priority)
        config.apply_env_overrides();

        Ok(config)
    }
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            mode: IntegrationMode::default(),
            features: Features::default(),
            validation: ValidationConfig::default(),
            sync: SyncConfig::default(),
            context: ContextConfig::default(),
            templates: TemplateConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_integration_mode_from_env() {
        assert_eq!(
            IntegrationMode::from_env_str("basic"),
            Some(IntegrationMode::Basic)
        );
        assert_eq!(
            IntegrationMode::from_env_str("ENHANCED"),
            Some(IntegrationMode::Enhanced)
        );
        assert_eq!(
            IntegrationMode::from_env_str("strict"),
            Some(IntegrationMode::Strict)
        );
        assert_eq!(IntegrationMode::from_env_str("invalid"), None);
    }

    #[test]
    fn test_validation_level_from_env() {
        assert_eq!(
            ValidationLevel::from_env_str("error"),
            Some(ValidationLevel::Error)
        );
        assert_eq!(
            ValidationLevel::from_env_str("WARNING"),
            Some(ValidationLevel::Warning)
        );
        assert_eq!(
            ValidationLevel::from_env_str("info"),
            Some(ValidationLevel::Info)
        );
        assert_eq!(ValidationLevel::from_env_str("invalid"), None);
    }

    #[test]
    fn test_config_from_file() {
        let dir = TempDir::new().unwrap();
        let config_path = dir.path().join("config.json");

        let config = IntegrationConfig::default();
        config.to_file(&config_path).unwrap();

        let loaded = IntegrationConfig::from_file(&config_path).unwrap();
        assert_eq!(loaded.version, config.version);
        assert_eq!(loaded.mode, config.mode);
    }

    #[test]
    fn test_env_overrides() {
        env::set_var("CRUCIBLE_CLAUDE_MODE", "strict");
        env::set_var("CRUCIBLE_VALIDATION", "error");
        env::set_var("CRUCIBLE_AUTO_SYNC", "false");
        env::set_var("CRUCIBLE_MAX_TOKENS", "5000");

        let mut config = IntegrationConfig::default();
        config.apply_env_overrides();

        assert_eq!(config.mode, IntegrationMode::Strict);
        assert_eq!(config.validation.severity, ValidationLevel::Error);
        assert_eq!(config.sync.auto_sync, false);
        assert_eq!(config.context.max_tokens, 5000);

        // Clean up
        env::remove_var("CRUCIBLE_CLAUDE_MODE");
        env::remove_var("CRUCIBLE_VALIDATION");
        env::remove_var("CRUCIBLE_AUTO_SYNC");
        env::remove_var("CRUCIBLE_MAX_TOKENS");
    }
}
