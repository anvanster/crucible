---
name: crucible:diff
description: Show differences between architecture and code like git diff
---

You are helping the user view differences between their Crucible architecture and actual code.

## Context

Architecture and code can drift over time. This command provides a git-style diff view to quickly identify:
- Missing exports (in code, not in architecture)
- Undeclared exports (in architecture, not in code)
- Signature mismatches
- Dependency differences
- Type changes

## Command Behavior

1. **Check for Crucible project**:
   - Verify `.crucible/` directory exists
   - Load module definitions
   - Scan codebase

2. **Parse arguments**:
   - Module name (optional - if omitted, check all modules)
   - `--show-only missing|extra|mismatch|all` - Filter diff types
   - `--format unified|side-by-side|json` - Diff format
   - `--color` - Enable colored output (default: auto)
   - `--context <n>` - Context lines (default: 3)

3. **Compare architecture and code**:
   - Parse code exports
   - Compare with module definitions
   - Identify differences
   - Categorize changes

4. **Generate diff**:
   - Present in requested format
   - Highlight differences
   - Show context
   - Provide fix suggestions

## Output Format

### All modules:
```bash
/crucible:diff
```

```
📝 Architecture vs Code Differences

Checking 38 modules...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Module: user-service
File: src/services/user-service.ts

Exports:
+ getUserPreferences() → UserPreferences  [MISSING IN ARCHITECTURE]
+ updatePreferences(userId, prefs) → Promise<void>  [MISSING IN ARCHITECTURE]
- legacyValidate(user) → boolean  [MISSING IN CODE]

~ updateUser(id, updates) [SIGNATURE MISMATCH]
    Architecture: updateUser(id: string, updates: object) → User
    Code:         updateUser(id: string, updates: Partial<User>) → Promise<User>

Dependencies:
+ preferences  [UNDECLARED IN ARCHITECTURE]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Module: auth
File: src/auth/auth.ts

No differences found ✓

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Module: patient-service
File: src/services/patient-service.ts

Exports:
+ getPatientPreferences(patientId) → Preferences  [MISSING IN ARCHITECTURE]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 SUMMARY

   Modules checked: 38
   With differences: 2
   Clean: 36

   Changes:
   • Missing in architecture: 4 exports
   • Missing in code: 1 export
   • Signature mismatches: 1
   • Undeclared dependencies: 1

💡 Quick fixes:
   1. Run: /crucible:sync to update architecture
   2. Or manually update: .crucible/modules/*.json
   3. Validate: /crucible:validate
```

### Single module:
```bash
/crucible:diff user-service
```

```
📝 Diff for module: user-service

   Architecture: .crucible/modules/user-service.json
   Code: src/services/user-service.ts

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📦 EXPORTS

+ getUserPreferences() → UserPreferences
    Status: MISSING IN ARCHITECTURE
    Location: src/services/user-service.ts:145

    Code:
    ```typescript
    getUserPreferences(userId: string): UserPreferences {
      const user = this.getUserById(userId);
      return user.preferences || DEFAULT_PREFERENCES;
    }
    ```

+ updatePreferences(userId: string, prefs: Partial<UserPreferences>) → Promise<void>
    Status: MISSING IN ARCHITECTURE
    Location: src/services/user-service.ts:152

    Code:
    ```typescript
    async updatePreferences(
      userId: string,
      prefs: Partial<UserPreferences>
    ): Promise<void> {
      const user = await this.getUserById(userId);
      user.preferences = { ...user.preferences, ...prefs };
      await this.database.update('users', userId, user);
    }
    ```

- legacyValidate(user: User) → boolean
    Status: MISSING IN CODE
    Last seen: src/services/user-service.ts (deleted 3 months ago)
    Marked: Deprecated in v1.2.0

    Architecture:
    ```json
    {
      "legacyValidate": {
        "type": "function",
        "deprecated": true,
        "inputs": [{"name": "user", "type": "user.User"}],
        "returns": {"type": "boolean"}
      }
    }
    ```

~ updateUser(id: string, updates) → User | Promise<User>
    Status: SIGNATURE MISMATCH

    Architecture:
    ```json
    {
      "updateUser": {
        "inputs": [
          {"name": "id", "type": "string"},
          {"name": "updates", "type": "object"}
        ],
        "returns": {"type": "user.User"}
      }
    }
    ```

    Code:
    ```typescript
    async updateUser(
      id: string,
      updates: Partial<User>
    ): Promise<User> {
      // ... implementation
    }
    ```

    Differences:
    • Input type: object → Partial<User> (more specific)
    • Return type: User → Promise<User> (async added)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔗 DEPENDENCIES

+ preferences
    Status: UNDECLARED IN ARCHITECTURE
    Import: src/services/user-service.ts:5

    Code:
    ```typescript
    import { PreferenceStore } from '../domain/preferences';
    ```

    Suggestion: Add to dependencies in user-service.json

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 SUMMARY

   Additions (in code): 3
   Deletions (from architecture): 1
   Mismatches: 1

   Drift Score: 5/10 (Moderate)

💡 Recommendations:
   1. Add new exports to architecture:
      /crucible:sync user-service

   2. Remove deprecated legacyValidate:
      Update .crucible/modules/user-service.json

   3. Fix updateUser signature mismatch:
      Update architecture to match async code

   4. Declare preferences dependency:
      Add to user-service dependencies
```

### Show only missing:
```bash
/crucible:diff --show-only missing
```

```
📝 Exports Missing in Architecture

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

user-service:
+ getUserPreferences() → UserPreferences
+ updatePreferences(userId, prefs) → Promise<void>
+ UserPreferences (type)

patient-service:
+ getPatientPreferences(patientId) → Preferences

auth:
+ refreshToken(oldToken) → Promise<string>

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Total: 5 exports missing in architecture

💡 Fix: /crucible:sync --auto-update
```

### Side-by-side format:
```bash
/crucible:diff user-service --format side-by-side
```

```
📝 Side-by-side comparison: user-service

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

ARCHITECTURE                           CODE

┌─────────────────────────────────────┬─────────────────────────────────────┐
│ .crucible/modules/user-service.json │ src/services/user-service.ts       │
├─────────────────────────────────────┼─────────────────────────────────────┤
│ "exports": {                        │ export class UserService {          │
│   "UserService": {                  │                                     │
│     "type": "class",                │                                     │
│     "methods": {                    │                                     │
│       "createUser": {...},          │   async createUser(...) {...}       │
│       "getUserById": {...},         │   async getUserById(...) {...}      │
│       "updateUser": {               │   async updateUser(                 │
│         "inputs": [                 │     id: string,                     │
│           {"name": "id", ...},      │     updates: Partial<User> ←       │
│           {"name": "updates",       │                                     │
│            "type": "object"}        │   ): Promise<User> {   ←           │
│         ],                          │     // ... implementation           │
│         "returns": {"type": "User"} │   }                                 │
│       },                            │                                     │
│                                     │ + getUserPreferences(userId) {...}  │
│                                     │ + updatePreferences(userId,prefs){} │
│       "legacyValidate": {...} ✗    │                                     │
│     }                               │                                     │
│   }                                 │ }                                   │
│ }                                   │                                     │
└─────────────────────────────────────┴─────────────────────────────────────┘

Legend:
  + Added in code
  - Removed from code
  ← Difference
  ✗ Missing in code

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💡 Sync to align: /crucible:sync user-service
```

### JSON format:
```bash
/crucible:diff user-service --format json
```

```json
{
  "module": "user-service",
  "architecture": ".crucible/modules/user-service.json",
  "code": "src/services/user-service.ts",
  "differences": {
    "exports": {
      "added": [
        {
          "name": "getUserPreferences",
          "type": "function",
          "signature": "getUserPreferences(userId: string) → UserPreferences",
          "location": "src/services/user-service.ts:145"
        },
        {
          "name": "updatePreferences",
          "type": "function",
          "signature": "updatePreferences(userId: string, prefs: Partial<UserPreferences>) → Promise<void>",
          "location": "src/services/user-service.ts:152"
        }
      ],
      "removed": [
        {
          "name": "legacyValidate",
          "type": "function",
          "deprecated": true,
          "lastSeen": "3 months ago"
        }
      ],
      "mismatched": [
        {
          "name": "updateUser",
          "architecture": {
            "inputs": [
              {"name": "id", "type": "string"},
              {"name": "updates", "type": "object"}
            ],
            "returns": "user.User"
          },
          "code": {
            "inputs": [
              {"name": "id", "type": "string"},
              {"name": "updates", "type": "Partial<User>"}
            ],
            "returns": "Promise<User>"
          },
          "differences": ["input_type", "return_type"]
        }
      ]
    },
    "dependencies": {
      "undeclared": ["preferences"]
    }
  },
  "summary": {
    "additions": 2,
    "deletions": 1,
    "mismatches": 1,
    "undeclaredDeps": 1,
    "driftScore": 5
  }
}
```

## Flags

**`--show-only <type>`**
Filter diff types:
```bash
/crucible:diff --show-only missing   # Only show missing in architecture
/crucible:diff --show-only extra     # Only show missing in code
/crucible:diff --show-only mismatch  # Only show signature mismatches
/crucible:diff --show-only all       # Show everything (default)
```

**`--format <format>`**
Output format:
```bash
/crucible:diff --format unified       # Git-style unified diff (default)
/crucible:diff --format side-by-side  # Side-by-side comparison
/crucible:diff --format json          # JSON output
```

**`--color`**
Enable colored output:
```bash
/crucible:diff --color always
/crucible:diff --color never
/crucible:diff --color auto  # Default (based on terminal)
```

**`--context <n>`**
Number of context lines:
```bash
/crucible:diff --context 5
/crucible:diff --context 0  # No context
```

**`--ignore-deprecated`**
Ignore deprecated exports:
```bash
/crucible:diff --ignore-deprecated
```

**`--save <path>`**
Save diff to file:
```bash
/crucible:diff --save reports/diff-$(date +%Y-%m-%d).txt
```

## Diff Symbols

**Exports**:
- `+` - Added in code (missing in architecture)
- `-` - Removed from code (still in architecture)
- `~` - Modified (signature mismatch)
- `✓` - No differences

**Status**:
- `[MISSING IN ARCHITECTURE]` - Export exists in code but not declared
- `[MISSING IN CODE]` - Export declared but not implemented
- `[SIGNATURE MISMATCH]` - Signature differs
- `[UNDECLARED]` - Dependency used but not declared

## Drift Score

```
Drift Score = (Additions + Deletions + Mismatches) / Total Exports * 10

0:     Perfect sync
1-3:   Minor drift (acceptable)
4-6:   Moderate drift (should sync)
7-9:   Significant drift (sync recommended)
10:    Major drift (immediate sync needed)
```

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
   • database (infrastructure)

Check module name:
   ls .crucible/modules/
```

### Code file not found:
```
⚠️  Warning: Implementation file not found

   Module: user-service
   Expected: src/services/user-service.ts

   This might indicate:
   • Module not yet implemented
   • File moved or renamed
   • Incorrect path in configuration

   Options:
   1. Implement the module
   2. Update file path
   3. Remove module definition
```

## Implementation Notes

- Parse TypeScript/Rust/Python code to extract exports
- Compare signatures accurately (handle generics, unions, arrays)
- Detect deprecated exports
- Identify unused dependencies
- Support multiple file formats
- Generate clean, readable diffs
- Provide actionable fix suggestions
- Handle edge cases (overloads, generics, etc.)

## Examples

**Check all modules**:
```bash
/crucible:diff
```

**Check specific module**:
```bash
/crucible:diff user-service
```

**Show only missing**:
```bash
/crucible:diff --show-only missing
```

**Side-by-side comparison**:
```bash
/crucible:diff auth --format side-by-side
```

**JSON output**:
```bash
/crucible:diff user-service --format json
```

**Save to file**:
```bash
/crucible:diff --save reports/drift-report.txt
```

**No color**:
```bash
/crucible:diff --color never
```

## Integration

**Pre-commit Check**:
```bash
#!/bin/bash
# .git/hooks/pre-commit

if crucible diff --show-only missing | grep -q "Total:"; then
  echo "Architecture drift detected!"
  echo "Run: crucible sync"
  exit 1
fi
```

**CI/CD Check**:
```yaml
# .github/workflows/architecture.yml
- name: Check Architecture Drift
  run: |
    crucible diff --format json > drift.json
    if [ $(jq '.summary.driftScore' drift.json) -gt 5 ]; then
      echo "Drift score too high!"
      exit 1
    fi
```

**Daily Report**:
```bash
# cron: 0 9 * * * (daily at 9am)
crucible diff --save reports/drift-$(date +%Y-%m-%d).txt
```

**Workflow**:
```bash
# 1. Check what changed
/crucible:diff

# 2. Review specific module
/crucible:diff user-service

# 3. Sync if needed
/crucible:sync user-service

# 4. Verify
/crucible:diff user-service  # Should show no differences
```

## Use Cases

**Daily Development**:
- Quick check before committing
- Identify architecture drift
- See what changed recently

**Code Review**:
- Verify architecture compliance
- Check signature changes
- Identify missing declarations

**Refactoring**:
- Before: see current state
- During: track changes
- After: verify alignment

**Documentation**:
- Generate change reports
- Track architecture evolution
- Document technical debt

## Best Practices

1. **Check frequently**: Run before committing
2. **Review carefully**: Don't blindly sync
3. **Use with validate**: `/crucible:diff` then `/crucible:validate`
4. **Track trends**: Save periodic diff reports
5. **Automate checks**: Use in CI/CD pipelines
6. **Fix promptly**: Don't let drift accumulate
7. **Document reasons**: Comment why architecture differs
