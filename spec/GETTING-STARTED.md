# Getting Started with Crucible

## What is Crucible?

Crucible (Crucible) is an open standard for defining application architecture in a structured, machine-readable format. It's designed to help AI coding assistants (like Claude Code, GitHub Copilot, Cursor) maintain consistency across your codebase.

## Why Use Crucible?

- **Prevent AI inconsistencies** - AI assistants can read your architecture and maintain consistency across files
- **Catch errors early** - Validate architecture before writing code
- **Document intentionally** - Architecture is explicit, not implicit
- **Scale AI development** - Maintain quality as AI generates more code

## Quick Start

### 1. Install Crucible CLI (Coming Soon)

```bash
cargo install crucible-cli
# or
npm install -g crucible-cli
```

### 2. Initialize a New Project

```bash
cd your-project
crucible init
```

This creates `.crucible/` directory:
```
.crucible/
├── manifest.json
├── modules/
├── types/
└── rules.json
```

### 3. Define Your First Module

Create `.crucible/modules/auth.json`:

```json
{
  "module": "auth",
  "version": "1.0.0",
  "layer": "application",
  "description": "Authentication services",
  
  "exports": {
    "AuthService": {
      "type": "class",
      "methods": {
        "login": {
          "inputs": [
            {"name": "email", "type": "string"},
            {"name": "password", "type": "string"}
          ],
          "returns": {"type": "Promise", "inner": "User"},
          "throws": ["InvalidCredentialsError"],
          "calls": ["database.UserRepository.findByEmail"],
          "effects": ["database.read"]
        }
      }
    }
  },
  
  "dependencies": {
    "database": "^1.0.0"
  }
}
```

### 4. Validate Your Architecture

```bash
crucible validate
```

Output:
```
✓ No circular dependencies
✓ All types exist
✓ All calls reference exported functions
✓ Layer boundaries respected

Architecture is valid!
```

### 5. Generate Code Interfaces

```bash
crucible generate --lang=typescript
```

Generates:
```typescript
// generated/auth.ts
export interface User {
  id: string;
  email: string;
}

export interface AuthService {
  login(email: string, password: string): Promise<User>;
}
```

## Core Concepts

### Modules

Modules are logical groupings of related functionality:

```json
{
  "module": "auth",
  "version": "1.0.0",
  "layer": "application",
  "exports": { /* ... */ },
  "dependencies": { /* ... */ }
}
```

### Exports

Exports define what a module provides:

**Classes:**
```json
{
  "AuthService": {
    "type": "class",
    "methods": { /* ... */ }
  }
}
```

**Interfaces:**
```json
{
  "User": {
    "type": "interface",
    "properties": {
      "id": {"type": "string", "required": true},
      "email": {"type": "string", "required": true}
    }
  }
}
```

**Functions:**
```json
{
  "hashPassword": {
    "type": "function",
    "inputs": [{"name": "password", "type": "string"}],
    "returns": {"type": "string"}
  }
}
```

### Dependencies

Declare which modules you depend on:

```json
{
  "dependencies": {
    "database": "^1.0.0",
    "crypto": "^2.1.0"
  }
}
```

In methods, reference their exports:

```json
{
  "calls": [
    "database.UserRepository.findByEmail",
    "crypto.hashPassword"
  ]
}
```

### Effects

Declare side effects your functions perform:

```json
{
  "effects": [
    "database.read",
    "database.write",
    "network.request",
    "file.write"
  ]
}
```

## Validation Rules

### Built-in Rules

Crucible validates:

1. **No circular dependencies** - Modules can't depend on each other in a cycle
2. **All calls exist** - Function calls must reference exported functions
3. **All types exist** - Referenced types must be defined
4. **Layer boundaries** - Modules respect architectural patterns
5. **Dependency declarations** - Used modules must be declared

### Custom Rules

Define your own rules in `rules.json`:

```json
{
  "custom_rules": [
    {
      "id": "repository-naming",
      "type": "naming-convention",
      "pattern": "^[A-Z][a-zA-Z]*Repository$",
      "severity": "warning"
    }
  ]
}
```

## Architectural Patterns

### Layered Architecture

```json
{
  "architecture": {
    "pattern": "layered",
    "layers": [
      {"name": "presentation", "can_depend_on": ["application"]},
      {"name": "application", "can_depend_on": ["domain"]},
      {"name": "domain", "can_depend_on": []}
    ]
  }
}
```

### Hexagonal Architecture

```json
{
  "architecture": {
    "pattern": "hexagonal",
    "layers": [
      {"name": "adapters", "can_depend_on": ["ports", "core"]},
      {"name": "ports", "can_depend_on": ["core"]},
      {"name": "core", "can_depend_on": []}
    ]
  }
}
```

## AI Assistant Integration

### With Claude Code

1. Place `.crucible/` in your project root
2. Claude automatically reads it when starting sessions
3. Claude validates changes against architecture
4. Claude updates architecture when adding features

Example session:
```
You: Add a register method to AuthService

Claude: I'll add the register method to the auth module.
First, let me check the architecture...

[Reads .crucible/modules/auth.json]

I'll add:
- Register method to AuthService
- Update dependencies if needed
- Validate against architectural rules

[Implements code]
[Updates .crucible/modules/auth.json]
[Runs crucible validate]

✓ Architecture updated and validated
```

### With GitHub Copilot / Cursor

Use Crucible as context for better completions:
- Place architecture files in workspace
- AI reads them for context
- Suggestions respect declared interfaces

## Common Workflows

### Starting a New Feature

```bash
# 1. Define architecture first
edit .crucible/modules/feature.json

# 2. Validate architecture
crucible validate

# 3. Generate interfaces
crucible generate --lang=typescript

# 4. Implement with AI assistance
# AI uses architecture as guide

# 5. Validate implementation matches architecture
crucible validate --check-implementation
```

### Refactoring

```bash
# 1. Update architecture
edit .crucible/modules/auth.json

# 2. See what changed
crucible diff

# 3. Generate updated interfaces
crucible generate --lang=typescript --update

# 4. Update implementation
# AI helps migrate to new architecture

# 5. Validate
crucible validate
```

### Code Review

```bash
# Check architectural changes
crucible diff main..feature-branch

# Validate PR doesn't violate rules
crucible validate --strict

# Generate architecture diagram
crucible graph --format=svg > architecture.svg
```

## Best Practices

### Start Small

Begin with high-level modules:
```
auth
user
database
api
```

Add details incrementally:
- Public APIs first
- Internal functions later
- Full details as needed

### Keep It Current

Update architecture when:
- Adding new modules
- Changing function signatures
- Modifying dependencies
- Refactoring structure

### Use Strict Mode

Enable strict validation in production:
```json
{
  "strict_validation": true
}
```

### Layer Your Application

Choose an architectural pattern:
- **Layered** - Traditional n-tier
- **Hexagonal** - Ports and adapters
- **Microservices** - Distributed services
- **Modular** - Independent modules

### Document Effects

Always declare side effects:
```json
{
  "effects": [
    "database.write",
    "network.request"
  ]
}
```

This helps understand system behavior.

## Examples

See `examples/` directory:

- **todo-app** - Simple CRUD application
- **e-commerce** - Multi-module system
- **microservices** - Distributed architecture

Each example includes:
- Complete `.crucible/` definition
- Generated code
- Validation results

## Troubleshooting

### "Circular dependency detected"

Break the cycle by:
1. Moving shared types to separate module
2. Using dependency inversion
3. Refactoring module boundaries

### "Call to undefined function"

Ensure:
1. Target module exports the function
2. Module is declared in dependencies
3. Call reference format is correct: `module.Export.method`

### "Layer boundary violation"

Check:
1. Module's layer declaration
2. Allowed dependencies for that layer
3. Whether you need to refactor

## Next Steps

1. **Read the full spec** - See SPEC.md for complete details
2. **Try examples** - Run through todo-app example
3. **Start using it** - Initialize Crucible in your project
4. **Join community** - Share feedback and contribute

## Resources

- Specification: `SPEC.md`
- Examples: `examples/`
- Schema: `schema.json`
- Repository: https://github.com/crucible-spec (coming soon)

## Contributing

Crucible is an open standard. Contributions welcome:
- Propose new features
- Report issues
- Improve documentation
- Build tooling

## License

Crucible uses dual licensing:
- **Specification**: CC0 1.0 Universal (Public Domain) - Use freely, no restrictions
- **Implementation code** (when released): Apache 2.0 - Permissive with patent grant

See [LICENSING.md](LICENSING.md) for complete details.
