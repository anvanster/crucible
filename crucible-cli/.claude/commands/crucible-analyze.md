---
name: crucible:analyze
description: Deep dive analysis of specific modules with dependency insights
---

You are helping the user perform deep analysis of a Crucible module.

## Context

Module analysis provides comprehensive insights into:
- Module structure and organization
- Dependency relationships (direct and transitive)
- Dependent modules (who uses this)
- Export usage statistics
- Complexity metrics
- Refactoring opportunities

This command helps understand module design and identify improvements.

## Command Behavior

1. **Check for Crucible project**:
   - Verify `.crucible/` directory exists
   - Load all module definitions

2. **Parse arguments**:
   - Module name (required)
   - `--graph` - Generate visual dependency graph
   - `--depth <n>` - Transitive dependency depth (default: 3)
   - `--usage` - Show export usage statistics
   - `--suggest-refactor` - AI-powered refactoring suggestions

3. **Load target module**:
   - Read module definition
   - Parse structure and exports
   - Identify relationships

4. **Analyze dependencies**:
   - Map direct dependencies
   - Trace transitive dependencies
   - Calculate dependency depth
   - Identify circular references

5. **Analyze dependents**:
   - Find modules that depend on target
   - Show which exports they use
   - Calculate coupling metrics

6. **Generate insights**:
   - Complexity assessment
   - Refactoring opportunities
   - Performance implications
   - Security considerations

## Output Format

### Basic analysis:
```bash
/crucible:analyze user-service
```

```
🔍 Analyzing module: user-service

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📦 BASIC INFORMATION

   Module: user-service
   Version: 1.3.0
   Layer: application
   File: src/services/user-service.ts
   Last modified: 2 days ago
   Size: 387 lines

   Description:
   User service - handles user-related business operations including
   creation, retrieval, updates, and authentication.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📤 EXPORTS (8)

   Classes (1):
   • UserService
     Methods:
     - createUser(email, name) → Promise<user.User>
     - getUserById(id) → Promise<user.User | null>
     - getAllUsers() → Promise<user.User[]>
     - updateUser(id, updates) → Promise<user.User>
     - deleteUser(id) → Promise<void>
     - getUserPreferences(userId) → UserPreferences
     - updatePreferences(userId, prefs) → Promise<void>

   Types (3):
   • CreateUserRequest
     Properties: email, name, role
   • UpdateUserRequest
     Properties: name?, role?, isActive?
   • UserPreferences
     Properties: theme, notifications, language

   Functions (2):
   • validateUserData(data) → boolean
   • formatUserDisplay(user) → string

   Enums (1):
   • UserStatus
     Values: active, inactive, suspended, deleted

   Interfaces (1):
   • IUserService
     Defines contract for user service implementations

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔗 DEPENDENCIES (3 direct, 5 transitive)

   Direct Dependencies:
   ┌────────────────┬─────────┬──────────────┐
   │ Module         │ Layer   │ Usage        │
   ├────────────────┼─────────┼──────────────┤
   │ user           │ domain  │ User type    │
   │ database       │ infra   │ Data access  │
   │ auth           │ app     │ Validation   │
   └────────────────┴─────────┴──────────────┘

   Transitive Dependencies (depth: 3):
   user-service
   ├─ user (domain)
   │  └─ validation (domain)
   │     └─ error-types (domain)
   ├─ database (infrastructure)
   │  └─ connection-pool (infrastructure)
   └─ auth (application)
      └─ token (infrastructure)

   Dependency Depth: 3 levels
   Max Dependency Chain: user-service → user → validation → error-types

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

👥 DEPENDENTS (6 modules)

   Modules that depend on user-service:

   1. api-gateway (application)
      Uses:
      • UserService.createUser
      • UserService.getUserById
      • UserService.getAllUsers
      Impact: HIGH (primary API consumer)

   2. admin-panel (application)
      Uses:
      • UserService (all methods)
      • UserStatus enum
      Impact: HIGH (administrative operations)

   3. notification-service (application)
      Uses:
      • UserService.getUserById
      • UserService.getUserPreferences
      Impact: MEDIUM (notification targeting)

   4. audit-logger (infrastructure)
      Uses:
      • UserService.getUserById
      Impact: LOW (user information for logs)

   5. analytics (application)
      Uses:
      • UserService.getAllUsers
      Impact: LOW (user statistics)

   6. auth (application)
      Uses:
      • UserService.getUserById
      • validateUserData
      Impact: MEDIUM (authentication flow)

   Total Dependents: 6
   Coupling Score: MEDIUM (acceptable but monitor)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 EXPORT USAGE STATISTICS

   Most Used Exports:
   1. UserService.getUserById - 6 references (100%)
   2. UserService.createUser - 2 references (33%)
   3. UserService.getAllUsers - 2 references (33%)
   4. UserStatus enum - 1 reference (17%)
   5. validateUserData - 1 reference (17%)

   Least Used Exports:
   • UserService.updatePreferences - 1 reference
   • UpdateUserRequest - 0 references (⚠️ unused)
   • formatUserDisplay - 0 references (⚠️ unused)

   Export Utilization: 75% (6/8 exports used)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📈 COMPLEXITY METRICS

   Module Complexity:
   • Exports: 8 (MODERATE)
   • Methods: 7 (MODERATE)
   • Dependencies: 3 (LOW)
   • Dependents: 6 (MEDIUM)
   • Lines of code: 387 (MODERATE)
   • Dependency depth: 3 (ACCEPTABLE)

   Complexity Score: 6/10 (Moderate)

   Cyclomatic Complexity:
   • createUser: 4 (simple)
   • getUserById: 2 (simple)
   • updateUser: 6 (moderate)
   • deleteUser: 3 (simple)

   Maintainability Index: 72/100 (Good)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚡ PERFORMANCE CONSIDERATIONS

   ✓ Direct dependencies: 3 (optimal)
   ⚠ Dependency depth: 3 levels (acceptable, monitor growth)
   ✓ Coupling: MEDIUM (6 dependents, manageable)

   Performance Score: 85/100 (Good)

   Recommendations:
   • Consider caching getUserById (high usage)
   • Monitor dependency chain growth
   • Optimize updateUser (moderate complexity)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔒 SECURITY ASSESSMENT

   Layer Compliance:
   ✓ Application layer (correct)
   ✓ Can depend on domain (user)
   ⚠ Direct database dependency (consider repository pattern)
   ✓ No circular dependencies

   Access Patterns:
   ✓ Authentication through auth module
   ⚠ Direct database access (bypass validation)
   ✓ Proper error handling

   Security Score: 75/100 (Good)

   Recommendations:
   • Use repository pattern for data access
   • Add input validation layer
   • Consider rate limiting for API endpoints

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💡 REFACTORING OPPORTUNITIES

   1. Remove Unused Exports (LOW EFFORT)
      Exports with no references:
      • UpdateUserRequest (can be removed)
      • formatUserDisplay (can be removed)

      Impact: Reduced surface area, cleaner API
      Effort: 15 minutes

   2. Extract Preferences Logic (MEDIUM EFFORT)
      user-service has user + preferences responsibilities

      Suggestion:
      • Create user-preferences module
      • Move getUserPreferences, updatePreferences
      • Move UserPreferences type
      • Reduce coupling

      Impact: Better separation of concerns
      Effort: 2-3 hours

   3. Implement Repository Pattern (HIGH EFFORT)
      Direct database dependency violates clean architecture

      Suggestion:
      • Create user-repository (infrastructure)
      • Move data access to repository
      • Inject repository into UserService
      • Improve testability

      Impact: Better architecture, easier testing
      Effort: 4-6 hours

   4. Add Caching Layer (MEDIUM EFFORT)
      getUserById is called frequently (6 references)

      Suggestion:
      • Add cache module dependency
      • Cache user lookups
      • Implement cache invalidation
      • Improve performance

      Impact: Faster lookups, reduced DB load
      Effort: 3-4 hours

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📋 SUMMARY

   Overall Health: 78/100 (Good)

   Strengths:
   ✓ Well-defined exports
   ✓ Reasonable complexity
   ✓ Good type safety
   ✓ Active usage (6 dependents)

   Areas for Improvement:
   ⚠ Direct database dependency
   ⚠ Mixed responsibilities (users + preferences)
   ⚠ Unused exports
   ⚠ High coupling potential

   Recommendation: OPTIMIZE AND REFACTOR
   Priority refactorings:
   1. Implement repository pattern (P1)
   2. Extract preferences logic (P2)
   3. Remove unused exports (P3)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💾 Analysis saved to: .crucible/reports/analysis-user-service.md

📝 Next steps:
   1. Review refactoring opportunities
   2. Address high-priority items
   3. Update module definition if needed: /crucible:module user-service --update
   4. Re-analyze after changes: /crucible:analyze user-service
```

### With dependency graph:
```bash
/crucible:analyze user-service --graph
```

```
🔍 Analyzing module: user-service

[... basic analysis ...]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 DEPENDENCY GRAPH

   Dependencies (incoming):

                    ┌─────────────────┐
                    │  user-service   │
                    └────────┬────────┘
                             │
                ┌────────────┼────────────┐
                │            │            │
          ┌─────▼────┐  ┌───▼──────┐  ┌─▼───┐
          │   user   │  │ database │  │ auth│
          │ (domain) │  │ (infra)  │  │(app)│
          └─────┬────┘  └───┬──────┘  └─┬───┘
                │           │            │
          ┌─────▼────┐ ┌───▼───────┐ ┌─▼─────┐
          │validation│ │connection-│ │ token │
          │ (domain) │ │pool(infra)│ │(infra)│
          └──────────┘ └───────────┘ └───────┘

   Dependents (outgoing):

          ┌─────────────┐
          │ user-service│
          └─────┬───────┘
                │
     ┌──────────┼──────────┬─────────────┬──────────────┬───────────┐
     │          │          │             │              │           │
┌────▼────┐ ┌──▼────┐ ┌───▼────────┐ ┌─▼──────────┐ ┌─▼──────┐ ┌─▼───┐
│api-     │ │admin- │ │notification│ │audit-      │ │analytics│ │auth │
│gateway  │ │panel  │ │-service    │ │logger      │ │         │ │     │
└─────────┘ └───────┘ └────────────┘ └────────────┘ └─────────┘ └─────┘

   Circular Dependencies: NONE ✓

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💾 Graph saved to: .crucible/graphs/user-service.dot

   Generate SVG:
   dot -Tsvg .crucible/graphs/user-service.dot -o user-service.svg
```

### With usage statistics:
```bash
/crucible:analyze auth --usage
```

```
🔍 Analyzing module: auth

[... basic analysis ...]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 DETAILED USAGE STATISTICS

   Export: AuthService
   References: 8

   Usage by module:
   1. api-gateway (application)
      Location: src/api/gateway.ts:23
      Methods used:
      • login → 5 calls
      • logout → 3 calls
      • validateToken → 12 calls

   2. admin-panel (application)
      Location: src/admin/auth-handler.ts:45
      Methods used:
      • login → 2 calls
      • validateToken → 8 calls

   3. user-service (application)
      Location: src/services/user-service.ts:67
      Methods used:
      • validateToken → 4 calls

   Export: AuthToken (type)
   References: 12

   Used in:
   • api-gateway (5 locations)
   • user-service (3 locations)
   • admin-panel (2 locations)
   • notification-service (2 locations)

   Export: login (method)
   Total calls: 7
   Call sites:
   • api-gateway: POST /auth/login
   • admin-panel: Admin login flow
   • mobile-app: Device authentication

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💡 Usage insights:
   • validateToken is most called (24 calls)
   • Consider caching validation results
   • login has moderate usage (7 calls)
   • AuthToken is widely used (12 references)
```

## Flags

**`--graph`**
Generate dependency graph visualization:
```bash
/crucible:analyze user-service --graph
```

**`--depth <n>`**
Set transitive dependency depth (default: 3):
```bash
/crucible:analyze payment-service --depth 5
```

**`--usage`**
Show detailed export usage statistics:
```bash
/crucible:analyze auth --usage
```

**`--suggest-refactor`**
Get AI-powered refactoring suggestions:
```bash
/crucible:analyze appointment-service --suggest-refactor
```

**`--format <json|markdown|html>`**
Output format:
```bash
/crucible:analyze user-service --format json
```

**`--save <path>`**
Save analysis to file:
```bash
/crucible:analyze user-service --save reports/analysis.md
```

## Metrics Explained

### Complexity Score (0-10)
- 0-3: Simple (easy to maintain)
- 4-6: Moderate (acceptable complexity)
- 7-8: Complex (needs attention)
- 9-10: Very complex (refactor recommended)

Calculated from:
- Number of exports
- Number of dependencies
- Dependency depth
- Lines of code
- Cyclomatic complexity

### Coupling Score
- LOW: 0-3 dependents (loosely coupled)
- MEDIUM: 4-8 dependents (acceptable)
- HIGH: 9-15 dependents (monitor carefully)
- VERY HIGH: 16+ dependents (refactor recommended)

### Maintainability Index (0-100)
- 80-100: Highly maintainable
- 60-79: Moderately maintainable
- 40-59: Difficult to maintain
- 0-39: Very difficult to maintain

## Error Handling

### No Crucible project:
```
❌ Error: Not a Crucible project

Initialize Crucible first:
   /crucible:init
```

### Module not found:
```
❌ Error: Module 'xyz' not found

Available modules:
   • user (domain)
   • user-service (application)
   • auth (application)
   • database (infrastructure)

Check module name:
   ls .crucible/modules/
```

### Invalid depth:
```
❌ Error: Invalid depth '999'

Depth must be between 1 and 10

Example:
   /crucible:analyze user-service --depth 5
```

## Implementation Notes

- Load module definition from JSON
- Parse dependency graph recursively
- Calculate metrics accurately
- Generate visual representations
- Identify refactoring patterns
- Provide actionable insights
- Save reports with timestamps
- Support multiple output formats

## Examples

**Basic analysis**:
```bash
/crucible:analyze user-service
```

**With dependency graph**:
```bash
/crucible:analyze payment-service --graph
```

**Deep dependency analysis**:
```bash
/crucible:analyze appointment-service --depth 5
```

**Usage statistics**:
```bash
/crucible:analyze auth --usage
```

**Refactoring suggestions**:
```bash
/crucible:analyze order-processor --suggest-refactor
```

**JSON output**:
```bash
/crucible:analyze user-service --format json
```

**Save to file**:
```bash
/crucible:analyze user-service --save reports/user-service-analysis.md
```

## Integration

**CI/CD**:
```yaml
- name: Analyze Critical Modules
  run: |
    crucible analyze auth --format json > reports/auth.json
    crucible analyze payment-service --format json > reports/payment.json
```

**Pre-refactor**:
```bash
# Before refactoring, understand the module
/crucible:analyze user-service --suggest-refactor
```

**Documentation**:
```bash
# Generate module documentation
/crucible:analyze user-service --format markdown --save docs/modules/user-service.md
```
