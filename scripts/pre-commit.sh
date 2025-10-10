#!/bin/bash
# Pre-commit hook for Code Guardian
# This script runs quality checks before allowing commits

set -e

echo "🔍 Running pre-commit quality checks..."
echo "======================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    local status=$1
    local message=$2
    if [ "$status" = "success" ]; then
        echo -e "${GREEN}✅ $message${NC}"
    elif [ "$status" = "warning" ]; then
        echo -e "${YELLOW}⚠️  $message${NC}"
    elif [ "$status" = "error" ]; then
        echo -e "${RED}❌ $message${NC}"
    else
        echo "$message"
    fi
}

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_status error "Not in a git repository"
    exit 1
fi

# Get list of staged Rust files
STAGED_RUST_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$' || true)

if [ -z "$STAGED_RUST_FILES" ]; then
    print_status success "No Rust files staged, skipping checks"
    exit 0
fi

print_status info "Checking staged Rust files: $STAGED_RUST_FILES"

# Run formatting check
print_status info "Checking code formatting..."
if ! cargo fmt --all -- --check > /dev/null 2>&1; then
    print_status error "Code formatting issues detected!"
    echo "Run 'make fmt' to fix formatting issues."
    echo "Or run 'make quality-fix' to auto-fix both formatting and clippy issues."
    exit 1
fi
print_status success "Code formatting is correct"

# Run clippy
print_status info "Running clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings > /dev/null 2>&1; then
    print_status error "Clippy issues detected!"
    echo "Run 'make lint-fix' to auto-fix clippy issues."
    echo "Or run 'make quality-fix' to auto-fix both formatting and clippy issues."
    exit 1
fi
print_status success "Clippy checks passed"

# Run tests (only if there are test files or if core functionality changed)
if echo "$STAGED_RUST_FILES" | grep -q "src/\|tests/\|benches/"; then
    print_status info "Running tests..."
    if ! cargo test > /dev/null 2>&1; then
        print_status error "Tests failed!"
        echo "Run 'make test' to see test failures."
        exit 1
    fi
    print_status success "Tests passed"
else
    print_status info "Skipping tests (no core files changed)"
fi

# Check for security issues in dependencies
print_status info "Checking for security vulnerabilities..."
if command -v cargo-audit > /dev/null 2>&1; then
    if ! cargo audit --quiet > /dev/null 2>&1; then
        print_status warning "Security vulnerabilities detected in dependencies!"
        echo "Run 'make audit' to see details."
        # Don't fail the commit for security issues, just warn
    else
        print_status success "No security vulnerabilities found"
    fi
else
    print_status warning "cargo-audit not installed, skipping security check"
fi

print_status success "All pre-commit checks passed!"
echo ""
echo "🎉 Your code is ready to commit!"