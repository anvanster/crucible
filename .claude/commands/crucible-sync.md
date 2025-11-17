---
name: crucible:sync
description: Sync architecture with code changes bidirectionally
---

You are helping the user synchronize their Crucible architecture with code changes.

## Context

Architecture and code can drift over time. This command helps maintain alignment by:
- **Code → Architecture**: Update module definitions based on code changes
- **Architecture → Code**: Generate code stubs based on architecture
- **Bidirectional**: Detect and resolve conflicts interactively

## Command Behavior

1. **Check for Crucible project**:
   - Verify `.crucible/` directory exists
   - Load all module definitions
   - Scan codebase for implementation files

2. **Parse arguments**:
   - `--direction code-to-arch|arch-to-code|both` - Sync direction (default: code-to-arch)
   - `--auto-update` - Auto-accept all changes (use with caution)
   - `--dry-run` - Show what would change without applying
   - `--interactive` - Prompt for each change (default)
   - `--module <name>` - Sync specific module only

3. **Analyze differences**:
   - Compare module definitions with actual code
   - Identify new exports not in architecture
   - Find removed code still in architecture
   - Detect signature changes
   - Discover new dependencies

4. **Interactive resolution**:
   - Present each change with context
   - Ask user to approve/reject/modify
   - Handle conflicts intelligently
   - Preserve manual annotations

5. **Apply changes**:
   - Update module JSON files
   - Generate code stubs if needed
   - Run validation
   - Report summary of changes

## Output Format

### Code to Architecture sync:
```bash
/crucible:sync
```

```
🔄 Syncing architecture with code changes...

🔍 Scanning codebase...
   • Found 38 implementation files
   • Loaded 38 module definitions

📋 Analyzing differences...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📝 DETECTED CHANGES

   Module: user-service
   File: src/services/user-service.ts

   ✨ NEW EXPORTS (3):

   1. getUserPreferences() → UserPreferences
      Function not declared in module definition

      📄 Code:
      ```typescript
      getUserPreferences(userId: string): UserPreferences {
        // ... implementation
      }
      ```

      Add to module definition? [Y/n]: Y

   2. updatePreferences(userId, prefs) → Promise<void>
      Async function not declared

      📄 Code:
      ```typescript
      async updatePreferences(
        userId: string,
        prefs: Partial<UserPreferences>
      ): Promise<void> {
        // ... implementation
      }
      ```

      Add to module definition? [Y/n]: Y

   3. UserPreferences (interface)
      Type exported from module

      📄 Code:
      ```typescript
      export interface UserPreferences {
        theme: 'light' | 'dark';
        notifications: boolean;
        language: string;
      }
      ```

      Add to module definition? [Y/n]: Y

   🗑️ REMOVED FROM CODE (1):

   1. legacyValidate(user) → boolean
      In architecture but not in code

      Status: Marked deprecated 3 months ago
      Last modified: 2025-08-15

      Remove from module definition? [Y/n]: Y

   🔗 NEW DEPENDENCIES (1):

   1. preferences module
      user-service now imports from preferences

      📄 Import statement:
      ```typescript
      import { PreferenceStore } from '../domain/preferences';
      ```

      Add to dependencies? [Y/n]: Y

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📝 SUMMARY OF CHANGES

   Module: user-service
   • Added 3 exports
   • Removed 1 export
   • Added 1 dependency
   • Version: 1.2.0 → 1.3.0

   Module: auth
   • No changes detected

   Module: patient-service
   • Added 1 export
   • Version: 2.1.0 → 2.2.0

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💾 APPLYING CHANGES

   ✓ Updated .crucible/modules/user-service.json
   ✓ Updated .crucible/modules/patient-service.json
   ✓ Bumped versions

🔍 VALIDATING

   Running: crucible validate
   ✓ No violations found

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Sync complete!

   📊 Statistics:
   • Modules analyzed: 38
   • Modules updated: 2
   • Exports added: 4
   • Exports removed: 1
   • Dependencies added: 1

   💡 Next steps:
   1. Review updated module definitions
   2. Commit changes to version control
   3. Update documentation if needed

   git add .crucible/modules/
   git commit -m "chore: sync architecture with code changes"
```

### Dry run mode:
```bash
/crucible:sync --dry-run
```

```
🔄 Dry run: Syncing architecture with code (no changes will be applied)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📝 WOULD MAKE THESE CHANGES:

   user-service.json:
   + Add getUserPreferences() function
   + Add updatePreferences() function
   + Add UserPreferences type
   - Remove legacyValidate() function
   + Add 'preferences' to dependencies

   patient-service.json:
   + Add getPatientPreferences() function

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

ℹ️  No changes applied (dry run mode)

   To apply changes:
   /crucible:sync
```

### Architecture to Code sync:
```bash
/crucible:sync --direction arch-to-code
```

```
🔄 Syncing code with architecture definitions...

⚠️  Warning: This will generate code stubs
   Existing implementations will NOT be overwritten

Continue? [y/N]: y

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📝 GENERATING CODE STUBS

   Module: notification-service
   Architecture: .crucible/modules/notification-service.json
   Target: src/services/notification-service.ts

   Status: File does not exist

   Exports to generate:
   • NotificationService (class)
     - sendEmail(to, subject, body) → Promise<void>
     - sendSMS(to, message) → Promise<void>
     - sendPush(userId, title, body) → Promise<void>

   Generate stub implementation? [Y/n]: Y

   ✓ Generated src/services/notification-service.ts

   📄 Generated code:
   ```typescript
   // src/services/notification-service.ts
   // Generated by Crucible - implement methods below

   export class NotificationService {
     /**
      * Send email notification
      * @param to - Email recipient
      * @param subject - Email subject
      * @param body - Email body
      */
     async sendEmail(
       to: string,
       subject: string,
       body: string
     ): Promise<void> {
       // TODO: Implement sendEmail
       throw new Error('Not implemented');
     }

     /**
      * Send SMS notification
      * @param to - Phone number
      * @param message - SMS message
      */
     async sendSMS(to: string, message: string): Promise<void> {
       // TODO: Implement sendSMS
       throw new Error('Not implemented');
     }

     /**
      * Send push notification
      * @param userId - User ID
      * @param title - Notification title
      * @param body - Notification body
      */
     async sendPush(
       userId: string,
       title: string,
       body: string
     ): Promise<void> {
       // TODO: Implement sendPush
       throw new Error('Not implemented');
     }
   }
   ```

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Code generation complete!

   📊 Statistics:
   • Files generated: 1
   • Classes generated: 1
   • Methods generated: 3

   💡 Next steps:
   1. Implement TODO methods
   2. Write tests for NotificationService
   3. Run: npm test
   4. Validate: /crucible:validate
```

### Specific module sync:
```bash
/crucible:sync --module user-service
```

```
🔄 Syncing module: user-service

📋 Analyzing user-service...
   Architecture: .crucible/modules/user-service.json
   Code: src/services/user-service.ts

✨ NEW EXPORTS (2):
   • getUserPreferences() → UserPreferences
   • updatePreferences(userId, prefs) → Promise<void>

Add these exports? [Y/n]: Y

✓ Updated user-service module

✅ Sync complete for user-service
```

### Auto-update mode:
```bash
/crucible:sync --auto-update
```

```
🔄 Auto-sync mode: Applying all changes automatically

⚠️  Warning: All changes will be applied without prompts

Continue? [y/N]: y

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📝 ANALYZING & APPLYING CHANGES

   user-service:
   ✓ Added 3 exports
   ✓ Removed 1 export
   ✓ Added 1 dependency

   patient-service:
   ✓ Added 1 export

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Auto-sync complete!

   2 modules updated
   Run /crucible:validate to verify
```

## Flags

**`--direction <direction>`**
Specify sync direction:
```bash
/crucible:sync --direction code-to-arch     # Code → Architecture (default)
/crucible:sync --direction arch-to-code     # Architecture → Code (generate stubs)
/crucible:sync --direction both             # Bidirectional
```

**`--auto-update`**
Auto-accept all changes:
```bash
/crucible:sync --auto-update
```

⚠️ **Use with caution** - reviews all changes first

**`--dry-run`**
Preview changes without applying:
```bash
/crucible:sync --dry-run
```

**`--interactive`**
Prompt for each change (default):
```bash
/crucible:sync --interactive
```

**`--module <name>`**
Sync specific module only:
```bash
/crucible:sync --module user-service
```

**`--force`**
Force overwrite (dangerous):
```bash
/crucible:sync --direction arch-to-code --force
```

⚠️ **Destructive** - overwrites existing code

## Sync Strategies

### Code → Architecture (Recommended)

**Use when:**
- Code has been implemented/modified
- Architecture needs updating
- Keeping architecture in sync with reality

**Process:**
1. Scan code for exports
2. Compare with module definitions
3. Propose additions/removals
4. Update architecture
5. Validate

**Safety:** High (no code changes)

### Architecture → Code

**Use when:**
- Designing new features (architecture-first)
- Generating boilerplate
- Creating stub implementations

**Process:**
1. Read module definitions
2. Generate code stubs
3. Preserve existing implementations
4. Add TODO comments
5. Validate

**Safety:** Medium (generates files, doesn't overwrite)

### Bidirectional

**Use when:**
- Both architecture and code have changed
- Need to reconcile differences
- Complex sync scenarios

**Process:**
1. Detect conflicts
2. Present both sides
3. Interactive resolution
4. Apply agreed changes
5. Validate

**Safety:** Medium (requires careful review)

## Conflict Resolution

### Export exists in both but signature differs:

```
⚠️  CONFLICT DETECTED

   Export: getUserById
   Module: user-service

   Architecture:
   getUserById(id: string) → User | null

   Code:
   getUserById(id: string) → Promise<User | null>

   Resolution options:
   1. Update architecture to match code (recommended)
   2. Update code to match architecture
   3. Skip this change

   Choose [1-3]: 1

   ✓ Updated architecture to match code
```

### Dependency conflict:

```
⚠️  DEPENDENCY CONFLICT

   Module: auth

   Architecture declares: user, token
   Code imports: user, token, database

   Resolution:
   • database is used but not declared

   Add 'database' to dependencies? [Y/n]: Y

   ✓ Added database to auth dependencies
```

## Error Handling

### No Crucible project:
```
❌ Error: Not a Crucible project

Initialize Crucible first:
   /crucible:init
```

### Code files not found:
```
⚠️  Warning: Implementation files not found

   Module: user-service
   Expected: src/services/user-service.ts

   Options:
   1. Specify correct path
   2. Generate stub from architecture
   3. Skip this module

   Choose [1-3]:
```

### Invalid direction:
```
❌ Error: Invalid sync direction 'xyz'

Valid directions:
   • code-to-arch - Update architecture from code
   • arch-to-code - Generate code from architecture
   • both - Bidirectional sync

Example:
   /crucible:sync --direction code-to-arch
```

### Validation fails after sync:
```
❌ Validation failed after sync

Violations:
   • auth → circular dependency detected
   • user-service → missing type reference

Revert changes? [Y/n]: Y

✓ Changes reverted
✗ Sync aborted due to validation errors

Fix issues and try again:
   /crucible:validate
```

## Best Practices

### When to Sync

**Regularly:**
- After significant code changes
- Before committing architecture
- After merging branches
- Weekly maintenance

**Not Recommended:**
- During active development
- With uncommitted changes
- In production environments

### Safe Workflow

1. **Always dry-run first:**
   ```bash
   /crucible:sync --dry-run
   ```

2. **Review changes carefully:**
   - Check each proposed change
   - Understand impact
   - Verify signatures

3. **Validate after sync:**
   ```bash
   /crucible:sync
   /crucible:validate
   ```

4. **Commit atomically:**
   ```bash
   git add .crucible/
   git commit -m "sync: update architecture from code"
   ```

### Conflict Prevention

- Sync frequently (avoid large drifts)
- Follow architecture-first for new features
- Review PRs for architecture changes
- Use CI/CD to enforce validation

## Examples

**Basic sync (code → architecture)**:
```bash
/crucible:sync
```

**Dry run first**:
```bash
/crucible:sync --dry-run
/crucible:sync
```

**Generate code stubs**:
```bash
/crucible:sync --direction arch-to-code
```

**Sync specific module**:
```bash
/crucible:sync --module auth
```

**Auto-update (careful!)**:
```bash
/crucible:sync --dry-run
/crucible:sync --auto-update
```

**Bidirectional sync**:
```bash
/crucible:sync --direction both --interactive
```

## Integration

**Git Workflow**:
```bash
# 1. Make code changes
git checkout -b feature/user-preferences

# 2. Sync architecture
/crucible:sync --dry-run
/crucible:sync

# 3. Validate
/crucible:validate

# 4. Commit together
git add src/ .crucible/
git commit -m "feat: add user preferences"
```

**CI/CD Check**:
```yaml
# .github/workflows/architecture.yml
- name: Check Architecture Sync
  run: |
    crucible sync --dry-run
    if [ $? -ne 0 ]; then
      echo "Architecture out of sync!"
      exit 1
    fi
```

**Pre-commit Hook**:
```bash
# .git/hooks/pre-commit
#!/bin/bash
crucible sync --dry-run --auto-update
if [ $? -ne 0 ]; then
  echo "Run: crucible sync"
  exit 1
fi
```

## Implementation Notes

- Parse code files to extract exports
- Use TypeScript AST for accurate parsing
- Preserve manual comments in JSON
- Handle multiple file patterns
- Support language-specific parsing (TS, Rust, Python)
- Detect signature changes accurately
- Version bump modules automatically
- Create backup before applying changes
- Rollback on validation failure
- Report detailed statistics
