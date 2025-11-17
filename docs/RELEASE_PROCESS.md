# Crucible Release Process

Manual release process for publishing Crucible to crates.io and GitHub releases.

## Prerequisites

- ✅ crates.io authentication configured (`cargo login`)
- ✅ Write access to GitHub repository
- ✅ Rust toolchain with cross-compilation support

## Version Bump

1. Update version in `Cargo.toml`:
   ```toml
   [workspace.package]
   version = "0.2.0"  # Update version
   ```

2. Update CHANGELOG (create if needed):
   ```bash
   # Add new version section to CHANGELOG.md
   ## [0.2.0] - 2024-XX-XX
   ### Added
   - New features...
   ### Changed
   - Breaking changes...
   ### Fixed
   - Bug fixes...
   ```

3. Commit version bump:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "chore: bump version to 0.2.0"
   git push origin main
   ```

## Build Release Binaries

### macOS (Apple Silicon)
```bash
cargo build --release --package crucible-cli
cp target/release/crucible crucible
tar czf crucible-cli-aarch64-apple-darwin.tar.gz crucible
rm crucible
```

### macOS (Intel)
```bash
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin --package crucible-cli
cp target/x86_64-apple-darwin/release/crucible crucible
tar czf crucible-cli-x86_64-apple-darwin.tar.gz crucible
rm crucible
```

### Linux (x86_64)
```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu --package crucible-cli
cp target/x86_64-unknown-linux-gnu/release/crucible crucible
tar czf crucible-cli-x86_64-unknown-linux-gnu.tar.gz crucible
rm crucible
```

### Linux (ARM)
```bash
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu --package crucible-cli
cp target/aarch64-unknown-linux-gnu/release/crucible crucible
tar czf crucible-cli-aarch64-unknown-linux-gnu.tar.gz crucible
rm crucible
```

### Windows (x86_64)
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu --package crucible-cli
cp target/x86_64-pc-windows-gnu/release/crucible.exe crucible.exe
tar czf crucible-cli-x86_64-pc-windows-gnu.tar.gz crucible.exe
rm crucible.exe
```

**Note**: Cross-compilation may require additional dependencies. Consider building on native platforms for best results.

## Create GitHub Release

1. Create and push git tag:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

2. Create GitHub release:
   - Go to https://github.com/anvanster/crucible/releases/new
   - Tag: `v0.2.0`
   - Title: `v0.2.0`
   - Description: Copy from CHANGELOG.md
   - Upload binary archives:
     - `crucible-cli-aarch64-apple-darwin.tar.gz`
     - `crucible-cli-x86_64-apple-darwin.tar.gz`
     - `crucible-cli-x86_64-unknown-linux-gnu.tar.gz`
     - `crucible-cli-aarch64-unknown-linux-gnu.tar.gz`
     - `crucible-cli-x86_64-pc-windows-gnu.tar.gz`
   - Check "Set as the latest release"
   - Click "Publish release"

## Publish to crates.io

1. Verify package contents:
   ```bash
   cargo package --package crucible-core --list
   cargo package --package crucible-cli --list
   ```

2. Test local package:
   ```bash
   cargo package --package crucible-core
   cargo package --package crucible-cli
   ```

3. Publish crucible-core first:
   ```bash
   cargo publish --package crucible-core
   ```

4. Wait for crates.io to index (usually 1-2 minutes)

5. Publish crucible-cli:
   ```bash
   cargo publish --package crucible-cli
   ```

## Verify Installation

Test all installation methods:

```bash
# Test cargo-binstall (uses GitHub releases)
cargo binstall crucible-cli --force

# Test crates.io installation
cargo install crucible-cli --force

# Test git installation
cargo install --git https://github.com/anvanster/crucible.git crucible-cli --force
```

## Post-Release

1. Announce release on relevant channels
2. Update documentation if needed
3. Close related GitHub issues/PRs

## Rollback

If issues are discovered:

1. Yank problematic version from crates.io:
   ```bash
   cargo yank --vers 0.2.0 crucible-cli
   cargo yank --vers 0.2.0 crucible-core
   ```

2. Delete GitHub release and tag:
   ```bash
   git tag -d v0.2.0
   git push origin :refs/tags/v0.2.0
   ```

3. Fix issues and release new patch version

## Release Checklist

- [ ] Version bumped in Cargo.toml
- [ ] CHANGELOG.md updated
- [ ] Changes committed and pushed
- [ ] All tests passing (`cargo test --all`)
- [ ] Validation passing (`crucible validate`)
- [ ] Binaries built for all platforms
- [ ] Git tag created and pushed
- [ ] GitHub release created with binaries
- [ ] crucible-core published to crates.io
- [ ] crucible-cli published to crates.io
- [ ] Installation methods verified
- [ ] Documentation updated

## cargo-binstall Compatibility

The release artifacts must follow this naming pattern for cargo-binstall to work:

```
crucible-cli-{target}.tar.gz
```

Where `{target}` is one of:
- `aarch64-apple-darwin`
- `x86_64-apple-darwin`
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-pc-windows-gnu`

The archive should contain the `crucible` binary (or `crucible.exe` on Windows) at the root level.
