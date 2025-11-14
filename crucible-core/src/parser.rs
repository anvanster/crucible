//! Parser for Crucible JSON files

use crate::error::{CrucibleError, Result};
use crate::types::{Manifest, Module, Project, Rules};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Parser {
    root_path: PathBuf,
}

impl Parser {
    /// Create a new parser for a Crucible project
    pub fn new<P: AsRef<Path>>(root_path: P) -> Self {
        Self {
            root_path: root_path.as_ref().to_path_buf(),
        }
    }

    /// Parse the entire project
    pub fn parse_project(&self) -> Result<Project> {
        let manifest = self.parse_manifest()?;
        let modules = self.parse_modules(&manifest.modules)?;
        let rules = self.parse_rules().ok();

        Ok(Project {
            manifest,
            modules,
            rules,
        })
    }

    /// Parse the manifest.json file
    pub fn parse_manifest(&self) -> Result<Manifest> {
        let manifest_path = self.root_path.join("manifest.json");
        let content = fs::read_to_string(&manifest_path)
            .map_err(|e| CrucibleError::FileRead {
                path: manifest_path.display().to_string(),
                source: e,
            })?;

        serde_json::from_str(&content)
            .map_err(|e| CrucibleError::ParseError {
                file: "manifest.json".to_string(),
                message: e.to_string(),
            })
    }

    /// Parse a module definition file
    pub fn parse_module(&self, name: &str) -> Result<Module> {
        let module_path = self.root_path.join("modules").join(format!("{}.json", name));
        let content = fs::read_to_string(&module_path)
            .map_err(|e| CrucibleError::FileRead {
                path: module_path.display().to_string(),
                source: e,
            })?;

        serde_json::from_str(&content)
            .map_err(|e| CrucibleError::ParseError {
                file: format!("{}.json", name),
                message: e.to_string(),
            })
    }

    /// Parse all modules listed in the manifest
    pub fn parse_modules(&self, module_names: &[String]) -> Result<Vec<Module>> {
        module_names
            .iter()
            .map(|name| self.parse_module(name))
            .collect()
    }

    /// Parse the rules.json file
    pub fn parse_rules(&self) -> Result<Rules> {
        let rules_path = self.root_path.join("rules.json");
        let content = fs::read_to_string(&rules_path)
            .map_err(|e| CrucibleError::FileRead {
                path: rules_path.display().to_string(),
                source: e,
            })?;

        serde_json::from_str(&content)
            .map_err(|e| CrucibleError::ParseError {
                file: "rules.json".to_string(),
                message: e.to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_parser_new() {
        let dir = tempdir().unwrap();
        let parser = Parser::new(dir.path());
        assert_eq!(parser.root_path, dir.path());
    }

    #[test]
    fn test_parse_valid_manifest() {
        let dir = tempdir().unwrap();
        let manifest_content = r#"{
            "version": "0.1.0",
            "project": {
                "name": "test-project",
                "language": "rust",
                "architecture_pattern": "layered"
            },
            "modules": ["module1", "module2"],
            "strict_validation": true
        }"#;

        fs::write(dir.path().join("manifest.json"), manifest_content).unwrap();

        let parser = Parser::new(dir.path());
        let manifest = parser.parse_manifest().unwrap();

        assert_eq!(manifest.version, "0.1.0");
        assert_eq!(manifest.project.name, "test-project");
        assert_eq!(manifest.modules.len(), 2);
        assert!(manifest.strict_validation);
    }

    #[test]
    fn test_parse_manifest_missing_file() {
        let dir = tempdir().unwrap();
        let parser = Parser::new(dir.path());
        let result = parser.parse_manifest();

        assert!(result.is_err());
        match result.unwrap_err() {
            CrucibleError::FileRead { path, .. } => {
                assert!(path.contains("manifest.json"));
            }
            _ => panic!("Expected FileRead error"),
        }
    }

    #[test]
    fn test_parse_manifest_invalid_json() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("manifest.json"), "not valid json").unwrap();

        let parser = Parser::new(dir.path());
        let result = parser.parse_manifest();

        assert!(result.is_err());
        match result.unwrap_err() {
            CrucibleError::ParseError { file, .. } => {
                assert_eq!(file, "manifest.json");
            }
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_parse_valid_module() {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("modules")).unwrap();

        let module_content = r#"{
            "module": "test-module",
            "version": "1.0.0",
            "layer": "core",
            "exports": {},
            "dependencies": {}
        }"#;

        fs::write(
            dir.path().join("modules/test-module.json"),
            module_content,
        )
        .unwrap();

        let parser = Parser::new(dir.path());
        let module = parser.parse_module("test-module").unwrap();

        assert_eq!(module.module, "test-module");
        assert_eq!(module.version, "1.0.0");
        assert_eq!(module.layer, Some("core".to_string()));
    }

    #[test]
    fn test_parse_module_missing_file() {
        let dir = tempdir().unwrap();
        let parser = Parser::new(dir.path());
        let result = parser.parse_module("nonexistent");

        assert!(result.is_err());
        match result.unwrap_err() {
            CrucibleError::FileRead { path, .. } => {
                assert!(path.contains("nonexistent.json"));
            }
            _ => panic!("Expected FileRead error"),
        }
    }

    #[test]
    fn test_parse_modules() {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("modules")).unwrap();

        // Create two module files
        let module_a = r#"{"module": "a", "version": "1.0.0", "exports": {}}"#;
        let module_b = r#"{"module": "b", "version": "1.0.0", "exports": {}}"#;

        fs::write(dir.path().join("modules/a.json"), module_a).unwrap();
        fs::write(dir.path().join("modules/b.json"), module_b).unwrap();

        let parser = Parser::new(dir.path());
        let modules = parser
            .parse_modules(&[String::from("a"), String::from("b")])
            .unwrap();

        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].module, "a");
        assert_eq!(modules[1].module, "b");
    }

    #[test]
    fn test_parse_modules_one_missing() {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("modules")).unwrap();

        let module_a = r#"{"module": "a", "version": "1.0.0", "exports": {}}"#;
        fs::write(dir.path().join("modules/a.json"), module_a).unwrap();

        let parser = Parser::new(dir.path());
        let result = parser.parse_modules(&[String::from("a"), String::from("missing")]);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_valid_rules() {
        let dir = tempdir().unwrap();
        let rules_content = r#"{
            "rules": [
                {
                    "id": "test-rule",
                    "enabled": true,
                    "severity": "error",
                    "description": "Test rule"
                }
            ]
        }"#;

        fs::write(dir.path().join("rules.json"), rules_content).unwrap();

        let parser = Parser::new(dir.path());
        let rules = parser.parse_rules().unwrap();

        assert_eq!(rules.rules.len(), 1);
        assert_eq!(rules.rules[0].id, "test-rule");
        assert!(rules.rules[0].enabled);
    }

    #[test]
    fn test_parse_rules_missing_file() {
        let dir = tempdir().unwrap();
        let parser = Parser::new(dir.path());
        let result = parser.parse_rules();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_complete_project() {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("modules")).unwrap();

        // Create manifest
        let manifest = r#"{
            "version": "0.1.0",
            "project": {"name": "test", "language": "rust"},
            "modules": ["core"]
        }"#;
        fs::write(dir.path().join("manifest.json"), manifest).unwrap();

        // Create module
        let module = r#"{"module": "core", "version": "1.0.0", "exports": {}}"#;
        fs::write(dir.path().join("modules/core.json"), module).unwrap();

        // Create rules
        let rules = r#"{"rules": []}"#;
        fs::write(dir.path().join("rules.json"), rules).unwrap();

        let parser = Parser::new(dir.path());
        let project = parser.parse_project().unwrap();

        assert_eq!(project.manifest.project.name, "test");
        assert_eq!(project.modules.len(), 1);
        assert!(project.rules.is_some());
    }

    #[test]
    fn test_parse_project_without_rules() {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("modules")).unwrap();

        // Create manifest
        let manifest = r#"{
            "version": "0.1.0",
            "project": {"name": "test", "language": "rust"},
            "modules": []
        }"#;
        fs::write(dir.path().join("manifest.json"), manifest).unwrap();

        let parser = Parser::new(dir.path());
        let project = parser.parse_project().unwrap();

        assert_eq!(project.manifest.project.name, "test");
        assert!(project.modules.is_empty());
        assert!(project.rules.is_none()); // Rules are optional
    }
}
