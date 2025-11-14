# Crucible

**An open standard for AI-native application architecture**

## Quick Start

```bash
# Build the project
cargo build

# Validate Crucible's own architecture
cargo run --bin crucible -- validate --path .crucible

# Create a new project
cargo run --bin crucible -- init --name my-app
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

- Specification: CC0 1.0 Universal (Public Domain)
- Implementation: Apache License 2.0
