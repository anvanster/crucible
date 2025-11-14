# Crucible Project Structure Guide

This document shows how to organize Crucible files in your project.

## Complete Project Structure

```
your-project/
├── .crucible/                    # Crucible architecture definitions
│   ├── manifest.json                # Project configuration
│   ├── modules/                     # Module definitions
│   │   ├── auth.json
│   │   ├── todo.json
│   │   ├── user.json
│   │   ├── database.json
│   │   └── api.json
│   ├── types/                       # Shared type definitions (optional)
│   │   └── common.json
│   └── rules.json                   # Validation rules
│
├── src/                             # Your application code
│   ├── auth/
│   ├── todo/
│   ├── user/
│   ├── database/
│   └── api/
│
├── generated/                       # Generated code (optional)
│   ├── auth.ts
│   ├── todo.ts
│   └── ...
│
├── README.md
├── package.json
└── tsconfig.json
```

## File Purposes

### `.crucible/manifest.json`
**Purpose**: Project-wide configuration
**Contains**:
- Crucible version
- Project metadata (name, language, architecture pattern)
- List of all modules
- Validation settings

**Example**:
```json
{
  "version": "0.1.0",
  "project": {
    "name": "todo-app",
    "language": "typescript",
    "architecture_pattern": "layered"
  },
  "modules": ["auth", "todo", "user", "database", "api"],
  "strict_validation": true
}
```

### `.crucible/modules/*.json`
**Purpose**: Define each module's architecture
**Contains**:
- Module metadata (name, version, layer)
- Exported classes, functions, interfaces
- Dependencies on other modules
- Method signatures and behavior

**One file per module**, named after the module (e.g., `auth.json` for auth module)

**Example**: See `example-module-auth.json`, `example-module-todo.json`, `example-module-api.json`

### `.crucible/types/*.json` (Optional)
**Purpose**: Shared type definitions used across modules
**Contains**:
- Common interfaces
- Shared enums
- Utility types

**Example**:
```json
{
  "types": {
    "Result": {
      "type": "interface",
      "generics": ["T", "E"],
      "properties": {
        "ok": {"type": "boolean", "required": true},
        "value": {"type": "T", "required": false},
        "error": {"type": "E", "required": false}
      }
    },
    "Timestamp": {
      "type": "type",
      "definition": "Date"
    }
  }
}
```

### `.crucible/rules.json`
**Purpose**: Architectural constraints and validation rules
**Contains**:
- Architectural pattern definition
- Layer boundaries
- Built-in validation rules
- Custom validation rules

**Example**: See `example-rules.json`

## Module Organization Patterns

### By Feature (Recommended for Most Apps)
```
.crucible/modules/
├── auth.json          # Authentication
├── todo.json          # Todo management
├── user.json          # User management
├── notification.json  # Notifications
└── api.json           # HTTP API layer
```

### By Layer (For Strict Layered Architecture)
```
.crucible/modules/
├── presentation/
│   ├── api.json
│   └── cli.json
├── application/
│   ├── auth-service.json
│   └── todo-service.json
├── domain/
│   ├── user-domain.json
│   └── todo-domain.json
└── infrastructure/
    ├── database.json
    └── cache.json
```

### By Bounded Context (For DDD/Microservices)
```
.crucible/modules/
├── identity-context/
│   ├── auth.json
│   └── user.json
├── todo-context/
│   ├── todo.json
│   └── list.json
└── shared-kernel/
    └── common.json
```

## Example: Todo App Complete Structure

```
todo-app/
├── .crucible/
│   ├── manifest.json
│   ├── modules/
│   │   ├── api.json           # HTTP endpoints (presentation)
│   │   ├── auth.json          # Authentication (application)
│   │   ├── todo.json          # Todo business logic (application)
│   │   ├── user.json          # User management (application)
│   │   └── database.json      # Data access (infrastructure)
│   └── rules.json
│
├── src/
│   ├── api/
│   │   ├── controllers/
│   │   │   ├── TodoController.ts
│   │   │   └── AuthController.ts
│   │   ├── middleware/
│   │   │   └── auth.ts
│   │   └── routes.ts
│   │
│   ├── auth/
│   │   ├── AuthService.ts
│   │   ├── types.ts
│   │   └── errors.ts
│   │
│   ├── todo/
│   │   ├── TodoService.ts
│   │   ├── TodoRepository.ts
│   │   ├── types.ts
│   │   └── errors.ts
│   │
│   ├── user/
│   │   ├── UserRepository.ts
│   │   ├── types.ts
│   │   └── errors.ts
│   │
│   ├── database/
│   │   ├── Database.ts
│   │   ├── connection.ts
│   │   └── migrations/
│   │
│   └── generated/           # Generated from Crucible
│       ├── auth.ts
│       ├── todo.ts
│       └── user.ts
│
├── tests/
│   ├── integration/
│   └── unit/
│
├── package.json
├── tsconfig.json
└── README.md
```

## Dependency Flow Visualization

```
┌─────────────────────────────────────┐
│         Presentation Layer          │
│            (api.json)                │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│        Application Layer            │
│  (auth.json, todo.json, user.json)  │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│      Infrastructure Layer           │
│         (database.json)              │
└─────────────────────────────────────┘
```

## Best Practices

### 1. Keep Modules Cohesive
Each module should have a single, well-defined responsibility:
- ✅ Good: `auth.json` (all authentication concerns)
- ❌ Bad: `utilities.json` (random helper functions)

### 2. Minimize Cross-Layer Dependencies
```
✅ Good:
api → auth → database

❌ Bad:
api ⇄ database (skips application layer)
```

### 3. Use Semantic Versioning
Update module versions when making breaking changes:
```json
{
  "module": "auth",
  "version": "2.0.0"  // Breaking change
}
```

### 4. Document Layers Clearly
```json
{
  "module": "auth",
  "layer": "application",
  "description": "Authentication and authorization services"
}
```

### 5. Declare All Dependencies
```json
{
  "dependencies": {
    "database": "^1.0.0",
    "crypto": "^2.1.0"
  }
}
```

### 6. Keep Generated Code Separate
```
src/
├── auth/              # Your implementation
└── generated/         # Crucible-generated code
    └── auth.ts        # Don't edit manually
```

## Migration Path for Existing Projects

### Step 1: Initialize
```bash
mkdir .architecture
mkdir .crucible/modules
```

### Step 2: Start with High-Level Modules
Create manifest and define major modules:
```json
{
  "modules": ["auth", "api", "database"]
}
```

### Step 3: Add Module Skeletons
For each module, create minimal definition:
```json
{
  "module": "auth",
  "version": "1.0.0",
  "exports": {},
  "dependencies": {}
}
```

### Step 4: Fill in Key Interfaces
Add critical exports:
- Public APIs
- Main services
- Key data types

### Step 5: Add Dependencies
Map actual code dependencies to architecture:
```json
{
  "dependencies": {
    "database": "^1.0.0"
  }
}
```

### Step 6: Enable Validation
Create `rules.json` with basic rules:
```json
{
  "rules": [
    {
      "id": "no-circular-dependencies",
      "enabled": true,
      "severity": "error"
    }
  ]
}
```

### Step 7: Validate and Iterate
```bash
crucible validate
# Fix issues
# Add more details
# Validate again
```

## Integration with AI Assistants

### Claude Code
Place `.crucible/` in project root. Claude automatically:
1. Reads architecture on session start
2. Validates changes against architecture
3. Updates architecture files when needed
4. Suggests improvements

### GitHub Copilot / Cursor
Add to workspace settings:
```json
{
  "files.watcherExclude": {
    ".crucible/**": false
  }
}
```

AI will use architecture files as context for better suggestions.

## CI/CD Integration

### GitHub Actions
```yaml
name: Validate Architecture
on: [push, pull_request]
jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Crucible
        run: cargo install crucible-cli
      - name: Validate Architecture
        run: crucible validate --strict
```

### Pre-commit Hook
```bash
#!/bin/bash
# .git/hooks/pre-commit
crucible validate || exit 1
```

## Troubleshooting

### Issue: Too many files in modules/
**Solution**: Use subdirectories for organization:
```
.crucible/modules/
├── auth/
│   ├── service.json
│   └── repository.json
└── todo/
    ├── service.json
    └── repository.json
```

### Issue: Circular dependencies detected
**Solution**: 
1. Extract shared types to separate module
2. Use dependency inversion
3. Refactor module boundaries

### Issue: Architecture drift from code
**Solution**:
1. Enable CI/CD validation
2. Update architecture in PRs
3. Use generated code as contracts

## Next Steps

1. **Read full specification**: See `SPEC.md`
2. **Review examples**: Check example module files
3. **Start small**: Begin with 2-3 modules
4. **Validate often**: Run `crucible validate` regularly
5. **Iterate**: Add details as you go

## Resources

- Full specification: `SPEC.md`
- Getting started guide: `GETTING-STARTED.md`
- Example files: All `example-*.json` files
- JSON Schema: `schema.json`
- Licensing guide: `LICENSING.md`

## License

This documentation is part of the Crucible specification and is released under CC0 1.0 Universal (Public Domain). Use it freely without restrictions.

When Crucible implementation code is released, it will be under Apache 2.0. See [LICENSING.md](LICENSING.md) for details.
