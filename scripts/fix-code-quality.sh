#!/bin/bash
# Code Quality Auto-Fix Script for Code Guardian
# This script applies formatting and clippy fixes automatically

set -e

echo "🔧 Code Guardian - Auto-fix Code Quality Issues"
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local status=$1
    local message=$2
    case $status in
        "success")
            echo -e "${GREEN}✅ $message${NC}"
            ;;
        "warning")
            echo -e "${YELLOW}⚠️  $message${NC}"
            ;;
        "error")
            echo -e "${RED}❌ $message${NC}"
            ;;
        "info")
            echo -e "${BLUE}ℹ️  $message${NC}"
            ;;
        *)
            echo "$message"
            ;;
    esac
}

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_status error "Not in a git repository"
    exit 1
fi

# Function to check if there are uncommitted changes
check_git_status() {
    if ! git diff --quiet || ! git diff --staged --quiet; then
        print_status warning "You have uncommitted changes."
        echo "   Consider committing or stashing them before running auto-fix."
        read -p "   Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_status error "Aborted."
            exit 1
        fi
    fi
}

# Function to apply formatting
apply_formatting() {
    print_status info "Checking code formatting..."

    if cargo fmt --all -- --check > /dev/null 2>&1; then
        print_status success "Code formatting is already correct."
        return 0
    else
        print_status info "Applying formatting fixes..."
        cargo fmt --all
        print_status success "Formatting applied."
        return 1
    fi
}

# Function to apply clippy fixes
apply_clippy() {
    print_status info "Checking clippy issues..."

    if cargo clippy --all-targets --all-features -- -D warnings > /dev/null 2>&1; then
        print_status success "No clippy issues found."
        return 0
    else
        print_status info "Applying clippy fixes..."
        cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged
        print_status success "Clippy fixes applied."
        return 1
    fi
}

# Function to commit changes
commit_changes() {
    if ! git diff --quiet; then
        print_status info "Committing auto-fix changes..."
        git add .
        git commit -m "auto-fix: apply code quality fixes

- Apply cargo fmt formatting
- Apply clippy suggestions

[automated commit]"
        print_status success "Changes committed."
    else
        print_status info "No changes to commit."
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
                echo "❌ Unknown option: $1"
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
    print_status info "Summary:"
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
    print_status success "Code quality check complete!"

    if [ "$format_changed" = true ] || [ "$clippy_changed" = true ]; then
        if [ "$auto_commit" = false ]; then
            echo "💡 Tip: Use --commit flag to automatically commit these changes."
        fi
    fi
}

main "$@"