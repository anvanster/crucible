# Claude Code Instructions: Implementing Crucible

## Overview
You will implement Crucible - an open standard for AI-native application architecture - using its own principles to define itself. This is a meta-implementation where Crucible defines its own architecture.

## Complete Implementation Steps

### Phase 1: Initial Setup (5 minutes)

```bash
# 1. Run the setup script to create structure
bash setup-crucible.sh

# 2. Navigate to the project
cd crucible

# 3. Verify the structure
tree -L 3 .crucible/
```

### Phase 2: Copy Specification Files (10 minutes)

Place all specification documents in `spec/`:

```bash
# Copy all these files to spec/:
- LICENSE-SPEC (CC0 1.0 Universal text)
- LICENSE-CODE (Apache 2.0 text) 
- SPEC.md
- README.md
- GETTING-STARTED.md
- PROJECT-STRUCTURE.md
- BRANDING.md
- LICENSING.md
- LICENSE-QUICK-REF.md
- INDEX.md
- REBRAND.md (optional)
- schema.json

# Copy example files to spec/examples/todo-app/:
- example-manifest.json → manifest.json
- example-module-auth.json → modules/auth.json
- example-module-todo.json → modules/todo.json
- example-module-api.json → modules/api.json
- example-rules.json → rules.json
```

### Phase 3: Implement Core Types (15 minutes)

Create `crucible-core/src/lib.rs`:
```rust
pub mod error;
pub mod parser;
pub mod types;
pub mod validator;
pub mod graph;

pub use error::{CrucibleError, Result};
pub use parser::Parser;
pub use types::{Manifest, Module, Project};
pub use validator::{ValidationResult, Validator};

pub const SPEC_VERSION: &str = "0.1.0";
```

Create `crucible-core/src/types.rs`:
```rust
// Copy the complete types.rs implementation from IMPLEMENTATION-GUIDE.md
// This includes all structs: Manifest, Module, Export, Method, etc.
```

Create `crucible-core/src/error.rs`:
```rust
// Copy the error.rs implementation from IMPLEMENTATION-GUIDE.md
// Includes CrucibleError enum with all error variants
```

Create `crucible-core/src/parser.rs`:
```rust
// Copy the parser.rs implementation from IMPLEMENTATION-GUIDE.md
// Includes Parser struct with parse_project, parse_manifest, etc.
```

Create `crucible-core/src/validator.rs`:
```rust
// Copy the validator.rs implementation from IMPLEMENTATION-GUIDE.md
// Includes Validator with all validation methods
```

Create `crucible-core/src/graph.rs`:
```rust
use crate::types::Module;
use petgraph::graph::DiGraph;
use petgraph::algo::is_cyclic_directed;
use std::collections::HashMap;

pub fn build_dependency_graph(modules: &[Module]) -> DiGraph<String, ()> {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();
    
    // Add nodes
    for module in modules {
        let node = graph.add_node(module.module.clone());
        node_map.insert(module.module.clone(), node);
    }
    
    // Add edges
    for module in modules {
        if let Some(from_node) = node_map.get(&module.module) {
            for (dep_name, _) in &module.dependencies {
                if let Some(to_node) = node_map.get(dep_name) {
                    graph.add_edge(*from_node, *to_node, ());
                }
            }
        }
    }
    
    graph
}

pub fn detect_cycles(graph: &DiGraph<String, ()>) -> bool {
    is_cyclic_directed(graph)
}
```

### Phase 4: Implement CLI (10 minutes)

Create `crucible-cli/src/main.rs`:
```rust
// Copy the complete main.rs implementation from IMPLEMENTATION-GUIDE.md
// Includes all CLI commands: init, validate, generate, graph
```

### Phase 5: Build and Test (5 minutes)

```bash
# 1. Build the project
cargo build

# 2. Validate Crucible's own architecture
cargo run --bin crucible -- validate --path .crucible

# Expected output:
# Validating architecture...
#   7 modules found
# 
# Architecture is valid!

# 3. Test on example
cargo run --bin crucible -- validate --path spec/examples/todo-app

# 4. Create a test project
mkdir ../test-project && cd ../test-project
../crucible/target/debug/crucible init --name test-app

# 5. Verify test project
ls -la .crucible/
cat .crucible/manifest.json
```

### Phase 6: Add Tests (15 minutes)

Create `crucible-core/tests/integration_test.rs`:
```rust
use crucible_core::{Parser, Validator};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_parse_valid_manifest() {
    let dir = tempdir().unwrap();
    let manifest_content = r#"{
        "version": "0.1.0",
        "project": {
            "name": "test",
            "language": "rust"
        },
        "modules": []
    }"#;
    
    fs::write(dir.path().join("manifest.json"), manifest_content).unwrap();
    
    let parser = Parser::new(dir.path());
    let manifest = parser.parse_manifest().unwrap();
    
    assert_eq!(manifest.project.name, "test");
    assert_eq!(manifest.version, "0.1.0");
}

#[test]
fn test_validate_no_circular_deps() {
    // Create a test project with no circular dependencies
    let dir = tempdir().unwrap();
    
    // Create manifest
    let manifest = r#"{
        "version": "0.1.0",
        "project": {"name": "test", "language": "rust"},
        "modules": ["a", "b"]
    }"#;
    fs::write(dir.path().join("manifest.json"), manifest).unwrap();
    
    // Create modules directory
    fs::create_dir(dir.path().join("modules")).unwrap();
    
    // Module A depends on nothing
    let module_a = r#"{
        "module": "a",
        "version": "1.0.0",
        "exports": {},
        "dependencies": {}
    }"#;
    fs::write(dir.path().join("modules/a.json"), module_a).unwrap();
    
    // Module B depends on A
    let module_b = r#"{
        "module": "b",
        "version": "1.0.0",
        "exports": {},
        "dependencies": {"a": "^1.0.0"}
    }"#;
    fs::write(dir.path().join("modules/b.json"), module_b).unwrap();
    
    // Parse and validate
    let parser = Parser::new(dir.path());
    let project = parser.parse_project().unwrap();
    let validator = Validator::new(project);
    let result = validator.validate();
    
    assert!(result.valid);
    assert!(result.errors.is_empty());
}
```

### Phase 7: Create GitHub Actions CI (5 minutes)

Create `.github/workflows/ci.yml`:
```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  validate-architecture:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Build
      run: cargo build --release
      
    - name: Validate Crucible's Architecture
      run: cargo run --bin crucible -- validate --path .crucible --strict
      
    - name: Run tests
      run: cargo test
      
    - name: Check formatting
      run: cargo fmt -- --check
      
    - name: Run clippy
      run: cargo clippy -- -D warnings
```

### Phase 8: Add Generator Stub (10 minutes)

Create `crucible-core/src/generator.rs`:
```rust
use crate::error::{CrucibleError, Result};
use crate::types::{Module, Project, Export, ExportType};
use std::fs;
use std::path::Path;

pub struct Generator {
    project: Project,
}

impl Generator {
    pub fn new(project: Project) -> Self {
        Self { project }
    }
    
    pub fn generate_typescript(&self, output_dir: &Path) -> Result<()> {
        fs::create_dir_all(output_dir).map_err(|e| {
            CrucibleError::FileRead {
                path: output_dir.display().to_string(),
                source: e,
            }
        })?;
        
        for module in &self.project.modules {
            let content = self.generate_typescript_module(module)?;
            let file_path = output_dir.join(format!("{}.ts", module.module));
            
            fs::write(&file_path, content).map_err(|e| {
                CrucibleError::FileRead {
                    path: file_path.display().to_string(),
                    source: e,
                }
            })?;
        }
        
        Ok(())
    }
    
    fn generate_typescript_module(&self, module: &Module) -> Result<String> {
        let mut output = String::new();
        
        // Header
        output.push_str(&format!(
            "// Generated from Crucible module: {}\n",
            module.module
        ));
        output.push_str(&format!("// Version: {}\n\n", module.version));
        
        // Generate exports
        for (name, export) in &module.exports {
            match export.export_type {
                ExportType::Interface => {
                    output.push_str(&format!("export interface {} {{\n", name));
                    if let Some(props) = &export.properties {
                        for (prop_name, prop) in props {
                            let optional = if prop.required { "" } else { "?" };
                            output.push_str(&format!(
                                "  {}{}: {};\n",
                                prop_name, optional, prop.prop_type
                            ));
                        }
                    }
                    output.push_str("}\n\n");
                }
                ExportType::Class => {
                    output.push_str(&format!("export class {} {{\n", name));
                    if let Some(methods) = &export.methods {
                        for (method_name, method) in methods {
                            output.push_str(&format!("  {}(", method_name));
                            
                            // Parameters
                            let params: Vec<String> = method.inputs.iter()
                                .map(|p| format!("{}: {}", p.name, p.param_type))
                                .collect();
                            output.push_str(&params.join(", "));
                            
                            // Return type
                            output.push_str(&format!("): {} {{\n", method.returns.return_type));
                            output.push_str("    throw new Error('Not implemented');\n");
                            output.push_str("  }\n\n");
                        }
                    }
                    output.push_str("}\n\n");
                }
                _ => {
                    // TODO: Implement other export types
                }
            }
        }
        
        Ok(output)
    }
}
```

Add to `crucible-core/src/lib.rs`:
```rust
pub mod generator;
pub use generator::Generator;
```

Update CLI to use generator in `crucible-cli/src/main.rs` generate command:
```rust
Commands::Generate { lang, output } => {
    let parser = CrucibleParser::new(".crucible");
    let project = parser.parse_project()?;
    
    match lang.as_str() {
        "typescript" | "ts" => {
            let gen = Generator::new(project);
            gen.generate_typescript(&output)?;
            println!("✓ Generated TypeScript interfaces in {}", output.display());
        }
        _ => {
            println!("Language '{}' not yet supported", lang);
        }
    }
}
```

### Phase 9: Final Validation (5 minutes)

```bash
# 1. Full build
cargo build --release

# 2. Run all tests
cargo test

# 3. Validate architecture
./target/release/crucible validate --path .crucible --strict

# 4. Test code generation
./target/release/crucible generate --lang typescript --output ./generated
ls -la ./generated/

# 5. Install globally (optional)
cargo install --path crucible-cli

# Now you can use 'crucible' from anywhere
crucible validate
```

## Success Checklist

- [ ] Project structure created with `.crucible/` directory
- [ ] Crucible's own architecture defined in `.crucible/`
- [ ] All specification documents placed in `spec/`
- [ ] Rust implementation compiles without errors
- [ ] `crucible validate` works on its own architecture
- [ ] Tests pass (`cargo test`)
- [ ] Can initialize new projects with `crucible init`
- [ ] Can validate the example todo-app
- [ ] Basic TypeScript generation works
- [ ] CI/CD workflow configured

## Key Files to Review

1. **Architecture Definition**: `.crucible/manifest.json`, `.crucible/modules/*.json`
2. **Core Implementation**: `crucible-core/src/types.rs`, `validator.rs`
3. **CLI**: `crucible-cli/src/main.rs`
4. **Tests**: `crucible-core/tests/integration_test.rs`
5. **Documentation**: `spec/SPEC.md`, `spec/README.md`

## Development Workflow

Always follow this pattern:

1. **Update Architecture First**
   ```bash
   # Edit .crucible/modules/[module].json
   crucible validate --path .crucible
   ```

2. **Implement Changes**
   ```bash
   # Edit src files
   cargo build
   cargo test
   ```

3. **Validate Again**
   ```bash
   crucible validate --path .crucible --strict
   ```

## Common Issues & Solutions

### Issue: "Module not found"
**Solution**: Check that module is listed in `.crucible/manifest.json`

### Issue: "Circular dependency detected"
**Solution**: Review module dependencies in `.crucible/modules/*.json`

### Issue: "Layer boundary violation"
**Solution**: Check `.crucible/rules.json` for layer definitions

### Issue: "Type not found"
**Solution**: Ensure type is exported in a module's exports section

## Next Steps After Implementation

1. **Add More Validators**: Implement custom rule validation
2. **Improve Generator**: Add Python, Rust, Go generators
3. **Create VS Code Extension**: Syntax highlighting, validation
4. **Build Graph Visualizer**: Export to GraphViz, D3.js
5. **Add AI Integration**: Claude Code, GitHub Copilot plugins

## Resources

- Specification: `spec/SPEC.md`
- Examples: `spec/examples/`
- Schema: `spec/schema.json`
- Getting Started: `spec/GETTING-STARTED.md`

---

**Remember**: This is Crucible implementing itself. The architecture defines the implementation, and the implementation validates the architecture. It's recursive validation at its finest! 🔥
