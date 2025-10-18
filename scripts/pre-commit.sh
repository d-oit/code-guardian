#!/bin/bash
# Pre-commit hook for Code Guardian
# This script runs quality checks before allowing commits with enhanced error handling

set -euo pipefail

# Load common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

log $LOG_INFO "🔍 Running pre-commit quality checks..."
log $LOG_INFO "======================================="

# Check if we're in a git repository
if ! is_git_repo; then
    log $LOG_ERROR "Not in a git repository"
    exit 1
fi

# Get list of staged Rust files
STAGED_RUST_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$' || true)

if [ -z "$STAGED_RUST_FILES" ]; then
    log $LOG_INFO "No Rust files staged, skipping checks"
    exit 0
fi

log $LOG_INFO "Checking staged Rust files:"
echo "$STAGED_RUST_FILES"

# Run formatting check
log $LOG_INFO "Checking code formatting..."
if execute "cargo fmt --all -- --check" "Check code formatting"; then
    log $LOG_INFO "Code formatting is correct"
else
    log $LOG_ERROR "Code formatting issues detected!"
    echo "Run 'make fmt' to fix formatting issues."
    echo "Or run 'make quality-fix' to auto-fix both formatting and clippy issues."
    exit 1
fi

# Run clippy
log $LOG_INFO "Running clippy..."
if execute "cargo clippy --all-targets --all-features -- -D warnings" "Run clippy"; then
    log $LOG_INFO "Clippy checks passed"
else
    log $LOG_ERROR "Clippy issues detected!"
    echo "Run 'make lint-fix' to auto-fix clippy issues."
    echo "Or run 'make quality-fix' to auto-fix both formatting and clippy issues."
    exit 1
fi

# Run tests (only if there are test files or if core functionality changed)
if echo "$STAGED_RUST_FILES" | grep -q "src/\|tests/\|benches/"; then
    log $LOG_INFO "Running tests..."
    if execute "cargo test" "Run tests"; then
        log $LOG_INFO "Tests passed"
    else
        log $LOG_ERROR "Tests failed!"
        echo "Run 'make test' to see test failures."
        exit 1
    fi
else
    log $LOG_INFO "Skipping tests (no core files changed)"
fi

# Check for security issues in dependencies
log $LOG_INFO "Checking for security vulnerabilities..."
if command_exists "cargo-audit"; then
    if execute "cargo audit --quiet" "Check security vulnerabilities"; then
        log $LOG_INFO "No security vulnerabilities found"
    else
        log $LOG_WARN "Security vulnerabilities detected in dependencies!"
        echo "Run 'make audit' to see details."
        # Don't fail the commit for security issues, just warn
    fi
else
    log $LOG_WARN "cargo-audit not installed, skipping security check"
fi

log $LOG_INFO "All pre-commit checks passed!"
echo ""
echo "🎉 Your code is ready to commit!"