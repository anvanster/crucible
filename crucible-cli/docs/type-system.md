# Crucible Type System Reference

Complete guide to Crucible's type system with examples for TypeScript, Rust, Python, and Go.

## Table of Contents

- [Overview](#overview)
- [Primitive Types](#primitive-types)
- [Array Types](#array-types)
- [Generic Types](#generic-types)
- [Union Types](#union-types)
- [Function Types](#function-types)
- [Nullable Types](#nullable-types)
- [Complex Types](#complex-types)
- [Language-Specific Types](#language-specific-types)
- [Best Practices](#best-practices)

---

## Overview

Crucible's type system is designed to be:
- **Language-agnostic** - Works with TypeScript, Rust, Python, Go, Java
- **Expressive** - Supports arrays, generics, unions, and complex types
- **Validatable** - Types can be checked across module boundaries

### Type Syntax

Types in Crucible use TypeScript-like syntax regardless of the target language:

```json
{
  "properties": {
    "name": {"type": "string"},
    "items": {"type": "Item[]"},
    "result": {"type": "Promise<User>"},
    "optional": {"type": "string | null"}
  }
}
```

---

## Primitive Types

### Built-in Primitives

```json
{
  "properties": {
    "text": {"type": "string"},
    "count": {"type": "number"},
    "active": {"type": "boolean"},
    "nothing": {"type": "void"},
    "empty": {"type": "null"},
    "missing": {"type": "undefined"}
  }
}
```

**Language Mappings:**

| Crucible | TypeScript | Rust | Python | Go |
|----------|------------|------|--------|-----|
| `string` | `string` | `String` | `str` | `string` |
| `number` | `number` | `f64` | `float` | `float64` |
| `boolean` | `boolean` | `bool` | `bool` | `bool` |
| `void` | `void` | `()` | `None` | - |
| `null` | `null` | `None` | `None` | `nil` |

---

## Array Types

### Array Syntax

Crucible supports two array syntaxes:

```json
{
  "properties": {
    "items1": {"type": "User[]"},
    "items2": {"type": "Array<User>"}
  }
}
```

Both are equivalent. The `[]` syntax is more concise.

### Nested Arrays

```json
{
  "properties": {
    "matrix": {"type": "number[][]"},
    "groups": {"type": "User[][]"}
  }
}
```

### Array of Primitives

```json
{
  "properties": {
    "tags": {"type": "string[]"},
    "scores": {"type": "number[]"},
    "flags": {"type": "boolean[]"}
  }
}
```

### Array in Method Returns

```json
{
  "methods": {
    "findAll": {
      "inputs": [],
      "returns": {"type": "User[]"}
    },
    "search": {
      "inputs": [{"name": "query", "type": "string"}],
      "returns": {"type": "SearchResult[]"}
    }
  }
}
```

**Language Mappings:**

| Crucible | TypeScript | Rust | Python | Go |
|----------|------------|------|--------|-----|
| `User[]` | `User[]` | `Vec<User>` | `list[User]` | `[]User` |
| `string[]` | `string[]` | `Vec<String>` | `list[str]` | `[]string` |

---

## Generic Types

### Promise

Asynchronous operations:

```json
{
  "methods": {
    "fetchUser": {
      "inputs": [{"name": "id", "type": "string"}],
      "returns": {"type": "Promise<User>"}
    },
    "saveData": {
      "inputs": [{"name": "data", "type": "Data"}],
      "returns": {"type": "Promise<void>"}
    }
  }
}
```

**Language Mappings:**

| Crucible | TypeScript | Rust | Python | Go |
|----------|------------|------|--------|-----|
| `Promise<T>` | `Promise<T>` | `Future<T>` | `Awaitable[T]` | `chan T` |

### Result/Option

Error handling and optional values:

```json
{
  "methods": {
    "parse": {
      "inputs": [{"name": "input", "type": "string"}],
      "returns": {"type": "Result<Config, Error>"}
    },
    "find": {
      "inputs": [{"name": "id", "type": "string"}],
      "returns": {"type": "Option<User>"}
    }
  }
}
```

**Language Mappings:**

| Crucible | TypeScript | Rust | Python | Go |
|----------|------------|------|--------|-----|
| `Option<T>` | `T \| undefined` | `Option<T>` | `Optional[T]` | `*T` |
| `Result<T,E>` | Custom | `Result<T,E>` | Custom | `(T, error)` |

### Map and Set

Collections:

```json
{
  "properties": {
    "cache": {"type": "Map<string, User>"},
    "seen": {"type": "Set<string>"},
    "index": {"type": "HashMap<number, Item>"}
  }
}
```

### TypeScript Utility Types

```json
{
  "properties": {
    "partial": {"type": "Partial<User>"},
    "required": {"type": "Required<UserDTO>"},
    "readonly": {"type": "Readonly<Config>"},
    "picked": {"type": "Pick<User, 'id' | 'email'>"},
    "omitted": {"type": "Omit<User, 'password'>"},
    "record": {"type": "Record<string, any>"}
  }
}
```

**Note:** These are primarily for TypeScript projects but can be documented for cross-language reference.

---

## Union Types

### Simple Unions

```json
{
  "properties": {
    "value": {"type": "string | number"},
    "status": {"type": "'pending' | 'active' | 'complete'"},
    "result": {"type": "Success | Error"}
  }
}
```

### Nullable Types

```json
{
  "properties": {
    "name": {"type": "string | null"},
    "user": {"type": "User | null"},
    "optional": {"type": "string | undefined"}
  }
}
```

### Union of Complex Types

```json
{
  "properties": {
    "event": {"type": "ClickEvent | KeyEvent | MouseEvent"},
    "response": {"type": "SuccessResponse | ErrorResponse"}
  }
}
```

**Language Mappings:**

| Crucible | TypeScript | Rust | Python | Go |
|----------|------------|------|--------|-----|
| `A \| B` | `A \| B` | `enum {A, B}` | `A \| B` | `interface{}` |
| `T \| null` | `T \| null` | `Option<T>` | `Optional[T]` | `*T` |

---

## Function Types

### Callback Functions

```json
{
  "properties": {
    "onClick": {"type": "() => void"},
    "onChange": {"type": "(value: string) => void"},
    "onSubmit": {"type": "(data: FormData) => Promise<void>"},
    "transform": {"type": "(input: string) => number"}
  }
}
```

### Function with Multiple Parameters

```json
{
  "properties": {
    "compare": {"type": "(a: number, b: number) => number"},
    "reducer": {"type": "(acc: T, item: U) => T"}
  }
}
```

### Generic Functions

```json
{
  "properties": {
    "map": {"type": "<T, U>(items: T[], fn: (item: T) => U) => U[]"},
    "filter": {"type": "<T>(items: T[], predicate: (item: T) => boolean) => T[]"}
  }
}
```

**Note:** Function types are primarily used in property definitions for callbacks and event handlers.

---

## Nullable Types

### Using Union with null

```json
{
  "properties": {
    "name": {"type": "string | null"},
    "user": {"type": "User | null"},
    "data": {"type": "Data | null | undefined"}
  }
}
```

### Using Property Required Flag

```json
{
  "properties": {
    "email": {
      "type": "string",
      "required": true
    },
    "phone": {
      "type": "string",
      "required": false
    }
  }
}
```

**Difference:**
- `"type": "string | null"` - Can be explicitly set to null
- `"required": false` - May be omitted from the object entirely

**Language Mappings:**

| Pattern | TypeScript | Rust | Python | Go |
|---------|------------|------|--------|-----|
| `T \| null` | `T \| null` | `Option<T>` | `Optional[T]` | `*T` |
| `required: false` | `T?` | `Option<T>` | `Optional[T]` | `*T` |

---

## Complex Types

### Nested Generics

```json
{
  "properties": {
    "response": {"type": "Promise<Result<User, Error>>"},
    "batch": {"type": "Promise<User>[]"},
    "nested": {"type": "Map<string, Set<number>>"}
  }
}
```

### Array of Promises

```json
{
  "methods": {
    "batchFetch": {
      "inputs": [{"name": "ids", "type": "string[]"}],
      "returns": {"type": "Promise<User>[]"}
    },
    "parallel": {
      "inputs": [{"name": "tasks", "type": "Task[]"}],
      "returns": {"type": "Promise<Promise<Result>[]>"}
    }
  }
}
```

### Tuple Types (TypeScript)

```json
{
  "properties": {
    "pair": {"type": "[string, number]"},
    "triple": {"type": "[User, Date, Status]"}
  }
}
```

### Intersection Types

```json
{
  "properties": {
    "combined": {"type": "User & Timestamps"},
    "extended": {"type": "BaseEntity & Auditable"}
  }
}
```

---

## Language-Specific Types

### TypeScript-Specific

```json
{
  "properties": {
    "jsx": {"type": "JSX.Element"},
    "react_node": {"type": "React.ReactNode"},
    "component": {"type": "React.FC<Props>"}
  }
}
```

### Rust-Specific

```json
{
  "properties": {
    "boxed": {"type": "Box<dyn Trait>"},
    "ref_counted": {"type": "Rc<RefCell<T>>"},
    "static_str": {"type": "&'static str"}
  }
}
```

### Python-Specific

```json
{
  "properties": {
    "callable": {"type": "Callable[[int, str], bool]"},
    "typed_dict": {"type": "TypedDict"},
    "any": {"type": "Any"}
  }
}
```

### Go-Specific

```json
{
  "properties": {
    "channel": {"type": "chan int"},
    "pointer": {"type": "*User"},
    "interface": {"type": "interface{}"}
  }
}
```

---

## Best Practices

### 1. Use Specific Types

❌ Avoid:
```json
{"type": "any"}
{"type": "object"}
```

✅ Prefer:
```json
{"type": "User"}
{"type": "Map<string, User>"}
```

### 2. Document Complex Types

```json
{
  "ResponseData": {
    "type": "interface",
    "properties": {
      "status": {"type": "number"},
      "data": {"type": "User | null"},
      "error": {"type": "Error | null"}
    }
  }
}
```

Then reference it:
```json
{
  "returns": {"type": "Promise<ResponseData>"}
}
```

### 3. Use Nullable Appropriately

```json
{
  "properties": {
    "required_email": {
      "type": "string",
      "required": true
    },
    "optional_phone": {
      "type": "string",
      "required": false
    },
    "nullable_avatar": {
      "type": "string | null",
      "required": true
    }
  }
}
```

- `required: true` - Must be present
- `required: false` - May be omitted
- `type: "T | null"` - Must be present, can be null

### 4. Consistent Array Syntax

Pick one syntax and stick with it:

```json
// ✅ Consistent
{
  "items": {"type": "User[]"},
  "tags": {"type": "string[]"},
  "scores": {"type": "number[]"}
}

// ❌ Inconsistent
{
  "items": {"type": "Array<User>"},
  "tags": {"type": "string[]"},
  "scores": {"type": "Array<number>"}
}
```

### 5. Explicit Promise Types

Always specify what the Promise resolves to:

```json
// ✅ Explicit
{"returns": {"type": "Promise<User>"}}

// ❌ Vague
{"returns": {"type": "Promise<any>"}}
```

### 6. Avoid Deeply Nested Generics

❌ Hard to read:
```json
{"type": "Promise<Result<Option<Map<string, Vec<User>>>, Error>>"}
```

✅ Break into named types:
```json
{
  "UserMap": {"type": "type", "properties": {"value": {"type": "Map<string, Vec<User>>"}}},
  "UserMapOption": {"type": "type", "properties": {"value": {"type": "Option<UserMap>"}}},
  "UserMapResult": {"type": "type", "properties": {"value": {"type": "Result<UserMapOption, Error>"}}}
}
```

Then use:
```json
{"returns": {"type": "Promise<UserMapResult>"}}
```

---

## Type Validation

Crucible validates types across module boundaries:

### Valid Cross-Module Reference

```json
// user.json (domain layer)
{
  "module": "user",
  "exports": {
    "User": {"type": "interface", "properties": {...}}
  }
}

// user-service.json (application layer)
{
  "module": "user-service",
  "dependencies": {
    "user": "User"
  },
  "exports": {
    "UserService": {
      "type": "class",
      "methods": {
        "create": {
          "inputs": [{"name": "data", "type": "CreateUserDTO"}],
          "returns": {"type": "Promise<User>"}  // ✅ Valid - User imported
        }
      }
    }
  }
}
```

### Invalid Cross-Module Reference

```json
// ❌ Error: Type 'Admin' not found
{
  "module": "user-service",
  "dependencies": {
    "user": "User"  // Only imported User
  },
  "exports": {
    "UserService": {
      "type": "class",
      "methods": {
        "promote": {
          "inputs": [{"name": "userId", "type": "string"}],
          "returns": {"type": "Promise<Admin>"}  // ❌ Admin not imported
        }
      }
    }
  }
}
```

**Fix:** Add Admin to dependencies:
```json
{
  "dependencies": {
    "user": "User,Admin"
  }
}
```

---

## Examples by Use Case

### REST API Response

```json
{
  "ApiResponse": {
    "type": "interface",
    "properties": {
      "data": {"type": "T | null"},
      "error": {"type": "Error | null"},
      "meta": {"type": "ResponseMetadata"}
    }
  }
}
```

### Database Repository

```json
{
  "Repository": {
    "type": "class",
    "methods": {
      "findById": {
        "inputs": [{"name": "id", "type": "string"}],
        "returns": {"type": "Promise<T | null>"}
      },
      "findMany": {
        "inputs": [{"name": "query", "type": "Query"}],
        "returns": {"type": "Promise<T[]>"}
      },
      "save": {
        "inputs": [{"name": "entity", "type": "T"}],
        "returns": {"type": "Promise<void>"}
      }
    }
  }
}
```

### Event Handler

```json
{
  "EventHandler": {
    "type": "interface",
    "properties": {
      "onClick": {"type": "(event: MouseEvent) => void"},
      "onKeyPress": {"type": "(event: KeyboardEvent) => void"},
      "onChange": {"type": "(value: string) => void"}
    }
  }
}
```

### Configuration

```json
{
  "Config": {
    "type": "interface",
    "properties": {
      "database": {"type": "DatabaseConfig"},
      "cache": {"type": "CacheConfig | null", "required": false},
      "features": {"type": "Map<string, boolean>"},
      "env": {"type": "'development' | 'staging' | 'production'"}
    }
  }
}
```

---

## See Also

- [Schema Reference](./schema-reference.md) - Complete schema documentation
- [Common Mistakes](./common-mistakes.md) - Type-related errors
- [Example Project](./examples/full-stack-app/) - Real-world type usage
