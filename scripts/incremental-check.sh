#!/bin/bash
# Incremental Quality Check Script - GOAP Phase 3 Implementation
# Only runs quality checks on changed files for faster development workflow

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🔍 GOAP Incremental Quality Check${NC}"

# Get changed files since last commit
CHANGED_FILES=$(git diff --name-only HEAD~1 HEAD | grep -E '\.(rs|toml)$' || true)

if [ -z "$CHANGED_FILES" ]; then
    echo -e "${GREEN}✅ No Rust files changed, skipping checks${NC}"
    exit 0
fi

echo -e "${YELLOW}📝 Changed files:${NC}"
echo "$CHANGED_FILES"

# Check only changed rust files
RUST_FILES=$(echo "$CHANGED_FILES" | grep '\.rs$' || true)

if [ ! -z "$RUST_FILES" ]; then
    echo -e "${YELLOW}🔍 Running format check on changed files...${NC}"
    for file in $RUST_FILES; do
        if [ -f "$file" ]; then
            rustfmt --check "$file" || {
                echo -e "${RED}❌ Format check failed for $file${NC}"
                exit 1
            }
        fi
    done
    
    echo -e "${YELLOW}📎 Running clippy on affected crates...${NC}"
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
        echo -e "${YELLOW}🔍 Checking crate: code-guardian-$crate${NC}"
        cargo clippy -p code-guardian-$crate --quiet -- -D warnings || {
            echo -e "${RED}❌ Clippy failed for crate: $crate${NC}"
            exit 1
        }
    done
    
    echo -e "${YELLOW}🧪 Running tests for affected crates...${NC}"
    for crate in $AFFECTED_CRATES; do
        echo -e "${YELLOW}🧪 Testing crate: code-guardian-$crate${NC}"
        cargo test -p code-guardian-$crate --quiet || {
            echo -e "${RED}❌ Tests failed for crate: $crate${NC}"
            exit 1
        }
    done
fi

echo -e "${GREEN}✅ Incremental quality check passed!${NC}"