#!/bin/bash
# Test code generation manually using the generator module

set -e

echo "Testing TypeScript code generation..."
echo

# Create a test Rust program to use the generator
cat > /tmp/test_gen.rs << 'EOF'
use crucible_core::{Parser, Generator};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the simple-app example
    let parser = Parser::new("spec/examples/simple-app");
    let project = parser.parse_project()?;

    println!("Parsed {} modules", project.modules.len());

    // Generate TypeScript
    let generator = Generator::new(project);
    let output_dir = Path::new("/tmp/generated-simple");

    std::fs::create_dir_all(output_dir)?;
    generator.generate_typescript(output_dir)?;

    println!("\n✅ Generated TypeScript files to /tmp/generated-simple/");
    println!("\nGenerated files:");
    for entry in std::fs::read_dir(output_dir)? {
        let entry = entry?;
        println!("  - {}", entry.file_name().to_string_lossy());
    }

    Ok(())
}
EOF

# Compile and run
cd /home/user/crucible
rustc --edition 2021 \
  -L target/debug/deps \
  --extern crucible_core=target/debug/libcrucible_core.rlib \
  --extern serde=target/debug/deps/libserde-*.rlib \
  --extern serde_json=target/debug/deps/libserde_json-*.rlib \
  /tmp/test_gen.rs -o /tmp/test_gen 2>/dev/null || {
    echo "Note: Direct compilation failed, using cargo instead"

    # Alternative: create a temp cargo project
    mkdir -p /tmp/gen-test
    cd /tmp/gen-test

    cat > Cargo.toml << 'CARGO_EOF'
[package]
name = "gen-test"
version = "0.1.0"
edition = "2021"

[dependencies]
crucible-core = { path = "/home/user/crucible/crucible-core" }
CARGO_EOF

    mkdir -p src
    mv /tmp/test_gen.rs src/main.rs

    cargo run --quiet
}

# Show generated content
echo
echo "Generated greeter.ts:"
echo "===================="
cat /tmp/generated-simple/greeter.ts || echo "File not found"

echo
echo
echo "Generated logger.ts:"
echo "==================="
cat /tmp/generated-simple/logger.ts || echo "File not found"
