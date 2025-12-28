//! Framework loader for compliance definitions

use crate::error::{ComplianceError, ComplianceResult};
use crate::framework::Framework;
use crucible_core::types::ComplianceFramework;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Loads compliance frameworks from the filesystem
#[derive(Debug, Default)]
pub struct FrameworkLoader {
    /// Loaded frameworks by name
    frameworks: HashMap<String, Framework>,
}

impl FrameworkLoader {
    /// Create a new empty loader
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a single framework from a file
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> ComplianceResult<&Framework> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|e| ComplianceError::ParseError {
            path: path.display().to_string(),
            message: e.to_string(),
        })?;

        let definition: ComplianceFramework =
            serde_json::from_str(&content).map_err(|e| ComplianceError::ParseError {
                path: path.display().to_string(),
                message: e.to_string(),
            })?;

        let name = definition.compliance_framework.clone();
        let framework = Framework::new(definition);
        self.frameworks.insert(name.clone(), framework);

        Ok(self.frameworks.get(&name).unwrap())
    }

    /// Load all frameworks from a directory
    pub fn load_directory<P: AsRef<Path>>(&mut self, dir: P) -> ComplianceResult<Vec<String>> {
        let dir = dir.as_ref();
        if !dir.exists() {
            return Ok(vec![]);
        }

        let mut loaded = Vec::new();

        for entry in WalkDir::new(dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                match self.load_file(path) {
                    Ok(framework) => {
                        loaded.push(framework.name().to_string());
                    }
                    Err(e) => {
                        // Log warning but continue loading other frameworks
                        eprintln!("Warning: Failed to load {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(loaded)
    }

    /// Load frameworks from a Crucible project's compliance directory
    pub fn load_from_project<P: AsRef<Path>>(
        &mut self,
        project_root: P,
    ) -> ComplianceResult<Vec<String>> {
        let compliance_dir = project_root.as_ref().join(".crucible").join("compliance");
        self.load_directory(compliance_dir)
    }

    /// Get a loaded framework by name
    pub fn get(&self, name: &str) -> Option<&Framework> {
        self.frameworks.get(name)
    }

    /// Get all loaded frameworks
    pub fn all(&self) -> impl Iterator<Item = &Framework> {
        self.frameworks.values()
    }

    /// Get the names of all loaded frameworks
    pub fn names(&self) -> Vec<&str> {
        self.frameworks.keys().map(|s| s.as_str()).collect()
    }

    /// Check if a framework is loaded
    pub fn has(&self, name: &str) -> bool {
        self.frameworks.contains_key(name)
    }

    /// Get the number of loaded frameworks
    pub fn count(&self) -> usize {
        self.frameworks.len()
    }

    /// Clear all loaded frameworks
    pub fn clear(&mut self) {
        self.frameworks.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_framework_file(dir: &Path, name: &str) -> std::io::Result<()> {
        let content = format!(
            r#"{{
                "compliance_framework": "{}",
                "version": "1.0.0",
                "description": "Test framework",
                "requirements": [],
                "rules": [
                    {{
                        "id": "test-rule",
                        "severity": "error",
                        "description": "Test rule",
                        "validates": {{
                            "type": "effect_check",
                            "when_effect": ["logging"],
                            "forbidden_data": ["@phi"]
                        }}
                    }}
                ]
            }}"#,
            name
        );

        let path = dir.join(format!("{}.json", name.to_lowercase()));
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    #[test]
    fn test_load_single_file() {
        let dir = TempDir::new().unwrap();
        create_test_framework_file(dir.path(), "HIPAA").unwrap();

        let mut loader = FrameworkLoader::new();
        let framework = loader.load_file(dir.path().join("hipaa.json")).unwrap();

        assert_eq!(framework.name(), "HIPAA");
        assert_eq!(framework.rule_count(), 1);
    }

    #[test]
    fn test_load_directory() {
        let dir = TempDir::new().unwrap();
        create_test_framework_file(dir.path(), "HIPAA").unwrap();
        create_test_framework_file(dir.path(), "PCI-DSS").unwrap();

        let mut loader = FrameworkLoader::new();
        let loaded = loader.load_directory(dir.path()).unwrap();

        assert_eq!(loaded.len(), 2);
        assert!(loader.has("HIPAA"));
        assert!(loader.has("PCI-DSS"));
    }

    #[test]
    fn test_load_nonexistent_directory() {
        let mut loader = FrameworkLoader::new();
        let loaded = loader.load_directory("/nonexistent/path").unwrap();
        assert!(loaded.is_empty());
    }

    #[test]
    fn test_get_framework() {
        let dir = TempDir::new().unwrap();
        create_test_framework_file(dir.path(), "TestFW").unwrap();

        let mut loader = FrameworkLoader::new();
        loader.load_file(dir.path().join("testfw.json")).unwrap();

        assert!(loader.get("TestFW").is_some());
        assert!(loader.get("NonExistent").is_none());
    }

    #[test]
    fn test_loader_count_and_names() {
        let dir = TempDir::new().unwrap();
        create_test_framework_file(dir.path(), "FW1").unwrap();
        create_test_framework_file(dir.path(), "FW2").unwrap();

        let mut loader = FrameworkLoader::new();
        loader.load_directory(dir.path()).unwrap();

        assert_eq!(loader.count(), 2);
        let names = loader.names();
        assert!(names.contains(&"FW1"));
        assert!(names.contains(&"FW2"));
    }
}
