---
name: crucible:init
description: Initialize Crucible in current project with Claude Code integration
---

You are helping the user initialize Crucible in their project.

## Context

Crucible is an architecture-first development framework. This command sets up the complete Crucible environment including:
- Architecture definition structure (`.crucible/` directory)
- Claude Code slash commands (`.claude/commands/` directory)
- Example modules
- Documentation

## Command Behavior

1. **Parse arguments**:
   - `--name <project>` - Create new project directory (mutually exclusive with --here)
   - `--here` - Initialize in current directory (mutually exclusive with --name)
   - `--template <lang>` - Force specific template (typescript, rust, python, go)
   - `--examples <level>` - Example complexity (minimal, standard, healthcare)
   - `--force` - Overwrite existing without prompting
   - `--claude-code` - Include Claude Code commands (default: true)
   - `--no-examples` - Skip example modules

2. **Detect project type**:
   - Check for `package.json` (TypeScript/JavaScript)
   - Check for `Cargo.toml` (Rust)
   - Check for `pyproject.toml` or `setup.py` (Python)
   - Check for `go.mod` (Go)
   - Default to TypeScript if ambiguous

3. **Check for existing Crucible**:
   - If `.crucible/` exists, ask to overwrite or skip
   - If `.claude/commands/crucible-*.md` exist, ask to update

4. **Run initialization**:
   ```bash
   crucible init --name my-project [--template typescript] [--examples standard]
   # OR
   crucible init --here [--template typescript] [--examples standard]
   ```

5. **Setup Claude Code integration**:
   - Create `.claude/commands/` directory
   - Generate all Crucible slash commands:
     - `crucible-validate.md`
     - `crucible-architecture.md`
     - `crucible-init.md` (this command)
     - `crucible-module.md`
     - `crucible-review.md`
     - `crucible-sync.md`
     - `crucible-analyze.md`
     - `crucible-diff.md`

6. **Provide guidance**:
   - Show directory structure created
   - List example modules
   - Suggest next steps
   - Recommend running validation

## Output Format

### Initial detection:
```
🔍 Detecting project type...

Found: package.json
✓ Detected TypeScript project

📦 Project info:
   • Name: my-app
   • Language: TypeScript
   • Framework: React + Node.js
```

### Confirmation prompt (if .crucible exists):
```
⚠️ Warning: .crucible/ directory already exists

Options:
   1. Merge with existing (safe, recommended)
   2. Overwrite completely (destructive)
   3. Skip initialization

Choose option [1-3]: [1]
```

### Initialization progress:
```
🚀 Initializing Crucible for TypeScript project...

📁 Creating directory structure...
   ✓ Created .crucible/
   ✓ Created .crucible/modules/
   ✓ Created .claude/commands/

📝 Generating files...
   ✓ .crucible/manifest.json
   ✓ .crucible/CRUCIBLE.md (documentation)
   ✓ .crucible/modules/user.json (example)
   ✓ .crucible/modules/user-service.json (example)
   ✓ .crucible/modules/database.json (example)

🔧 Installing Claude Code commands...
   ✓ .claude/commands/crucible-validate.md
   ✓ .claude/commands/crucible-architecture.md
   ✓ .claude/commands/crucible-init.md
   ✓ .claude/commands/crucible-module.md
   ✓ .claude/commands/crucible-review.md
   ✓ .claude/commands/crucible-sync.md
   ✓ .claude/commands/crucible-analyze.md
   ✓ .claude/commands/crucible-diff.md

🔍 Running initial validation...
   ✓ Validation passed: 0 violations

✅ Crucible initialized successfully!

📚 What was created:

.crucible/
├── manifest.json           # Project manifest
├── CRUCIBLE.md            # Documentation and guide
└── modules/               # Architecture definitions
    ├── user.json          # Domain: User entity
    ├── user-service.json  # Application: User service
    └── database.json      # Infrastructure: Database

.claude/commands/
├── crucible-validate.md      # Validate architecture
├── crucible-architecture.md  # Design new features
├── crucible-init.md          # Initialize Crucible
├── crucible-module.md        # Create/update modules
├── crucible-review.md        # Architecture review
├── crucible-sync.md          # Sync arch ↔ code
├── crucible-analyze.md       # Analyze modules
└── crucible-diff.md          # Show differences

📖 Example modules included:

1. user (domain layer)
   • User type with id, username, email
   • Business logic for user domain

2. user-service (application layer)
   • UserService class
   • CRUD operations: create, get, update, delete
   • Uses: user (domain), database (infrastructure)

3. database (infrastructure layer)
   • Connection type
   • Query execution interface

💡 Next steps:

1. **Review documentation**:
   cat .crucible/CRUCIBLE.md

2. **Try Claude Code commands**:
   Type '/' in Claude Code and look for crucible: commands

3. **Customize example modules**:
   Edit .crucible/modules/*.json to match your project

4. **Design your first feature**:
   /crucible:architecture "your feature description"

5. **Validate architecture**:
   /crucible:validate

🎯 Quick start commands:

   /crucible:validate          # Check current architecture
   /crucible:architecture      # Design a new feature
   /crucible:module            # Create a module
   /crucible:review            # Comprehensive review

📚 Learn more:
   • Architecture-first TDD: .crucible/CRUCIBLE.md
   • TypeScript type system: .crucible/CRUCIBLE.md#typescript
   • Layer architecture: .crucible/CRUCIBLE.md#layers
   • Best practices: .crucible/CRUCIBLE.md#best-practices

🎉 Happy architecture-first development!
```

### With template flag:
```bash
/crucible:init --template rust --examples minimal
```

```
🚀 Initializing Crucible for Rust project...

📝 Using template: rust
📦 Examples: minimal (1 module)

✓ Created .crucible/
✓ Created .crucible/modules/config.rs.json

✅ Initialization complete!

📖 Example module:
   • config (infrastructure layer)
   • Rust-specific: Result<T, E>, Option<T>

💡 Next: /crucible:architecture "your feature"
```

### Healthcare examples:
```bash
/crucible:init --examples healthcare
```

```
🚀 Initializing Crucible with healthcare examples...

✓ Created 12 modules:
   Domain (5):
   • patient, appointment, provider, medication, insurance

   Application (4):
   • patient-service, appointment-service, billing-service, notification-service

   Infrastructure (3):
   • database, email-service, payment-gateway

💡 This is a complete example of a healthcare management system.
   Explore modules: ls .crucible/modules/
```

## Flags

**`--template <typescript|rust|python|go>`**
Force specific language template:
```bash
/crucible:init --template rust
```

**`--examples <minimal|standard|healthcare>`**
Control example complexity:
- `minimal` - 1 simple module
- `standard` - 3 modules (domain, application, infrastructure) [default]
- `healthcare` - 12 modules (complete example system)

```bash
/crucible:init --examples healthcare
```

**`--force`**
Overwrite existing .crucible/ directory with confirmation prompt:
```bash
/crucible:init --force
```
When --force is used with an existing .crucible/ directory, you will be prompted to confirm:
- Type 'yes' to proceed with deletion and reinitialization
- Type anything else to cancel the operation
- Existing architecture is preserved if cancelled

**`--no-examples`**
Skip example modules (empty project):
```bash
/crucible:init --no-examples
```

**`--no-claude-code`**
Skip Claude Code commands:
```bash
/crucible:init --no-claude-code
```

## Error Handling

### Crucible not installed:
```
❌ Error: 'crucible' command not found

📦 Install Crucible:
   cargo install crucible-cli

Verify installation:
   crucible --version
```

### Already initialized (without --force):
```
❌ Error: .crucible/ directory already exists in current directory

Options:
  1. Use --force to overwrite
  2. Remove existing .crucible/ directory first
  3. Initialize in a different directory

Example:
  crucible init --here --force
```

### Already initialized (with --force):
```
⚠️  Warning: Replacing existing .crucible/ directory
  ⚠️  This will delete all existing architecture definitions!

  Type 'yes' to continue: yes

✓ Proceeding with reinitialization...
```
If you type anything other than 'yes', the operation is cancelled and existing architecture is preserved.

### No write permissions:
```
❌ Error: Permission denied

Cannot create .crucible/ directory.

Check permissions:
   ls -la .

Fix:
   chmod +w .
```

### Invalid template:
```
❌ Error: Unknown template 'xyz'

Valid templates:
   • typescript - TypeScript/JavaScript projects
   • rust - Rust projects
   • python - Python projects
   • go - Go projects

Example:
   /crucible:init --template typescript
```

## What Gets Created

### .crucible/ directory:
```
.crucible/
├── manifest.json          # Project manifest with metadata
├── CRUCIBLE.md           # Comprehensive documentation
└── modules/              # Module definitions
    ├── <module1>.json
    ├── <module2>.json
    └── ...
```

### .claude/commands/ directory:
```
.claude/commands/
├── crucible-validate.md      # Run validation
├── crucible-architecture.md  # Design architecture
├── crucible-init.md          # Initialize project
├── crucible-module.md        # Create/update modules
├── crucible-review.md        # Comprehensive review
├── crucible-sync.md          # Sync architecture
├── crucible-analyze.md       # Analyze modules
└── crucible-diff.md          # Show differences
```

### manifest.json:
```json
{
  "project": "my-app",
  "version": "1.0.0",
  "language": "typescript",
  "modules": [
    ".crucible/modules/user.json",
    ".crucible/modules/user-service.json",
    ".crucible/modules/database.json"
  ]
}
```

### CRUCIBLE.md:
- Getting started guide
- Architecture-first TDD explanation
- TypeScript type system features
- Layer architecture principles
- Best practices
- Example workflows
- Common patterns
- Troubleshooting

## Implementation Notes

- Run `crucible init` CLI command via Bash tool
- Parse output and format for readability
- Verify all files created successfully
- Run initial validation to ensure setup is correct
- Provide contextual next steps based on project type
- Link to documentation throughout
- Show clickable file paths
- Highlight Claude Code integration prominently
- Suggest trying slash commands immediately

## Integration with Other Commands

After initialization:
- Suggest running `/crucible:validate` first
- Recommend `/crucible:architecture` for first feature
- Point to `/crucible:review` for learning existing structure
- Use `/crucible:module` to customize examples

## Post-Initialization Checklist

Present this checklist after successful initialization:

```
📋 Post-initialization checklist:

   ☐ Review .crucible/CRUCIBLE.md documentation
   ☐ Understand the 3 example modules
   ☐ Run /crucible:validate to see it work
   ☐ Customize or delete example modules
   ☐ Design your first real feature with /crucible:architecture
   ☐ Write tests before implementation (TDD)
   ☐ Implement feature
   ☐ Validate with /crucible:validate
   ☐ Commit .crucible/ to version control

💡 Pro tip: Add .crucible/ to your git commits!
   This ensures your architecture is versioned alongside code.

   git add .crucible/ .claude/
   git commit -m "feat: Initialize Crucible architecture framework"
```

## Examples

### Create new project:
```bash
/crucible:init --name my-project
```

### Initialize in existing project:
```bash
/crucible:init --here
```

### Rust project in current directory:
```bash
/crucible:init --here --template rust
```

### New project with minimal examples:
```bash
/crucible:init --name my-app --examples minimal
```

### Healthcare example (learning):
```bash
/crucible:init --name healthcare-demo --examples healthcare
```

### Empty project (no examples):
```bash
/crucible:init --here --no-examples
```

### Force overwrite existing:
```bash
/crucible:init --here --force
```

### Skip Claude Code integration:
```bash
/crucible:init --name my-app --no-claude-code
```
