# Crucible CLI Reference

Complete command-line interface reference for the Crucible framework.

## Table of Contents

- [Installation](#installation)
- [Global Options](#global-options)
- [Commands](#commands)
  - [init](#init)
  - [validate](#validate)
  - [generate](#generate)
  - [graph](#graph)
  - [claude](#claude)
- [Common Workflows](#common-workflows)
- [Exit Codes](#exit-codes)
- [Examples](#examples)

---

## Installation

### From crates.io

```bash
cargo install crucible-cli
```

### Using cargo-binstall

```bash
cargo binstall crucible-cli
```

### From source

```bash
git clone https://github.com/anvanster/crucible.git
cd crucible
cargo build --release
./target/release/crucible --version
```

### Verify Installation

```bash
crucible --version
# Output: crucible 0.1.6
```

---

## Global Options

```bash
crucible [OPTIONS] <COMMAND>
```

**Options:**
- `-h, --help` - Print help information
- `-V, --version` - Print version information

---

## Commands

### `init`

Initialize a new Crucible project with architecture definitions.

#### Syntax

```bash
crucible init [OPTIONS]
```

#### Options

| Option | Description | Default | Required |
|--------|-------------|---------|----------|
| `--name <NAME>` | Project name | - | Yes (unless `--here`) |
| `--here` | Initialize in current directory | false | No |
| `--force` | Force overwrite if `.crucible/` exists | false | No |
| `--language <LANG>` | Programming language | `typescript` | No |
| `--pattern <PATTERN>` | Architecture pattern | `layered` | No |

#### Languages

- `typescript` - TypeScript/JavaScript projects
- `rust` - Rust projects
- `python` - Python projects
- `go` - Go projects
- `java` - Java projects

#### Patterns

- `layered` - Classic layered architecture (domain → infrastructure → application → presentation)
- `hexagonal` - Ports and adapters pattern
- `microservices` - Microservices architecture
- `modular` - Modular monolith

#### Examples

**Create new project:**
```bash
crucible init --name my-app --language typescript --pattern layered
```

**Initialize in existing project:**
```bash
cd my-existing-project
crucible init --here --force
```

**Initialize with specific pattern:**
```bash
crucible init --name my-service --language rust --pattern hexagonal
```

#### Generated Structure

```
.crucible/
├── manifest.json      # Project manifest
├── rules.json        # Architecture rules
└── modules/          # Module definitions (empty initially)
```

**Also generates Claude Code slash commands:**
```
.claude/commands/
├── crucible-validate.md
├── crucible-architecture.md
├── crucible-module.md
└── ... (8 total commands)
```

#### Interactive Prompts

When using `--force` with existing `.crucible/`:
```
⚠️  Warning: .crucible/ directory already exists
This will DELETE all existing architecture definitions.

Continue? (y/N):
```

#### Errors

- **Missing `--name`**: "error: the following required arguments were not provided: --name"
  - **Fix**: Provide `--name` or use `--here` flag

- **Directory exists**: "Error: .crucible/ directory already exists"
  - **Fix**: Use `--force` to overwrite (with confirmation)

---

### `validate`

Validate architecture definitions against rules and check for violations.

#### Syntax

```bash
crucible validate [OPTIONS]
```

#### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--path <PATH>` | Path to `.crucible` directory | `.crucible` |
| `--strict` | Enable strict validation | false |

#### Validation Checks

1. **Schema Validation**
   - Module JSON structure correctness
   - Required fields present
   - Valid export types

2. **Type Validation**
   - Referenced types exist
   - Cross-module type references valid
   - Built-in types recognized

3. **Dependency Validation**
   - Declared dependencies used
   - Used dependencies declared
   - No circular dependencies

4. **Layer Validation**
   - Layer boundaries respected
   - Dependencies follow layer rules
   - No upward dependencies

5. **Method Call Validation**
   - Called methods exist
   - Method signatures match
   - Cross-module calls valid

#### Output

**Success:**
```
Validating architecture...
  34 modules found
Architecture is valid!
```

**With Warnings:**
```
Validating architecture...
  34 modules found
⚠ declared-dependencies-must-be-used: Dependency 'user' is declared but not used
    at user-service
Architecture is valid!
```

**With Errors:**
```
Validating architecture...
  34 modules found
❌ Layer boundary violation: domain module 'user' cannot depend on application module 'user-service'
    at user (line 15)
❌ Type not found: 'UnknownType' referenced in user-service
    at user-service.UserService.create (line 23)

Validation failed with 2 errors.
```

#### Examples

**Validate current project:**
```bash
crucible validate
```

**Validate specific directory:**
```bash
crucible validate --path ./my-service/.crucible
```

**Strict validation:**
```bash
crucible validate --strict
```

#### Exit Codes

- `0` - Validation successful
- `1` - Validation failed with errors
- `2` - Invalid arguments or file not found

---

### `generate`

Generate code scaffolding from architecture definitions.

#### Syntax

```bash
crucible generate --lang <LANG> [OPTIONS]
```

#### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--path <PATH>` | Path to `.crucible` directory | `.crucible` |
| `--lang <LANG>` | Target language | - (required) |
| `--output <DIR>` | Output directory | `./generated` |

#### Languages

- `typescript` - Generate TypeScript interfaces and classes
- `rust` - Generate Rust structs and traits
- `python` - Generate Python classes and type hints
- `go` - Generate Go structs and interfaces

#### Generated Code

**TypeScript:**
```typescript
// Generated from user.json
export interface User {
  id: string;
  email: string;
  created: Date;
}

export class UserService {
  async createUser(data: CreateUserDTO): Promise<User> {
    // TODO: Implement createUser
    throw new Error('Not implemented');
  }
}
```

**Rust:**
```rust
// Generated from user.json
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub created: DateTime<Utc>,
}

pub struct UserService;

impl UserService {
    pub async fn create_user(&self, data: CreateUserDTO) -> Result<User, Error> {
        // TODO: Implement create_user
        unimplemented!()
    }
}
```

#### Examples

**Generate TypeScript:**
```bash
crucible generate --lang typescript --output ./src/generated
```

**Generate Rust:**
```bash
crucible generate --lang rust --output ./src/generated
```

**Generate from specific path:**
```bash
crucible generate --path ./services/user/.crucible --lang typescript
```

#### Errors

- **Missing language**: "error: the following required arguments were not provided: --lang"
  - **Fix**: Specify `--lang typescript` or other supported language

---

### `graph`

Display module dependency graph (ASCII visualization).

#### Syntax

```bash
crucible graph [OPTIONS]
```

#### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--path <PATH>` | Path to `.crucible` directory | `.crucible` |

#### Output

```
Module Dependency Graph:

user-service
  ↓
  ├─→ user (domain)
  ├─→ user-repository (infrastructure)
  └─→ validation-service (application)

user-repository
  ↓
  └─→ user (domain)
```

#### Examples

**Show dependency graph:**
```bash
crucible graph
```

**Specific project:**
```bash
crucible graph --path ./my-service/.crucible
```

---

### `claude`

Claude Code integration commands (automatically available in Claude Code).

#### Syntax

```bash
crucible claude <SUBCOMMAND>
```

#### Subcommands

These commands are primarily used by Claude Code slash commands:

- `crucible claude validate` - Validate with Claude-friendly output
- `crucible claude architecture` - Architecture design workflow
- `crucible claude module` - Module creation/update
- `crucible claude review` - Architecture review
- `crucible claude sync` - Sync architecture ↔ code
- `crucible claude analyze` - Dependency analysis
- `crucible claude diff` - Show architecture vs code differences

**Note:** These are typically invoked via Claude Code slash commands:
- `/crucible:validate`
- `/crucible:architecture`
- `/crucible:module`
- etc.

See `.claude/commands/` for full slash command documentation.

---

## Common Workflows

### New Project Setup

```bash
# 1. Initialize project
crucible init --name my-app --language typescript

# 2. Create your first module
cat > .crucible/modules/user.json <<EOF
{
  "module": "user",
  "version": "1.0.0",
  "layer": "domain",
  "exports": {
    "User": {
      "type": "interface",
      "properties": {
        "id": {"type": "string"},
        "email": {"type": "string"}
      }
    }
  },
  "dependencies": {}
}
EOF

# 3. Update manifest
# Add "user" to modules array in .crucible/manifest.json

# 4. Validate
crucible validate

# 5. Generate code
crucible generate --lang typescript
```

### Adding New Module

```bash
# 1. Create module definition
vim .crucible/modules/user-service.json

# 2. Update manifest.json
# Add "user-service" to modules array

# 3. Validate
crucible validate

# 4. Generate code
crucible generate --lang typescript --output ./src
```

### Architecture Review

```bash
# 1. Validate current state
crucible validate --strict

# 2. Show dependency graph
crucible graph

# 3. Review layer boundaries
# Check rules.json and module layers

# 4. Fix any violations
# Update module definitions as needed

# 5. Re-validate
crucible validate
```

### Migration to Crucible

```bash
# 1. Initialize in existing project
cd existing-project
crucible init --here

# 2. Create module definitions
# Manually create .json files or use extraction tools

# 3. Validate incrementally
crucible validate

# 4. Fix errors
# See docs/common-mistakes.md

# 5. Set up pre-commit hook
# Prevent invalid architecture changes
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Validation failed or command error |
| `2` | Invalid arguments or missing files |
| `101` | Panic or unexpected error |

---

## Examples

### Example 1: Quick Start

```bash
# Create new project
crucible init --name todo-app --language typescript

# Create domain entity
cat > .crucible/modules/task.json <<'EOF'
{
  "module": "task",
  "version": "1.0.0",
  "layer": "domain",
  "exports": {
    "Task": {
      "type": "interface",
      "properties": {
        "id": {"type": "string"},
        "title": {"type": "string"},
        "completed": {"type": "boolean"}
      }
    }
  },
  "dependencies": {}
}
EOF

# Update manifest
# Edit .crucible/manifest.json, add "task" to modules array

# Validate
crucible validate

# Generate TypeScript
crucible generate --lang typescript --output ./src/generated
```

### Example 2: Multi-Layer Architecture

```bash
# Domain layer
cat > .crucible/modules/user.json <<'EOF'
{
  "module": "user",
  "version": "1.0.0",
  "layer": "domain",
  "exports": {
    "User": {
      "type": "interface",
      "properties": {
        "id": {"type": "string"},
        "email": {"type": "string"}
      }
    }
  },
  "dependencies": {}
}
EOF

# Infrastructure layer
cat > .crucible/modules/user-repository.json <<'EOF'
{
  "module": "user-repository",
  "version": "1.0.0",
  "layer": "infrastructure",
  "exports": {
    "UserRepository": {
      "type": "class",
      "methods": {
        "save": {
          "inputs": [{"name": "user", "type": "User"}],
          "returns": {"type": "Promise<void>"}
        }
      }
    }
  },
  "dependencies": {
    "user": "User"
  }
}
EOF

# Application layer
cat > .crucible/modules/user-service.json <<'EOF'
{
  "module": "user-service",
  "version": "1.0.0",
  "layer": "application",
  "exports": {
    "UserService": {
      "type": "class",
      "methods": {
        "createUser": {
          "inputs": [{"name": "email", "type": "string"}],
          "returns": {"type": "Promise<User>"}
        }
      }
    }
  },
  "dependencies": {
    "user": "User",
    "user-repository": "UserRepository"
  }
}
EOF

# Update manifest.json with all three modules
# Then validate
crucible validate --strict
```

### Example 3: CI/CD Integration

```bash
# .github/workflows/validate.yml
name: Validate Architecture

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install Crucible
        run: cargo install crucible-cli
      - name: Validate Architecture
        run: crucible validate --strict
```

### Example 4: Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "🔍 Validating architecture..."

crucible validate --strict

if [ $? -ne 0 ]; then
  echo "❌ Architecture validation failed!"
  echo "💡 Fix errors before committing or use 'git commit --no-verify' to skip"
  exit 1
fi

echo "✅ Architecture validation passed"
exit 0
```

Make executable:
```bash
chmod +x .git/hooks/pre-commit
```

---

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `CRUCIBLE_PATH` | Default path to .crucible directory | `.crucible` |
| `CRUCIBLE_STRICT` | Enable strict validation | `false` |

Example:
```bash
export CRUCIBLE_PATH="./services/user/.crucible"
export CRUCIBLE_STRICT=true
crucible validate
```

---

## Configuration Files

### `.crucible/manifest.json`

Project manifest with module list:
```json
{
  "version": "0.1.0",
  "project": {
    "name": "my-app",
    "language": "typescript",
    "architecture_pattern": "layered"
  },
  "modules": ["user", "user-service"],
  "strict_validation": true
}
```

### `.crucible/rules.json`

Architecture rules and layer definitions:
```json
{
  "architecture": {
    "pattern": "layered",
    "layers": [
      {"name": "domain", "can_depend_on": ["domain"]},
      {"name": "application", "can_depend_on": ["domain", "infrastructure"]}
    ]
  },
  "rules": [
    {"id": "no-circular-dependencies", "enabled": true, "severity": "error"}
  ]
}
```

---

## Troubleshooting

### "Module not found" Error

```
Error: Module 'user' not found in manifest
```

**Fix:** Add module name to `modules` array in `manifest.json`

### Validation Fails but No Error Shown

Try strict mode:
```bash
crucible validate --strict
```

### "Permission denied" on Init

**Fix:** Check directory write permissions:
```bash
ls -la .
# Ensure current directory is writable
```

---

## See Also

- [Schema Reference](./schema-reference.md) - Module JSON format
- [Common Mistakes](./common-mistakes.md) - Error fixes
- [Type System](./type-system.md) - Type syntax
- [Example Project](./examples/full-stack-app/) - Real-world example
- [Claude Code Integration](../.claude/commands/) - Slash commands

---

## Getting Help

- **Documentation**: https://github.com/anvanster/crucible/blob/main/README.md
- **Issues**: https://github.com/anvanster/crucible/issues
- **Discussions**: https://github.com/anvanster/crucible/discussions
- **Crates.io**: https://crates.io/crates/crucible-cli
