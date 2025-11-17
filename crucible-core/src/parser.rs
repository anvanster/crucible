//! Parser for Crucible JSON files with caching support

use crate::cache::ArchitectureCache;
use crate::error::{CrucibleError, Result};
use crate::types::{Manifest, Module, Project, Rules};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub struct Parser {
    root_path: PathBuf,
    cache: Arc<Mutex<ArchitectureCache>>,
}

impl Parser {
    /// Create a new parser for a Crucible project
    pub fn new<P: AsRef<Path>>(root_path: P) -> Self {
        Self {
            root_path: root_path.as_ref().to_path_buf(),
            cache: Arc::new(Mutex::new(ArchitectureCache::new())),
        }
    }

    /// Create a parser with caching disabled
    pub fn new_without_cache<P: AsRef<Path>>(root_path: P) -> Self {
        Self {
            root_path: root_path.as_ref().to_path_buf(),
            cache: Arc::new(Mutex::new(ArchitectureCache::disabled())),
        }
    }

    /// Parse the entire project with caching
    pub fn parse_project(&self) -> Result<Project> {
        // Try to get cached project first
        let manifest_path = self.root_path.join("manifest.json");

        {
            let cache = self.cache.lock().unwrap();
            if let Some(cached_project) = cache.get_project(&manifest_path)? {
                return Ok(cached_project);
            }
        }

        // Not cached, parse normally
        let manifest = self.parse_manifest()?;
        let modules = self.parse_modules(&manifest.modules)?;
        let rules = self.parse_rules().ok();

        let project = Project {
            manifest,
            modules,
            rules,
        };

        // Cache the result
        {
            let mut cache = self.cache.lock().unwrap();
            cache.cache_project(&manifest_path, project.clone())?;
        }

        Ok(project)
    }

    /// Parse the manifest.json file
    pub fn parse_manifest(&self) -> Result<Manifest> {
        let manifest_path = self.root_path.join("manifest.json");
        let content = fs::read_to_string(&manifest_path).map_err(|e| CrucibleError::FileRead {
            path: manifest_path.display().to_string(),
            source: e,
        })?;

        serde_json::from_str(&content).map_err(|e| CrucibleError::ParseError {
            file: "manifest.json".to_string(),
            message: e.to_string(),
        })
    }

    /// Parse a module definition file with caching
    pub fn parse_module(&self, name: &str) -> Result<Module> {
        let module_path = self
            .root_path
            .join("modules")
            .join(format!("{}.json", name));

        // Check cache first
        {
            let cache = self.cache.lock().unwrap();
            if let Some(cached_module) = cache.get_module(&module_path)? {
                return Ok(cached_module);
            }
        }

        // Not cached, parse normally
        let content = fs::read_to_string(&module_path).map_err(|e| CrucibleError::FileRead {
            path: module_path.display().to_string(),
            source: e,
        })?;

        let module: Module = serde_json::from_str(&content).map_err(|e| CrucibleError::ParseError {
            file: format!("{}.json", name),
            message: e.to_string(),
        })?;

        // Cache the parsed module
        {
            let mut cache = self.cache.lock().unwrap();
            cache.cache_module(module_path, module.clone())?;
        }

        Ok(module)
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
        let content = fs::read_to_string(&rules_path).map_err(|e| CrucibleError::FileRead {
            path: rules_path.display().to_string(),
            source: e,
        })?;

        serde_json::from_str(&content).map_err(|e| CrucibleError::ParseError {
            file: "rules.json".to_string(),
            message: e.to_string(),
        })
    }

    /// Clear the cache
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> crate::cache::CacheStats {
        let cache = self.cache.lock().unwrap();
        cache.stats()
    }

    /// Enable or disable caching
    pub fn set_caching_enabled(&self, enabled: bool) {
        let mut cache = self.cache.lock().unwrap();
        cache.set_enabled(enabled);
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

        // Caching should be enabled by default
        let stats = parser.cache_stats();
        assert!(stats.enabled);
    }

    #[test]
    fn test_parser_without_cache() {
        let dir = tempdir().unwrap();
        let parser = Parser::new_without_cache(dir.path());

        let stats = parser.cache_stats();
        assert!(!stats.enabled);
    }

    #[test]
    fn test_parse_manifest() {
        let dir = tempdir().unwrap();
        let manifest_content = r#"{
            "version": "0.1.0",
            "project": {
                "name": "test-project",
                "language": "rust",
                "architecture_pattern": "layered"
            },
            "modules": ["test"],
            "strict_validation": false
        }"#;

        fs::write(dir.path().join("manifest.json"), manifest_content).unwrap();

        let parser = Parser::new(dir.path());
        let manifest = parser.parse_manifest().unwrap();

        assert_eq!(manifest.project.name, "test-project");
        assert_eq!(manifest.modules, vec!["test"]);
    }

    #[test]
    fn test_parse_module_with_cache() {
        let dir = tempdir().unwrap();
        let modules_dir = dir.path().join("modules");
        fs::create_dir(&modules_dir).unwrap();

        let module_content = r#"{
            "module": "test",
            "version": "1.0.0",
            "layer": "core",
            "description": "Test module",
            "exports": {},
            "dependencies": {}
        }"#;

        fs::write(modules_dir.join("test.json"), module_content).unwrap();

        let parser = Parser::new(dir.path());

        // First parse should cache
        let module1 = parser.parse_module("test").unwrap();
        assert_eq!(module1.module, "test");

        // Check cache has the module
        let stats = parser.cache_stats();
        assert_eq!(stats.modules_cached, 1);

        // Second parse should use cache
        let module2 = parser.parse_module("test").unwrap();
        assert_eq!(module2.module, "test");
    }
}