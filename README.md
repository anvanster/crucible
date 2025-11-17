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

# Validate architecture
crucible validate

# Initialize Claude Code integration
crucible claude init --mode enhanced
```

## Project Structure

- `.crucible/` - Crucible's own architecture definition
- `crucible-core/` - Core library implementation
- `crucible-cli/` - Command-line interface
- `spec/` - Specification documents

## Development

This project uses Crucible to define its own architecture. Before making changes:

1. Update `.crucible/` definitions
2. Validate: `cargo run --bin crucible -- validate --path .crucible`
3. Implement changes
4. Run tests: `cargo test`

## License

Crucible uses dual licensing:

- **Specification** (`spec/`): [CC0 1.0 Universal](spec/LICENSE-SPEC) (Public Domain)
  The Crucible specification, schema, and examples are freely available for anyone to implement.

- **Implementation** (`crucible-core/`, `crucible-cli/`): [Apache License 2.0](LICENSE)
  The Rust implementation code is licensed under Apache 2.0.

See [spec/LICENSING.md](spec/LICENSING.md) for detailed licensing information.
