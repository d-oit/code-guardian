#!/bin/bash
# Code Quality Auto-Fix Script for Code Guardian
# This script applies formatting and clippy fixes automatically with enhanced error handling

set -euo pipefail

# Load common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

log $LOG_INFO "🔧 Code Guardian - Auto-fix Code Quality Issues"
log $LOG_INFO "=============================================="

# Check if we're in a git repository
if ! is_git_repo; then
    log $LOG_ERROR "Not in a git repository"
    exit 1
fi

# Function to check if there are uncommitted changes
check_git_status() {
    if has_uncommitted_changes; then
        log $LOG_WARN "You have uncommitted changes."
        echo "   Consider committing or stashing them before running auto-fix."
        
        if confirm "Continue anyway?" "n"; then
            log $LOG_INFO "Proceeding with uncommitted changes"
        else
            log $LOG_ERROR "Aborted."
            exit 1
        fi
    fi
}

# Function to apply formatting
apply_formatting() {
    log $LOG_INFO "Checking code formatting..."

    if execute "cargo fmt --all -- --check" "Check code formatting"; then
        log $LOG_INFO "Code formatting is already correct."
        return 0
    else
        log $LOG_INFO "Applying formatting fixes..."
        execute "cargo fmt --all" "Apply formatting fixes"
        log $LOG_INFO "Formatting applied."
        return 1
    fi
}

# Function to apply clippy fixes
apply_clippy() {
    log $LOG_INFO "Checking clippy issues..."

    if execute "cargo clippy --all-targets --all-features -- -D warnings" "Check clippy issues"; then
        log $LOG_INFO "No clippy issues found."
        return 0
    else
        log $LOG_INFO "Applying clippy fixes..."
        execute "cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged" "Apply clippy fixes"
        log $LOG_INFO "Clippy fixes applied."
        return 1
    fi
}

# Function to commit changes
commit_changes() {
    if has_uncommitted_changes; then
        log $LOG_INFO "Committing auto-fix changes..."
        execute "git add ." "Stage changes"
        execute "git commit -m \"auto-fix: apply code quality fixes

- Apply cargo fmt formatting
- Apply clippy suggestions

[automated commit]\"" "Commit changes"
        log $LOG_INFO "Changes committed."
    else
        log $LOG_INFO "No changes to commit."
    fi
}

# Main execution
main() {
    local format_changed=false
    local clippy_changed=false
    local auto_commit=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --commit|-c)
                auto_commit=true
                shift
                ;;
            --help|-h)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --commit, -c    Automatically commit changes"
                echo "  --help, -h      Show this help message"
                echo ""
                echo "This script applies cargo fmt and clippy --fix to improve code quality."
                exit 0
                ;;
            *)
                log $LOG_ERROR "Unknown option: $1"
                echo "Use --help for usage information."
                exit 1
                ;;
        esac
    done
    
    if [ "$auto_commit" = true ]; then
        check_git_status
    fi
    
    # Apply fixes
    if ! apply_formatting; then
        format_changed=true
    fi
    
    if ! apply_clippy; then
        clippy_changed=true
    fi
    
    # Summary
    echo ""
    log $LOG_INFO "Summary:"
    if [ "$format_changed" = true ]; then
        echo "  🎨 Formatting: Fixed"
    else
        echo "  🎨 Formatting: Already correct"
    fi

    if [ "$clippy_changed" = true ]; then
        echo "  📎 Clippy: Fixed"
    else
        echo "  📎 Clippy: No issues"
    fi

    # Auto-commit if requested and there are changes
    if [ "$auto_commit" = true ] && ([ "$format_changed" = true ] || [ "$clippy_changed" = true ]); then
        echo ""
        commit_changes
    fi

    echo ""
    log $LOG_INFO "Code quality check complete!"

    if [ "$format_changed" = true ] || [ "$clippy_changed" = true ]; then
        if [ "$auto_commit" = false ]; then
            echo "💡 Tip: Use --commit flag to automatically commit these changes."
        fi
    fi
}

# Run main function
main "$@"