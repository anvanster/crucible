# Crucible JSON Schema Reference

Complete reference for Crucible's JSON schema format with TypeScript-style type definitions.

## Table of Contents

- [Module Definition](#module-definition)
- [Manifest](#manifest)
- [Rules](#rules)
- [Export Types](#export-types)
- [Type System](#type-system)
- [Dependencies](#dependencies)
- [Quick Reference](#quick-reference)

---

## Module Definition

Each module is defined in a separate JSON file in `.crucible/modules/`.

### Visual Structure Overview

```
┌─────────────────────────────────────────────────────────────────┐
│ ModuleDefinition (.crucible/modules/my-module.json)            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  module: string (REQUIRED)          ─┐                         │
│  version: string (REQUIRED)          │ Basic Info              │
│  layer: string (REQUIRED)            │                         │
│  description?: string                ─┘                        │
│                                                                 │
│  exports: { [name: string]: Export } (REQUIRED)                │
│    │                                                            │
│    ├─► class Export                                            │
│    │     ├─ type: "class"                                      │
│    │     └─ methods: { [name: string]: Method }               │
│    │          ├─ inputs: Parameter[]                           │
│    │          ├─ returns: ReturnType                           │
│    │          ├─ throws?: string[]                             │
│    │          ├─ calls?: string[]                              │
│    │          └─ effects?: string[]                            │
│    │                                                            │
│    ├─► function Export                                         │
│    │     ├─ type: "function"                                   │
│    │     ├─ inputs: Parameter[]                                │
│    │     └─ returns: ReturnType                                │
│    │                                                            │
│    ├─► interface Export                                        │
│    │     ├─ type: "interface"                                  │
│    │     └─ properties: { [name: string]: Property }          │
│    │          ├─ type: string                                  │
│    │          ├─ required?: boolean                            │
│    │          └─ description?: string                          │
│    │                                                            │
│    ├─► type Export (alias)                                     │
│    │     ├─ type: "type"                                       │
│    │     └─ properties: { [name: string]: Property }          │
│    │                                                            │
│    ├─► enum Export                                             │
│    │     ├─ type: "enum"                                       │
│    │     └─ values: string[]                                   │
│    │                                                            │
│    ├─► event Export (domain events)                            │
│    │     ├─ type: "event"                                      │
│    │     └─ payload: { [name: string]: Property }              │
│    │          ├─ type: string                                  │
│    │          ├─ required?: boolean                            │
│    │          └─ description?: string                          │
│    │                                                            │
│    └─► trait Export (Rust-style traits)                        │
│          ├─ type: "trait"                                      │
│          └─ methods: { [name: string]: Method }               │
│               ├─ inputs: Parameter[]                           │
│               ├─ returns: ReturnType                           │
│               └─ async?: boolean                               │
│                                                                 │
│  dependencies?: { [module: string]: string }                   │
│    ├─ "user": "User"                    (single export)        │
│    └─ "user-service": "User,UserDTO"    (multiple exports)     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

Key:
  REQUIRED fields must be present
  ? optional fields can be omitted
  [name: string] means a map/object with string keys
  string[] means an array of strings
```

### ModuleDefinition

```typescript
interface ModuleDefinition {
  // Required: Module identifier (kebab-case recommended)
  module: string;

  // Required: Semantic version
  version: string;

  // Required: Architecture layer this module belongs to
  // Must match a layer defined in rules.json
  layer: string;

  // Optional: Human-readable description
  description?: string;

  // Required: Exported types, interfaces, classes, functions
  // Key = export name, Value = export definition
  exports: { [exportName: string]: Export };

  // Optional: Dependencies on other modules
  // Key = module name, Value = comma-separated export names
  // Example: {"user-service": "User,CreateUserDTO"}
  dependencies?: { [moduleName: string]: string };
}
```

### Export

```typescript
interface Export {
  // Required: Type of export
  type: 'class' | 'function' | 'interface' | 'type' | 'enum' | 'event' | 'trait';

  // For classes, functions, and traits: method definitions
  methods?: { [methodName: string]: Method };

  // For interfaces and types: property definitions
  properties?: { [propertyName: string]: Property };

  // For enums: allowed values
  values?: string[];

  // For events: payload data carried by the event
  payload?: { [fieldName: string]: Property };

  // Optional: Module dependencies specific to this export
  dependencies?: Dependency[];
}
```

### Method

```typescript
interface Method {
  // Required: Input parameters (use empty array if none)
  inputs: Parameter[];

  // Required: Return type specification
  returns: ReturnType;

  // Optional: Exception types this method can throw
  throws?: string[];

  // Optional: External methods/functions this method calls
  // Format: "module-name.ExportName.methodName"
  calls?: string[];

  // Optional: Side effects (file I/O, network, etc.)
  effects?: string[];

  // Optional: Is this method async? (default: false)
  // For TypeScript: wraps return type in Promise<T>
  // For Rust: generates async fn signature
  async?: boolean;
}
```

### Parameter

```typescript
interface Parameter {
  // Required: Parameter name
  name: string;

  // Required: Parameter type
  // Can be: primitive, custom type, array (Type[]), generic (Promise<T>)
  type: string;

  // Optional: Is this parameter optional? (default: false)
  optional?: boolean;

  // Optional: Parameter description
  description?: string;
}
```

### ReturnType

```typescript
interface ReturnType {
  // Required: Return type
  // Can be: primitive, custom type, array (Type[]), generic (Promise<T>)
  type: string;

  // Optional: For generic types, the inner type
  // Deprecated: Use full type syntax instead (e.g., "Promise<User>")
  inner?: string;
}
```

### Property

```typescript
interface Property {
  // Required: Property type
  // Can be: primitive, custom type, array (Type[]), generic (Option<T>)
  type: string;

  // Optional: Is this property required? (default: true)
  required?: boolean;

  // Optional: Property description
  description?: string;
}
```

### Dependency

```typescript
interface Dependency {
  // Required: Module name to depend on
  module: string;

  // Required: List of exports to import from that module
  imports: string[];
}
```

---

## Manifest

The manifest file (`.crucible/manifest.json`) defines the overall project structure.

### Manifest

```typescript
interface Manifest {
  // Required: Manifest version (typically "0.1.0")
  version: string;

  // Required: Project configuration
  project: ProjectConfig;

  // Required: List of module names (matches .json filenames)
  modules: string[];

  // Optional: Enable strict validation (default: true)
  strict_validation?: boolean;

  // Optional: Project metadata
  metadata?: Metadata;
}
```

### ProjectConfig

```typescript
interface ProjectConfig {
  // Required: Project name
  name: string;

  // Required: Primary programming language
  language: 'typescript' | 'rust' | 'python' | 'go' | 'java';

  // Optional: Architecture pattern
  architecture_pattern?: 'layered' | 'hexagonal' | 'microservices' | 'modular';
}
```

### Metadata

```typescript
interface Metadata {
  // Optional: Project author
  author?: string;

  // Optional: Repository URL
  repository?: string;

  // Optional: Creation timestamp
  created?: string;
}
```

---

## Rules

The rules file (`.crucible/rules.json`) defines architectural constraints.

### Rules

```typescript
interface Rules {
  // Optional: Architecture configuration
  architecture?: Architecture;

  // Required: Validation rules to enforce
  rules: Rule[];
}
```

### Architecture

```typescript
interface Architecture {
  // Required: Architecture pattern
  pattern: 'layered' | 'hexagonal' | 'microservices' | 'modular';

  // Required: Layer definitions with dependency rules
  layers: Layer[];
}
```

### Layer

```typescript
interface Layer {
  // Required: Layer name (referenced by modules)
  name: string;

  // Required: Layers this layer can depend on
  // Include the layer's own name to allow intra-layer dependencies
  // Example: ["domain"] allows domain modules to depend on each other
  can_depend_on: string[];
}
```

### Rule

```typescript
interface Rule {
  // Required: Unique rule identifier
  id: string;

  // Required: Is this rule enabled?
  enabled: boolean;

  // Required: Severity level
  severity: 'error' | 'warning';
}
```

**Available Rule IDs:**
- `no-circular-dependencies` - Prevent circular module dependencies
- `all-calls-must-exist` - Verify all method calls reference valid exports
- `used-dependencies-declared` - Ensure used dependencies are declared
- `declared-dependencies-must-be-used` - Warn about unused dependencies

---

## Export Types

### Class Export

```json
{
  "UserService": {
    "type": "class",
    "methods": {
      "createUser": {
        "inputs": [
          {"name": "data", "type": "CreateUserDTO"}
        ],
        "returns": {"type": "Promise<User>"}
      },
      "findById": {
        "inputs": [
          {"name": "id", "type": "string"}
        ],
        "returns": {"type": "Promise<User | null>"}
      }
    }
  }
}
```

### Function Export

```json
{
  "validateEmail": {
    "type": "function",
    "inputs": [
      {"name": "email", "type": "string"}
    ],
    "returns": {"type": "boolean"}
  },
  "Button": {
    "type": "function",
    "inputs": [
      {"name": "props", "type": "ButtonProps"}
    ],
    "returns": {"type": "JSX.Element"}
  }
}
```

### Interface Export

```json
{
  "User": {
    "type": "interface",
    "properties": {
      "id": {"type": "string", "required": true},
      "email": {"type": "string", "required": true},
      "name": {"type": "string", "required": true},
      "avatar": {"type": "string", "required": false}
    }
  }
}
```

### Type Alias Export

```json
{
  "UserId": {
    "type": "type",
    "properties": {
      "value": {"type": "string"}
    }
  }
}
```

### Enum Export

```json
{
  "UserRole": {
    "type": "enum",
    "values": ["admin", "editor", "viewer"]
  }
}
```

### Event Export

Domain events represent significant state changes in the system. They carry typed payload data.

```json
{
  "UserCreated": {
    "type": "event",
    "payload": {
      "userId": {"type": "string", "required": true},
      "email": {"type": "string", "required": true},
      "timestamp": {"type": "Date", "required": false}
    }
  },
  "OrderPlaced": {
    "type": "event",
    "payload": {
      "orderId": {"type": "OrderId", "required": true},
      "items": {"type": "OrderItem[]", "required": true},
      "total": {"type": "number", "required": true}
    }
  }
}
```

**Generated TypeScript:**
```typescript
export type UserCreated = {
  readonly type: 'UserCreated';
  readonly timestamp: Date;
  readonly payload: {
    userId: string;
    email: string;
    timestamp?: Date;
  };
};

export function createUserCreated(userId: string, email: string): UserCreated {
  return {
    type: 'UserCreated',
    timestamp: new Date(),
    payload: { userId, email },
  };
}
```

### Trait Export

Traits define behavioral contracts (similar to Rust traits or TypeScript interfaces with methods). They support async methods for asynchronous operations.

```json
{
  "Repository": {
    "type": "trait",
    "methods": {
      "findById": {
        "inputs": [{"name": "id", "type": "string"}],
        "returns": {"type": "Entity"},
        "async": true
      },
      "save": {
        "inputs": [{"name": "entity", "type": "Entity"}],
        "returns": {"type": "void"},
        "async": true
      },
      "validate": {
        "inputs": [{"name": "entity", "type": "Entity"}],
        "returns": {"type": "boolean"},
        "async": false
      }
    }
  },
  "EventHandler": {
    "type": "trait",
    "methods": {
      "handle": {
        "inputs": [{"name": "event", "type": "DomainEvent"}],
        "returns": {"type": "Result<void, Error>"},
        "async": true
      }
    }
  }
}
```

**Generated TypeScript:**
```typescript
export interface Repository {
  findById(id: string): Promise<Entity>;
  save(entity: Entity): Promise<void>;
  validate(entity: Entity): boolean;
}

export interface EventHandler {
  handle(event: DomainEvent): Promise<Result<void, Error>>;
}
```

---

## Type System

Crucible supports TypeScript-like type expressions:

### Primitive Types

- `string` - String values
- `number` - Numeric values
- `boolean` - Boolean values
- `void` - No return value
- `null` - Null value
- `undefined` - Undefined value

### Built-in Object Types

- `Date` - Date/time values
- `Buffer` - Binary data
- `Error` - Error objects
- `RegExp` - Regular expressions
- `Map` - Key-value maps
- `Set` - Unique value sets

### Database/Connection Types

- `Connection` - Database connections
- `Transaction` - Database transactions
- `QueryResult` - Query results

### Special Types

- `object` - Any object
- `any` - Any type (use sparingly)
- `unknown` - Unknown type (safer than any)
- `never` - Never occurs

### Array Types

```typescript
// Array shorthand syntax
"items": {"type": "User[]"}

// Nested arrays
"matrix": {"type": "number[][]"}

// Array of custom types
"users": {"type": "User[]"}
```

### Generic Types

```typescript
// Promise
"returns": {"type": "Promise<User>"}

// Array (generic form)
"items": {"type": "Array<string>"}

// Map
"cache": {"type": "Map<string, User>"}

// TypeScript utility types
"partial": {"type": "Partial<User>"}
"required": {"type": "Required<User>"}
"readonly": {"type": "Readonly<User>"}
"pick": {"type": "Pick<User, 'id' | 'email'>"}
"omit": {"type": "Omit<User, 'password'>"}
```

### Union Types

```typescript
// Nullable type
"result": {"type": "User | null"}

// Multiple types
"value": {"type": "string | number"}

// With undefined
"optional": {"type": "string | undefined"}
```

### Function Types

```typescript
// Callback function
"onClick": {"type": "() => void"}

// Function with parameters
"onChange": {"type": "(value: string) => void"}

// Function with return value
"transform": {"type": "(input: string) => number"}
```

### Complex Types

```typescript
// Custom generic types
"result": {"type": "Result<User, Error>"}

// Nested generics
"promise": {"type": "Promise<Result<User, Error>>"}

// Array of promises
"batch": {"type": "Promise<User>[]"}
```

---

## Dependencies

Dependencies specify which exports from other modules are used.

### Single Export

```json
{
  "dependencies": {
    "user-service": "UserService"
  }
}
```

### Multiple Exports from Same Module

Use comma-separated export names:

```json
{
  "dependencies": {
    "user-module": "User,CreateUserDTO,UpdateUserDTO",
    "auth-service": "AuthService,AuthToken"
  }
}
```

### Cross-Layer Dependencies

Ensure dependencies respect layer boundaries defined in `rules.json`:

```json
// ✅ Valid: Application can depend on Domain
{
  "module": "user-service",
  "layer": "application",
  "dependencies": {
    "user": "User"  // Domain module
  }
}

// ❌ Invalid: Domain cannot depend on Application
{
  "module": "user",
  "layer": "domain",
  "dependencies": {
    "user-service": "UserService"  // Application module
  }
}
```

---

## Quick Reference

### Minimal Module

```json
{
  "module": "example",
  "version": "1.0.0",
  "layer": "domain",
  "exports": {
    "Example": {
      "type": "interface",
      "properties": {
        "id": {"type": "string"}
      }
    }
  },
  "dependencies": {}
}
```

### Complete Module Example

```json
{
  "module": "user-service",
  "version": "1.0.0",
  "layer": "application",
  "description": "User management service with CRUD operations",
  "exports": {
    "UserService": {
      "type": "class",
      "methods": {
        "createUser": {
          "inputs": [
            {"name": "data", "type": "CreateUserDTO"}
          ],
          "returns": {"type": "Promise<User>"},
          "throws": ["ValidationError", "DatabaseError"],
          "calls": ["user-repository.UserRepository.save"],
          "effects": ["database:write"]
        },
        "findById": {
          "inputs": [
            {"name": "id", "type": "string"}
          ],
          "returns": {"type": "Promise<User | null>"},
          "calls": ["user-repository.UserRepository.findById"],
          "effects": ["database:read"]
        }
      }
    },
    "CreateUserDTO": {
      "type": "interface",
      "properties": {
        "email": {"type": "string", "required": true},
        "name": {"type": "string", "required": true},
        "password": {"type": "string", "required": true}
      }
    }
  },
  "dependencies": {
    "user": "User",
    "user-repository": "UserRepository",
    "errors": "ValidationError,DatabaseError"
  }
}
```

### Complete Manifest Example

```json
{
  "version": "0.1.0",
  "project": {
    "name": "my-app",
    "language": "typescript",
    "architecture_pattern": "layered"
  },
  "modules": [
    "user",
    "user-service",
    "user-repository"
  ],
  "strict_validation": true,
  "metadata": {
    "author": "Your Name",
    "repository": "https://github.com/you/my-app",
    "created": "2025-01-17"
  }
}
```

### Complete Rules Example

```json
{
  "architecture": {
    "pattern": "layered",
    "layers": [
      {
        "name": "presentation",
        "can_depend_on": ["presentation", "application", "infrastructure", "domain"]
      },
      {
        "name": "application",
        "can_depend_on": ["application", "infrastructure", "domain"]
      },
      {
        "name": "infrastructure",
        "can_depend_on": ["infrastructure", "domain"]
      },
      {
        "name": "domain",
        "can_depend_on": ["domain"]
      }
    ]
  },
  "rules": [
    {
      "id": "no-circular-dependencies",
      "enabled": true,
      "severity": "error"
    },
    {
      "id": "all-calls-must-exist",
      "enabled": true,
      "severity": "error"
    },
    {
      "id": "used-dependencies-declared",
      "enabled": true,
      "severity": "error"
    },
    {
      "id": "declared-dependencies-must-be-used",
      "enabled": true,
      "severity": "warning"
    }
  ]
}
```

---

## Common Patterns

### Domain Entity

```json
{
  "module": "user",
  "version": "1.0.0",
  "layer": "domain",
  "exports": {
    "User": {
      "type": "interface",
      "properties": {
        "id": {"type": "string"},
        "email": {"type": "string"},
        "created": {"type": "Date"}
      }
    }
  },
  "dependencies": {}
}
```

### Application Service

```json
{
  "module": "user-service",
  "version": "1.0.0",
  "layer": "application",
  "exports": {
    "UserService": {
      "type": "class",
      "methods": {
        "execute": {
          "inputs": [{"name": "command", "type": "Command"}],
          "returns": {"type": "Promise<Result>"}
        }
      }
    }
  },
  "dependencies": {
    "user": "User"
  }
}
```

### Infrastructure Repository

```json
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
```

### React Component

```json
{
  "module": "button",
  "version": "1.0.0",
  "layer": "presentation",
  "exports": {
    "Button": {
      "type": "function",
      "inputs": [{"name": "props", "type": "ButtonProps"}],
      "returns": {"type": "JSX.Element"}
    },
    "ButtonProps": {
      "type": "interface",
      "properties": {
        "label": {"type": "string"},
        "onClick": {"type": "() => void"},
        "disabled": {"type": "boolean", "required": false}
      }
    }
  },
  "dependencies": {}
}
```

### Domain Event

```json
{
  "module": "user-events",
  "version": "1.0.0",
  "layer": "domain",
  "exports": {
    "UserCreated": {
      "type": "event",
      "payload": {
        "userId": {"type": "UserId", "required": true},
        "email": {"type": "string", "required": true},
        "name": {"type": "string", "required": true}
      }
    },
    "UserDeleted": {
      "type": "event",
      "payload": {
        "userId": {"type": "UserId", "required": true},
        "deletedAt": {"type": "Date", "required": true}
      }
    }
  },
  "dependencies": {
    "user": "UserId"
  }
}
```

### Repository Trait

```json
{
  "module": "repository-trait",
  "version": "1.0.0",
  "layer": "domain",
  "exports": {
    "Repository": {
      "type": "trait",
      "methods": {
        "findById": {
          "inputs": [{"name": "id", "type": "string"}],
          "returns": {"type": "T"},
          "async": true
        },
        "findAll": {
          "inputs": [],
          "returns": {"type": "T[]"},
          "async": true
        },
        "save": {
          "inputs": [{"name": "entity", "type": "T"}],
          "returns": {"type": "void"},
          "async": true
        },
        "delete": {
          "inputs": [{"name": "id", "type": "string"}],
          "returns": {"type": "void"},
          "async": true
        }
      }
    }
  },
  "dependencies": {}
}
```

---

## See Also

- [Common Mistakes Guide](./common-mistakes.md)
- [Type System Documentation](./type-system.md)
- [CLI Reference](./cli-reference.md)
- [Example Project](./examples/full-stack-app/)
