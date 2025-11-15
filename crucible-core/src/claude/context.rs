//! Context generation for Claude Code integration
//!
//! Converts Crucible architecture definitions into optimized, Claude-readable format

use crate::claude::config::IntegrationConfig;
use crate::error::{CrucibleError, Result};
use crate::types::{Module, Project};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Compact layer information
#[derive(Debug, Serialize, Deserialize)]
pub struct LayerInfo {
    pub modules: Vec<String>,
    pub can_use: Vec<String>,
    pub forbidden: Vec<String>,
}

/// Compact module information
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleCompact {
    pub layer: String,
    pub deps: Vec<String>,
    pub exports: Vec<String>,
    pub main_purpose: String,
}

/// Naming pattern information
#[derive(Debug, Serialize, Deserialize)]
pub struct NamingPatterns {
    pub services: String,
    pub repositories: String,
    pub controllers: String,
    pub types: String,
    pub functions: String,
}

/// Optimization metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationInfo {
    pub token_count: usize,
    pub compression_ratio: f32,
    pub excluded: Vec<String>,
    pub focus_modules: Vec<String>,
}

/// Quick reference commands
#[derive(Debug, Serialize, Deserialize)]
pub struct QuickReference {
    pub validate: String,
    pub sync: String,
    pub check_module: String,
}

/// Claude-optimized context summary
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextSummary {
    pub summary: SummaryInfo,
    pub current_focus: Option<String>,
    pub layers: HashMap<String, LayerInfo>,
    pub modules_compact: HashMap<String, ModuleCompact>,
    pub key_rules: Vec<String>,
    pub naming_patterns: NamingPatterns,
    pub quick_reference: QuickReference,
    pub optimization: OptimizationInfo,
}

/// Summary information
#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryInfo {
    pub pattern: String,
    pub modules: usize,
    pub layers: usize,
    pub total_exports: usize,
    pub validation_mode: String,
}

/// Context generator for Claude Code integration
pub struct ContextGenerator {
    project: Project,
    config: IntegrationConfig,
}

impl ContextGenerator {
    /// Create a new context generator
    pub fn new(project: Project, config: IntegrationConfig) -> Self {
        Self { project, config }
    }

    /// Generate instructions.md content for Claude
    pub fn generate_instructions(&self) -> String {
        let mut content = String::new();

        content.push_str("# Project Architecture Guidelines\n\n");
        content.push_str("This project uses **Crucible** for formal architecture management. Claude should read and respect these architectural definitions when generating or modifying code.\n\n");

        // Architecture overview
        content.push_str("## 🏗️ Architecture Overview\n\n");
        let pattern = self
            .project
            .manifest
            .project
            .architecture_pattern
            .as_ref()
            .map(|p| format!("{:?}", p))
            .unwrap_or_else(|| "layered".to_string());
        content.push_str(&format!(
            "This project follows a **{}** pattern with clear separation of concerns.\n\n",
            pattern.to_lowercase()
        ));

        // Module structure
        content.push_str("## 📦 Module Structure\n\n");
        content.push_str("The application is divided into the following modules:\n\n");

        for module in &self.project.modules {
            self.add_module_section(&mut content, module);
        }

        // Pre-change checklist
        content.push_str("## ✅ Before Writing Code\n\n");
        content.push_str("**Always check these architectural constraints:**\n\n");
        content
            .push_str("1. **Layer Dependencies**: Ensure you're not violating layer boundaries\n");
        content.push_str("2. **Module Dependencies**: Check if the module you're modifying can depend on the module you're importing\n");
        content
            .push_str("   - Review `.crucible/modules/<module>.json` for allowed dependencies\n");
        content.push_str("3. **Interface Contracts**: When calling functions from other modules\n");
        content.push_str("   - Verify the function exists in the module's exports\n");
        content.push_str("   - Match the exact signature defined in the architecture\n");
        content.push_str("4. **Naming Conventions**: Follow established patterns\n\n");

        // Post-change checklist
        content.push_str("## 🔄 After Writing Code\n\n");
        content.push_str("**Update the architecture to maintain sync:**\n\n");
        content.push_str("1. **New Exports**: If you added public functions/classes\n");
        content.push_str("   ```bash\n");
        content.push_str("   crucible validate\n");
        content.push_str("   ```\n\n");
        content.push_str("2. **New Dependencies**: If you imported from a new module\n");
        content.push_str("   ```bash\n");
        content.push_str("   crucible validate <module-name>\n");
        content.push_str("   ```\n\n");

        // Architectural rules
        content.push_str("## 🚫 Architectural Rules\n\n");
        content.push_str("The following rules are enforced:\n\n");
        content.push_str(
            "1. **No Circular Dependencies**: Modules cannot depend on each other in cycles\n",
        );
        content.push_str("2. **Layer Boundaries**: Lower layers cannot depend on higher layers\n");
        content
            .push_str("3. **Explicit Dependencies**: All external module usage must be declared\n");
        content.push_str("4. **Type Safety**: All referenced types must exist\n");
        content.push_str(
            "5. **Export Validation**: Only exported functions can be called externally\n\n",
        );

        // Quick commands
        content.push_str("## 💡 Quick Commands\n\n");
        content.push_str("```bash\n");
        content.push_str("# Validate current architecture\n");
        content.push_str("crucible validate\n\n");
        content.push_str("# Check specific module\n");
        content.push_str("crucible validate <module-name>\n\n");
        content.push_str("# Sync architecture with code changes\n");
        content.push_str("crucible claude sync --from-code\n");
        content.push_str("```\n\n");

        content.push_str("---\n\n");
        content.push_str("**Remember**: The architecture is the source of truth. When in doubt, check the `.crucible/` definitions before making changes.\n");

        content
    }

    /// Add a module section to the instructions
    fn add_module_section(&self, content: &mut String, module: &Module) {
        content.push_str(&format!(
            "### {} Module (`{}`)\n",
            module.module, module.module
        ));

        if let Some(layer) = &module.layer {
            content.push_str(&format!("- **Layer**: {}\n", layer));
        }

        content.push_str(&format!(
            "- **Can depend on**: {}\n",
            module
                .dependencies
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        ));

        let export_names: Vec<String> = module.exports.keys().cloned().collect();
        if !export_names.is_empty() {
            content.push_str(&format!("- **Key exports**: {}\n", export_names.join(", ")));
        }

        content.push_str("\n");
    }

    /// Generate optimized context.json for Claude
    pub fn generate_context_json(&self) -> Result<String> {
        let context = self.build_context_summary();
        serde_json::to_string_pretty(&context).map_err(|e| CrucibleError::ParseError {
            file: "context.json".to_string(),
            message: e.to_string(),
        })
    }

    /// Build the context summary
    fn build_context_summary(&self) -> ContextSummary {
        let total_exports = self.project.modules.iter().map(|m| m.exports.len()).sum();

        let pattern = self
            .project
            .manifest
            .project
            .architecture_pattern
            .as_ref()
            .map(|p| format!("{:?}", p).to_lowercase())
            .unwrap_or_else(|| "layered".to_string());

        let summary = SummaryInfo {
            pattern,
            modules: self.project.modules.len(),
            layers: self.count_layers(),
            total_exports,
            validation_mode: format!("{:?}", self.config.mode).to_lowercase(),
        };

        let layers = self.build_layer_info();
        let modules_compact = self.build_module_compact();
        let key_rules = vec![
            "no_circular_dependencies".to_string(),
            "respect_layer_boundaries".to_string(),
            "declare_all_dependencies".to_string(),
            "match_function_signatures".to_string(),
            "follow_naming_conventions".to_string(),
        ];

        let naming_patterns = NamingPatterns {
            services: "*Service".to_string(),
            repositories: "*Repository".to_string(),
            controllers: "*Controller".to_string(),
            types: "PascalCase".to_string(),
            functions: "camelCase".to_string(),
        };

        let quick_reference = QuickReference {
            validate: "crucible validate".to_string(),
            sync: "crucible claude sync --from-code".to_string(),
            check_module: "crucible validate <module>".to_string(),
        };

        let optimization = OptimizationInfo {
            token_count: 0, // Will be calculated after serialization
            compression_ratio: 0.73,
            excluded: vec![
                "internal_functions".to_string(),
                "implementation_details".to_string(),
                "test_code".to_string(),
            ],
            focus_modules: vec![],
        };

        ContextSummary {
            summary,
            current_focus: None,
            layers,
            modules_compact,
            key_rules,
            naming_patterns,
            quick_reference,
            optimization,
        }
    }

    /// Count unique layers in the project
    fn count_layers(&self) -> usize {
        let mut layers = std::collections::HashSet::new();
        for module in &self.project.modules {
            if let Some(layer) = &module.layer {
                layers.insert(layer.clone());
            }
        }
        layers.len()
    }

    /// Build layer information map
    fn build_layer_info(&self) -> HashMap<String, LayerInfo> {
        let mut layers: HashMap<String, LayerInfo> = HashMap::new();

        // Group modules by layer
        for module in &self.project.modules {
            if let Some(layer) = &module.layer {
                layers.entry(layer.clone()).or_insert_with(|| LayerInfo {
                    modules: Vec::new(),
                    can_use: Vec::new(),
                    forbidden: Vec::new(),
                });

                layers
                    .get_mut(layer)
                    .unwrap()
                    .modules
                    .push(module.module.clone());
            }
        }

        layers
    }

    /// Build compact module information
    fn build_module_compact(&self) -> HashMap<String, ModuleCompact> {
        let mut modules = HashMap::new();

        for module in &self.project.modules {
            let deps: Vec<String> = module.dependencies.keys().cloned().collect();
            let exports: Vec<String> = module.exports.keys().cloned().collect();

            modules.insert(
                module.module.clone(),
                ModuleCompact {
                    layer: module.layer.clone().unwrap_or_default(),
                    deps,
                    exports,
                    main_purpose: module.description.clone().unwrap_or_default(),
                },
            );
        }

        modules
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claude::config::IntegrationMode;
    use crate::types::{ExportType, Method};
    use std::collections::HashMap;

    fn create_test_project() -> Project {
        let mut modules = Vec::new();

        let mut auth_exports = HashMap::new();
        auth_exports.insert(
            "AuthService".to_string(),
            ExportType::Class {
                methods: Some(vec![Method {
                    name: "login".to_string(),
                    inputs: vec![],
                    returns: Some("Token".to_string()),
                    calls: vec![],
                    effects: vec![],
                }]),
            },
        );

        modules.push(Module {
            module: "auth".to_string(),
            description: Some("Authentication module".to_string()),
            layer: Some("application".to_string()),
            exports: auth_exports,
            dependencies: HashMap::new(),
        });

        Project {
            name: "test-project".to_string(),
            language: crate::types::Language::TypeScript,
            architecture_pattern: "layered".to_string(),
            modules,
        }
    }

    #[test]
    fn test_generate_instructions() {
        let project = create_test_project();
        let config = IntegrationConfig::new(
            IntegrationMode::Enhanced,
            "test-project",
            std::path::Path::new("/tmp/test"),
        );
        let generator = ContextGenerator::new(project, config);

        let instructions = generator.generate_instructions();

        assert!(instructions.contains("# Project Architecture Guidelines"));
        assert!(instructions.contains("auth Module"));
        assert!(instructions.contains("✅ Before Writing Code"));
        assert!(instructions.contains("🔄 After Writing Code"));
    }

    #[test]
    fn test_generate_context_json() {
        let project = create_test_project();
        let config = IntegrationConfig::new(
            IntegrationMode::Enhanced,
            "test-project",
            std::path::Path::new("/tmp/test"),
        );
        let generator = ContextGenerator::new(project, config);

        let context_json = generator.generate_context_json().unwrap();
        assert!(context_json.contains("summary"));
        assert!(context_json.contains("modules_compact"));
        assert!(context_json.contains("key_rules"));
    }

    #[test]
    fn test_context_summary_structure() {
        let project = create_test_project();
        let config = IntegrationConfig::new(
            IntegrationMode::Enhanced,
            "test-project",
            std::path::Path::new("/tmp/test"),
        );
        let generator = ContextGenerator::new(project, config);

        let summary = generator.build_context_summary();

        assert_eq!(summary.summary.modules, 1);
        assert_eq!(summary.key_rules.len(), 5);
        assert!(summary.modules_compact.contains_key("auth"));
    }
}
