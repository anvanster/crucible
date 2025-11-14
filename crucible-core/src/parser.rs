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
