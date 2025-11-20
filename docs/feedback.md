 Crucible Framework: Observations & Feedback

  📋 Summary

  Crucible is a powerful architecture-first TDD framework, but the documentation and examples need significant improvements for smoother adoption. I encountered multiple
  schema mismatches between examples and actual validation requirements.

  ---
  ❌ Critical Issues Encountered

  1. Schema Documentation Mismatch

  Problem: The module schema format in examples doesn't match what Crucible actually validates.

  Examples showed:
  {
    "name": "module-name",  // ❌ Wrong field name
    "exports": [             // ❌ Array instead of object
      {
        "name": "ExportName",
        "type": "interface",
        "properties": {
          "version": "string"  // ❌ Simple string instead of object
        }
      }
    ],
    "dependencies": [        // ❌ Array instead of object
      "dependency-name"
    ]
  }

  Actual required format:
  {
    "module": "module-name",  // ✅ Correct field
    "exports": {              // ✅ Object/HashMap
      "ExportName": {
        "type": "interface",
        "properties": {
          "version": {"type": "string"}  // ✅ Nested object
        }
      }
    },
    "dependencies": {         // ✅ Object/HashMap
      "module-name": "ExportName"
    }
  }

  Impact: Had to rewrite all 32 modules after initial validation failures.

  ---
  2. Method Parameter Naming Confusion

  Problem: Method definitions use inputs not parameters.

  What I tried first (based on common conventions):
  {
    "methods": {
      "execute": {
        "parameters": [],  // ❌ Wrong field name
        "returns": "Promise<void>"
      }
    }
  }

  Correct format:
  {
    "methods": {
      "execute": {
        "inputs": [],  // ✅ Crucible uses "inputs"
        "returns": {"type": "Promise<void>"}
      }
    }
  }

  Validation Error: missing field 'inputs' at line 13 column 9

  Impact: Had to batch-update all modules with method definitions.

  ---
  3. Return Type Structure Ambiguity

  Problem: Return types require nested object structure, not simple strings.

  Incorrect (seems intuitive):
  "returns": "string"

  Correct:
  "returns": {"type": "string"}

  Validation Error: invalid type: string "Promise<void>", expected struct ReturnType

  ---
  4. Dependency Format Confusion

  Problem: Dependencies should be simple string mappings, not arrays.

  Incorrect:
  "dependencies": {
    "module-name": ["Export1", "Export2"]
  }

  Correct (for single export):
  "dependencies": {
    "module-name": "Export1"
  }

  Question: How to specify multiple exports from same module? The sed fix I applied changed ["A", "B"] to "A,B" which might not be correct either.

  ---
  5. Type Variant Restrictions

  Problem: Export types are restricted to specific variants without clear documentation.

  Attempted:
  {
    "type": "react-component"  // ❌ Not recognized
  }

  Valid types only: class, function, interface, type, enum

  Validation Error: unknown variant 'react-component', expected one of 'class', 'function', 'interface', 'type', 'enum'

  Impact: Had to restructure all React component definitions as functions with separate Props interfaces.

  ---
  6. Layer Dependency Assumptions

  Problem: Default examples suggest layers can't depend on themselves, but real-world architectures need this.

  Initial rules.json (seemed standard):
  {
    "layers": [
      {"name": "domain", "can_depend_on": []}  // ❌ Too restrictive
    ]
  }

  Needed for real projects:
  {
    "layers": [
      {"name": "domain", "can_depend_on": ["domain"]}  // ✅ Domain entities reference each other
    ]
  }

  Impact: Got 14 layer boundary violations until I updated rules to allow intra-layer dependencies.

  ---
  🎯 Wishlist: Documentation Improvements

  1. Complete JSON Schema Reference

  Need: A comprehensive JSON schema document with:
  - All required fields clearly marked
  - All optional fields documented
  - Field types explicitly stated (object vs array vs string)
  - Valid enum values listed
  - Default values documented

  Example format:
  interface ModuleDefinition {
    module: string;           // Required: module identifier
    version: string;          // Required: semantic version
    layer: LayerName;         // Required: one of defined layers
    description?: string;     // Optional: module description
    exports: {                // Required: HashMap<ExportName, Export>
      [key: string]: Export;
    };
    dependencies?: {          // Optional: HashMap<ModuleName, ExportName>
      [key: string]: string;
    };
  }

  interface Export {
    type: 'class' | 'function' | 'interface' | 'type' | 'enum';  // Required
    // ... type-specific fields
  }

  ---
  2. Real-World Examples

  Need: Complete example projects showing:

  Example A: Monorepo with Multiple Languages
  - Rust backend modules
  - TypeScript/React frontend modules
  - Shared domain layer
  - How to handle cross-language type references

  Example B: Microservices Architecture
  - Multiple independent services
  - Shared domain contracts
  - Infrastructure modules
  - How modules map to services

  Example C: Full-Stack Web App
  - Domain layer: business entities
  - Infrastructure: database, API clients, file storage
  - Application: services and use cases
  - Presentation: React components

  ---
  3. Method/Function Definition Guide

  Need: Clear documentation for defining functions/methods:

  {
    "type": "function",
    "inputs": [              // Array of input parameters
      {
        "name": "param1",    // Parameter name
        "type": "string"     // Parameter type
      }
    ],
    "returns": {             // Return type structure
      "type": "Promise<T>"   // Return type
    }
  }

  Questions to answer:
  - How to specify optional parameters?
  - How to specify rest parameters?
  - How to specify generic types?
  - How to specify union types?
  - How to specify function types (callbacks)?

  ---
  4. Dependency Specification Guide

  Need: Documentation covering:

  Single export dependency:
  "dependencies": {
    "user-module": "User"
  }

  Multiple exports from same module (unclear):
  "dependencies": {
    "types-module": "User,Post,Comment"  // Is this correct?
    // OR
    "types-module": ["User", "Post", "Comment"]  // Or this?
  }

  Type-only vs runtime dependencies?

  ---
  5. Layer Architecture Patterns

  Need: Documentation showing different layering strategies:

  Strict Layering (no intra-layer dependencies):
  {
    "layers": [
      {"name": "domain", "can_depend_on": []}
    ]
  }

  Relaxed Layering (allows intra-layer):
  {
    "layers": [
      {"name": "domain", "can_depend_on": ["domain"]}
    ]
  }

  Hexagonal/Ports & Adapters:
  {
    "pattern": "hexagonal",
    "layers": [
      {"name": "core", "can_depend_on": []},
      {"name": "ports", "can_depend_on": ["core"]},
      {"name": "adapters", "can_depend_on": ["ports", "core"]}
    ]
  }

  When to use which pattern?

  ---
  6. Error Message Improvements

  Current: invalid type: sequence, expected a map at line 7 column 13

  Better:
  Error in module 'project-config.json':
    Line 7: Field 'exports' must be an object (HashMap), not an array.

    Found:    "exports": [...]
    Expected: "exports": {...}

    See: https://docs.crucible.dev/module-schema#exports

  Wishlist:
  - Error messages that explain what's wrong
  - Suggestions for how to fix
  - Links to relevant documentation
  - Show both found and expected formats

  ---
  7. Type System Documentation

  Need: Clear guide on how Crucible's type system works:

  Primitive types: string, number, boolean, Date

  Array types: Type[] or Array<Type>?

  Generic types: How to specify Promise<T>, Result<T, E>, Option<T>?

  Union types: Type1 | Type2 or some other format?

  Function types: (param: Type) => ReturnType or different format?

  Nullable types: Type | null or Type? or Optional<Type>?

  ---
  8. CLI Command Reference

  Need: Complete CLI documentation:

  Current knowledge (discovered through trial):
  - crucible validate - Validate architecture
  - crucible init - Initialize project (guessing)
  - crucible check - Check something (unclear)

  Want to know:
  crucible validate           # Validate architecture
  crucible validate --verbose # Show detailed validation info
  crucible diff              # Show architecture vs code diff
  crucible sync              # Sync architecture with code
  crucible graph             # Generate dependency graph
  crucible analyze           # Analyze architecture health
  crucible status            # Show implementation status
  crucible coverage          # Show test coverage alignment

  ---
  9. Integration with Testing Frameworks

  Need: Documentation for:

  Rust integration:
  [dev-dependencies]
  crucible-test = "0.1.0"

  #[crucible::test(module = "user-service")]
  fn test_create_user() {
      // Test implementation
  }

  TypeScript integration:
  import { crucibleTest } from '@crucible/test';

  crucibleTest('user-service', 'createUser', () => {
    // Test implementation
  });

  Questions:
  - How to ensure tests align with module contracts?
  - How to auto-generate test stubs from module definitions?
  - How to validate implementations match exports?

  ---
  10. VS Code Extension Features

  Wishlist:
  - ✅ Syntax highlighting for .json module files
  - ✅ Auto-completion for export types, layer names
  - ✅ Inline validation errors with quick fixes
  - ✅ "Go to definition" from code to module definition
  - ✅ "Find all references" for module exports
  - ✅ Auto-generate module definitions from code
  - ✅ Visualize dependency graph in editor
  - ✅ Show module implementation status in sidebar

  ---
  🚀 Feature Requests

  1. Partial Module Definitions

  Allow incremental definition of complex modules:

  // user-module.json
  {
    "module": "user",
    "layer": "domain",
    "exports": {
      "User": {"$ref": "./exports/User.json"}  // External file
    }
  }

  ---
  2. Module Templates

  CLI command to generate module definitions:

  crucible new module user-service \
    --layer application \
    --type class \
    --methods createUser,updateUser,deleteUser

  Generates:
  {
    "module": "user-service",
    "layer": "application",
    "exports": {
      "UserService": {
        "type": "class",
        "methods": {
          "createUser": {"inputs": [], "returns": {"type": "User"}},
          "updateUser": {"inputs": [], "returns": {"type": "User"}},
          "deleteUser": {"inputs": [], "returns": {"type": "void"}}
        }
      }
    }
  }

  ---
  3. Reverse Engineering

  Generate module definitions from existing code:

  crucible extract src/services/user.rs > .crucible/modules/user-service.json

  ---
  4. Code Generation

  Generate code stubs from module definitions:

  crucible generate user-service --lang rust --output src/services/

  Creates:
  // src/services/user_service.rs
  pub struct UserService {
      // TODO: Add fields
  }

  impl UserService {
      pub fn create_user(&self) -> User {
          todo!("Implement createUser")
      }
  }

  ---
  5. Diff and Sync Commands

  # Show differences between architecture and code
  crucible diff

  # Sync architecture to match code
  crucible sync --from-code

  # Sync code to match architecture
  crucible sync --from-architecture

  ---
  📊 Summary of Pain Points

  | Issue                     | Severity    | Time Lost | Fix Difficulty         |
  |---------------------------|-------------|-----------|------------------------|
  | Schema format mismatch    | 🔴 Critical | 2 hours   | Easy with docs         |
  | inputs vs parameters      | 🟠 High     | 30 min    | Trivial with docs      |
  | Return type structure     | 🟠 High     | 20 min    | Trivial with docs      |
  | Dependency format         | 🟡 Medium   | 15 min    | Easy with docs         |
  | Type variant restrictions | 🟡 Medium   | 30 min    | Easy with docs         |
  | Layer dependency rules    | 🟢 Low      | 15 min    | Requires understanding |

  Total time spent on schema issues: ~3.5 hours

  With proper documentation: Could be reduced to ~15 minutes

  ---
  ✅ What Works Well

  1. Validation is fast - Sub-second validation on 33 modules
  2. Error messages show file and line - Easy to locate issues
  3. Layer boundary checking - Catches architectural violations
  4. Circular dependency detection - Important safeguard
  5. Simple JSON format - Easy to version control and review

  ---
  🎯 Immediate Documentation Needs

  Priority 1 (would have saved 80% of time):
  1. Complete JSON schema reference with TypeScript-style type definitions
  2. Real-world example project (full-stack app with 20+ modules)
  3. Migration guide from common mistakes

  Priority 2 (quality of life):
  4. Better error messages with fix suggestions
  5. CLI command reference
  6. Type system documentation

  Priority 3 (advanced features):
  7. Testing framework integration guide
  8. Code generation capabilities
  9. VS Code extension

  ---
  Bottom line: Crucible has excellent potential, but needs comprehensive documentation to match its ambitions. The core validation works well, but discovery through
  trial-and-error is painful and time-consuming.