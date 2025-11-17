# Crucible

**Architecture-first development framework for AI-native applications**

Crucible helps teams maintain architectural integrity through formal architecture definitions, automated validation, and seamless Claude Code integration.

## Installation

### Option 1: Using cargo-binstall (Recommended - Fast)

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) installed:

```bash
cargo binstall crucible-cli
```

This downloads prebuilt binaries from GitHub releases, making installation much faster than compiling from source.

Don't have cargo-binstall? Install it with:
```bash
cargo install cargo-binstall
```

### Option 2: From crates.io (Standard)

```bash
cargo install crucible-cli
```

This compiles from source and may take a few minutes.

### Option 3: From GitHub (Latest Development)

```bash
cargo install --git https://github.com/anvanster/crucible.git crucible-cli
```

This installs the latest unreleased version from the main branch.

### Verify Installation

```bash
crucible --version
```

## Quick Start

```bash
# Create a new project with architecture
crucible init --name my-app

# Or initialize in an existing project
cd my-existing-project
crucible init --here

# Reinitialize existing project (requires confirmation)
crucible init --here --force

# Validate architecture
crucible validate

# Initialize Claude Code integration (optional - init does this automatically)
crucible claude init --mode enhanced
```

## Claude Code Integration

Crucible includes 8 native slash commands for Claude Code, automatically generated on `crucible init`:

**Essential Commands**:
- `/crucible:validate` - Run architecture validation with actionable fixes
- `/crucible:architecture` - Design architecture for new features (architecture-first TDD)
- `/crucible:init` - Initialize Crucible in current project

**Module Management**:
- `/crucible:module` - Create or update module definitions interactively
- `/crucible:review` - Comprehensive architecture review with health scoring

**Sync & Analysis**:
- `/crucible:sync` - Sync architecture ↔ code bidirectionally
- `/crucible:analyze` - Deep dive into module dependencies and usage
- `/crucible:diff` - Show git-style differences between architecture and code

### Example Workflow

```bash
# 1. Initialize project (auto-creates slash commands)
crucible init --name healthcare-app

# 2. In Claude Code, design architecture
/crucible:architecture "Patient management service"

# 3. Write tests (TDD approach)
# [Write failing tests based on architecture]

# 4. Implement feature
# [Implement to make tests pass]

# 5. Validate compliance
/crucible:validate

# 6. Keep architecture in sync
/crucible:sync
```

See [docs/CLAUDE_CODE_INTEGRATION.md](docs/CLAUDE_CODE_INTEGRATION.md) for complete documentation.

## Project Structure

- `.crucible/` - Crucible's own architecture definition
- `crucible-core/` - Core library implementation
- `crucible-cli/` - Command-line interface
- `spec/` - Specification documents

## Development

This project uses Crucible to define its own architecture. Before making changes:

1. Update `crucible-core/.crucible/` definitions
2. Validate: `crucible validate --path crucible-core/.crucible --strict`
3. Implement changes
4. Run tests: `cargo test --all`

### Pre-push Checks

Before pushing to main, run all CI checks locally:

```bash
./scripts/pre-push.sh
```

This runs:
- Code formatting check (`cargo fmt`)
- Clippy lints (`cargo clippy`)
- Release build
- Architecture validation
- All tests
- Documentation check

See [scripts/README.md](scripts/README.md) for setup as a Git hook.

## License

Crucible uses dual licensing:

- **Specification** (`spec/`): [CC0 1.0 Universal](spec/LICENSE-SPEC) (Public Domain)
  The Crucible specification, schema, and examples are freely available for anyone to implement.

- **Implementation** (`crucible-core/`, `crucible-cli/`): [Apache License 2.0](LICENSE)
  The Rust implementation code is licensed under Apache 2.0.

See [spec/LICENSING.md](spec/LICENSING.md) for detailed licensing information.
