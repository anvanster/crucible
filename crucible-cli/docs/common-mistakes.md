# Common Mistakes and How to Fix Them

Migration guide for users transitioning to Crucible or fixing validation errors. Covers 16 common mistakes with solutions.

## Table of Contents

- [Schema Format Errors](#schema-format-errors)
- [Field Naming Issues](#field-naming-issues)
- [Type Structure Mistakes](#type-structure-mistakes)
- [Dependency Format Errors](#dependency-format-errors)
- [Export Type Restrictions](#export-type-restrictions)
- [Event and Trait Mistakes](#event-and-trait-mistakes)
- [Layer Dependency Issues](#layer-dependency-issues)
- [Quick Fix Patterns](#quick-fix-patterns)

---

## Schema Format Errors

### ❌ Mistake 1: Using `name` instead of `module`

**Error Message:**
```
missing field `module` at line 2 column 3
```

**Incorrect:**
```json
{
  "name": "user-service",
  "version": "1.0.0"
}
```

**Correct:**
```json
{
  "module": "user-service",
  "version": "1.0.0"
}
```

**Why:** Crucible uses `module` as the field name to avoid confusion with export names.

**Fix Script:**
```bash
# Batch fix all modules
find .crucible/modules -name "*.json" -exec sed -i '' 's/"name": /"module": /g' {} \;
```

---

### ❌ Mistake 2: Exports as Array

**Error Message:**
```
invalid type: sequence, expected a map at line 7 column 13
```

**Incorrect:**
```json
{
  "exports": [
    {
      "name": "UserService",
      "type": "class"
    }
  ]
}
```

**Correct:**
```json
{
  "exports": {
    "UserService": {
      "type": "class"
    }
  }
}
```

**Why:** Exports are a HashMap (object) where keys are export names, not an array.

**Impact:** This mistake requires restructuring your entire exports section.

**Migration Pattern:**
```javascript
// Convert array to object
const exportsArray = [
  {"name": "User", "type": "interface"},
  {"name": "UserService", "type": "class"}
];

const exportsObject = {};
exportsArray.forEach(exp => {
  const {name, ...rest} = exp;
  exportsObject[name] = rest;
});
```

---

### ❌ Mistake 3: Dependencies as Array

**Error Message:**
```
invalid type: sequence, expected a map at line 25 column 18
```

**Incorrect:**
```json
{
  "dependencies": [
    "user-repository",
    "validation-service"
  ]
}
```

**Correct:**
```json
{
  "dependencies": {
    "user-repository": "UserRepository",
    "validation-service": "ValidationService"
  }
}
```

**Why:** Dependencies map module names to specific export names.

---

## Field Naming Issues

### ❌ Mistake 4: Using `parameters` instead of `inputs`

**Error Message:**
```
missing field `inputs` at line 13 column 9
```

**Incorrect:**
```json
{
  "methods": {
    "createUser": {
      "parameters": [
        {"name": "data", "type": "CreateUserDTO"}
      ],
      "returns": {"type": "User"}
    }
  }
}
```

**Correct:**
```json
{
  "methods": {
    "createUser": {
      "inputs": [
        {"name": "data", "type": "CreateUserDTO"}
      ],
      "returns": {"type": "User"}
    }
  }
}
```

**Why:** Crucible uses `inputs` to distinguish from other parameter-like concepts.

**Fix Script:**
```bash
# Batch fix all modules
find .crucible/modules -name "*.json" -exec sed -i '' 's/"parameters": /"inputs": /g' {} \;
```

---

## Type Structure Mistakes

### ❌ Mistake 5: Simple String for Property Types

**Error Message:**
```
invalid type: string "string", expected struct Property
```

**Incorrect:**
```json
{
  "properties": {
    "email": "string",
    "name": "string"
  }
}
```

**Correct:**
```json
{
  "properties": {
    "email": {
      "type": "string",
      "required": true
    },
    "name": {
      "type": "string",
      "required": true
    }
  }
}
```

**Why:** Each property must be an object with at least a `type` field.

**Fix Script:**
```bash
# Manual fix required - pattern too complex for sed
# Use this Node.js script:
```

```javascript
// fix-properties.js
const fs = require('fs');
const glob = require('glob');

glob('.crucible/modules/*.json', (err, files) => {
  files.forEach(file => {
    const data = JSON.parse(fs.readFileSync(file, 'utf8'));

    Object.values(data.exports || {}).forEach(exp => {
      if (exp.properties) {
        Object.keys(exp.properties).forEach(key => {
          const val = exp.properties[key];
          if (typeof val === 'string') {
            exp.properties[key] = {
              type: val,
              required: true
            };
          }
        });
      }
    });

    fs.writeFileSync(file, JSON.stringify(data, null, 2));
  });
});
```

---

### ❌ Mistake 6: Simple String for Return Type

**Error Message:**
```
invalid type: string "Promise<void>", expected struct ReturnType
```

**Incorrect:**
```json
{
  "methods": {
    "execute": {
      "inputs": [],
      "returns": "Promise<void>"
    }
  }
}
```

**Correct:**
```json
{
  "methods": {
    "execute": {
      "inputs": [],
      "returns": {"type": "Promise<void>"}
    }
  }
}
```

**Why:** Return types must be objects with a `type` field.

**Fix Script:**
```bash
# Manual fix required - use similar Node.js pattern as above
```

---

## Dependency Format Errors

### ❌ Mistake 7: Array of Exports from Same Module

**Error Message:**
```
invalid type: sequence, expected a string at line 32 column 20
```

**Incorrect:**
```json
{
  "dependencies": {
    "user-module": ["User", "CreateUserDTO", "UpdateUserDTO"]
  }
}
```

**Correct:**
```json
{
  "dependencies": {
    "user-module": "User,CreateUserDTO,UpdateUserDTO"
  }
}
```

**Why:** Multiple exports from the same module are specified as a comma-separated string.

**Fix Script:**
```bash
# Convert arrays to comma-separated strings
# Manual fix recommended for accuracy
```

**Note:** This is a common confusion point. Crucible uses comma-separated strings for multiple exports.

---

### ❌ Mistake 8: Missing Export Name in Dependency

**Error Message:**
```
Dependency 'user-service' is declared but not used at module-name
```

**Incorrect:**
```json
{
  "dependencies": {
    "user-service": ""
  }
}
```

**Correct:**
```json
{
  "dependencies": {
    "user-service": "UserService"
  }
}
```

**Why:** Dependencies must specify which export(s) are being used.

---

## Export Type Restrictions

### ❌ Mistake 9: Invalid Export Type

**Error Message:**
```
unknown variant 'react-component', expected one of 'class', 'function', 'interface', 'type', 'enum', 'event', 'trait'
```

**Incorrect:**
```json
{
  "Button": {
    "type": "react-component",
    "properties": {
      "label": {"type": "string"}
    }
  }
}
```

**Correct:**
```json
{
  "Button": {
    "type": "function",
    "inputs": [
      {"name": "props", "type": "ButtonProps"}
    ],
    "returns": {"type": "JSX.Element"}
  },
  "ButtonProps": {
    "type": "interface",
    "properties": {
      "label": {"type": "string"}
    }
  }
}
```

**Why:** Crucible supports 7 export types: `class`, `function`, `interface`, `type`, `enum`, `event`, `trait`.

**React Component Pattern:**
- Use `function` for functional components
- Use `interface` for Props types
- Return type is `JSX.Element`

---

### ❌ Mistake 10: Properties on Function Export

**Error Message:**
```
Function exports should use 'inputs' and 'returns', not 'properties'
```

**Incorrect:**
```json
{
  "validateEmail": {
    "type": "function",
    "properties": {
      "email": {"type": "string"}
    }
  }
}
```

**Correct:**
```json
{
  "validateEmail": {
    "type": "function",
    "inputs": [
      {"name": "email", "type": "string"}
    ],
    "returns": {"type": "boolean"}
  }
}
```

**Why:** Functions use `inputs` and `returns`, not `properties`.

---

## Event and Trait Mistakes

### ❌ Mistake 11: Using Methods on Event Export

**Error Message:**
```
Event exports should use 'payload', not 'methods'
```

**Incorrect:**
```json
{
  "UserCreated": {
    "type": "event",
    "methods": {
      "getData": {
        "inputs": [],
        "returns": {"type": "object"}
      }
    }
  }
}
```

**Correct:**
```json
{
  "UserCreated": {
    "type": "event",
    "payload": {
      "userId": {"type": "string", "required": true},
      "email": {"type": "string", "required": true},
      "createdAt": {"type": "Date", "required": true}
    }
  }
}
```

**Why:** Events represent immutable facts that have occurred. They carry data via `payload`, not behavior via `methods`.

---

### ❌ Mistake 12: Using Properties on Trait Export

**Error Message:**
```
Trait exports should use 'methods', not 'properties'
```

**Incorrect:**
```json
{
  "Repository": {
    "type": "trait",
    "properties": {
      "connection": {"type": "Connection"}
    }
  }
}
```

**Correct:**
```json
{
  "Repository": {
    "type": "trait",
    "methods": {
      "findById": {
        "inputs": [{"name": "id", "type": "string"}],
        "returns": {"type": "object | null"},
        "async": true
      },
      "save": {
        "inputs": [{"name": "entity", "type": "object"}],
        "returns": {"type": "object"},
        "async": true
      }
    }
  }
}
```

**Why:** Traits define behavioral contracts (like Rust traits or Go interfaces). They specify methods that implementors must provide, not data properties.

---

### ❌ Mistake 13: Missing Async Flag on Trait Methods

**Error Message:**
```
Method 'findById' appears to be async but missing 'async: true' flag
```

**Incorrect:**
```json
{
  "Repository": {
    "type": "trait",
    "methods": {
      "findById": {
        "inputs": [{"name": "id", "type": "string"}],
        "returns": {"type": "Promise<User>"}
      }
    }
  }
}
```

**Correct:**
```json
{
  "Repository": {
    "type": "trait",
    "methods": {
      "findById": {
        "inputs": [{"name": "id", "type": "string"}],
        "returns": {"type": "User | null"},
        "async": true
      }
    }
  }
}
```

**Why:** Use the `async: true` flag instead of wrapping return types in `Promise<T>`. The code generator handles async wrapping based on the target language.

**Note:** The `async` flag works on any method, not just traits. It's especially useful for traits where you want consistent async behavior across implementations.

---

### ❌ Mistake 14: Using Payload on Non-Event Types

**Error Message:**
```
Only event exports can have 'payload' field
```

**Incorrect:**
```json
{
  "UserDTO": {
    "type": "interface",
    "payload": {
      "id": {"type": "string"},
      "name": {"type": "string"}
    }
  }
}
```

**Correct:**
```json
{
  "UserDTO": {
    "type": "interface",
    "properties": {
      "id": {"type": "string", "required": true},
      "name": {"type": "string", "required": true}
    }
  }
}
```

**Why:** The `payload` field is specific to event exports. For interfaces and types, use `properties`.

---

## Layer Dependency Issues

### ❌ Mistake 15: Restrictive Layer Rules

**Error Message:**
```
Layer boundary violation: domain module 'user' cannot depend on domain module 'role'
```

**Incorrect `rules.json`:**
```json
{
  "layers": [
    {
      "name": "domain",
      "can_depend_on": []
    }
  ]
}
```

**Correct `rules.json`:**
```json
{
  "layers": [
    {
      "name": "domain",
      "can_depend_on": ["domain"]
    }
  ]
}
```

**Why:** Domain entities often reference each other. Include the layer name itself to allow intra-layer dependencies.

**Common Layer Patterns:**

**Strict Layering (no intra-layer):**
```json
{
  "layers": [
    {"name": "presentation", "can_depend_on": ["application", "infrastructure", "domain"]},
    {"name": "application", "can_depend_on": ["infrastructure", "domain"]},
    {"name": "infrastructure", "can_depend_on": ["domain"]},
    {"name": "domain", "can_depend_on": []}
  ]
}
```

**Relaxed Layering (allows intra-layer):**
```json
{
  "layers": [
    {"name": "presentation", "can_depend_on": ["presentation", "application", "infrastructure", "domain"]},
    {"name": "application", "can_depend_on": ["application", "infrastructure", "domain"]},
    {"name": "infrastructure", "can_depend_on": ["infrastructure", "domain"]},
    {"name": "domain", "can_depend_on": ["domain"]}
  ]
}
```

---

### ❌ Mistake 16: Wrong Layer Dependency Direction

**Error Message:**
```
Layer boundary violation: domain module 'user' cannot depend on application module 'user-service'
```

**Problem:**
```json
{
  "module": "user",
  "layer": "domain",
  "dependencies": {
    "user-service": "UserService"  // Application layer
  }
}
```

**Why:** Dependency direction is wrong. Domain layer cannot depend on application layer.

**Fix:** Reverse the dependency direction.

**Correct:**
```json
{
  "module": "user-service",
  "layer": "application",
  "dependencies": {
    "user": "User"  // Domain layer
  }
}
```

**Layering Rule:** Inner layers cannot depend on outer layers.
- Domain (innermost) → no external dependencies
- Infrastructure → can depend on Domain
- Application → can depend on Infrastructure, Domain
- Presentation (outermost) → can depend on Application, Infrastructure, Domain

---

## Quick Fix Patterns

### Pattern 1: Convert from Common Schema Format

If you have modules in a different format:

```javascript
// convert.js - Convert common schema to Crucible format
const fs = require('fs');

function convertModule(oldFormat) {
  return {
    module: oldFormat.name,
    version: oldFormat.version,
    layer: oldFormat.layer,
    description: oldFormat.description,
    exports: convertExports(oldFormat.exports),
    dependencies: convertDependencies(oldFormat.dependencies)
  };
}

function convertExports(exportsArray) {
  const result = {};

  (exportsArray || []).forEach(exp => {
    const {name, ...rest} = exp;
    result[name] = {
      ...rest,
      properties: convertProperties(rest.properties)
    };
  });

  return result;
}

function convertProperties(propsObject) {
  if (!propsObject) return undefined;

  const result = {};
  Object.entries(propsObject).forEach(([key, val]) => {
    result[key] = typeof val === 'string'
      ? {type: val, required: true}
      : val;
  });

  return result;
}

function convertDependencies(depsArray) {
  if (!depsArray) return {};

  // Convert array of strings to object
  const result = {};
  depsArray.forEach(dep => {
    // Assuming dependency format is "module-name:ExportName"
    const [module, exportName] = dep.split(':');
    result[module] = exportName || 'default';
  });

  return result;
}
```

---

### Pattern 2: Validate Before Commit

Add pre-commit hook to catch errors early:

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "Validating Crucible architecture..."
crucible validate

if [ $? -ne 0 ]; then
  echo "❌ Crucible validation failed. Please fix errors before committing."
  exit 1
fi

echo "✅ Crucible validation passed"
```

---

### Pattern 3: Progressive Migration

Migrate modules incrementally:

1. **Start with domain layer** (fewest dependencies)
2. **Move to infrastructure** (depends on domain)
3. **Then application** (depends on infrastructure, domain)
4. **Finally presentation** (depends on all)

This order minimizes circular dependency issues during migration.

---

## Validation Error Decoder

### Common Error Messages

| Error Message | Likely Cause | Section |
|---------------|--------------|---------|
| `missing field 'module'` | Used `name` instead of `module` | [Mistake 1](#-mistake-1-using-name-instead-of-module) |
| `expected a map, got sequence` | Used array instead of object | [Mistake 2](#-mistake-2-exports-as-array), [Mistake 3](#-mistake-3-dependencies-as-array) |
| `missing field 'inputs'` | Used `parameters` instead of `inputs` | [Mistake 4](#-mistake-4-using-parameters-instead-of-inputs) |
| `expected struct Property` | Used string instead of object for property | [Mistake 5](#-mistake-5-simple-string-for-property-types) |
| `expected struct ReturnType` | Used string instead of object for return | [Mistake 6](#-mistake-6-simple-string-for-return-type) |
| `unknown variant 'X'` | Invalid export type | [Mistake 9](#-mistake-9-invalid-export-type) |
| `Event exports should use 'payload'` | Used methods/properties on event | [Mistake 11](#-mistake-11-using-methods-on-event-export) |
| `Trait exports should use 'methods'` | Used properties on trait | [Mistake 12](#-mistake-12-using-properties-on-trait-export) |
| `Method appears to be async` | Missing async flag | [Mistake 13](#-mistake-13-missing-async-flag-on-trait-methods) |
| `Only event exports can have 'payload'` | Used payload on non-event | [Mistake 14](#-mistake-14-using-payload-on-non-event-types) |
| `Layer boundary violation` | Wrong dependency direction or restrictive rules | [Mistake 15](#-mistake-15-restrictive-layer-rules), [Mistake 16](#-mistake-16-wrong-layer-dependency-direction) |

---

## Getting Help

If you encounter an error not covered here:

1. **Check the error message** against the table above
2. **Review the [Schema Reference](./schema-reference.md)** for correct format
3. **Run validation with verbose output** (coming soon)
4. **File an issue** at https://github.com/anvanster/crucible/issues with:
   - Full error message
   - Module JSON file
   - Expected behavior

---

## Summary

**Most Common Time Wasters:**
1. ❌ `name` → ✅ `module` (5 minutes to fix)
2. ❌ `exports: []` → ✅ `exports: {}` (2 hours to restructure)
3. ❌ `properties: "string"` → ✅ `properties: {type: "string"}` (30 minutes to batch fix)
4. ❌ `parameters` → ✅ `inputs` (10 minutes to batch fix)
5. ❌ Layer rules without intra-layer deps (15 minutes + understanding)
6. ❌ `methods` on events → ✅ `payload` (5 minutes to fix)
7. ❌ `properties` on traits → ✅ `methods` (10 minutes to restructure)

**Total typical migration time:**
- With this guide: **~30 minutes**
- Without this guide: **~3.5 hours** (per feedback)

**Prevention:**
- Use the [Schema Reference](./schema-reference.md) as your source of truth
- Copy examples from the [example project](./examples/full-stack-app/)
- Run `crucible validate` frequently during development
- Add pre-commit hooks for automatic validation

---

## See Also

- [Schema Reference](./schema-reference.md) - Complete schema documentation
- [Type System](./type-system.md) - Type syntax and examples
- [CLI Reference](./cli-reference.md) - Command-line tools
- [Example Project](./examples/full-stack-app/) - Real-world example
