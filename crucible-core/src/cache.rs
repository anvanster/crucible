//! Caching layer for parsed architecture definitions

use crate::error::{CrucibleError, Result};
use crate::types::{Module, Project};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Cache for parsed module definitions to avoid repeated parsing
pub struct ArchitectureCache {
    /// Cached modules with their last modified time
    modules: HashMap<PathBuf, (Module, SystemTime)>,

    /// Cached project manifest with last modified time
    project: Option<(Project, SystemTime)>,

    /// Whether caching is enabled
    enabled: bool,
}

impl ArchitectureCache {
    /// Create a new cache instance
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            project: None,
            enabled: true,
        }
    }

    /// Create a disabled cache (for testing or when caching is not desired)
    pub fn disabled() -> Self {
        Self {
            modules: HashMap::new(),
            project: None,
            enabled: false,
        }
    }

    /// Enable or disable caching
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.clear();
        }
    }

    /// Get a cached module if it's still valid
    pub fn get_module(&self, path: &Path) -> Result<Option<Module>> {
        if !self.enabled {
            return Ok(None);
        }

        if let Some((module, cached_time)) = self.modules.get(path) {
            // Check if file has been modified since caching
            let metadata = fs::metadata(path).map_err(|e| CrucibleError::FileRead {
                path: path.display().to_string(),
                source: e,
            })?;

            let modified = metadata
                .modified()
                .map_err(|e| CrucibleError::FileRead {
                    path: path.display().to_string(),
                    source: e,
                })?;

            if modified <= *cached_time {
                // Cache is still valid
                return Ok(Some(module.clone()));
            }
        }

        Ok(None)
    }

    /// Cache a module definition
    pub fn cache_module(&mut self, path: PathBuf, module: Module) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let metadata = fs::metadata(&path).map_err(|e| CrucibleError::FileRead {
            path: path.display().to_string(),
            source: e,
        })?;

        let modified = metadata
            .modified()
            .map_err(|e| CrucibleError::FileRead {
                path: path.display().to_string(),
                source: e,
            })?;

        self.modules.insert(path, (module, modified));
        Ok(())
    }

    /// Get the cached project if it's still valid
    pub fn get_project(&self, manifest_path: &Path) -> Result<Option<Project>> {
        if !self.enabled {
            return Ok(None);
        }

        if let Some((project, cached_time)) = &self.project {
            let metadata = fs::metadata(manifest_path).map_err(|e| CrucibleError::FileRead {
                path: manifest_path.display().to_string(),
                source: e,
            })?;

            let modified = metadata
                .modified()
                .map_err(|e| CrucibleError::FileRead {
                    path: manifest_path.display().to_string(),
                    source: e,
                })?;

            if modified <= *cached_time {
                return Ok(Some(project.clone()));
            }
        }

        Ok(None)
    }

    /// Cache the project manifest
    pub fn cache_project(&mut self, manifest_path: &Path, project: Project) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let metadata = fs::metadata(manifest_path).map_err(|e| CrucibleError::FileRead {
            path: manifest_path.display().to_string(),
            source: e,
        })?;

        let modified = metadata
            .modified()
            .map_err(|e| CrucibleError::FileRead {
                path: manifest_path.display().to_string(),
                source: e,
            })?;

        self.project = Some((project, modified));
        Ok(())
    }

    /// Clear all cached data
    pub fn clear(&mut self) {
        self.modules.clear();
        self.project = None;
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            modules_cached: self.modules.len(),
            project_cached: self.project.is_some(),
            enabled: self.enabled,
        }
    }
}

impl Default for ArchitectureCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the cache
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of modules in cache
    pub modules_cached: usize,

    /// Whether project is cached
    pub project_cached: bool,

    /// Whether caching is enabled
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    fn test_cache_disabled() {
        let mut cache = ArchitectureCache::disabled();
        assert!(!cache.enabled);

        let module = Module {
            module: "test".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: Some("Test".to_string()),
            exports: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.json");
        fs::write(&path, "{}").unwrap();

        // Caching should do nothing when disabled
        cache.cache_module(path.clone(), module.clone()).unwrap();
        assert_eq!(cache.modules.len(), 0);

        // Getting should return None when disabled
        assert!(cache.get_module(&path).unwrap().is_none());
    }

    #[test]
    fn test_cache_enabled() {
        let mut cache = ArchitectureCache::new();
        assert!(cache.enabled);

        let module = Module {
            module: "test".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: Some("Test".to_string()),
            exports: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.json");
        fs::write(&path, "{}").unwrap();

        // Cache the module
        cache.cache_module(path.clone(), module.clone()).unwrap();
        assert_eq!(cache.modules.len(), 1);

        // Should return cached module
        let cached = cache.get_module(&path).unwrap().unwrap();
        assert_eq!(cached.module, "test");
    }

    #[test]
    fn test_cache_invalidation() {
        let mut cache = ArchitectureCache::new();

        let module = Module {
            module: "test".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: Some("Test".to_string()),
            exports: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.json");
        fs::write(&path, "{}").unwrap();

        // Cache the module
        cache.cache_module(path.clone(), module).unwrap();

        // Modify the file (this simulates a change)
        std::thread::sleep(std::time::Duration::from_millis(10));
        fs::write(&path, "{\"modified\": true}").unwrap();

        // Cache should be invalidated
        assert!(cache.get_module(&path).unwrap().is_none());
    }
}