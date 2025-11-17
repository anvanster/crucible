//! Bidirectional sync between code and architecture

use crate::claude::rust_parser::{DiscoveredModule, RustParser};
use crate::error::{CrucibleError, Result};
use crate::types::Project;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Sync manager for bidirectional synchronization
pub struct SyncManager {
    project: Project,
}

/// Sync report showing what was discovered
#[derive(Debug)]
pub struct SyncReport {
    pub modules_discovered: usize,
    pub new_modules: Vec<String>,
    pub updated_modules: Vec<String>,
    pub new_exports: HashMap<String, Vec<String>>,
    pub new_dependencies: HashMap<String, Vec<String>>,
}

impl SyncManager {
    /// Create a new sync manager
    pub fn new(project: Project) -> Self {
        Self { project }
    }

    /// Sync architecture from code changes (Rust only for now)
    /// Returns (SyncReport, discovered_modules) for use in apply_sync_updates
    pub fn sync_from_code(&self, source_dir: &Path) -> Result<(SyncReport, Vec<DiscoveredModule>)> {
        // 1. Parse source files using Rust parser
        let discovered = RustParser::discover_modules(source_dir)?;

        // 2. Build dependency map
        let dep_map = RustParser::build_dependency_map(&discovered);

        // 3. Compare with existing architecture
        let mut new_modules = Vec::new();
        let mut new_exports = HashMap::new();
        let mut new_dependencies = HashMap::new();

        for disc_module in &discovered {
            // Check if module exists in architecture
            let exists = self
                .project
                .modules
                .iter()
                .any(|m| m.module == disc_module.name);

            if !exists {
                new_modules.push(disc_module.name.clone());
            }

            // Find new exports
            if let Some(arch_module) = self
                .project
                .modules
                .iter()
                .find(|m| m.module == disc_module.name)
            {
                let new_exp: Vec<String> = disc_module
                    .exports
                    .iter()
                    .filter(|exp| !arch_module.exports.contains_key(*exp))
                    .cloned()
                    .collect();

                if !new_exp.is_empty() {
                    new_exports.insert(disc_module.name.clone(), new_exp);
                }
            }

            // Check dependencies
            if let Some(deps) = dep_map.get(&disc_module.name) {
                if let Some(arch_module) = self
                    .project
                    .modules
                    .iter()
                    .find(|m| m.module == disc_module.name)
                {
                    let new_deps: Vec<String> = deps
                        .iter()
                        .filter(|dep| !arch_module.dependencies.contains_key(*dep))
                        .cloned()
                        .collect();

                    if !new_deps.is_empty() {
                        new_dependencies.insert(disc_module.name.clone(), new_deps);
                    }
                }
            }
        }

        // Determine which modules will be updated vs created
        let updated_modules: Vec<String> = self
            .project
            .modules
            .iter()
            .filter(|m| {
                new_exports.contains_key(&m.module) || new_dependencies.contains_key(&m.module)
            })
            .map(|m| m.module.clone())
            .collect();

        let report = SyncReport {
            modules_discovered: discovered.len(),
            new_modules,
            updated_modules,
            new_exports,
            new_dependencies,
        };

        Ok((report, discovered))
    }

    /// Sync code from architecture changes
    pub fn sync_from_architecture(&self, _target_dir: &Path) -> Result<()> {
        // TODO: Implement code generation from architecture
        // 1. Read architecture definitions
        // 2. Generate interfaces
        // 3. Create boilerplate
        // 4. Update existing files
        Err(CrucibleError::ParseError {
            file: "sync".to_string(),
            message: "sync_from_architecture not yet implemented".to_string(),
        })
    }

    /// Detect conflicts between code and architecture
    pub fn detect_conflicts(&self, source_dir: &Path) -> Result<Vec<String>> {
        let (report, _discovered) = self.sync_from_code(source_dir)?;
        let mut conflicts = Vec::new();

        if !report.new_modules.is_empty() {
            conflicts.push(format!(
                "Found {} new modules not in architecture: {}",
                report.new_modules.len(),
                report.new_modules.join(", ")
            ));
        }

        for (module, exports) in &report.new_exports {
            conflicts.push(format!(
                "Module '{}' has {} new exports: {}",
                module,
                exports.len(),
                exports.join(", ")
            ));
        }

        for (module, deps) in &report.new_dependencies {
            conflicts.push(format!(
                "Module '{}' has {} new dependencies: {}",
                module,
                deps.len(),
                deps.join(", ")
            ));
        }

        Ok(conflicts)
    }

    /// Generate a JSON module definition from discovered code
    pub fn generate_module_definition(
        &self,
        module_name: &str,
        discovered: &DiscoveredModule,
    ) -> serde_json::Value {
        let mut exports_obj = serde_json::Map::new();

        // Convert discovered exports to module definition format
        for export_name in &discovered.exports {
            // Determine export type based on naming conventions
            let export_type = if export_name.chars().next().unwrap().is_uppercase() {
                if export_name.contains("Error") || export_name.contains("Result") {
                    "type"
                } else {
                    "class"
                }
            } else {
                "function"
            };

            exports_obj.insert(
                export_name.clone(),
                json!({
                    "type": export_type,
                    "description": format!("Auto-generated from code"),
                    "methods": {}
                }),
            );
        }

        // Build dependency map
        let mut deps_obj = serde_json::Map::new();
        for import in &discovered.imports {
            deps_obj.insert(import.clone(), json!("^0.1.0"));
        }

        json!({
            "module": module_name,
            "version": "0.1.0",
            "layer": "core",
            "description": format!("Auto-generated module definition for {}", module_name),
            "exports": exports_obj,
            "dependencies": deps_obj
        })
    }

    /// Update an existing module JSON with new exports and dependencies
    pub fn update_existing_module(
        &self,
        module_path: &Path,
        new_exports: &[String],
        new_dependencies: &[String],
        _discovered: &DiscoveredModule,
    ) -> Result<String> {
        // Read existing module JSON
        let existing_content =
            fs::read_to_string(module_path).map_err(|e| CrucibleError::FileRead {
                path: module_path.display().to_string(),
                source: e,
            })?;

        let mut module_json: serde_json::Value =
            serde_json::from_str(&existing_content).map_err(|e| CrucibleError::ParseError {
                file: module_path.display().to_string(),
                message: format!("Failed to parse existing module JSON: {}", e),
            })?;

        // Merge new exports
        if let Some(exports) = module_json.get_mut("exports") {
            if let Some(exports_obj) = exports.as_object_mut() {
                for export_name in new_exports {
                    // Only add if not already present
                    if !exports_obj.contains_key(export_name) {
                        // Determine export type based on naming conventions
                        let export_type = if export_name.chars().next().unwrap().is_uppercase() {
                            if export_name.contains("Error") || export_name.contains("Result") {
                                "type"
                            } else {
                                "class"
                            }
                        } else {
                            "function"
                        };

                        exports_obj.insert(
                            export_name.clone(),
                            json!({
                                "type": export_type,
                                "description": "Auto-generated from code",
                                "methods": {}
                            }),
                        );
                    }
                }
            }
        }

        // Merge new dependencies
        if let Some(deps) = module_json.get_mut("dependencies") {
            if let Some(deps_obj) = deps.as_object_mut() {
                for dep_name in new_dependencies {
                    // Only add if not already present
                    if !deps_obj.contains_key(dep_name) {
                        deps_obj.insert(dep_name.clone(), json!("^0.1.0"));
                    }
                }
            }
        }

        // Serialize back to JSON string
        let updated_json =
            serde_json::to_string_pretty(&module_json).map_err(|e| CrucibleError::ParseError {
                file: module_path.display().to_string(),
                message: format!("Failed to serialize updated module JSON: {}", e),
            })?;

        Ok(updated_json)
    }

    /// Format an interactive sync prompt for the user
    pub fn format_sync_prompt(&self, report: &SyncReport) -> String {
        let mut prompt = String::new();

        prompt.push_str("\n🔄 Sync Analysis Complete\n\n");

        if report.new_modules.is_empty()
            && report.updated_modules.is_empty()
            && report.new_exports.is_empty()
            && report.new_dependencies.is_empty()
        {
            prompt.push_str("✅ Architecture is in sync with code!\n");
            return prompt;
        }

        prompt.push_str("The following changes were detected:\n\n");

        if !report.new_modules.is_empty() {
            prompt.push_str(&format!("📦 {} new modules:\n", report.new_modules.len()));
            for module in &report.new_modules {
                prompt.push_str(&format!("   - {}\n", module));
            }
            prompt.push_str("\n");
        }

        if !report.updated_modules.is_empty() {
            prompt.push_str(&format!(
                "🔄 {} modules will be updated:\n",
                report.updated_modules.len()
            ));
            for module in &report.updated_modules {
                prompt.push_str(&format!("   - {}\n", module));
            }
            prompt.push_str("\n");
        }

        if !report.new_exports.is_empty() {
            prompt.push_str(&format!(
                "📤 New exports in {} modules:\n",
                report.new_exports.len()
            ));
            for (module, exports) in report.new_exports.iter().take(3) {
                prompt.push_str(&format!("   - {} ({} exports)\n", module, exports.len()));
            }
            if report.new_exports.len() > 3 {
                prompt.push_str(&format!(
                    "   ... and {} more modules\n",
                    report.new_exports.len() - 3
                ));
            }
            prompt.push_str("\n");
        }

        if !report.new_dependencies.is_empty() {
            prompt.push_str(&format!(
                "🔗 New dependencies in {} modules:\n",
                report.new_dependencies.len()
            ));
            for (module, deps) in report.new_dependencies.iter().take(3) {
                prompt.push_str(&format!("   - {} ({} deps)\n", module, deps.len()));
            }
            if report.new_dependencies.len() > 3 {
                prompt.push_str(&format!(
                    "   ... and {} more modules\n",
                    report.new_dependencies.len() - 3
                ));
            }
            prompt.push_str("\n");
        }

        prompt.push_str("Would you like to auto-update the architecture? [y/N]: ");

        prompt
    }

    /// Apply sync updates to architecture files
    pub fn apply_sync_updates(
        &self,
        report: &SyncReport,
        discovered_modules: &[DiscoveredModule],
        interactive: bool,
    ) -> Result<usize> {
        if report.new_modules.is_empty()
            && report.updated_modules.is_empty()
            && report.new_exports.is_empty()
            && report.new_dependencies.is_empty()
        {
            return Ok(0);
        }

        let mut updates_applied = 0;

        if interactive {
            // Show prompt and wait for user input
            let prompt = self.format_sync_prompt(report);
            print!("{}", prompt);
            io::stdout().flush().map_err(|e| CrucibleError::FileRead {
                path: "stdout".to_string(),
                source: e,
            })?;

            let mut response = String::new();
            io::stdin()
                .read_line(&mut response)
                .map_err(|e| CrucibleError::FileRead {
                    path: "stdin".to_string(),
                    source: e,
                })?;

            if !response.trim().eq_ignore_ascii_case("y") {
                println!("Sync cancelled. No changes made.");
                return Ok(0);
            }
        }

        println!("\n📝 Applying architecture updates...\n");

        // Create .crucible/modules directory if it doesn't exist
        let modules_dir = PathBuf::from(".crucible/modules");
        fs::create_dir_all(&modules_dir).map_err(|e| CrucibleError::FileRead {
            path: modules_dir.display().to_string(),
            source: e,
        })?;

        // Generate and write module definitions for new modules
        for module_name in &report.new_modules {
            if let Some(discovered) = discovered_modules.iter().find(|m| &m.name == module_name) {
                let module_def = self.generate_module_definition(module_name, discovered);
                let file_path = modules_dir.join(format!("{}.json", module_name));

                let json_str = serde_json::to_string_pretty(&module_def).map_err(|e| {
                    CrucibleError::ParseError {
                        file: module_name.clone(),
                        message: format!("Failed to serialize module definition: {}", e),
                    }
                })?;

                fs::write(&file_path, json_str).map_err(|e| CrucibleError::FileRead {
                    path: file_path.display().to_string(),
                    source: e,
                })?;

                println!("   ✅ Created .crucible/modules/{}.json", module_name);
                updates_applied += 1;
            }
        }

        // Update existing modules with new exports and dependencies
        for module_name in &report.updated_modules {
            let file_path = modules_dir.join(format!("{}.json", module_name));

            // Get the new exports and dependencies for this module
            let new_exports = report
                .new_exports
                .get(module_name)
                .map(|v| v.as_slice())
                .unwrap_or(&[]);

            let new_deps = report
                .new_dependencies
                .get(module_name)
                .map(|v| v.as_slice())
                .unwrap_or(&[]);

            // Find the discovered module for type inference
            if let Some(discovered) = discovered_modules.iter().find(|m| &m.name == module_name) {
                let updated_json =
                    self.update_existing_module(&file_path, new_exports, new_deps, discovered)?;

                fs::write(&file_path, updated_json).map_err(|e| CrucibleError::FileRead {
                    path: file_path.display().to_string(),
                    source: e,
                })?;

                println!("   🔄 Updated .crucible/modules/{}.json", module_name);
                updates_applied += 1;
            }
        }

        if updates_applied > 0 {
            println!("\n✨ Successfully applied {} updates!", updates_applied);
            println!("   Run `crucible validate` to verify the updated architecture.\n");
        }

        Ok(updates_applied)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Language, Project};

    #[test]
    fn test_sync_manager_new() {
        let project = Project {
            manifest: crate::types::Manifest {
                version: "0.1.0".to_string(),
                project: crate::types::ProjectConfig {
                    name: "test".to_string(),
                    language: Language::TypeScript,
                    architecture_pattern: Some(crate::types::ArchitecturePattern::Layered),
                },
                modules: vec![],
                strict_validation: false,
                metadata: None,
            },
            modules: vec![],
            rules: None,
        };

        let _manager = SyncManager::new(project);
    }
}
