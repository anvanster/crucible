# Development Scripts

Helper scripts for Crucible development.

## pre-push.sh

Runs all CI checks locally before pushing to main. This helps catch issues early and ensures your code will pass CI.

### Usage

```bash
./scripts/pre-push.sh
```

### What it checks

1. **Code formatting** - `cargo fmt --all -- --check`
2. **Clippy lints** - `cargo clippy --all-targets --all-features -- -D warnings`
3. **Release build** - `cargo build --release --all`
4. **Architecture validation** - Validates Crucible's own architecture using strict mode
5. **Tests** - `cargo test --all`
6. **Documentation** - `cargo doc --no-deps --all-features` with warnings as errors

### Setup as Git Hook (Optional)

To automatically run these checks before every push:

```bash
# Create a pre-push hook
cat > .git/hooks/pre-push << 'EOF'
#!/bin/bash
./scripts/pre-push.sh
EOF

# Make it executable
chmod +x .git/hooks/pre-push
```

Now the checks will run automatically whenever you `git push`. If any check fails, the push will be aborted.

### Skip Hook (Emergency Only)

If you absolutely need to skip the checks (not recommended):

```bash
git push --no-verify
```

## Quick Fixes

If checks fail, here are quick fixes:

**Formatting fails:**
```bash
cargo fmt --all
```

**Clippy warnings:**
```bash
cargo clippy --fix --all-targets --all-features --allow-dirty
```

**Architecture violations:**
```bash
# Update architecture definitions in crucible-core/.crucible/
crucible validate --path crucible-core/.crucible --strict
```

**Test failures:**
```bash
# Run specific test to debug
cargo test --package crucible-core --lib -- test_name
```
