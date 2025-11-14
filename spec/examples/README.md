# Crucible Examples

This directory contains example Crucible architectures demonstrating various features and patterns.

## ✅ Working Examples

### 1. simple-app
**Status:** ✅ Validates successfully

A minimal example demonstrating:
- Basic module structure
- Layer dependencies (application → infrastructure)
- Interface and class exports
- Enum types
- Function calls between modules

**Structure:**
- `greeter` (application layer) - Greeting functionality
- `logger` (infrastructure layer) - Logging functionality

**To validate:**
```bash
cargo run --bin crucible -- validate --path spec/examples/simple-app
```

### 2. calculator-app
**Status:** ✅ Validates successfully

A more comprehensive example demonstrating:
- Multiple dependencies
- Cross-module type references (e.g., `math.Operation`)
- Error types and throws declarations
- Function exports
- Interface exports
- Enum exports
- Effect declarations
- Three-layer architecture (application → domain + infrastructure)

**Structure:**
- `math` (domain layer) - Pure mathematical operations
- `history` (infrastructure layer) - State management
- `calculator` (application layer) - Orchestration logic

**To validate:**
```bash
cargo run --bin crucible -- validate --path spec/examples/calculator-app
```

---

## ⚠️ Incomplete Examples

### 3. todo-app
**Status:** ❌ Does not validate (missing dependencies)

This example demonstrates a more complex application but is **incomplete**:

**Missing modules:**
- `user` - Referenced by api.json and todo.json but not defined
- `database` - Referenced by todo.json but not defined

**Missing types:**
- Standard library types not recognized as primitives (e.g., `Date`)
- Error types referenced but not defined

**To fix this example, you would need to:**
1. Create `modules/user.json` defining User and UserRepository
2. Create `modules/database.json` defining Database class
3. Add these modules to `manifest.json`
4. Define all error types (ValidationError, DatabaseError, etc.)
5. Add `Date` to the validator's recognized primitive types

**Current validation will fail with errors like:**
```
✗ all-types-must-exist: Type 'Date' not found
✗ all-types-must-exist: Type 'ValidationError' not found
```

---

## Example Comparison

| Feature | simple-app | calculator-app | todo-app |
|---------|-----------|----------------|----------|
| **Validates** | ✅ | ✅ | ❌ |
| **Modules** | 2 | 3 | 3 |
| **Layers** | 2 | 3 | 3 |
| **Interfaces** | 1 | 1 | Many |
| **Classes** | 1 | 4 | Many |
| **Functions** | 0 | 4 | 0 |
| **Enums** | 1 | 1 | Several |
| **Cross-module refs** | Yes | Yes | Yes |
| **Error types** | No | Yes | Yes |
| **Effects** | Yes | Yes | Yes |
| **Complexity** | Simple | Medium | Complex |

---

## Creating Your Own Example

### Step 1: Create directory structure
```bash
mkdir -p spec/examples/my-app/modules
```

### Step 2: Create manifest.json
```json
{
  "version": "0.1.0",
  "project": {
    "name": "my-app",
    "language": "typescript",
    "architecture_pattern": "layered"
  },
  "modules": ["module1", "module2"],
  "strict_validation": true
}
```

### Step 3: Create module definitions
Create `modules/<module-name>.json` for each module listed in manifest.

**Required fields:**
- `module` - Module name (must match filename)
- `version` - Semantic version
- `exports` - Object with exported types
- `dependencies` - Object with module dependencies (can be empty)

**Optional fields:**
- `layer` - Layer name for architectural validation
- `description` - Human-readable description

### Step 4: Create rules.json
```json
{
  "architecture": {
    "pattern": "layered",
    "layers": [
      {"name": "layer1", "can_depend_on": ["layer2"]},
      {"name": "layer2", "can_depend_on": []}
    ]
  },
  "rules": [
    {
      "id": "no-circular-dependencies",
      "enabled": true,
      "severity": "error"
    },
    {
      "id": "respect-layer-boundaries",
      "enabled": true,
      "severity": "error"
    },
    {
      "id": "all-types-must-exist",
      "enabled": true,
      "severity": "error"
    }
  ]
}
```

### Step 5: Validate
```bash
cargo run --bin crucible -- validate --path spec/examples/my-app
```

---

## Export Types Reference

### Interface
```json
{
  "type": "interface",
  "properties": {
    "propertyName": {
      "type": "string",
      "required": true,
      "description": "Optional description"
    }
  }
}
```

### Class
```json
{
  "type": "class",
  "methods": {
    "methodName": {
      "inputs": [
        {"name": "param", "type": "string"}
      ],
      "returns": {"type": "void"},
      "throws": [],
      "calls": [],
      "effects": []
    }
  },
  "dependencies": [
    {"module": "other", "imports": ["Type"]}
  ]
}
```

### Function
```json
{
  "type": "function",
  "methods": {
    "functionName": {
      "inputs": [...],
      "returns": {...},
      "throws": [],
      "calls": [],
      "effects": []
    }
  }
}
```

### Enum
```json
{
  "type": "enum",
  "values": ["value1", "value2", "value3"]
}
```

---

## Type References

### Primitives
- `string`
- `number`
- `boolean`
- `void`
- `null`

### Generic Types
- `Array<T>` - Array of type T
- `Promise<T>` - Async return value
- `Map<K,V>` - Key-value map
- `HashMap<K,V>` - Alternative map syntax
- `Vec<T>` - Alternative array syntax (Rust-style)
- `Result<T,E>` - Result type (Rust-style)
- `Optional<T>` - Nullable value
- `Option<T>` - Alternative optional syntax

### Cross-Module References
Format: `module.TypeName`

Example: `math.Operation`, `logger.LogLevel`

---

## Effect Types Reference

### Database
- `database.read` - Reads from database
- `database.write` - Writes to database
- `database.transaction` - Database transaction

### Network
- `network.request` - Makes HTTP request
- `network.listen` - Listens for connections

### File System
- `file.read` - Reads from file system
- `file.write` - Writes to file system

### State
- `state.read` - Reads global/shared state
- `state.write` - Modifies global/shared state

### Session
- `session.create` - Creates user session
- `session.delete` - Deletes user session
- `session.read` - Reads session data

---

## Tips for Success

1. **Start simple** - Begin with simple-app as a template
2. **Define all dependencies** - Make sure every referenced module exists
3. **Check types** - All types must be defined or be primitives
4. **Respect layers** - Ensure dependencies flow in the allowed direction
5. **Test frequently** - Validate after each module addition
6. **Use cross-module refs** - Reference types as `module.TypeName`
7. **Declare calls** - List all function calls in the `calls` array

---

## Validation Rules

Currently implemented rules:

1. **no-circular-dependencies** - Modules cannot have circular dependencies
2. **respect-layer-boundaries** - Modules can only depend on allowed layers
3. **all-types-must-exist** - All referenced types must be defined

---

## Next Steps

1. Try validating the working examples
2. Examine the module JSON files to understand the structure
3. Create your own example based on simple-app
4. Explore the calculator-app for more advanced patterns
5. Check IMPLEMENTATION-ANALYSIS.md for current limitations
