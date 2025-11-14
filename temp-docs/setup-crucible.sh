#!/bin/bash

# Crucible Project Setup Script
# This script creates the complete Crucible project structure with all files

set -e

echo "🔥 Setting up Crucible - Architecture for AI-Native Development"
echo "============================================================="

# Create main project directory
mkdir -p crucible
cd crucible

# Create directory structure
echo "📁 Creating directory structure..."
mkdir -p .crucible/modules
mkdir -p spec/examples/todo-app/modules
mkdir -p crucible-core/src/{parser,validator,types,error,graph}
mkdir -p crucible-cli/src

# Create root Cargo.toml (workspace)
echo "📝 Creating workspace configuration..."
cat > Cargo.toml << 'EOF'
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
colored = "2.0"
EOF

# Create .crucible/manifest.json (Crucible's own architecture)
echo "🏗️  Creating Crucible's self-referential architecture..."
cat > .crucible/manifest.json << 'EOF'
{
  "version": "0.1.0",
  "project": {
    "name": "crucible",
    "language": "rust",
    "architecture_pattern": "layered"
  },
  "modules": [
    "types",
    "error",
    "parser",
    "validator",
    "graph",
    "generator",
    "cli"
  ],
  "strict_validation": true,
  "metadata": {
    "author": "Crucible Contributors",
    "repository": "https://github.com/crucible-spec/crucible",
    "created": "2025-11-14T00:00:00Z"
  }
}
EOF

# Create .crucible/modules/types.json
cat > .crucible/modules/types.json << 'EOF'
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
    "Module": {
      "type": "interface",
      "properties": {
        "module": {"type": "string", "required": true},
        "version": {"type": "string", "required": true},
        "layer": {"type": "string", "required": false},
        "exports": {"type": "HashMap<string, Export>", "required": true},
        "dependencies": {"type": "HashMap<string, string>", "required": false}
      }
    },
    "Export": {
      "type": "interface",
      "properties": {
        "export_type": {"type": "ExportType", "required": true},
        "methods": {"type": "HashMap<string, Method>", "required": false},
        "properties": {"type": "HashMap<string, Property>", "required": false}
      }
    },
    "Project": {
      "type": "interface",
      "properties": {
        "manifest": {"type": "Manifest", "required": true},
        "modules": {"type": "Vec<Module>", "required": true},
        "rules": {"type": "Rules", "required": false}
      }
    }
  },
  
  "dependencies": {}
}
EOF

# Create .crucible/modules/parser.json
cat > .crucible/modules/parser.json << 'EOF'
{
  "module": "parser",
  "version": "0.1.0",
  "layer": "core",
  "description": "Parses Crucible JSON files into internal representations",
  
  "exports": {
    "Parser": {
      "type": "class",
      "methods": {
        "parse_project": {
          "inputs": [],
          "returns": {
            "type": "Result",
            "inner": "types.Project, error.CrucibleError"
          },
          "calls": [
            "parser.parse_manifest",
            "parser.parse_modules",
            "parser.parse_rules"
          ],
          "effects": ["file.read"]
        },
        "parse_manifest": {
          "inputs": [],
          "returns": {
            "type": "Result",
            "inner": "types.Manifest, error.CrucibleError"
          },
          "calls": [],
          "effects": ["file.read"]
        },
        "parse_module": {
          "inputs": [
            {"name": "name", "type": "string"}
          ],
          "returns": {
            "type": "Result",
            "inner": "types.Module, error.CrucibleError"
          },
          "calls": [],
          "effects": ["file.read"]
        }
      },
      "dependencies": [
        {"module": "types", "imports": ["Manifest", "Module", "Project"]},
        {"module": "error", "imports": ["CrucibleError"]}
      ]
    }
  },
  
  "dependencies": {
    "types": "^0.1.0",
    "error": "^0.1.0"
  }
}
EOF

# Create .crucible/modules/validator.json
cat > .crucible/modules/validator.json << 'EOF'
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
          "inputs": [],
          "returns": {
            "type": "ValidationResult"
          },
          "calls": [
            "validator.check_circular_dependencies",
            "validator.check_layer_boundaries",
            "validator.check_type_existence"
          ],
          "effects": []
        },
        "check_circular_dependencies": {
          "inputs": [],
          "returns": {
            "type": "Option<Vec<ValidationIssue>>"
          },
          "calls": [
            "graph.build_dependency_graph",
            "graph.detect_cycles"
          ],
          "effects": []
        }
      },
      "dependencies": [
        {"module": "types", "imports": ["Project", "Module", "Rules"]},
        {"module": "error", "imports": ["ValidationError"]},
        {"module": "graph", "imports": ["build_dependency_graph"]}
      ]
    },
    "ValidationResult": {
      "type": "interface",
      "properties": {
        "valid": {"type": "boolean", "required": true},
        "errors": {"type": "Vec<ValidationIssue>", "required": true},
        "warnings": {"type": "Vec<ValidationIssue>", "required": true}
      }
    }
  },
  
  "dependencies": {
    "types": "^0.1.0",
    "error": "^0.1.0",
    "graph": "^0.1.0"
  }
}
EOF

# Create .crucible/modules/error.json
cat > .crucible/modules/error.json << 'EOF'
{
  "module": "error",
  "version": "0.1.0",
  "layer": "core",
  "description": "Error types for Crucible",
  
  "exports": {
    "CrucibleError": {
      "type": "enum",
      "values": [
        "FileRead",
        "ParseError",
        "ModuleNotFound",
        "ExportNotFound",
        "CircularDependency",
        "LayerViolation",
        "TypeNotFound",
        "ValidationFailed"
      ]
    },
    "Result": {
      "type": "type",
      "definition": "std::result::Result<T, CrucibleError>"
    }
  },
  
  "dependencies": {}
}
EOF

# Create .crucible/modules/graph.json
cat > .crucible/modules/graph.json << 'EOF'
{
  "module": "graph",
  "version": "0.1.0",
  "layer": "core",
  "description": "Dependency graph analysis",
  
  "exports": {
    "build_dependency_graph": {
      "type": "function",
      "inputs": [
        {"name": "modules", "type": "Vec<types.Module>"}
      ],
      "returns": {
        "type": "DiGraph"
      },
      "effects": []
    },
    "detect_cycles": {
      "type": "function",
      "inputs": [
        {"name": "graph", "type": "DiGraph"}
      ],
      "returns": {
        "type": "boolean"
      },
      "effects": []
    }
  },
  
  "dependencies": {
    "types": "^0.1.0"
  }
}
EOF

# Create .crucible/modules/generator.json
cat > .crucible/modules/generator.json << 'EOF'
{
  "module": "generator",
  "version": "0.1.0",
  "layer": "application",
  "description": "Code generation from Crucible definitions",
  
  "exports": {
    "Generator": {
      "type": "class",
      "methods": {
        "generate": {
          "inputs": [
            {"name": "project", "type": "types.Project"},
            {"name": "language", "type": "Language"},
            {"name": "output_dir", "type": "string"}
          ],
          "returns": {
            "type": "Result",
            "inner": "void, error.CrucibleError"
          },
          "calls": [
            "generator.generate_typescript",
            "generator.generate_rust"
          ],
          "effects": ["file.write"]
        }
      },
      "dependencies": [
        {"module": "types", "imports": ["Project", "Module"]},
        {"module": "error", "imports": ["CrucibleError"]}
      ]
    },
    "Language": {
      "type": "enum",
      "values": ["TypeScript", "Rust", "Python", "Go", "Java"]
    }
  },
  
  "dependencies": {
    "types": "^0.1.0",
    "error": "^0.1.0"
  }
}
EOF

# Create .crucible/modules/cli.json
cat > .crucible/modules/cli.json << 'EOF'
{
  "module": "cli",
  "version": "0.1.0",
  "layer": "presentation",
  "description": "Command-line interface for Crucible",
  
  "exports": {
    "run": {
      "type": "function",
      "inputs": [],
      "returns": {
        "type": "Result",
        "inner": "void, error.CrucibleError"
      },
      "calls": [
        "parser.Parser.parse_project",
        "validator.Validator.validate",
        "generator.Generator.generate"
      ],
      "effects": ["file.read", "file.write"]
    }
  },
  
  "dependencies": {
    "parser": "^0.1.0",
    "validator": "^0.1.0",
    "generator": "^0.1.0",
    "types": "^0.1.0",
    "error": "^0.1.0"
  }
}
EOF

# Create .crucible/rules.json
cat > .crucible/rules.json << 'EOF'
{
  "architecture": {
    "pattern": "layered",
    "layers": [
      {
        "name": "presentation",
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
    }
  ]
}
EOF

# Create crucible-core/Cargo.toml
echo "📦 Setting up crucible-core package..."
cat > crucible-core/Cargo.toml << 'EOF'
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
EOF

# Create crucible-cli/Cargo.toml
echo "📦 Setting up crucible-cli package..."
cat > crucible-cli/Cargo.toml << 'EOF'
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
colored.workspace = true
EOF

# Create a basic README
echo "📚 Creating README..."
cat > README.md << 'EOF'
# Crucible

**An open standard for AI-native application architecture**

## Quick Start

```bash
# Build the project
cargo build

# Validate Crucible's own architecture
cargo run --bin crucible -- validate --path .crucible

# Create a new project
cargo run --bin crucible -- init --name my-app
```

## Project Structure

- `.crucible/` - Crucible's own architecture definition
- `crucible-core/` - Core library implementation
- `crucible-cli/` - Command-line interface
- `spec/` - Specification documents

## Development

This project uses Crucible to define its own architecture. Before making changes:

1. Update `.crucible/` definitions
2. Validate: `cargo run --bin crucible -- validate --path .crucible`
3. Implement changes
4. Run tests: `cargo test`

## License

- Specification: CC0 1.0 Universal (Public Domain)
- Implementation: Apache License 2.0
EOF

# Create .gitignore
echo "🔧 Creating .gitignore..."
cat > .gitignore << 'EOF'
# Rust
target/
Cargo.lock
**/*.rs.bk

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Test outputs
test-output/
generated/
EOF

echo ""
echo "✅ Crucible project structure created successfully!"
echo ""
echo "📋 Next steps:"
echo "   1. Copy Rust implementation files from IMPLEMENTATION-GUIDE.md"
echo "   2. Copy specification documents to spec/ directory"
echo "   3. Build: cargo build"
echo "   4. Validate: cargo run --bin crucible -- validate --path .crucible"
echo ""
echo "🔥 Crucible is ready to forge architecture that withstands AI at scale!"
EOF
