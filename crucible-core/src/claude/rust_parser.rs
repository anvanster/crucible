//! Basic Rust code parser for architecture discovery

use crate::error::{CrucibleError, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Information about a discovered module
#[derive(Debug, Clone)]
pub struct DiscoveredModule {
    pub name: String,
    pub file_path: String,
    pub exports: Vec<String>,
    pub imports: Vec<String>,
}

/// Basic Rust code parser
pub struct RustParser;

impl RustParser {
    /// Discover modules from a Rust project
    pub fn discover_modules(source_root: &Path) -> Result<Vec<DiscoveredModule>> {
        let mut modules = Vec::new();

        // Find all .rs files
        for entry in WalkDir::new(source_root)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Some(module) = Self::parse_file(path)? {
                    modules.push(module);
                }
            }
        }

        Ok(modules)
    }

    /// Check if a file path represents a test file
    pub fn is_test_file(path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        // Check if file is in tests/ directory
        if path_str.contains("/tests/") || path_str.contains("\\tests\\") {
            return true;
        }

        // Check if file is in any test/ directory
        if path_str.contains("/test/") || path_str.contains("\\test\\") {
            return true;
        }

        // Check filename patterns
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            // Skip files ending with _test.rs
            if file_name.ends_with("_test.rs") {
                return true;
            }

            // Skip files starting with test_
            if file_name.starts_with("test_") {
                return true;
            }
        }

        false
    }

    /// Parse a single Rust file
    fn parse_file(path: &Path) -> Result<Option<DiscoveredModule>> {
        // Skip test files using comprehensive detection
        if Self::is_test_file(path) {
            return Ok(None);
        }

        let content = fs::read_to_string(path).map_err(|e| CrucibleError::FileRead {
            path: path.display().to_string(),
            source: e,
        })?;

        let module_name = Self::extract_module_name(path)?;
        let exports = Self::extract_exports(&content);
        let imports = Self::extract_imports(&content);

        Ok(Some(DiscoveredModule {
            name: module_name,
            file_path: path.display().to_string(),
            exports,
            imports,
        }))
    }

    /// Extract module name from file path
    fn extract_module_name(path: &Path) -> Result<String> {
        let file_stem =
            path.file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| CrucibleError::ParseError {
                    file: path.display().to_string(),
                    message: "Invalid file name".to_string(),
                })?;

        // mod.rs files take the name of their directory
        if file_stem == "mod" {
            if let Some(parent) = path.parent() {
                if let Some(dir_name) = parent.file_name().and_then(|s| s.to_str()) {
                    return Ok(dir_name.to_string());
                }
            }
        }

        Ok(file_stem.to_string())
    }

    /// Extract public exports from Rust code
    fn extract_exports(content: &str) -> Vec<String> {
        let mut exports = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // pub struct Name
            if let Some(rest) = trimmed.strip_prefix("pub struct ") {
                if let Some(name) = rest.split_whitespace().next() {
                    exports.push(name.trim_end_matches('<').to_string());
                }
            }

            // pub enum Name
            if let Some(rest) = trimmed.strip_prefix("pub enum ") {
                if let Some(name) = rest.split_whitespace().next() {
                    exports.push(name.trim_end_matches('<').to_string());
                }
            }

            // pub fn name
            if let Some(rest) = trimmed.strip_prefix("pub fn ") {
                if let Some(name) = rest.split('(').next() {
                    exports.push(name.trim().to_string());
                }
            }

            // pub type Name
            if let Some(rest) = trimmed.strip_prefix("pub type ") {
                if let Some(name) = rest.split_whitespace().next() {
                    exports.push(name.trim_end_matches('=').to_string());
                }
            }
        }

        exports
    }

    /// Extract imports from Rust code
    fn extract_imports(content: &str) -> Vec<String> {
        let mut imports = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // use crate::module::Item
            if trimmed.starts_with("use crate::") {
                if let Some(rest) = trimmed.strip_prefix("use crate::") {
                    // Extract module name (first segment after crate::)
                    if let Some(module) = rest.split("::").next() {
                        let module_clean = module.trim_end_matches(';');
                        if !imports.contains(&module_clean.to_string()) {
                            imports.push(module_clean.to_string());
                        }
                    }
                }
            }
        }

        imports
    }

    /// Build dependency map from discovered modules
    pub fn build_dependency_map(modules: &[DiscoveredModule]) -> HashMap<String, Vec<String>> {
        let mut deps = HashMap::new();

        for module in modules {
            let module_deps: Vec<String> = module
                .imports
                .iter()
                .filter(|imp| modules.iter().any(|m| &m.name == *imp))
                .cloned()
                .collect();

            deps.insert(module.name.clone(), module_deps);
        }

        deps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_exports() {
        let code = r#"
pub struct MyStruct {
    field: String,
}

pub enum MyEnum {
    Variant1,
    Variant2,
}

pub fn my_function() {}

pub type MyType = String;
        "#;

        let exports = RustParser::extract_exports(code);
        assert_eq!(exports.len(), 4);
        assert!(exports.contains(&"MyStruct".to_string()));
        assert!(exports.contains(&"MyEnum".to_string()));
        assert!(exports.contains(&"my_function".to_string()));
        assert!(exports.contains(&"MyType".to_string()));
    }

    #[test]
    fn test_extract_imports() {
        let code = r#"
use crate::types::Module;
use crate::error::CrucibleError;
use crate::parser::Parser;
use std::collections::HashMap;
        "#;

        let imports = RustParser::extract_imports(code);
        assert_eq!(imports.len(), 3);
        assert!(imports.contains(&"types".to_string()));
        assert!(imports.contains(&"error".to_string()));
        assert!(imports.contains(&"parser".to_string()));
    }
}
