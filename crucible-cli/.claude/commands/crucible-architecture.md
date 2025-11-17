---
name: crucible:architecture
description: Design architecture for a new feature using architecture-first TDD approach
---

You are helping the user design architecture for a new feature using Crucible's architecture-first TDD methodology.

## Context

Crucible follows an **architecture-first Test-Driven Development** approach:
1. Design architecture (module definitions)
2. Write failing tests based on architecture
3. Implement code to make tests pass
4. Validate architecture compliance

This command guides users through step 1: designing the architecture.

## Command Behavior

1. **Parse feature description**:
   - Extract feature name and requirements from user input
   - Ask clarifying questions if needed
   - Understand the problem domain

2. **Design module architecture**:
   - Suggest module name (kebab-case)
   - Determine appropriate layer (domain, application, infrastructure)
   - Identify dependencies on existing modules
   - Define exports (types, functions, classes, interfaces)
   - Consider TypeScript type system features (generics, unions, arrays)

3. **Interactive workflow**:
   - Ask about layer if not obvious
   - Confirm module name
   - Suggest dependencies based on existing modules
   - Help define exports with proper TypeScript types
   - Validate against existing architecture

4. **Generate module definition**:
   - Create `.crucible/modules/<module-name>.json`
   - Follow Crucible JSON schema
   - Include all exports with proper type signatures
   - Declare dependencies
   - Add metadata (version, layer, language)

5. **Validate architecture**:
   - Run `crucible validate` to check new module
   - Ensure no circular dependencies
   - Verify layer boundaries
   - Check type references

6. **Provide TDD guidance**:
   - Suggest test file location
   - List tests to write (based on exports)
   - Recommend test framework commands
   - Guide towards implementation

## Output Format

### Interactive Design Flow:

```
🏗️ Designing architecture for: User authentication with JWT tokens

📋 Understanding requirements...

❓ What layer should this module belong to?
   1. domain (business logic, core types)
   2. application (use cases, services, orchestration)
   3. infrastructure (external systems, databases, APIs)

   Based on "authentication service", I suggest: application
   Press Enter to accept, or type 1-3: [2]

✓ Layer: application

📝 Module name suggestion: auth
   (from "authentication")
   Press Enter to accept, or type custom name: [auth]

✓ Module: auth

🔍 Analyzing existing modules...
   Found 15 modules in project

🔗 Suggested dependencies:
   • user (domain) - for User type
   • token (infrastructure) - for JWT token generation
   • database (infrastructure) - for credential storage

   Add more dependencies? (comma-separated, or Enter to continue): []

✓ Dependencies: user, token, database

📦 Defining exports...

For an authentication service, I suggest these exports:

1. AuthService (class)
   - login(username: string, password: string) → Promise<user.User>
   - logout(userId: string) → Promise<void>
   - validateToken(token: string) → Promise<boolean>
   - refreshToken(oldToken: string) → Promise<string>

2. AuthToken (type)
   Properties:
   - token: string
   - expiresAt: Date
   - userId: string

3. LoginRequest (type)
   Properties:
   - username: string
   - password: string

4. LoginResponse (type)
   Properties:
   - user: user.User
   - token: AuthToken

Looks good? (Y/n): [Y]

✓ Exports defined

💾 Generating module definition...
✓ Created: .crucible/modules/auth.json

🔍 Validating architecture...
✓ Validation passed: 0 violations
✓ No circular dependencies
✓ Layer boundaries respected
✓ All type references valid

📊 Module summary:
   • Name: auth
   • Layer: application
   • Dependencies: 3 (user, token, database)
   • Exports: 4 (1 class, 3 types)
   • Language: TypeScript

📝 Next steps (TDD approach):

1. **Write failing tests** (RED phase):
   Create: src/auth/auth.test.ts

   ```typescript
   import { AuthService } from './auth';
   import { describe, it, expect, beforeEach } from 'vitest';

   describe('AuthService', () => {
     let authService: AuthService;

     beforeEach(() => {
       authService = new AuthService(/* dependencies */);
     });

     it('should login with valid credentials', async () => {
       const result = await authService.login('user', 'pass');
       expect(result).toBeDefined();
       expect(result.username).toBe('user');
     });

     it('should reject invalid credentials', async () => {
       await expect(
         authService.login('user', 'wrong')
       ).rejects.toThrow('Invalid credentials');
     });

     it('should validate valid token', async () => {
       const isValid = await authService.validateToken('valid-token');
       expect(isValid).toBe(true);
     });

     // Add more tests...
   });
   ```

2. **Run tests** (should FAIL):
   npm test
   # All tests should fail - implementation doesn't exist yet

3. **Implement AuthService** (GREEN phase):
   Create: src/auth/auth.ts

   ```typescript
   import { User } from '../domain/user';
   import { TokenService } from '../infrastructure/token';
   import { Database } from '../infrastructure/database';

   export class AuthService {
     constructor(
       private tokenService: TokenService,
       private database: Database
     ) {}

     async login(username: string, password: string): Promise<User> {
       // Implementation here
     }

     async logout(userId: string): Promise<void> {
       // Implementation here
     }

     async validateToken(token: string): Promise<boolean> {
       // Implementation here
     }

     async refreshToken(oldToken: string): Promise<string> {
       // Implementation here
     }
   }

   export interface AuthToken {
     token: string;
     expiresAt: Date;
     userId: string;
   }

   export interface LoginRequest {
     username: string;
     password: string;
   }

   export interface LoginResponse {
     user: User;
     token: AuthToken;
   }
   ```

4. **Run tests again**:
   npm test
   # Tests should PASS

5. **Validate architecture**:
   /crucible:validate
   # Should show 0 violations

6. **Refactor if needed** (REFACTOR phase):
   - Improve code quality
   - Add error handling
   - Optimize performance
   - Re-run tests to ensure still passing

✅ Architecture-first TDD workflow complete!

🎯 Key benefits:
   • Architecture designed upfront (prevents rework)
   • Tests define expected behavior (living documentation)
   • Implementation guided by tests (less bugs)
   • Validation ensures compliance (no drift)
```

## Flags

**`--layer <domain|application|infrastructure>`**
Skip layer selection prompt:
```bash
/crucible:architecture "Payment processing" --layer application
```

**`--depends <modules>`**
Pre-specify dependencies:
```bash
/crucible:architecture "User repository" --depends user,database
```

**`--template <type>`**
Use predefined template:
```bash
/crucible:architecture "Order service" --template service
```

Templates:
- `service` - Application service class
- `repository` - Data access repository
- `controller` - API controller/handler
- `entity` - Domain entity with business logic
- `value-object` - Immutable value object

**`--language <typescript|rust|python|go>`**
Target programming language:
```bash
/crucible:architecture "Config loader" --language rust
```

**`--non-interactive`**
Skip all prompts (use defaults):
```bash
/crucible:architecture "Cache service" --layer infrastructure --non-interactive
```

## Examples

### Basic usage:
```bash
/crucible:architecture "User authentication with JWT tokens"
```

### With layer specified:
```bash
/crucible:architecture "Payment processing service" --layer application
```

### With dependencies:
```bash
/crucible:architecture "User repository" --depends user,database --layer infrastructure
```

### Using template:
```bash
/crucible:architecture "Order service" --template service --depends order,payment
```

### Rust project:
```bash
/crucible:architecture "Config loader" --language rust --layer infrastructure
```

## Error Handling

### No Crucible project:
```
❌ Error: Not a Crucible project

Initialize Crucible first:
   /crucible:init
```

### Invalid layer:
```
❌ Error: Invalid layer 'xyz'

Valid layers:
   • domain - Business logic, core types, entities
   • application - Use cases, services, orchestration
   • infrastructure - External systems, databases, APIs

💡 Learn more:
   Domain-Driven Design (DDD) layer architecture
   Clean Architecture principles
```

### Module already exists:
```
⚠️ Warning: Module 'auth' already exists

Options:
   1. Update existing module (recommended)
   2. Create with different name
   3. Overwrite (destructive)

Choose option [1-3]:
```

### Circular dependency detected:
```
❌ Error: Circular dependency detected

   auth → user → user-service → auth

This creates a circular dependency chain. Consider:
   • Extract shared types to separate module
   • Use dependency injection
   • Restructure module responsibilities

Would you like help refactoring? (Y/n):
```

### Layer violation:
```
⚠️ Warning: Potential layer violation

You're creating a 'domain' module that depends on 'database' (infrastructure).

Domain modules should not depend on infrastructure.

Suggestions:
   1. Change layer to 'application' or 'infrastructure'
   2. Use repository pattern (inject database dependency)
   3. Move shared types to domain, keep implementation in infrastructure

Proceed anyway? (y/N):
```

## Architecture Best Practices

The command should guide users towards good architecture:

1. **Layer boundaries**:
   - Domain → No external dependencies
   - Application → Can depend on domain
   - Infrastructure → Can depend on domain, application

2. **Module naming**:
   - Kebab-case: `user-service`, `payment-gateway`
   - Descriptive: reflects module responsibility
   - Consistent: follow project conventions

3. **Dependency management**:
   - Minimize dependencies
   - Avoid circular dependencies
   - Use dependency injection

4. **Type design**:
   - Use TypeScript features: generics, unions, types
   - Proper nullability: `Type | null` vs `Type`
   - Arrays: `Type[]` for collections
   - Generics: `Promise<T>`, `Partial<T>` for utilities

5. **Export organization**:
   - Group related exports
   - Clear naming conventions
   - Proper visibility (public vs internal)

## Integration with Other Commands

After designing architecture:
- Run `/crucible:validate` to verify
- Use `/crucible:module <name> --update` to modify
- Run `/crucible:review` for architectural analysis
- Use `/crucible:sync` if implementing first and syncing back

## Implementation Notes

- Parse feature description intelligently (NLP-style)
- Suggest sensible defaults based on naming patterns
- Detect existing modules and suggest dependencies
- Validate as you go (prevent invalid architectures)
- Provide rich examples in TDD guidance
- Generate TypeScript-aware type definitions
- Support multiple programming languages
- Be opinionated but allow overrides
- Educate about architecture patterns
- Link to documentation for complex topics
