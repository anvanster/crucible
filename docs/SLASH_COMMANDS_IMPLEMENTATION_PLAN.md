# Crucible Slash Commands Implementation Plan

## Overview

Integrate Crucible validation and architecture-first workflows directly into Claude Code via slash commands.

**Goal**: Make Crucible a native part of the Claude Code development experience.

---

## Architecture

### Command Structure

```
.claude/
└── commands/
    ├── crucible-validate.md
    ├── crucible-init.md
    ├── crucible-architecture.md
    ├── crucible-module.md
    ├── crucible-review.md
    ├── crucible-sync.md
    ├── crucible-analyze.md
    └── crucible-diff.md
```

### Command Design Principles

1. **Auto-Detection**: Check for `.crucible/` directory before running
2. **Contextual Output**: Provide structured output Claude Code can act on
3. **Chainable**: Commands should work well with other Claude Code workflows
4. **Flag Support**: Customizable behavior via flags
5. **Error Handling**: Clear error messages with actionable guidance
6. **Performance**: Fast execution (<5s for most commands)

---

## Phase 1: Essential Commands (Week 1)

### 1. `/crucible:validate`

**Purpose**: Run architecture validation and report violations

**Usage**:
```bash
/crucible:validate
/crucible:validate @src/
/crucible:validate --focus security
/crucible:validate --severity error
```

**Implementation**:
- Check for `.crucible/` directory
- Run `crucible validate` command
- Parse output and format for Claude Code
- Provide file:line references for violations
- Suggest fixes based on violation type

**Output Format**:
```
✅ Validation passed: 0 violations
  or
❌ Found 5 violations:
  • src/user-service.ts:45 → Missing export 'UserService' in module definition
  • src/auth.ts:23 → Undeclared dependency on 'database' module

💡 Suggested actions:
  1. Add UserService to user-service module exports
  2. Declare database dependency in auth module
```

**Flags**:
- `--path <path>`: Validate specific module/directory
- `--focus security|performance|dependencies`: Focus on specific concerns
- `--severity error|warning|info`: Filter by severity level
- `--json`: Output raw JSON for programmatic use

### 2. `/crucible:architecture`

**Purpose**: Design architecture for a new feature using architecture-first TDD

**Usage**:
```bash
/crucible:architecture "User authentication with JWT tokens"
/crucible:architecture "Payment processing service" --layer application
```

**Implementation**:
- Interactive workflow:
  1. Ask about feature requirements
  2. Suggest module name and layer
  3. Identify dependencies on existing modules
  4. Define exports (types, functions, classes)
  5. Generate module JSON file
  6. Validate against existing architecture
  7. Provide TDD guidance (test-first approach)

**Output Format**:
```
🏗️ Designing architecture for: User authentication with JWT tokens

📋 Module: auth
📂 Layer: application
🔗 Dependencies: user (domain), token (infrastructure)

📦 Exports:
  • AuthService (class)
    - login(username, password) → Promise<user.User>
    - logout(userId) → Promise<void>
    - validateToken(token) → Promise<boolean>
  • AuthToken (type)

✅ Created: .crucible/modules/auth.json
✅ Validated: 0 violations

📝 Next steps (TDD approach):
  1. Write failing tests for AuthService
  2. Run: npm test (should fail)
  3. Implement AuthService to make tests pass
  4. Run: /crucible:validate
```

**Flags**:
- `--layer domain|application|infrastructure`: Specify layer
- `--depends <module>`: Pre-specify dependencies
- `--template <type>`: Use template (service, repository, controller)
- `--language typescript|rust|python|go`: Target language

### 3. `/crucible:init`

**Purpose**: Initialize Crucible in current project

**Usage**:
```bash
/crucible:init
/crucible:init --template typescript
/crucible:init --examples healthcare
```

**Implementation**:
- Detect project type (package.json, Cargo.toml, etc.)
- Create `.crucible/` structure:
  - `modules/` directory
  - `manifest.json`
  - `CRUCIBLE.md` README
- Generate example modules based on template
- Run initial validation

**Output Format**:
```
🚀 Initializing Crucible for TypeScript project...

✅ Created .crucible/ directory structure
✅ Generated manifest.json
✅ Created 3 example modules:
   • user (domain)
   • user-service (application)
   • database (infrastructure)
✅ Generated CRUCIBLE.md documentation

📚 Next steps:
  1. Review .crucible/CRUCIBLE.md for TypeScript patterns
  2. Customize example modules or create your own
  3. Run: /crucible:validate to check architecture
  4. Start building with: /crucible:architecture <feature>
```

**Flags**:
- `--template typescript|rust|python|go`: Project template
- `--examples minimal|healthcare|ecommerce`: Example complexity
- `--force`: Overwrite existing `.crucible/`

---

## Phase 2: High Value Commands (Week 2)

### 4. `/crucible:module`

**Purpose**: Create or update a module definition

**Usage**:
```bash
/crucible:module payment-service --layer application
/crucible:module user --update
```

**Implementation**:
- Interactive prompts for module properties
- Validate module name (kebab-case)
- Suggest layer based on naming conventions
- Auto-detect dependencies from code imports
- Generate or update module JSON
- Run validation

**Output Format**:
```
📦 Creating module: payment-service

🔍 Auto-detected:
   • Layer: application (based on '-service' suffix)
   • Dependencies: payment (domain), stripe (infrastructure)
   • Language: TypeScript

📝 Define exports:
   1. PaymentService (class)
   2. ProcessPaymentRequest (type)
   3. PaymentStatus (enum)

✅ Created: .crucible/modules/payment-service.json
✅ Validated: 0 violations
```

**Flags**:
- `--layer <layer>`: Specify layer
- `--depends <modules>`: Comma-separated dependencies
- `--update`: Update existing module
- `--from-code <path>`: Generate from existing code

### 5. `/crucible:review`

**Purpose**: Comprehensive architecture review

**Usage**:
```bash
/crucible:review
/crucible:review --focus security
/crucible:review --report markdown
```

**Implementation**:
- Run validation across all modules
- Check for architectural patterns:
  - Circular dependencies
  - Layer violations
  - Missing exports
  - Unused modules
  - Security concerns
  - Performance anti-patterns
- Generate comprehensive report
- Provide prioritized recommendations

**Output Format**:
```
🔍 Architecture Review Report

📊 Summary:
   • 15 modules analyzed
   • 3 violations found
   • 2 warnings
   • Architecture health: 87/100

🚨 Critical Issues:
   ❌ Circular dependency: auth ↔ user-service
   ❌ Layer violation: domain/user depends on infrastructure/database

⚠️ Warnings:
   • payment module has no dependents (unused?)
   • 5 modules missing version field

💡 Recommendations (by priority):
   1. Break circular dependency (auth ↔ user-service)
   2. Move database access to repository layer
   3. Consider deprecating unused payment module
   4. Add version fields to modules

📈 Trends:
   • Average dependencies per module: 2.4
   • Most depended-on module: user (8 dependents)
   • Deepest dependency chain: 4 levels
```

**Flags**:
- `--focus security|performance|dependencies|layering`: Focus area
- `--report json|markdown|html`: Report format
- `--threshold <score>`: Minimum acceptable health score
- `--save <path>`: Save report to file

---

## Phase 3: Integration Commands (Week 3)

### 6. `/crucible:sync`

**Purpose**: Sync architecture with code changes

**Usage**:
```bash
/crucible:sync
/crucible:sync --direction code-to-arch
/crucible:sync --auto-update
```

**Implementation**:
- Detect changes in code vs architecture
- Analyze:
  - New files/exports not in architecture
  - Removed code that's in architecture
  - Changed signatures
  - New dependencies
- Interactive prompts for resolution
- Update module definitions
- Preserve manual annotations/comments

**Output Format**:
```
🔄 Syncing architecture with code...

📋 Detected changes:
   ✨ New exports in user-service.ts:
      • getUserPreferences() → UserPreferences
      • updatePreferences(userId, prefs) → Promise<void>

   🗑️ Removed from auth.ts:
      • legacyLogin() [marked deprecated 3 months ago]

   🔗 New dependency detected:
      • user-service now imports from preferences module

❓ Actions needed:
   1. Add getUserPreferences to user-service module? [Y/n] Y
   2. Add updatePreferences to user-service module? [Y/n] Y
   3. Remove legacyLogin from auth module? [Y/n] Y
   4. Add preferences to user-service dependencies? [Y/n] Y

✅ Updated 2 module definitions
✅ Validation passed: 0 violations
```

**Flags**:
- `--direction code-to-arch|arch-to-code|both`: Sync direction
- `--auto-update`: Auto-accept all changes (use with caution)
- `--dry-run`: Show what would change without applying
- `--interactive`: Prompt for each change (default)

### 7. `/crucible:analyze`

**Purpose**: Deep dive into specific module

**Usage**:
```bash
/crucible:analyze user-service
/crucible:analyze user-service --graph
```

**Implementation**:
- Load module definition
- Analyze:
  - Dependencies (direct and transitive)
  - Dependents (who uses this module)
  - Exports and their usage
  - Layer compliance
  - Complexity metrics
- Generate visualization data
- Identify refactoring opportunities

**Output Format**:
```
🔍 Analyzing module: user-service

📦 Basic Info:
   • Layer: application
   • Version: 1.2.0
   • File: src/services/user-service.ts

🔗 Dependencies (3 direct, 5 transitive):
   Direct:
   • user (domain) → types and business logic
   • database (infrastructure) → data persistence
   • auth (application) → authentication

   Transitive:
   • connection-pool ← database
   • validation ← user
   • ...

👥 Dependents (6 modules depend on this):
   • api-gateway (uses: UserService)
   • admin-panel (uses: UserService, createUser)
   • notification-service (uses: getUserById)
   • ...

📊 Metrics:
   • Exports: 8 functions, 4 types
   • Most used export: getUserById (6 references)
   • Least used export: updateUserMetadata (1 reference)
   • Dependency depth: 3 levels

💡 Suggestions:
   • Consider splitting: high cohesion with user-preferences
   • Refactor: direct database dependency violates clean architecture
   • Optimize: updateUserMetadata is rarely used, consider deprecating
```

**Flags**:
- `--graph`: Generate dependency graph visualization
- `--depth <n>`: Transitive dependency depth (default: 3)
- `--usage`: Show export usage statistics
- `--suggest-refactor`: AI-powered refactoring suggestions

### 8. `/crucible:diff`

**Purpose**: Show differences between architecture and code

**Usage**:
```bash
/crucible:diff
/crucible:diff user-service
/crucible:diff --show-only missing
```

**Implementation**:
- Compare module definitions with actual code
- Identify:
  - Missing exports (in code, not in architecture)
  - Undeclared exports (in architecture, not in code)
  - Signature mismatches
  - Dependency drift
- Present as git-style diff

**Output Format**:
```
📝 Architecture vs Code Diff

Module: user-service

Exports:
+ getUserPreferences() → UserPreferences [MISSING IN ARCHITECTURE]
- legacyLogin(username, password) [MISSING IN CODE]
~ updateUser(id, updates) [SIGNATURE MISMATCH]
    Architecture: updateUser(id: string, updates: object) → User
    Code:         updateUser(id: string, updates: Partial<User>) → Promise<User>

Dependencies:
+ preferences [UNDECLARED IN ARCHITECTURE]

💡 Quick fixes:
  1. Run: /crucible:sync to auto-update architecture
  2. Or manually update: .crucible/modules/user-service.json
```

**Flags**:
- `--module <name>`: Check specific module
- `--show-only missing|extra|mismatch`: Filter diff types
- `--format unified|side-by-side`: Diff format

---

## Implementation Strategy

### Technical Approach

1. **Command Files**: Markdown files in `.claude/commands/` directory
   - Each file defines one slash command
   - Contains prompt that Claude Code expands

2. **Command Structure**:
   ```markdown
   ---
   name: crucible-validate
   description: Run Crucible architecture validation
   ---

   You are helping the user validate their Crucible architecture.

   [Detailed instructions for Claude Code...]
   ```

3. **Crucible Integration**:
   - Commands invoke `crucible` CLI via Bash tool
   - Parse JSON output from Crucible
   - Format for human-readable display
   - Provide contextual guidance

4. **State Management**:
   - Commands are stateless (each invocation is independent)
   - Use `.crucible/` directory as single source of truth
   - No session state needed

### Error Handling

**Common Errors**:
1. **No Crucible project**: `Error: .crucible/ directory not found. Run /crucible:init first.`
2. **Invalid module**: `Error: Module 'xyz' not found in manifest.`
3. **Validation failed**: Show violations with actionable fixes
4. **Crucible not installed**: `Error: crucible command not found. Install: cargo install crucible-cli`

### Testing Strategy

**Manual Testing**:
1. Test each command in isolation
2. Test command chaining workflows
3. Test error scenarios
4. Test with real Crucible projects (healthcare demo)

**Integration Testing**:
1. Verify commands work in Claude Code interface
2. Test flag combinations
3. Test interactive prompts
4. Validate output formatting

---

## Documentation

### User Documentation

1. **README Update**: Add "Claude Code Integration" section
2. **Command Reference**: Document all flags and usage patterns
3. **Workflow Examples**: Show common command sequences
4. **Video Tutorial**: Screen recording of slash commands in action

### Developer Documentation

1. **Command Development Guide**: How to add new commands
2. **Testing Guide**: How to test slash commands
3. **Architecture Decision Records**: Why certain design choices

---

## Success Metrics

### Adoption Metrics
- Number of slash command invocations
- Most used commands
- User retention (weekly active users)

### Quality Metrics
- Command success rate (>95%)
- Average response time (<5s)
- User satisfaction (survey)

### Impact Metrics
- Reduction in architecture violations
- Time to validate (vs manual)
- Developer productivity (self-reported)

---

## Timeline

### Week 1: Essential Commands
- Day 1-2: `/crucible:validate` implementation and testing
- Day 3-4: `/crucible:architecture` implementation and testing
- Day 5: `/crucible:init` implementation and testing
- Day 6-7: Integration testing, documentation

### Week 2: High Value Commands
- Day 1-2: `/crucible:module` implementation and testing
- Day 3-4: `/crucible:review` implementation and testing
- Day 5-7: Integration testing, user feedback, iterations

### Week 3: Integration Commands
- Day 1-2: `/crucible:sync` implementation
- Day 3: `/crucible:analyze` implementation
- Day 4: `/crucible:diff` implementation
- Day 5-7: Final testing, documentation, release

---

## Next Steps

1. Create `.claude/commands/` directory structure
2. Implement Phase 1 commands (validate, architecture, init)
3. Test with healthcare demo project
4. Gather user feedback
5. Iterate and refine
6. Implement Phase 2 and Phase 3 commands
7. Document and release

---

## Open Questions

1. **Command Naming**: `/crucible:validate` vs `/validate:crucible` vs `/crucible-validate`?
   - **Decision**: Use `/crucible:validate` (colon separator, namespace prefix)

2. **Interactive vs Non-Interactive**: How much interactivity?
   - **Decision**: Default to interactive with `--yes` flag for automation

3. **Output Verbosity**: How detailed should output be?
   - **Decision**: Balanced by default, `--verbose` for details, `--quiet` for minimal

4. **Integration with CI/CD**: Should commands support CI mode?
   - **Decision**: Yes, add `--ci` flag for non-interactive, machine-readable output

5. **Telemetry**: Collect usage metrics?
   - **Decision**: Opt-in telemetry with clear privacy policy
