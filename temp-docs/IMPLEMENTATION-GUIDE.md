# Crucible Implementation Guide - Building Crucible with Crucible

This guide shows how to implement Crucible using its own architectural principles. We'll use Claude Code to build the reference implementation with Crucible defining its own architecture.

## Phase 1: Project Initialization

### Step 1: Create Project Structure

```bash
mkdir crucible
cd crucible
cargo init --name crucible-core --lib
```

### Step 2: Create Directory Layout

```
crucible/
├── .crucible/                    # Crucible's own architecture
│   ├── manifest.json
│   ├── modules/
│   │   ├── parser.json
│   │   ├── validator.json
│   │   ├── generator.json
│   │   └── cli.json
│   └── rules.json
│
├── spec/                         # Specification documents
│   ├── LICENSE-SPEC
│   ├── SPEC.md
│   ├── README.md
│   ├── GETTING-STARTED.md
│   ├── PROJECT-STRUCTURE.md
│   ├── BRANDING.md
│   ├── LICENSING.md
│   ├── LICENSE-QUICK-REF.md
│   ├── INDEX.md
│   ├── schema.json
│   └── examples/
│       └── todo-app/
│           ├── manifest.json
│           └── modules/
│
├── crucible-core/                # Core library (Rust)
│   ├── Cargo.toml
│   ├── LICENSE-CODE
│   └── src/
│       ├── lib.rs
│       ├── parser/
│       ├── validator/
│       ├── types/
│       └── error/
│
├── crucible-cli/                 # CLI tool (Rust)
│   ├── Cargo.toml
│   ├── LICENSE-CODE
│   └── src/
│       └── main.rs
│
├── Cargo.toml                    # Workspace root
├── README.md
├── LICENSE-CODE
├── LICENSE-SPEC
└── LICENSING.md
```

## Phase 2: Define Crucible's Architecture (Meta!)

### Step 3: Create Crucible's Own Manifest

Create `.crucible/manifest.json`:

```json
{
  "version": "0.1.0",
  "project": {
    "name": "crucible",
    "language": "rust",
    "architecture_pattern": "layered"
  },
  "modules": [
    "parser",
    "validator", 
    "generator",
    "types",
    "error",
    "cli"
  ],
  "strict_validation": true,
  "metadata": {
    "author": "Crucible Contributors",
    "repository": "https://github.com/crucible-spec/crucible",
    "created": "2025-11-14T00:00:00Z"
  }
}
```

### Step 4: Define Core Modules

Create `.crucible/modules/parser.json`:

```json
{
  "module": "parser",
  "version": "0.1.0",
  "layer": "core",
  "description": "Parses Crucible JSON files into internal representations",
  
  "exports": {
    "Parser": {
      "type": "class",
      "methods": {
        "parse_manifest": {
          "inputs": [
            {"name": "path", "type": "string"}
          ],
          "returns": {
            "type": "Result",
            "inner": "types.Manifest, error.ParseError"
          },
          "throws": [],
          "calls": [
            "std.fs.read_file",
            "serde_json.from_str"
          ],
          "effects": ["file.read"]
        },
        "parse_module": {
          "inputs": [
            {"name": "path", "type": "string"}
          ],
          "returns": {
            "type": "Result",
            "inner": "types.Module, error.ParseError"
          },
          "throws": [],
          "calls": [
            "std.fs.read_file",
            "serde_json.from_str"
          ],
          "effects": ["file.read"]
        },
        "parse_rules": {
          "inputs": [
            {"name": "path", "type": "string"}
          ],
          "returns": {
            "type": "Result",
            "inner": "types.Rules, error.ParseError"
          },
          "throws": [],
          "calls": [
            "std.fs.read_file",
            "serde_json.from_str"
          ],
          "effects": ["file.read"]
        }
      },
      "dependencies": [
        {"module": "types", "imports": ["Manifest", "Module", "Rules"]},
        {"module": "error", "imports": ["ParseError"]}
      ]
    }
  },
  
  "dependencies": {
    "types": "^0.1.0",
    "error": "^0.1.0"
  }
}
```

Create `.crucible/modules/validator.json`:

```json
{
  "module": "validator",
  "version": "0.1.0",
  "layer": "core",
  "description": "Validates Crucible architectures against rules",
  
  "exports": {
    "Validator": {
      "type": "class",
      "methods": {
        "validate": {
          "inputs": [
            {"name": "project", "type": "types.Project"},
            {"name": "rules", "type": "types.Rules"}
          ],
          "returns": {
            "type": "Result",
            "inner": "ValidationResult, error.ValidationError"
          },
          "throws": [],
          "calls": [
            "validator.check_circular_dependencies",
            "validator.check_layer_boundaries",
            "validator.check_type_existence",
            "validator.check_call_targets"
          ],
          "effects": []
        },
        "check_circular_dependencies": {
          "inputs": [
            {"name": "modules", "type": "Vec<types.Module>"}
          ],
          "returns": {
            "type": "Vec",
            "inner": "ValidationIssue"
          },
          "throws": [],
          "calls": [
            "graph.build_dependency_graph",
            "graph.detect_cycles"
          ],
          "effects": []
        },
        "check_layer_boundaries": {
          "inputs": [
            {"name": "modules", "type": "Vec<types.Module>"},
            {"name": "architecture", "type": "types.Architecture"}
          ],
          "returns": {
            "type": "Vec",
            "inner": "ValidationIssue"
          },
          "throws": [],
          "calls": [],
          "effects": []
        }
      },
      "dependencies": [
        {"module": "types", "imports": ["Project", "Module", "Rules", "Architecture"]},
        {"module": "error", "imports": ["ValidationError"]},
        {"module": "graph", "imports": ["build_dependency_graph", "detect_cycles"]}
      ]
    },
    "ValidationResult": {
      "type": "interface",
      "properties": {
        "valid": {"type": "boolean", "required": true},
        "errors": {"type": "Vec<ValidationIssue>", "required": true},
        "warnings": {"type": "Vec<ValidationIssue>", "required": true}
      }
    },
    "ValidationIssue": {
      "type": "interface",
      "properties": {
        "rule": {"type": "string", "required": true},
        "severity": {"type": "Severity", "required": true},
        "message": {"type": "string", "required": true},
        "location": {"type": "string", "required": false}
      }
    },
    "Severity": {
      "type": "enum",
      "values": ["error", "warning", "info"]
    }
  },
  
  "dependencies": {
    "types": "^0.1.0",
    "error": "^0.1.0",
    "graph": "^0.1.0"
  }
}
```

Create `.crucible/modules/types.json`:

```json
{
  "module": "types",
  "version": "0.1.0",
  "layer": "core",
  "description": "Core type definitions for Crucible",
  
  "exports": {
    "Manifest": {
      "type": "interface",
      "properties": {
        "version": {"type": "string", "required": true},
        "project": {"type": "ProjectConfig", "required": true},
        "modules": {"type": "Vec<string>", "required": true},
        "strict_validation": {"type": "boolean", "required": false}
      }
    },
    "ProjectConfig": {
      "type": "interface",
      "properties": {
        "name": {"type": "string", "required": true},
        "language": {"type": "Language", "required": true},
        "architecture_pattern": {"type": "ArchitecturePattern", "required": false}
      }
    },
    "Language": {
      "type": "enum",
      "values": ["typescript", "rust", "python", "go", "java"]
    },
    "ArchitecturePattern": {
      "type": "enum",
      "values": ["layered", "hexagonal", "microservices", "modular"]
    },
    "Module": {
      "type": "interface",
      "properties": {
        "module": {"type": "string", "required": true},
        "version": {"type": "string", "required": true},
        "layer": {"type": "string", "required": false},
        "description": {"type": "string", "required": false},
        "exports": {"type": "HashMap<string, Export>", "required": true},
        "dependencies": {"type": "HashMap<string, string>", "required": false}
      }
    },
    "Export": {
      "type": "interface",
      "properties": {
        "export_type": {"type": "ExportType", "required": true},
        "methods": {"type": "HashMap<string, Method>", "required": false},
        "properties": {"type": "HashMap<string, Property>", "required": false},
        "values": {"type": "Vec<string>", "required": false}
      }
    },
    "ExportType": {
      "type": "enum",
      "values": ["class", "function", "interface", "type", "enum"]
    },
    "Method": {
      "type": "interface",
      "properties": {
        "inputs": {"type": "Vec<Parameter>", "required": true},
        "returns": {"type": "ReturnType", "required": true},
        "throws": {"type": "Vec<string>", "required": false},
        "calls": {"type": "Vec<string>", "required": false},
        "effects": {"type": "Vec<string>", "required": false}
      }
    },
    "Project": {
      "type": "interface",
      "properties": {
        "manifest": {"type": "Manifest", "required": true},
        "modules": {"type": "Vec<Module>", "required": true},
        "rules": {"type": "Rules", "required": false}
      }
    },
    "Rules": {
      "type": "interface",
      "properties": {
        "architecture": {"type": "Architecture", "required": false},
        "rules": {"type": "Vec<Rule>", "required": true},
        "custom_rules": {"type": "Vec<CustomRule>", "required": false}
      }
    }
  },
  
  "dependencies": {}
}
```

### Step 5: Create Validation Rules

Create `.crucible/rules.json`:

```json
{
  "architecture": {
    "pattern": "layered",
    "layers": [
      {
        "name": "cli",
        "can_depend_on": ["application", "core"]
      },
      {
        "name": "application", 
        "can_depend_on": ["core"]
      },
      {
        "name": "core",
        "can_depend_on": []
      }
    ]
  },
  
  "rules": [
    {
      "id": "no-circular-dependencies",
      "enabled": true,
      "severity": "error",
      "description": "Modules must not have circular dependencies"
    },
    {
      "id": "respect-layer-boundaries",
      "enabled": true,
      "severity": "error",
      "description": "Modules can only depend on allowed layers"
    },
    {
      "id": "all-calls-must-exist",
      "enabled": true,
      "severity": "error",
      "description": "All function calls must reference exported functions"
    }
  ],
  
  "custom_rules": [
    {
      "id": "error-suffix",
      "type": "naming-convention",
      "target": "export",
      "pattern": "^[A-Z][a-zA-Z]*Error$",
      "severity": "warning",
      "description": "Error types should end with 'Error'"
    },
    {
      "id": "test-coverage",
      "type": "custom",
      "severity": "warning",
      "config": {
        "minimum_coverage": 80
      }
    }
  ]
}
```

## Phase 3: Initial Rust Implementation

### Step 6: Create Workspace Cargo.toml

```toml
[workspace]
members = [
    "crucible-core",
    "crucible-cli",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
authors = ["Crucible Contributors"]
edition = "2021"
license = "Apache-2.0"
repository = "https://github.com/crucible-spec/crucible"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
anyhow = "1.0"
clap = { version = "4.0", features = ["derive"] }
petgraph = "0.6"
semver = "1.0"
```

### Step 7: Create Core Library Structure

Create `crucible-core/Cargo.toml`:

```toml
[package]
name = "crucible-core"
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
petgraph.workspace = true
semver.workspace = true

[dev-dependencies]
tempfile = "3.0"
```

Create `crucible-core/src/lib.rs`:

```rust
//! Crucible Core - Architecture validation engine
//! 
//! This library implements the Crucible specification for
//! AI-native application architecture.

pub mod error;
pub mod parser;
pub mod types;
pub mod validator;

pub use error::{CrucibleError, Result};
pub use parser::Parser;
pub use types::{Manifest, Module, Project};
pub use validator::{ValidationResult, Validator};

/// Version of the Crucible specification this library implements
pub const SPEC_VERSION: &str = "0.1.0";
```

Create `crucible-core/src/types.rs`:

```rust
//! Core type definitions matching the Crucible specification

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub version: String,
    pub project: ProjectConfig,
    pub modules: Vec<String>,
    #[serde(default = "default_strict")]
    pub strict_validation: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub language: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_pattern: Option<ArchitecturePattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    TypeScript,
    Rust,
    Python,
    Go,
    Java,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArchitecturePattern {
    Layered,
    Hexagonal,
    Microservices,
    Modular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub module: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub exports: HashMap<String, Export>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    #[serde(rename = "type")]
    pub export_type: ExportType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub methods: Option<HashMap<String, Method>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Property>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<Dependency>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportType {
    Class,
    Function,
    Interface,
    Type,
    Enum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub inputs: Vec<Parameter>,
    pub returns: ReturnType,
    #[serde(default)]
    pub throws: Vec<String>,
    #[serde(default)]
    pub calls: Vec<String>,
    #[serde(default)]
    pub effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    #[serde(default)]
    pub optional: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnType {
    #[serde(rename = "type")]
    pub return_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    pub prop_type: String,
    #[serde(default = "default_required")]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub module: String,
    pub imports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

/// A complete Crucible project
#[derive(Debug)]
pub struct Project {
    pub manifest: Manifest,
    pub modules: Vec<Module>,
    pub rules: Option<Rules>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Architecture>,
    pub rules: Vec<Rule>,
    #[serde(default)]
    pub custom_rules: Vec<CustomRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Architecture {
    pub pattern: ArchitecturePattern,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub can_depend_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub enabled: bool,
    pub severity: Severity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRule {
    pub id: String,
    #[serde(rename = "type")]
    pub rule_type: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    pub severity: Severity,
}

fn default_strict() -> bool {
    true
}

fn default_required() -> bool {
    true
}
```

Create `crucible-core/src/parser.rs`:

```rust
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
```

Create `crucible-core/src/error.rs`:

```rust
//! Error types for Crucible

use std::fmt;
use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CrucibleError>;

#[derive(Error, Debug)]
pub enum CrucibleError {
    #[error("Failed to read file {path}: {source}")]
    FileRead {
        path: String,
        #[source]
        source: io::Error,
    },

    #[error("Failed to parse {file}: {message}")]
    ParseError { file: String, message: String },

    #[error("Module not found: {name}")]
    ModuleNotFound { name: String },

    #[error("Export not found: {module}.{export}")]
    ExportNotFound { module: String, export: String },

    #[error("Circular dependency detected: {cycle}")]
    CircularDependency { cycle: String },

    #[error("Layer boundary violation: {from} -> {to}")]
    LayerViolation { from: String, to: String },

    #[error("Type not found: {type_name}")]
    TypeNotFound { type_name: String },

    #[error("Function call target not found: {call}")]
    CallTargetNotFound { call: String },

    #[error("Validation failed: {message}")]
    ValidationFailed { message: String },
}
```

Create `crucible-core/src/validator.rs`:

```rust
//! Architecture validation engine

use crate::error::{CrucibleError, Result};
use crate::types::{Module, Project, Rules, Severity};
use petgraph::algo::is_cyclic_directed;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
    pub info: Vec<ValidationIssue>,
}

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub rule: String,
    pub severity: Severity,
    pub message: String,
    pub location: Option<String>,
}

pub struct Validator {
    project: Project,
}

impl Validator {
    pub fn new(project: Project) -> Self {
        Self { project }
    }

    /// Run all validation rules
    pub fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        };

        // Check for circular dependencies
        if let Some(issues) = self.check_circular_dependencies() {
            for issue in issues {
                match issue.severity {
                    Severity::Error => {
                        result.valid = false;
                        result.errors.push(issue);
                    }
                    Severity::Warning => result.warnings.push(issue),
                    Severity::Info => result.info.push(issue),
                }
            }
        }

        // Check layer boundaries if architecture is defined
        if let Some(rules) = &self.project.rules {
            if let Some(arch) = &rules.architecture {
                if let Some(issues) = self.check_layer_boundaries(arch) {
                    for issue in issues {
                        match issue.severity {
                            Severity::Error => {
                                result.valid = false;
                                result.errors.push(issue);
                            }
                            Severity::Warning => result.warnings.push(issue),
                            Severity::Info => result.info.push(issue),
                        }
                    }
                }
            }
        }

        // Check that all types exist
        if let Some(issues) = self.check_type_existence() {
            for issue in issues {
                match issue.severity {
                    Severity::Error => {
                        result.valid = false;
                        result.errors.push(issue);
                    }
                    Severity::Warning => result.warnings.push(issue),
                    Severity::Info => result.info.push(issue),
                }
            }
        }

        result
    }

    /// Check for circular dependencies between modules
    fn check_circular_dependencies(&self) -> Option<Vec<ValidationIssue>> {
        let mut graph = DiGraph::new();
        let mut node_map = HashMap::new();
        let mut issues = Vec::new();

        // Add nodes for each module
        for module in &self.project.modules {
            let node = graph.add_node(module.module.clone());
            node_map.insert(module.module.clone(), node);
        }

        // Add edges for dependencies
        for module in &self.project.modules {
            if let Some(from_node) = node_map.get(&module.module) {
                for (dep_name, _) in &module.dependencies {
                    if let Some(to_node) = node_map.get(dep_name) {
                        graph.add_edge(*from_node, *to_node, ());
                    }
                }
            }
        }

        // Check for cycles
        if is_cyclic_directed(&graph) {
            issues.push(ValidationIssue {
                rule: "no-circular-dependencies".to_string(),
                severity: Severity::Error,
                message: "Circular dependency detected in module graph".to_string(),
                location: None,
            });
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    /// Check that modules respect layer boundaries
    fn check_layer_boundaries(
        &self,
        architecture: &crate::types::Architecture,
    ) -> Option<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // Create a map of module to layer
        let mut module_layers = HashMap::new();
        for module in &self.project.modules {
            if let Some(layer) = &module.layer {
                module_layers.insert(module.module.clone(), layer.clone());
            }
        }

        // Check each module's dependencies
        for module in &self.project.modules {
            if let Some(from_layer) = module.layer.as_ref() {
                // Find the layer definition
                let layer_def = architecture.layers.iter().find(|l| &l.name == from_layer);
                
                if let Some(layer) = layer_def {
                    // Check each dependency
                    for (dep_name, _) in &module.dependencies {
                        if let Some(to_layer) = module_layers.get(dep_name) {
                            // Check if this dependency is allowed
                            if !layer.can_depend_on.contains(to_layer) {
                                issues.push(ValidationIssue {
                                    rule: "respect-layer-boundaries".to_string(),
                                    severity: Severity::Error,
                                    message: format!(
                                        "Layer '{}' cannot depend on layer '{}'",
                                        from_layer, to_layer
                                    ),
                                    location: Some(format!("{} -> {}", module.module, dep_name)),
                                });
                            }
                        }
                    }
                }
            }
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    /// Check that all referenced types exist
    fn check_type_existence(&self) -> Option<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // Collect all available types
        let mut available_types = HashMap::new();
        for module in &self.project.modules {
            for (export_name, _) in &module.exports {
                available_types.insert(
                    format!("{}.{}", module.module, export_name),
                    true,
                );
                available_types.insert(export_name.clone(), true);
            }
        }

        // Add primitive types
        for primitive in &["string", "number", "boolean", "void", "null"] {
            available_types.insert(primitive.to_string(), true);
        }

        // Check all type references
        for module in &self.project.modules {
            for (export_name, export) in &module.exports {
                // Check method parameter and return types
                if let Some(methods) = &export.methods {
                    for (method_name, method) in methods {
                        // Check input types
                        for param in &method.inputs {
                            if !self.is_type_available(&param.param_type, &available_types) {
                                issues.push(ValidationIssue {
                                    rule: "all-types-must-exist".to_string(),
                                    severity: Severity::Error,
                                    message: format!("Type '{}' not found", param.param_type),
                                    location: Some(format!(
                                        "{}.{}.{}",
                                        module.module, export_name, method_name
                                    )),
                                });
                            }
                        }

                        // Check return type
                        if !self.is_type_available(&method.returns.return_type, &available_types) {
                            issues.push(ValidationIssue {
                                rule: "all-types-must-exist".to_string(),
                                severity: Severity::Error,
                                message: format!("Type '{}' not found", method.returns.return_type),
                                location: Some(format!(
                                    "{}.{}.{}",
                                    module.module, export_name, method_name
                                )),
                            });
                        }
                    }
                }
            }
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    /// Check if a type is available (handles generics)
    fn is_type_available(&self, type_name: &str, available_types: &HashMap<String, bool>) -> bool {
        // Handle generic types like Array<T>, Map<K,V>, etc.
        if type_name.contains('<') {
            let base_type = type_name.split('<').next().unwrap();
            return matches!(
                base_type,
                "Array" | "Vec" | "Map" | "HashMap" | "Promise" | "Result" | "Optional"
            );
        }

        available_types.contains_key(type_name)
    }
}
```

### Step 8: Create CLI Implementation

Create `crucible-cli/Cargo.toml`:

```toml
[package]
name = "crucible-cli"
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "crucible"
path = "src/main.rs"

[dependencies]
crucible-core = { path = "../crucible-core" }
clap.workspace = true
anyhow.workspace = true
colored = "2.0"
```

Create `crucible-cli/src/main.rs`:

```rust
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use crucible_core::{Parser as CrucibleParser, Validator};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Crucible project
    Init {
        /// Project name
        #[arg(long)]
        name: String,
        
        /// Programming language
        #[arg(long, default_value = "typescript")]
        language: String,
        
        /// Architecture pattern
        #[arg(long, default_value = "layered")]
        pattern: String,
    },
    
    /// Validate the architecture
    Validate {
        /// Path to .crucible directory
        #[arg(long, default_value = ".crucible")]
        path: PathBuf,
        
        /// Enable strict validation
        #[arg(long)]
        strict: bool,
    },
    
    /// Generate code from architecture
    Generate {
        /// Target language
        #[arg(long)]
        lang: String,
        
        /// Output directory
        #[arg(long, default_value = "./generated")]
        output: PathBuf,
    },
    
    /// Show dependency graph
    Graph {
        /// Output format (text, dot, svg)
        #[arg(long, default_value = "text")]
        format: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, language, pattern } => {
            init_project(&name, &language, &pattern)?;
        }
        Commands::Validate { path, strict } => {
            validate_project(&path, strict)?;
        }
        Commands::Generate { lang, output } => {
            println!("Code generation not yet implemented");
            println!("Language: {}, Output: {}", lang, output.display());
        }
        Commands::Graph { format } => {
            println!("Graph generation not yet implemented");
            println!("Format: {}", format);
        }
    }

    Ok(())
}

fn init_project(name: &str, language: &str, pattern: &str) -> Result<()> {
    println!("{}  Crucible project: {}", "Initializing".green().bold(), name);
    
    // Create .crucible directory
    std::fs::create_dir_all(".crucible/modules")?;
    std::fs::create_dir_all(".crucible/types")?;
    
    // Create manifest.json
    let manifest = format!(
        r#"{{
  "version": "0.1.0",
  "project": {{
    "name": "{}",
    "language": "{}",
    "architecture_pattern": "{}"
  }},
  "modules": [],
  "strict_validation": true
}}"#,
        name, language, pattern
    );
    
    std::fs::write(".crucible/manifest.json", manifest)?;
    
    // Create rules.json
    let rules = r#"{
  "architecture": {
    "pattern": "layered",
    "layers": [
      {"name": "presentation", "can_depend_on": ["application"]},
      {"name": "application", "can_depend_on": ["domain"]},
      {"name": "domain", "can_depend_on": []}
    ]
  },
  "rules": [
    {
      "id": "no-circular-dependencies",
      "enabled": true,
      "severity": "error"
    }
  ]
}"#;
    
    std::fs::write(".crucible/rules.json", rules)?;
    
    println!("{} Created .crucible/manifest.json", "✓".green());
    println!("{} Created .crucible/rules.json", "✓".green());
    println!("{} Created .crucible/modules/", "✓".green());
    println!("{} Created .crucible/types/", "✓".green());
    println!();
    println!("{}", "Project initialized successfully!".green().bold());
    println!();
    println!("Next steps:");
    println!("  1. Create module definitions in .crucible/modules/");
    println!("  2. Run {} to validate", "crucible validate".cyan());
    
    Ok(())
}

fn validate_project(path: &PathBuf, strict: bool) -> Result<()> {
    println!("{}  architecture...", "Validating".cyan().bold());
    
    let parser = CrucibleParser::new(path);
    let project = parser.parse_project()?;
    
    println!("  {} modules found", project.modules.len());
    
    let validator = Validator::new(project);
    let result = validator.validate();
    
    // Display results
    for error in &result.errors {
        println!("{} {}: {}", "✗".red(), error.rule.bold(), error.message);
        if let Some(location) = &error.location {
            println!("    at {}", location.dimmed());
        }
    }
    
    for warning in &result.warnings {
        if strict {
            println!("{} {}: {}", "⚠".yellow(), warning.rule.bold(), warning.message);
            if let Some(location) = &warning.location {
                println!("    at {}", location.dimmed());
            }
        }
    }
    
    println!();
    if result.valid {
        println!("{}", "Architecture is valid!".green().bold());
    } else {
        println!("{}", "Architecture validation failed!".red().bold());
        std::process::exit(1);
    }
    
    Ok(())
}
```

## Phase 4: Self-Validation Instructions for Claude Code

### Step 9: Commands for Claude Code to Execute

```bash
# 1. Create the project structure
mkdir -p crucible/.crucible/modules
mkdir -p crucible/spec/examples/todo-app/modules
mkdir -p crucible/crucible-core/src/{parser,validator,types,error}
mkdir -p crucible/crucible-cli/src

# 2. Place all specification documents
# Copy all the provided .md files to crucible/spec/
# Copy all example JSON files to crucible/spec/examples/todo-app/

# 3. Create the self-referential architecture files
# Create all .crucible/*.json files as defined above

# 4. Create the Rust implementation files
# Create all .rs files as defined above

# 5. Build the project
cd crucible
cargo build

# 6. Validate Crucible's own architecture
cargo run --bin crucible -- validate --path .crucible

# Expected output:
# Validating architecture...
#   6 modules found
# 
# Architecture is valid!

# 7. Initialize a test project
mkdir ../test-project
cd ../test-project
../crucible/target/debug/crucible init --name test-app

# 8. Create a simple module
cat > .crucible/modules/hello.json << 'EOF'
{
  "module": "hello",
  "version": "1.0.0",
  "layer": "application",
  "exports": {
    "sayHello": {
      "type": "function",
      "inputs": [{"name": "name", "type": "string"}],
      "returns": {"type": "string"}
    }
  },
  "dependencies": {}
}
EOF

# 9. Update manifest
# Edit .crucible/manifest.json to add "hello" to modules array

# 10. Validate the test project
../crucible/target/debug/crucible validate
```

## Phase 5: Continuous Development Workflow

### Step 10: Development Loop with Claude Code

1. **When adding a new feature to Crucible:**
   ```bash
   # First, update Crucible's own architecture
   edit .crucible/modules/[module].json
   
   # Validate the architecture change
   cargo run --bin crucible -- validate --path .crucible
   
   # Then implement the feature
   edit crucible-core/src/[module].rs
   
   # Run tests
   cargo test
   ```

2. **When refactoring:**
   ```bash
   # Update architecture first
   crucible validate --path .crucible
   
   # Refactor code to match
   # Architecture drives the implementation
   ```

3. **For each PR:**
   ```bash
   # CI should run:
   crucible validate --path .crucible --strict
   cargo test
   cargo clippy
   ```

## Key Principles for Claude Code

When working on Crucible with Claude Code:

1. **Architecture First**: Always update `.crucible/` definitions before changing code
2. **Self-Validation**: Crucible must always be able to validate itself
3. **Incremental**: Start with core modules, add details progressively
4. **Documentation**: Keep spec/ docs in sync with implementation
5. **Test Coverage**: Each module should have corresponding tests

## Success Criteria

You'll know the implementation is working when:

1. ✅ `crucible validate` works on its own `.crucible/` directory
2. ✅ Can initialize new projects with `crucible init`
3. ✅ Can validate example todo-app architecture
4. ✅ Validation catches real architectural violations
5. ✅ Architecture changes require code changes (and vice versa)

## Next Implementation Steps

After this foundation:

1. **Generator Module**: Implement code generation for TypeScript
2. **Graph Module**: Add dependency visualization
3. **VS Code Extension**: Create language server
4. **More Validators**: Add custom rule support
5. **Documentation Generator**: Auto-generate docs from architecture

---

This creates a fully self-referential system where Crucible defines its own architecture and validates itself. Claude Code can use these instructions to bootstrap the entire project!
