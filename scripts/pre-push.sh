#!/bin/bash
set -e

echo "🔍 Running pre-push checks..."
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
        exit 1
    fi
}

# 1. Check formatting
echo "📝 Checking code formatting..."
cargo fmt --all -- --check
print_status $? "Code formatting check"
echo ""

# 2. Run clippy
echo "🔎 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings
print_status $? "Clippy check"
echo ""

# 3. Build release
echo "🔨 Building release..."
cargo build --release --all
print_status $? "Release build"
echo ""

# 4. Validate Crucible's own architecture
echo "🏗️  Validating Crucible's architecture..."
cargo run --bin crucible --release -- validate --path crucible-core/.crucible --strict
print_status $? "Architecture validation"
echo ""

# 5. Run tests
echo "🧪 Running tests..."
cargo test --all
print_status $? "Tests"
echo ""

# 6. Check documentation
echo "📚 Checking documentation..."
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
print_status $? "Documentation check"
echo ""

echo -e "${GREEN}✅ All checks passed!${NC}"
echo ""
echo "🚀 Ready to push to main"
