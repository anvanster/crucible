---
name: crucible:validate
description: Run Crucible architecture validation and report violations
---

You are helping the user validate their Crucible architecture against their codebase.

## Context

Crucible is an architecture-first development framework that validates code against predefined architecture definitions. This command runs validation and provides actionable feedback.

## Command Behavior

1. **Check for Crucible project**:
   - Look for `.crucible/` directory in the current working directory
   - If not found, suggest running `/crucible:init` first
   - If found, proceed with validation

2. **Parse user arguments** (if provided):
   - `@<path>` - Validate specific module or directory
   - `--focus <area>` - Focus on: security, performance, dependencies, layering
   - `--severity <level>` - Filter by: error, warning, info
   - `--json` - Output raw JSON for programmatic use
   - `--fix` - Suggest automated fixes for violations

3. **Run validation**:
   ```bash
   crucible validate [path]
   ```

4. **Parse and format output**:
   - Count total violations by severity
   - Group violations by type
   - Provide file:line references
   - Suggest specific fixes for each violation

5. **Provide actionable guidance**:
   - Link violations to module definitions
   - Suggest which files to edit
   - Recommend running tests after fixes
   - Offer to run `/crucible:sync` if drift detected

## Output Format

### Success (0 violations):
```
✅ Architecture validation passed!

📊 Summary:
   • Modules checked: 15
   • Exports validated: 87
   • Dependencies verified: 34
   • No violations found

🎯 Architecture health: 100/100
```

### Violations found:
```
❌ Found 5 violations (3 errors, 2 warnings)

🚨 Errors (must fix):
   1. src/services/user-service.ts:45
      → Missing export 'UserService' in module definition
      📝 Fix: Add to .crucible/modules/user-service.json:
      {
        "exports": {
          "UserService": {
            "type": "class",
            "methods": { ... }
          }
        }
      }

   2. src/auth/auth.ts:23
      → Undeclared dependency on 'database' module
      📝 Fix: Add to .crucible/modules/auth.json:
      {
        "dependencies": ["user", "database"]
      }

   3. src/domain/user.ts:67
      → Layer violation: domain module depends on infrastructure/database
      📝 Fix: Move database access to repository layer or use dependency injection

⚠️ Warnings (should fix):
   1. src/payment/payment-service.ts:100
      → Export 'legacyPayment' is deprecated but still in use
      📝 Fix: Remove from code or update callers

   2. src/api/gateway.ts:15
      → Unused import from 'notification' module
      📝 Fix: Remove import or declare in dependencies

💡 Suggested actions:
   1. Fix 3 errors (required for clean build)
   2. Review 2 warnings (code quality)
   3. Run tests after fixes: npm test
   4. Re-validate: /crucible:validate
   5. Consider running: /crucible:sync to auto-update architecture

📊 Architecture health: 67/100 (needs improvement)
```

### Focused validation:
```bash
/crucible:validate --focus security
```

```
🔒 Security-focused validation

✅ No security violations found

📋 Security checklist:
   ✓ No domain modules depend on infrastructure
   ✓ All authentication modules properly isolated
   ✓ Database access properly abstracted
   ✓ No circular dependencies in auth chain

💡 Security recommendations:
   • Consider adding rate limiting module
   • Review token expiration in auth module
   • Add input validation in API gateway
```

## Error Handling

### Crucible not installed:
```
❌ Error: 'crucible' command not found

📦 Install Crucible:
   cargo install crucible-cli

Or check installation:
   which crucible
```

### No .crucible/ directory:
```
❌ Error: Not a Crucible project

📁 This directory doesn't have a .crucible/ folder.

🚀 Initialize Crucible:
   /crucible:init

Or if this is intentional, specify path:
   /crucible:validate @../other-project/
```

### Invalid module path:
```
❌ Error: Module 'xyz' not found

Available modules:
   • user (domain)
   • user-service (application)
   • auth (application)
   • database (infrastructure)

💡 Check module name:
   ls .crucible/modules/
```

## Implementation Notes

- Always run validation from the project root (where .crucible/ is located)
- Parse JSON output from `crucible validate` for structured data
- Use colored output for better readability
- Provide file:line clickable references when possible
- Keep output concise but actionable
- If many violations (>10), summarize and offer to show details
- Suggest running `/crucible:review` for comprehensive analysis
- Link to documentation for complex violations

## Examples

**Basic validation**:
```bash
/crucible:validate
```

**Validate specific module**:
```bash
/crucible:validate @src/services/user-service.ts
```

**Security-focused validation**:
```bash
/crucible:validate --focus security
```

**Show only errors**:
```bash
/crucible:validate --severity error
```

**Get raw JSON**:
```bash
/crucible:validate --json
```

## Next Steps After Validation

- If violations found → Guide user to fix them
- If architecture drift detected → Suggest `/crucible:sync`
- If clean validation → Congratulate and suggest running tests
- If complex issues → Recommend `/crucible:review` for deeper analysis
- If new feature needed → Suggest `/crucible:architecture <feature>`
