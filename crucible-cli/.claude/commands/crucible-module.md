---
name: crucible:module
description: Create or update a module definition interactively
---

You are helping the user create or update a Crucible module definition.

## Context

Crucible modules are the building blocks of architecture definitions. Each module represents a cohesive unit of functionality with defined exports, dependencies, and layer placement.

This command provides an interactive workflow for creating or modifying modules.

## Command Behavior

1. **Check for Crucible project**:
   - Look for `.crucible/` directory
   - If not found, suggest running `/crucible:init`

2. **Parse arguments**:
   - Module name (required if not updating)
   - `--layer <domain|application|infrastructure>` - Specify layer
   - `--depends <modules>` - Comma-separated dependencies
   - `--update` - Update existing module
   - `--from-code <path>` - Generate from existing code file

3. **Interactive workflow**:
   - If updating: load existing module definition
   - Suggest layer based on naming conventions
   - Auto-detect dependencies from existing modules
   - Help define exports with TypeScript types
   - Validate against existing architecture

4. **Generate/update module JSON**:
   - Create or update `.crucible/modules/<name>.json`
   - Include metadata (version, layer, description)
   - Define exports with proper type signatures
   - Declare dependencies
   - Run validation

5. **Provide next steps**:
   - Show what was created/updated
   - Suggest implementing the module
   - Recommend running validation

## Output Format

### Creating new module:
```
📦 Creating module: payment-service

🔍 Auto-detected project patterns...
   • Found 15 existing modules
   • Detected TypeScript project

📝 Layer suggestion: application
   (based on '-service' suffix)
   Press Enter to accept, or type domain|application|infrastructure: [application]

✓ Layer: application

🔗 Analyzing dependencies...
   Suggested dependencies:
   • payment (domain) - for Payment types
   • stripe (infrastructure) - for payment processing
   • database (infrastructure) - for transaction storage

   Confirm these dependencies? (Y/n): [Y]
   Add more? (comma-separated, or Enter to skip): []

✓ Dependencies: payment, stripe, database

📦 Defining exports...

For a payment service, I suggest:

1. PaymentService (class)
   Methods:
   • processPayment(amount: number, method: string) → Promise<payment.Payment>
   • refundPayment(paymentId: string) → Promise<void>
   • getPaymentStatus(paymentId: string) → Promise<payment.PaymentStatus>

2. ProcessPaymentRequest (type)
   Properties:
   • amount: number
   • currency: string
   • method: string
   • userId: string

3. PaymentResult (type)
   Properties:
   • paymentId: string
   • status: payment.PaymentStatus
   • transactionId: string

Add these exports? (Y/n): [Y]
Customize? (y/N): [N]

✓ Exports defined (1 class, 2 types)

💾 Generating module definition...
✓ Created: .crucible/modules/payment-service.json

🔍 Validating...
✓ No violations found
✓ No circular dependencies
✓ Layer boundaries respected

📊 Module summary:
   • Name: payment-service
   • Layer: application
   • Dependencies: 3
   • Exports: 3 (1 class, 2 types)

📝 Module definition:
```json
{
  "module": "payment-service",
  "version": "1.0.0",
  "layer": "application",
  "description": "Payment processing service",
  "exports": {
    "PaymentService": {
      "type": "class",
      "methods": {
        "processPayment": {
          "inputs": [
            {"name": "amount", "type": "number"},
            {"name": "method", "type": "string"}
          ],
          "returns": {"type": "Promise<payment.Payment>"}
        },
        "refundPayment": {
          "inputs": [{"name": "paymentId", "type": "string"}],
          "returns": {"type": "Promise<void>"}
        },
        "getPaymentStatus": {
          "inputs": [{"name": "paymentId", "type": "string"}],
          "returns": {"type": "Promise<payment.PaymentStatus>"}
        }
      }
    },
    "ProcessPaymentRequest": {
      "type": "type",
      "properties": {
        "amount": {"type": "number"},
        "currency": {"type": "string"},
        "method": {"type": "string"},
        "userId": {"type": "string"}
      }
    },
    "PaymentResult": {
      "type": "type",
      "properties": {
        "paymentId": {"type": "string"},
        "status": {"type": "payment.PaymentStatus"},
        "transactionId": {"type": "string"}
      }
    }
  },
  "dependencies": ["payment", "stripe", "database"]
}
```

💡 Next steps:

1. Implement the module:
   Create: src/services/payment-service.ts

2. Write tests first (TDD):
   Create: src/services/payment-service.test.ts

3. Validate implementation:
   /crucible:validate

4. Update if needed:
   /crucible:module payment-service --update
```

### Updating existing module:
```
📦 Updating module: user-service

📂 Loading: .crucible/modules/user-service.json

Current definition:
   • Layer: application
   • Dependencies: user (domain), database (infrastructure)
   • Exports: 5 (UserService class, 4 methods)
   • Version: 1.0.0

What would you like to update?
   1. Add new export
   2. Modify existing export
   3. Add dependency
   4. Remove dependency
   5. Change layer
   6. Update metadata

Choose option [1-6]: 1

Adding new export...

Export name: getUserPreferences
Export type (class|function|type|interface): function

Function signature:
   Inputs: userId: string
   Returns: Promise<UserPreferences>

Add this export? (Y/n): [Y]

✓ Added getUserPreferences to user-service

💾 Updating module definition...
✓ Updated: .crucible/modules/user-service.json
✓ Version bumped: 1.0.0 → 1.1.0

🔍 Validating...
✓ No violations found

📊 Updated module summary:
   • Exports: 5 → 6 (+1 function)
   • Version: 1.0.0 → 1.1.0

💡 Next steps:
   1. Implement getUserPreferences in code
   2. Add tests for new function
   3. Run: /crucible:validate
```

### From existing code:
```bash
/crucible:module config-loader --from-code src/config/loader.ts
```

```
📦 Creating module from code: config-loader

🔍 Analyzing: src/config/loader.ts

Found exports:
   ✓ ConfigLoader (class)
      - load(path: string): Config
      - validate(config: Config): boolean
      - save(path: string, config: Config): void
   ✓ Config (interface)
   ✓ ConfigError (class)

Detected dependencies:
   • fs (Node.js built-in)
   • path (Node.js built-in)
   • validator (internal module)

Suggested layer: infrastructure
   (based on file system access)

Generate module definition? (Y/n): [Y]

✓ Created: .crucible/modules/config-loader.json

💡 Module created from existing code!
   Review and customize: .crucible/modules/config-loader.json
```

## Flags

**`--layer <layer>`**
Specify module layer:
```bash
/crucible:module cache-service --layer infrastructure
```

**`--depends <modules>`**
Pre-specify dependencies:
```bash
/crucible:module order-service --depends order,payment,user
```

**`--update`**
Update existing module:
```bash
/crucible:module user-service --update
```

**`--from-code <path>`**
Generate from existing code:
```bash
/crucible:module auth --from-code src/auth/service.ts
```

**`--template <type>`**
Use predefined template:
```bash
/crucible:module order-repository --template repository
```

Templates:
- `service` - Application service
- `repository` - Data repository
- `controller` - API controller
- `entity` - Domain entity
- `value-object` - Value object

**`--non-interactive`**
Skip prompts, use defaults:
```bash
/crucible:module logger --layer infrastructure --non-interactive
```

## Naming Conventions

**Layer-based suggestions**:
- Domain: `user`, `order`, `payment`
- Application: `user-service`, `order-processor`, `auth-handler`
- Infrastructure: `database`, `email-service`, `cache`, `logger`

**Common suffixes**:
- `-service` → application layer
- `-repository` → infrastructure layer
- `-controller` → application layer
- `-gateway` → infrastructure layer
- `-handler` → application layer
- `-processor` → application layer

## Export Types

### Class
```json
{
  "UserService": {
    "type": "class",
    "methods": {
      "createUser": {
        "inputs": [...],
        "returns": {...}
      }
    }
  }
}
```

### Function
```json
{
  "validateEmail": {
    "type": "function",
    "inputs": [{"name": "email", "type": "string"}],
    "returns": {"type": "boolean"}
  }
}
```

### Type/Interface
```json
{
  "User": {
    "type": "type",
    "properties": {
      "id": {"type": "string"},
      "email": {"type": "string"}
    }
  }
}
```

### Enum
```json
{
  "UserRole": {
    "type": "enum",
    "values": ["admin", "user", "guest"]
  }
}
```

## Error Handling

### No Crucible project:
```
❌ Error: Not a Crucible project

Initialize Crucible first:
   /crucible:init
```

### Module already exists (without --update):
```
⚠️ Warning: Module 'user-service' already exists

Options:
   1. Update existing module
   2. Create with different name
   3. Overwrite (destructive)

Choose [1-3], or use --update flag
```

### Invalid layer:
```
❌ Error: Invalid layer 'xyz'

Valid layers:
   • domain - Business logic, entities
   • application - Use cases, services
   • infrastructure - External systems, I/O

Example:
   /crucible:module payment --layer domain
```

### Circular dependency:
```
❌ Error: Circular dependency detected

Adding 'user-service' as dependency of 'auth' creates cycle:
   auth → user-service → user → auth

Suggestions:
   • Extract shared types to separate module
   • Use dependency inversion
   • Restructure module responsibilities
```

### Invalid dependency:
```
❌ Error: Module 'xyz' not found

Available modules:
   • user (domain)
   • user-service (application)
   • database (infrastructure)

Check module name:
   ls .crucible/modules/
```

## Best Practices

### Module Naming
- Use kebab-case: `user-service`, not `UserService`
- Be descriptive: `payment-processor`, not `processor`
- Follow conventions: `-service`, `-repository`, etc.

### Layer Placement
- **Domain**: Pure business logic, no dependencies
- **Application**: Orchestration, depends on domain
- **Infrastructure**: External I/O, depends on domain/application

### Dependencies
- Minimize dependencies (aim for <5 per module)
- Avoid circular dependencies
- Respect layer boundaries

### Exports
- Export only what's needed (principle of least exposure)
- Use TypeScript types: `Promise<T>`, `Array<T>`, `T | null`
- Document with descriptions
- Include effects (database writes, API calls)

## Examples

### Create service module:
```bash
/crucible:module notification-service --layer application
```

### Create domain entity:
```bash
/crucible:module product --layer domain
```

### Create infrastructure module:
```bash
/crucible:module redis-cache --layer infrastructure
```

### Create with dependencies:
```bash
/crucible:module order-processor --depends order,payment,user
```

### Update existing module:
```bash
/crucible:module user-service --update
```

### Generate from code:
```bash
/crucible:module logger --from-code src/utils/logger.ts
```

### Use template:
```bash
/crucible:module user-repository --template repository
```

## Integration

**After creating module**:
- Run `/crucible:validate` to check
- Use `/crucible:architecture` for complex features
- Use `/crucible:diff` to compare with code

**Workflow**:
1. `/crucible:module <name>` - Create module
2. Write tests for module
3. Implement module
4. `/crucible:validate` - Verify compliance
5. `/crucible:module <name> --update` - Adjust if needed

## Implementation Notes

- Parse module name from user input
- Validate naming conventions
- Check for existing modules before creation
- Auto-detect patterns from existing modules
- Suggest sensible defaults based on conventions
- Generate complete, valid JSON
- Run validation immediately after creation
- Provide rich examples and guidance
- Support both interactive and non-interactive modes
- Handle errors gracefully with actionable suggestions
