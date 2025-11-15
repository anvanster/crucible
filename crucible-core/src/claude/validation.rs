//! Validation hooks for Claude Code integration

use crate::types::Project;
use crate::validator::{ValidationIssue, ValidationResult};

/// Architecture-aware suggestion for fixing a validation issue
#[derive(Debug, Clone)]
pub struct ValidationSuggestion {
    pub title: String,
    pub description: String,
    pub example: Option<String>,
    pub fix_type: SuggestionType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SuggestionType {
    AddDependency,
    RemoveDependency,
    ChangeLayer,
    ExportType,
    RenameModule,
    RefactorCircular,
    UpdateArchitecture,
}

/// Validation hooks generator
pub struct ValidationHooks {
    project: Project,
}

impl ValidationHooks {
    /// Create a new validation hooks generator
    pub fn new(project: Project) -> Self {
        Self { project }
    }

    /// Generate hooks.md content with validation prompts
    pub fn generate_hooks(&self) -> String {
        let mut content = String::new();

        content.push_str("# Crucible Validation Hooks\n\n");
        content.push_str(
            "These prompts help maintain architectural integrity during development.\n\n",
        );

        content.push_str("## 🎯 Pre-Change Validation\n\n");
        content.push_str("Before implementing any feature or fix, validate your approach:\n\n");

        content.push_str("### Checklist for New Features\n\n");
        content.push_str("- [ ] Have I identified which module this belongs to?\n");
        content.push_str("- [ ] Is this module in the correct architectural layer?\n");
        content.push_str("- [ ] Are all required dependencies already declared?\n");
        content.push_str("- [ ] Will this create any circular dependencies?\n");
        content.push_str("- [ ] Do the types I need exist in accessible modules?\n");
        content.push_str("- [ ] Am I following the naming conventions?\n\n");

        content.push_str("### Checklist for Module Modifications\n\n");
        content.push_str("- [ ] Am I only using exported functions from other modules?\n");
        content.push_str("- [ ] Are my new exports properly documented?\n");
        content.push_str("- [ ] Do my function signatures match the architecture?\n");
        content.push_str("- [ ] Have I checked layer boundary constraints?\n");
        content.push_str("- [ ] Will my changes break existing contracts?\n\n");

        content.push_str("## 🔄 Post-Change Sync\n\n");
        content.push_str("After implementing changes, ensure architecture stays synchronized:\n\n");

        content.push_str("### Validation Commands\n\n");
        content.push_str("```bash\n");
        content.push_str("# After adding new code\n");
        content.push_str("crucible validate <module>\n\n");
        content.push_str("# After refactoring\n");
        content.push_str("crucible validate --all\n\n");
        content.push_str("# To sync architecture with code\n");
        content.push_str("crucible claude sync --from-code\n");
        content.push_str("```\n\n");

        content.push_str("## ⚠️ Common Violations to Avoid\n\n");
        content.push_str("### 1. Layer Violations\n");
        content.push_str("❌ **Wrong**: Domain layer importing from Presentation layer\n");
        content.push_str("✅ **Right**: Presentation layer importing from Domain layer\n\n");

        content.push_str("### 2. Circular Dependencies\n");
        content.push_str("❌ **Wrong**: Module A → Module B → Module A\n");
        content.push_str("✅ **Right**: Module A → Module B → Module C\n\n");

        content.push_str("### 3. Undeclared Dependencies\n");
        content.push_str("❌ **Wrong**: Using a module without declaring it in dependencies\n");
        content.push_str("✅ **Right**: First declare dependency, then use the module\n\n");

        content
    }

    /// Generate pre-change validation prompt for a specific module
    pub fn generate_pre_change_prompt(&self, module_name: &str) -> String {
        format!(
            "Before modifying the '{}' module, please verify:\n\
             1. Check allowed dependencies in .crucible/modules/{}.json\n\
             2. Verify you're not creating circular dependencies\n\
             3. Ensure new exports follow naming conventions\n\
             4. Confirm layer boundary rules are respected",
            module_name, module_name
        )
    }

    /// Generate post-change sync checklist
    pub fn generate_post_change_checklist(&self) -> String {
        "After making changes:\n\
         1. Update module exports if new functions added\n\
         2. Declare new dependencies if external modules used\n\
         3. Run `crucible validate` to ensure consistency\n\
         4. Update type definitions if interfaces changed"
            .to_string()
    }

    /// Generate architecture-aware suggestions for a validation issue
    pub fn generate_suggestions(&self, issue: &ValidationIssue) -> Vec<ValidationSuggestion> {
        let mut suggestions = Vec::new();

        match issue.rule.as_str() {
            "no-circular-dependencies" => {
                suggestions.push(ValidationSuggestion {
                    title: "Refactor to remove circular dependency".to_string(),
                    description: "Circular dependencies violate clean architecture. Consider extracting shared logic into a new module that both can depend on.".to_string(),
                    example: Some("Create a new 'shared' or 'common' module for shared functionality".to_string()),
                    fix_type: SuggestionType::RefactorCircular,
                });
                suggestions.push(ValidationSuggestion {
                    title: "Use dependency inversion".to_string(),
                    description: "Define interfaces in the lower-level module and implement them in the higher-level module.".to_string(),
                    example: Some("Module A depends on IService interface, Module B implements IService".to_string()),
                    fix_type: SuggestionType::RefactorCircular,
                });
            }
            "respect-layer-boundaries" => {
                suggestions.push(ValidationSuggestion {
                    title: "Move module to correct layer".to_string(),
                    description: "The module is in the wrong layer for its dependencies. Review the layered architecture and move the module accordingly.".to_string(),
                    example: Some("If module depends on 'application' layer, it should be in 'presentation' layer".to_string()),
                    fix_type: SuggestionType::ChangeLayer,
                });
                suggestions.push(ValidationSuggestion {
                    title: "Remove invalid dependency".to_string(),
                    description: "Remove the dependency that violates layer boundaries and refactor to use proper abstractions.".to_string(),
                    example: Some("Instead of directly depending on higher layer, use dependency injection or events".to_string()),
                    fix_type: SuggestionType::RemoveDependency,
                });
            }
            "all-types-must-exist" => {
                suggestions.push(ValidationSuggestion {
                    title: "Add missing type export".to_string(),
                    description: "The referenced type doesn't exist in the architecture. Add it to the module's exports.".to_string(),
                    example: Some("In .crucible/modules/<module>.json, add the type to 'exports' section".to_string()),
                    fix_type: SuggestionType::ExportType,
                });
                suggestions.push(ValidationSuggestion {
                    title: "Fix type reference".to_string(),
                    description:
                        "The type name might be incorrect. Check the spelling or module path."
                            .to_string(),
                    example: Some(
                        "Use 'moduleName.TypeName' format for types from other modules".to_string(),
                    ),
                    fix_type: SuggestionType::UpdateArchitecture,
                });
            }
            "all-calls-must-exist" => {
                suggestions.push(ValidationSuggestion {
                    title: "Add function to module exports".to_string(),
                    description: "The called function doesn't exist in the target module's exports. Add it if it should be public.".to_string(),
                    example: Some("In target module's .json, add the function to 'exports'".to_string()),
                    fix_type: SuggestionType::ExportType,
                });
                suggestions.push(ValidationSuggestion {
                    title: "Use correct call format".to_string(),
                    description: "Ensure call format is 'module.function' or 'module.Class.method'"
                        .to_string(),
                    example: Some(
                        "Correct: 'types.Project.new()' or 'parser.parse_project()'".to_string(),
                    ),
                    fix_type: SuggestionType::UpdateArchitecture,
                });
            }
            "used-dependencies-declared" => {
                suggestions.push(ValidationSuggestion {
                    title: "Declare the dependency".to_string(),
                    description: "Add the used module to this module's dependencies section.".to_string(),
                    example: Some("In .crucible/modules/<module>.json, add to 'dependencies': {\"moduleName\": \"^0.1.0\"}".to_string()),
                    fix_type: SuggestionType::AddDependency,
                });
            }
            "declared-dependencies-must-be-used" => {
                suggestions.push(ValidationSuggestion {
                    title: "Remove unused dependency".to_string(),
                    description: "This dependency is declared but not used. Remove it to keep architecture clean.".to_string(),
                    example: Some("Remove from 'dependencies' section in module's .json file".to_string()),
                    fix_type: SuggestionType::RemoveDependency,
                });
                suggestions.push(ValidationSuggestion {
                    title: "Use the declared dependency".to_string(),
                    description: "If you plan to use this module, add calls to it in your methods."
                        .to_string(),
                    example: None,
                    fix_type: SuggestionType::UpdateArchitecture,
                });
            }
            _ => {
                suggestions.push(ValidationSuggestion {
                    title: "Review architecture documentation".to_string(),
                    description: "Check the architecture rules and ensure your changes comply."
                        .to_string(),
                    example: Some(
                        "Run 'crucible validate' for detailed error information".to_string(),
                    ),
                    fix_type: SuggestionType::UpdateArchitecture,
                });
            }
        }

        suggestions
    }

    /// Format validation results with architectural context and suggestions
    pub fn format_with_context(&self, result: &ValidationResult) -> String {
        let mut content = String::new();

        if result.valid {
            content.push_str("✅ **Architecture is valid!**\n\n");
            content.push_str(
                "All validation checks passed. Your code complies with the defined architecture.\n",
            );
            return content;
        }

        content.push_str("# 🏗️ Architecture Validation Report\n\n");

        // Summary
        content.push_str(&format!(
            "**Status**: ❌ Validation Failed  \n\
             **Errors**: {}  \n\
             **Warnings**: {}  \n\n",
            result.errors.len(),
            result.warnings.len()
        ));

        // Detailed errors with suggestions
        if !result.errors.is_empty() {
            content.push_str("## ❌ Critical Errors\n\n");
            content.push_str("These issues must be fixed before the architecture is valid.\n\n");

            for (idx, error) in result.errors.iter().enumerate() {
                content.push_str(&format!(
                    "### {} Error {}: {}\n\n",
                    "🚨",
                    idx + 1,
                    error.rule
                ));
                content.push_str(&format!("**Message**: {}\n\n", error.message));

                if let Some(location) = &error.location {
                    content.push_str(&format!("**Location**: `{}`\n\n", location));
                }

                // Generate and add suggestions
                let suggestions = self.generate_suggestions(error);
                if !suggestions.is_empty() {
                    content.push_str("**💡 Suggested Fixes**:\n\n");
                    for (i, suggestion) in suggestions.iter().enumerate() {
                        content.push_str(&format!("{}. **{}**\n", i + 1, suggestion.title));
                        content.push_str(&format!("   {}\n", suggestion.description));
                        if let Some(example) = &suggestion.example {
                            content.push_str(&format!("   \n   *Example*: {}\n", example));
                        }
                        content.push_str("\n");
                    }
                }

                content.push_str("---\n\n");
            }
        }

        // Warnings
        if !result.warnings.is_empty() {
            content.push_str("## ⚠️ Warnings\n\n");
            content.push_str("These issues should be addressed but don't block validation.\n\n");

            for warning in &result.warnings {
                content.push_str(&format!("- **{}**: {}\n", warning.rule, warning.message));
                if let Some(location) = &warning.location {
                    content.push_str(&format!("  *Location*: `{}`\n", location));
                }

                // Add brief suggestions for warnings
                let suggestions = self.generate_suggestions(warning);
                if let Some(first_suggestion) = suggestions.first() {
                    content.push_str(&format!(
                        "  *Suggestion*: {}\n",
                        first_suggestion.description
                    ));
                }
                content.push_str("\n");
            }
        }

        // Next steps
        content.push_str("## 📋 Next Steps\n\n");
        content.push_str("1. Review the suggested fixes above\n");
        content.push_str("2. Update the relevant module definitions in `.crucible/modules/`\n");
        content.push_str("3. Run `crucible validate` to verify fixes\n");
        content.push_str(
            "4. Use `crucible claude sync --from-code` to sync architecture with code\n\n",
        );

        // Architecture context
        content.push_str("## 🎯 Architecture Context\n\n");
        if let Some(pattern) = &self.project.manifest.project.architecture_pattern {
            content.push_str(&format!("- **Pattern**: {:?}\n", pattern));
        }
        content.push_str(&format!("- **Modules**: {}\n", self.project.modules.len()));
        content.push_str(&format!(
            "- **Language**: {:?}\n",
            self.project.manifest.project.language
        ));

        content
    }

    /// Format validation errors for Claude's understanding
    pub fn format_validation_errors(&self, result: &ValidationResult) -> String {
        let mut content = String::new();

        if !result.errors.is_empty() {
            content.push_str("## ❌ Validation Errors\n\n");
            for error in &result.errors {
                content.push_str(&format!("- **{}**: {}\n", error.rule, error.message));
                if let Some(location) = &error.location {
                    content.push_str(&format!("  Location: {}\n", location));
                }
            }
            content.push_str("\n");
        }

        if !result.warnings.is_empty() {
            content.push_str("## ⚠️ Warnings\n\n");
            for warning in &result.warnings {
                content.push_str(&format!("- **{}**: {}\n", warning.rule, warning.message));
                if let Some(location) = &warning.location {
                    content.push_str(&format!("  Location: {}\n", location));
                }
            }
        }

        content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Language, Module, Project};
    use std::collections::HashMap;

    fn create_test_project() -> Project {
        Project {
            name: "test".to_string(),
            language: Language::TypeScript,
            architecture_pattern: "layered".to_string(),
            modules: vec![Module {
                module: "auth".to_string(),
                description: None,
                layer: Some("application".to_string()),
                exports: HashMap::new(),
                dependencies: HashMap::new(),
            }],
        }
    }

    #[test]
    fn test_generate_hooks() {
        let project = create_test_project();
        let hooks = ValidationHooks::new(project);
        let content = hooks.generate_hooks();

        assert!(content.contains("Validation Hooks"));
        assert!(content.contains("Pre-Change Validation"));
        assert!(content.contains("Post-Change Sync"));
    }

    #[test]
    fn test_pre_change_prompt() {
        let project = create_test_project();
        let hooks = ValidationHooks::new(project);
        let prompt = hooks.generate_pre_change_prompt("auth");

        assert!(prompt.contains("auth"));
        assert!(prompt.contains("dependencies"));
    }
}
