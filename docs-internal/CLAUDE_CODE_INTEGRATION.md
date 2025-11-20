# Claude Code Integration

## Overview

Crucible now includes native Claude Code integration through slash commands, making architecture-first development workflows seamlessly accessible within Claude Code interface.

## Quick Start

### 1. Initialize Crucible

```bash
crucible init --name my-project
cd my-project
```

This automatically creates:
- `.crucible/` - Architecture definitions
- `.claude/commands/` - 8 slash commands for Claude Code

### 2. Use Slash Commands in Claude Code

Open the project in Claude Code and type `/` to see available Crucible commands:

**Phase 1 (Essential)**:
- `/crucible:validate` - Validate architecture
- `/crucible:architecture` - Design new features
- `/crucible:init` - Initialize Crucible

**Phase 2 (High Value)**:
- `/crucible:module` - Create/update modules
- `/crucible:review` - Comprehensive review

**Phase 3 (Integration)**:
- `/crucible:sync` - Sync architecture ↔ code
- `/crucible:analyze` - Deep module analysis
- `/crucible:diff` - Show differences

## All Commands (Complete)

### `/crucible:validate`

**Purpose**: Run architecture validation and report violations

**Usage**:
```bash
/crucible:validate                          # Validate entire project
/crucible:validate @src/                   # Validate specific path
/crucible:validate --focus security        # Focus on security
/crucible:validate --severity error        # Show only errors
```

**What it does**:
1. Checks for `.crucible/` directory
2. Runs `crucible validate` command
3. Parses and formats violations
4. Provides actionable fixes with file:line references
5. Suggests next steps

**Example output**:
```
✅ Architecture validation passed!

📊 Summary:
   • Modules checked: 15
   • Exports validated: 87
   • Dependencies verified: 34
   • No violations found

🎯 Architecture health: 100/100
```

### `/crucible:architecture`

**Purpose**: Design architecture for a new feature using architecture-first TDD

**Usage**:
```bash
/crucible:architecture "User authentication with JWT tokens"
/crucible:architecture "Payment service" --layer application
/crucible:architecture "Config loader" --language rust
```

**What it does**:
1. Parses feature description
2. Interactive workflow:
   - Suggests module name and layer
   - Identifies dependencies
   - Defines exports with TypeScript types
   - Generates module JSON file
   - Validates against existing architecture
3. Provides TDD guidance:
   - Test file structure
   - Example tests to write
   - Implementation guidance
   - Validation steps

**Example output**:
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

✅ Created: .crucible/modules/auth.json
✅ Validated: 0 violations

📝 Next steps (TDD approach):
  1. Write failing tests for AuthService
  2. Run: npm test (should fail)
  3. Implement AuthService to make tests pass
  4. Run: /crucible:validate
```

### `/crucible:init`

**Purpose**: Initialize Crucible in current project with Claude Code integration

**Usage**:
```bash
/crucible:init                              # Auto-detect project type
/crucible:init --here                       # Initialize in current directory
/crucible:init --name my-project            # Create new project directory
/crucible:init --template rust              # Force Rust template
/crucible:init --examples healthcare        # Use healthcare examples
/crucible:init --no-examples                # Empty project
/crucible:init --here --force               # Reinitialize (requires confirmation)
```

**What it does**:
1. Detects project type (package.json, Cargo.toml, etc.)
2. Creates `.crucible/` structure:
   - `manifest.json` - Project manifest
   - `modules/` - Module definitions directory
   - `CRUCIBLE.md` - Documentation
   - Example modules (user, user-service, database)
3. Creates `.claude/commands/` with all slash commands
4. Runs initial validation
5. Provides next steps guidance

**Example output**:
```
🚀 Initializing Crucible for TypeScript project...

✅ Created .crucible/
✅ Created 3 example modules
✅ Created .claude/commands/ with 3 slash commands

💡 Next steps:
  1. Review .crucible/CRUCIBLE.md
  2. Try /crucible:validate
  3. Design your first feature with /crucible:architecture
```

## How It Works

### Command Generation

When you run `crucible init`, it automatically:

1. **Creates `.claude/commands/` directory**
2. **Generates slash command files** (`.md` files)
3. **Embeds at compile time** using Rust `include_str!` macro

The slash command files contain:
- YAML frontmatter (name, description)
- Detailed instructions for Claude Code
- Output format specifications
- Error handling guidance
- Usage examples

### Command Structure

Each command file follows this pattern:

```markdown
---
name: crucible:validate
description: Run Crucible architecture validation
---

You are helping the user validate their Crucible architecture.

## Context
[Background information about Crucible and validation]

## Command Behavior
[Step-by-step behavior specification]

## Output Format
[Exact format for success/error cases]

## Error Handling
[How to handle common errors]

## Examples
[Usage examples]
```

## Implementation Details

### Code Location

**CLI code**: `crucible-cli/src/main.rs`
- `create_claude_commands()` function
- Called from `init_project()`
- Embeds command files at compile time

**Command files**: `.claude/commands/`
- `crucible-validate.md` (212 lines)
- `crucible-architecture.md` (400+ lines)
- `crucible-init.md` (300+ lines)

### Compilation

```rust
fn create_claude_commands(project_path: &Path) -> Result<()> {
    std::fs::create_dir_all(project_path.join(".claude/commands"))?;

    // Embed command files at compile time
    let crucible_validate = include_str!("../../.claude/commands/crucible-validate.md");
    let crucible_architecture = include_str!("../../.claude/commands/crucible-architecture.md");
    let crucible_init = include_str!("../../.claude/commands/crucible-init.md");

    // Write to project directory
    std::fs::write(
        project_path.join(".claude/commands/crucible-validate.md"),
        crucible_validate,
    )?;
    // ... (more commands)

    Ok(())
}
```

### Claude Code Discovery

1. User types `/` in Claude Code
2. Claude Code scans `.claude/commands/` directory
3. Parses YAML frontmatter from `.md` files
4. Displays commands with `crucible:` prefix
5. When invoked, expands full command prompt

### `/crucible:module`

**Purpose**: Create or update module definitions interactively

**Usage**:
```bash
/crucible:module payment-service --layer application
/crucible:module user-service --update
/crucible:module auth --from-code src/auth/service.ts
```

**What it does**:
1. Interactive workflow for creating modules
2. Auto-detects layer based on naming conventions
3. Suggests dependencies from existing modules
4. Helps define exports with TypeScript types
5. Validates against existing architecture
6. Generates or updates module JSON
7. Supports templates (service, repository, controller, etc.)

### `/crucible:review`

**Purpose**: Comprehensive architecture review with health scoring

**Usage**:
```bash
/crucible:review
/crucible:review --focus security
/crucible:review --report markdown --save reports/review.md
```

**What it does**:
1. Analyzes entire architecture
2. Identifies violations, circular dependencies, layer violations
3. Calculates health score (0-100)
4. Provides security, performance, and quality assessments
5. Generates prioritized action items
6. Supports focused reviews (security, performance, etc.)
7. Tracks trends over time

### `/crucible:sync`

**Purpose**: Sync architecture with code changes bidirectionally

**Usage**:
```bash
/crucible:sync                              # Code → Architecture
/crucible:sync --direction arch-to-code     # Generate code stubs
/crucible:sync --module user-service        # Sync specific module
/crucible:sync --dry-run                    # Preview changes
```

**What it does**:
1. Detects differences between architecture and code
2. Proposes additions/deletions/modifications
3. Interactive resolution with user approval
4. Generates code stubs from architecture
5. Updates module definitions from code
6. Handles conflicts intelligently
7. Validates after syncing

### `/crucible:analyze`

**Purpose**: Deep dive analysis of specific modules

**Usage**:
```bash
/crucible:analyze user-service
/crucible:analyze payment-service --graph
/crucible:analyze auth --usage
/crucible:analyze order-processor --suggest-refactor
```

**What it does**:
1. Analyzes module structure and organization
2. Maps dependencies (direct and transitive)
3. Shows dependent modules and their usage
4. Provides complexity metrics
5. Generates visual dependency graphs
6. Identifies refactoring opportunities
7. Calculates coupling and maintainability scores

### `/crucible:diff`

**Purpose**: Show differences between architecture and code

**Usage**:
```bash
/crucible:diff                              # Check all modules
/crucible:diff user-service                 # Check specific module
/crucible:diff --show-only missing          # Filter diff types
/crucible:diff --format side-by-side        # Visual comparison
```

**What it does**:
1. Compares module definitions with actual code
2. Shows git-style unified diff
3. Identifies missing exports, signature mismatches
4. Detects undeclared dependencies
5. Calculates drift score
6. Supports multiple output formats (unified, side-by-side, JSON)
7. Provides fix suggestions

## Benefits

### For Developers

- **Integrated workflow**: No context switching between tools
- **Interactive guidance**: Step-by-step architecture design
- **Immediate validation**: Fast feedback on violations
- **TDD support**: Built-in test-first guidance
- **Type-aware**: Full TypeScript type system support

### For Claude Code

- **Native integration**: Slash commands feel natural
- **Contextual help**: Rich inline documentation
- **Actionable output**: File:line references for quick fixes
- **Chainable**: Commands work well together

### For Architecture

- **Enforced standards**: Validation prevents drift
- **Design-first**: Architecture before code
- **Documentation**: Self-documenting architecture
- **Collaboration**: Version-controlled in `.crucible/`

## Usage Patterns

### Pattern 1: New Feature

```bash
# 1. Design architecture
/crucible:architecture "User authentication with JWT"

# 2. Write tests (TDD)
# [Write failing tests based on architecture]

# 3. Implement
# [Implement feature to make tests pass]

# 4. Validate
/crucible:validate
```

### Pattern 2: Fix Violations

```bash
# 1. Run validation
/crucible:validate

# 2. Fix violations
# [Edit code or architecture based on suggestions]

# 3. Re-validate
/crucible:validate
```

### Pattern 3: Code Review

```bash
# 1. Comprehensive review
/crucible:review

# 2. Focus on security
/crucible:review --focus security

# 3. Address issues
# [Fix based on recommendations]

# 4. Validate again
/crucible:validate
```

## Future Enhancements

### Short-term (Next Release)

- Enhanced TypeScript type inference from code
- Auto-fix suggestions for common violations
- Interactive visual dependency graphs (web UI)
- Real-time validation feedback

### Medium-term

- Multi-language code generation (Rust, Python, Go)
- CI/CD integration examples and templates
- VSCode extension with inline validation
- Architecture pattern library and templates

### Long-term

- Real-time validation in editor as you type
- AI-powered architecture suggestions and refactoring
- Team collaboration features (architecture reviews, comments)
- Architecture evolution tracking and analytics

## Testing

### Manual Testing

```bash
# 1. Create test project
crucible init --name test-project
cd test-project

# 2. Verify commands created
ls .claude/commands/

# 3. Test in Claude Code
# Open project in Claude Code and try:
/crucible:validate
/crucible:architecture "Test feature"
```

### Integration Testing

The slash commands are tested as part of the healthcare demo validation:
- 38 modules, 0 violations
- Tests TypeScript type system
- Validates all command workflows

## Documentation

- **Implementation Plan**: `docs/SLASH_COMMANDS_IMPLEMENTATION_PLAN.md`
- **This Guide**: `docs/CLAUDE_CODE_INTEGRATION.md`
- **TypeScript Support**: `docs/TYPESCRIPT_TYPE_SYSTEM.md`
- **Roadmap**: `docs/IMPLEMENTATION_ROADMAP.md`

## Contributing

To add new slash commands:

1. Create `.claude/commands/crucible-<name>.md` file
2. Follow existing command structure
3. Add to `create_claude_commands()` in `main.rs`
4. Test with `crucible init`
5. Update documentation

## Feedback

Share your experience:
- GitHub Issues: https://github.com/anvanster/crucible/issues
- Discussions: https://github.com/anvanster/crucible/discussions

## Version

**Current Version**: v0.1.5 (pending release)
**Integration Status**: All Phases Complete (8 commands)
**Last Updated**: November 17, 2025

### Changelog

**v0.1.5** (Pending):
- ✅ Phase 1 Complete: validate, architecture, init
- ✅ Phase 2 Complete: module, review
- ✅ Phase 3 Complete: sync, analyze, diff
- ✅ All 8 commands auto-generated on `crucible init`
- ✅ Comprehensive documentation for all commands
