# Crucible Specification v0.1.0

## Overview

Crucible is an open standard for defining application architecture in a structured, machine-readable format optimized for AI-assisted development. It provides a formal specification for describing modules, interfaces, dependencies, and architectural constraints that AI coding assistants can use to maintain consistency across large codebases.

## Goals

1. **AI-Native**: Designed for consumption by AI coding assistants (Claude Code, GitHub Copilot, Cursor)
2. **Validatable**: Enable static analysis and validation before code implementation
3. **Language-Agnostic**: Support multiple programming languages with consistent semantics
4. **Minimal**: Start with essential architectural elements, expand as needed
5. **Standard**: Open specification that any tool can implement

## Non-Goals (v0.1)

- Implementation details or business logic
- Performance characteristics or optimization
- Database schemas (use existing tools like Prisma, SQLAlchemy)
- Deployment configuration (use Kubernetes, Docker Compose, etc.)
- UI/UX specifications

## File Structure

```
.crucible/
├── manifest.json          # Project metadata and configuration
├── modules/              # Module definitions (one per file)
│   ├── auth.json
│   ├── database.json
│   └── api.json
├── types/                # Shared type definitions
│   └── common.json
└── rules.json           # Architectural constraints
```

## Core Concepts

### Module

A module is a logical grouping of related functionality. It exports types and functions that other modules can depend on.

**Properties:**
- `module`: Unique module identifier
- `version`: Semantic version
- `layer`: Optional architectural layer name
- `description`: Human-readable description
- `exports`: Functions, classes, and types exported by this module
- `dependencies`: Other modules this module depends on

### Export

An export represents a function, class, interface, or type that a module makes available to other modules.

**Export Types:**
- `class`: A class with methods and state
- `function`: A standalone function
- `interface`: A type definition without implementation
- `type`: A type alias or union type
- `enum`: An enumeration

### Dependency

A dependency declares that one module requires another module's exports.

**Properties:**
- `module`: The module name
- `imports`: Specific exports used from the module
- `version`: Optional version constraint

### Architectural Rule

Rules define constraints on the architecture that must be validated.

**Rule Categories:**
- Structural rules (no circular dependencies)
- Layer rules (respect architectural patterns)
- Dependency rules (declared dependencies match usage)
- Type rules (all types exist and are used correctly)

## Schema Definitions

### manifest.json

```json
{
  "version": "0.1.0",
  "project": {
    "name": "string",
    "language": "typescript | rust | python | go | java",
    "architecture_pattern": "layered | hexagonal | microservices | modular"
  },
  "modules": ["string"],
  "strict_validation": boolean,
  "metadata": {
    "author": "string",
    "repository": "string",
    "created": "ISO8601 timestamp"
  }
}
```

### Module Definition (modules/*.json)

```json
{
  "module": "string",
  "version": "semver",
  "layer": "string | null",
  "description": "string",
  
  "exports": {
    "ExportName": {
      "type": "class | function | interface | type | enum",
      
      // For classes
      "methods": {
        "methodName": {
          "inputs": [
            {"name": "string", "type": "string", "optional": boolean}
          ],
          "returns": {"type": "string", "inner": "string | null"},
          "throws": ["string"],
          "calls": ["module.Export.method"],
          "effects": ["database.read | database.write | network.request | file.read | file.write"]
        }
      },
      "properties": {
        "propertyName": {"type": "string", "required": boolean}
      },
      "dependencies": [
        {"module": "string", "imports": ["string"]}
      ],
      
      // For functions
      "inputs": [...],
      "returns": {...},
      "throws": [...],
      "calls": [...],
      "effects": [...],
      
      // For interfaces/types
      "properties": {
        "propertyName": {
          "type": "string",
          "required": boolean,
          "description": "string"
        }
      },
      
      // For enums
      "values": ["string"]
    }
  },
  
  "dependencies": {
    "moduleName": "semver constraint"
  }
}
```

### Type Definitions (types/*.json)

```json
{
  "types": {
    "TypeName": {
      "type": "interface | type | enum",
      "properties": {
        "propertyName": {
          "type": "string",
          "required": boolean,
          "description": "string"
        }
      }
    }
  }
}
```

### Architectural Rules (rules.json)

```json
{
  "architecture": {
    "pattern": "layered | hexagonal | microservices | modular",
    "layers": [
      {
        "name": "string",
        "can_depend_on": ["string"]
      }
    ]
  },
  
  "rules": [
    {
      "id": "string",
      "enabled": boolean,
      "severity": "error | warning | info",
      "description": "string",
      "config": {}
    }
  ],
  
  "custom_rules": [
    {
      "id": "string",
      "type": "no-dependency | max-dependencies | naming-convention",
      "target": "module | export | method",
      "pattern": "string",
      "severity": "error | warning | info"
    }
  ]
}
```

## Built-in Validation Rules

### Structural Rules

**no-circular-dependencies**
- Detects circular dependencies between modules
- Severity: error
- No configuration

**all-modules-exist**
- Ensures all referenced modules are defined
- Severity: error
- No configuration

**all-exports-exist**
- Ensures all referenced exports exist in their modules
- Severity: error
- No configuration

### Dependency Rules

**declared-dependencies-used**
- All declared dependencies must be used in calls
- Severity: warning
- No configuration

**used-dependencies-declared**
- All function calls must reference declared dependencies
- Severity: error
- No configuration

**version-constraints-valid**
- Semantic version constraints are valid
- Severity: error
- No configuration

### Architectural Rules

**respect-layer-boundaries**
- Modules can only depend on allowed layers
- Severity: error
- Requires: architecture.layers configuration

**no-skip-layers**
- Modules cannot skip intermediate layers
- Severity: error
- Requires: layered architecture pattern

### Type Rules

**all-types-exist**
- All referenced types must be defined
- Severity: error
- No configuration

**return-types-valid**
- Return types match declarations
- Severity: error
- No configuration

**input-types-valid**
- Input parameter types are defined
- Severity: error
- No configuration

## Type System

Crucible uses a simplified type system that maps to common programming languages:

### Primitive Types
- `string`
- `number` (maps to int, float, double depending on language)
- `boolean`
- `void`
- `null`

### Complex Types
- `Array<T>` - Array of type T
- `Map<K, V>` - Map with key type K and value type V
- `Promise<T>` - Asynchronous return of type T
- `Result<T, E>` - Result type (for Rust-like error handling)
- `Optional<T>` - Nullable/optional value

### User-Defined Types
- References to exported interfaces, types, or classes
- Format: `ModuleName.TypeName` or `TypeName` (for same module)

### Generic Types
```json
{
  "type": "interface",
  "generics": ["T", "E"],
  "properties": {
    "value": {"type": "T"},
    "error": {"type": "E"}
  }
}
```

## Effect System

Effects declare side effects that functions perform:

### Database Effects
- `database.read` - Reads from database
- `database.write` - Writes to database
- `database.transaction` - Database transaction

### Network Effects
- `network.request` - Makes HTTP/network request
- `network.listen` - Listens for network connections

### File System Effects
- `file.read` - Reads from file system
- `file.write` - Writes to file system

### State Effects
- `state.read` - Reads global/shared state
- `state.write` - Modifies global/shared state

### Session Effects
- `session.create` - Creates user session
- `session.delete` - Deletes user session
- `session.read` - Reads session data

## Call References

Function calls reference other functions in the architecture:

**Format:** `module.Export.method`

**Examples:**
- `database.UserRepository.findByEmail` - Method on a class
- `crypto.hashPassword` - Standalone function
- `auth.AuthService.login` - Method call

## Versioning

Crucible follows semantic versioning (semver):

**Module versions:**
- `1.0.0` - Exact version
- `^1.0.0` - Compatible with 1.x.x
- `~1.0.0` - Compatible with 1.0.x
- `>=1.0.0 <2.0.0` - Range

**Specification version:**
- Major: Breaking changes to schema
- Minor: Backward-compatible additions
- Patch: Clarifications and fixes

## Architectural Patterns

### Layered Architecture

```json
{
  "pattern": "layered",
  "layers": [
    {"name": "presentation", "can_depend_on": ["application"]},
    {"name": "application", "can_depend_on": ["domain", "infrastructure"]},
    {"name": "domain", "can_depend_on": []},
    {"name": "infrastructure", "can_depend_on": ["domain"]}
  ]
}
```

### Hexagonal Architecture (Ports & Adapters)

```json
{
  "pattern": "hexagonal",
  "layers": [
    {"name": "adapters", "can_depend_on": ["ports", "core"]},
    {"name": "ports", "can_depend_on": ["core"]},
    {"name": "core", "can_depend_on": []}
  ]
}
```

### Microservices

```json
{
  "pattern": "microservices",
  "layers": [
    {"name": "gateway", "can_depend_on": []},
    {"name": "service", "can_depend_on": ["shared"]},
    {"name": "shared", "can_depend_on": []}
  ]
}
```

## Tool Integration

### CLI Commands

```bash
# Validation
crucible validate                           # Run all validations
crucible validate --rule=no-cycles         # Run specific rule
crucible validate --strict                 # Fail on warnings

# Analysis
crucible graph                             # Show dependency graph
crucible graph --format=dot                # Export as GraphViz
crucible analyze                           # Show metrics
crucible trace auth.AuthService.login      # Trace function calls

# Generation
crucible generate --lang=typescript        # Generate interfaces
crucible generate --lang=rust --full       # Generate full boilerplate

# Initialization
crucible init                              # Create .crucible/
crucible init --from-code                  # Generate from existing code
```

### AI Assistant Integration

AI assistants should:

1. **Read architecture on session start**
   - Load relevant module definitions
   - Understand architectural constraints
   - Identify current layer/module context

2. **Validate changes against architecture**
   - Check new functions match declared interfaces
   - Verify dependencies are declared
   - Ensure layer boundaries respected

3. **Update architecture with code changes**
   - Add new exports to module definitions
   - Update function signatures
   - Declare new dependencies

4. **Suggest architectural improvements**
   - Identify missing abstractions
   - Recommend layer refactoring
   - Detect architectural violations

### IDE Integration

Via Language Server Protocol (LSP):
- Syntax highlighting for .json files
- Autocomplete for module/export names
- Inline validation errors
- Go-to-definition for cross-module references
- Hover documentation from descriptions

## Examples

See `examples/` directory for complete sample architectures:
- `todo-app/` - Simple CRUD application
- `e-commerce/` - Multi-module e-commerce system
- `microservices/` - Distributed microservices architecture

## Migration Path

For existing projects:

1. **Start with high-level modules**
   - Define major components
   - Declare dependencies between them

2. **Add key interfaces**
   - Public APIs
   - Critical data types

3. **Incrementally add details**
   - Function signatures
   - Effect declarations
   - Architectural rules

4. **Enforce gradually**
   - Start with warnings
   - Address violations
   - Increase strictness

## Future Considerations (Post v0.1)

- **Concurrency annotations** - Race condition detection
- **Performance constraints** - Complexity bounds, timing requirements
- **State machines** - Formal state transition definitions
- **Test coverage requirements** - Architectural-level test specifications
- **Security annotations** - Authentication, authorization requirements
- **Observability hooks** - Logging, metrics, tracing declarations

## Contributing

This specification is open for community feedback. Proposed changes should:
- Maintain backward compatibility (for minor versions)
- Include rationale and use cases
- Provide example implementations
- Consider AI assistant consumption

## License

This specification is released under [CC0 1.0 Universal](LICENSE-SPEC) (Public Domain).

You may use, modify, and implement this specification without any restrictions. No attribution is required, though it is appreciated.

**For implementation code**: When Crucible tools and libraries are released, they will be licensed under [Apache 2.0](LICENSE-CODE) to provide contributor protections while remaining permissive.

See [LICENSING.md](LICENSING.md) for complete licensing details.

## Changelog

### 0.1.0 (Initial Release)
- Core schema definitions
- Module and export system
- Dependency management
- Basic validation rules
- Architectural patterns
- Effect system
- Type system
