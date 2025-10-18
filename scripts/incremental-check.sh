#!/bin/bash
# Incremental Quality Check Script - GOAP Phase 3 Implementation
# Only runs quality checks on changed files for faster development workflow with enhanced error handling

set -euo pipefail

# Load common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

log $LOG_INFO "🔍 GOAP Incremental Quality Check"

# Get changed files since last commit
CHANGED_FILES=$(git diff --name-only HEAD~1 HEAD | grep -E '\.(rs|toml)$' || true)

if [ -z "$CHANGED_FILES" ]; then
    log $LOG_INFO "No Rust files changed, skipping checks"
    exit 0
fi

log $LOG_INFO "📝 Changed files:"
echo "$CHANGED_FILES"

# Check only changed rust files
RUST_FILES=$(echo "$CHANGED_FILES" | grep '\.rs$' || true)

if [ ! -z "$RUST_FILES" ]; then
    log $LOG_INFO "🔍 Running format check on changed files..."
    for file in $RUST_FILES; do
        if file_exists "$file"; then
            if execute "rustfmt --check \"$file\"" "Check formatting for $file"; then
                log $LOG_INFO "✓ Format check passed for $file"
            else
                log $LOG_ERROR "❌ Format check failed for $file"
                exit 1
            fi
        else
            log $LOG_WARN "File $file not found, skipping"
        fi
    done
    
    log $LOG_INFO "📎 Running clippy on affected crates..."
    # Determine which crates are affected
    AFFECTED_CRATES=""
    for file in $RUST_FILES; do
        if [[ $file == crates/cli/* ]]; then
            AFFECTED_CRATES="$AFFECTED_CRATES cli"
        elif [[ $file == crates/core/* ]]; then
            AFFECTED_CRATES="$AFFECTED_CRATES core"
        elif [[ $file == crates/output/* ]]; then
            AFFECTED_CRATES="$AFFECTED_CRATES output"
        elif [[ $file == crates/storage/* ]]; then
            AFFECTED_CRATES="$AFFECTED_CRATES storage"
        fi
    done
    
    # Remove duplicates
    AFFECTED_CRATES=$(echo $AFFECTED_CRATES | tr ' ' '\n' | sort -u | tr '\n' ' ')
    
    for crate in $AFFECTED_CRATES; do
        log $LOG_INFO "🔍 Checking crate: code_guardian_$crate"
        if execute "cargo clippy -p code_guardian_$crate --quiet -- -D warnings" "Check clippy for $crate"; then
            log $LOG_INFO "✓ Clippy check passed for crate: $crate"
        else
            log $LOG_ERROR "❌ Clippy failed for crate: $crate"
            exit 1
        fi
    done
    
    log $LOG_INFO "🧪 Running tests for affected crates..."
    for crate in $AFFECTED_CRATES; do
        log $LOG_INFO "🧪 Testing crate: code_guardian_$crate"
        if execute "cargo test -p code_guardian_$crate --quiet" "Test crate $crate"; then
            log $LOG_INFO "✓ Tests passed for crate: $crate"
        else
            log $LOG_ERROR "❌ Tests failed for crate: $crate"
            exit 1
        fi
    done
fi

log $LOG_INFO "✅ Incremental quality check passed!"