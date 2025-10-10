#!/bin/bash

# Best Practices Implementation Verification Script
# Verifies all components of the Code-Guardian best practices

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
WARNING_CHECKS=0

# Function to print colored output
print_header() {
    echo -e "${MAGENTA}=====================================${NC}"
    echo -e "${MAGENTA}$1${NC}"
    echo -e "${MAGENTA}=====================================${NC}"
    echo
}

print_section() {
    echo -e "${CYAN}🔍 $1${NC}"
    echo "-----------------------------------"
}

print_check() {
    local status="$1"
    local message="$2"
    local details="${3:-}"
    
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    
    case "$status" in
        "PASS")
            echo -e "  ✅ ${GREEN}PASS${NC} - $message"
            PASSED_CHECKS=$((PASSED_CHECKS + 1))
            ;;
        "FAIL")
            echo -e "  ❌ ${RED}FAIL${NC} - $message"
            FAILED_CHECKS=$((FAILED_CHECKS + 1))
            if [ -n "$details" ]; then
                echo -e "     ${RED}$details${NC}"
            fi
            ;;
        "WARN")
            echo -e "  ⚠️  ${YELLOW}WARN${NC} - $message"
            WARNING_CHECKS=$((WARNING_CHECKS + 1))
            if [ -n "$details" ]; then
                echo -e "     ${YELLOW}$details${NC}"
            fi
            ;;
        "INFO")
            echo -e "  ℹ️  ${BLUE}INFO${NC} - $message"
            if [ -n "$details" ]; then
                echo -e "     ${BLUE}$details${NC}"
            fi
            ;;
    esac
}

# Helper functions
check_file_exists() {
    local file="$1"
    local description="$2"
    
    if [ -f "$file" ]; then
        print_check "PASS" "$description exists" "$file"
        return 0
    else
        print_check "FAIL" "$description missing" "$file"
        return 1
    fi
}

check_command_exists() {
    local cmd="$1"
    local description="$2"
    
    if command -v "$cmd" >/dev/null 2>&1; then
        local version=$($cmd --version 2>/dev/null | head -n1 || echo "unknown version")
        print_check "PASS" "$description available" "$version"
        return 0
    else
        print_check "FAIL" "$description not installed" "Install with package manager"
        return 1
    fi
}

check_github_api() {
    local endpoint="$1"
    local description="$2"
    
    if gh api "$endpoint" >/dev/null 2>&1; then
        print_check "PASS" "$description accessible"
        return 0
    else
        print_check "FAIL" "$description not accessible" "Check permissions"
        return 1
    fi
}

# Main verification functions

verify_prerequisites() {
    print_section "Prerequisites & Tools"
    
    check_command_exists "git" "Git version control"
    check_command_exists "gh" "GitHub CLI"
    check_command_exists "cargo" "Rust toolchain"
    check_command_exists "rustc" "Rust compiler"
    check_command_exists "rustfmt" "Rust formatter"
    check_command_exists "clippy" "Rust linter"
    
    # Check optional tools
    if command -v cargo-audit >/dev/null 2>&1; then
        print_check "PASS" "cargo-audit installed"
    else
        print_check "WARN" "cargo-audit not installed" "Install with: cargo install cargo-audit"
    fi
    
    if command -v cargo-deny >/dev/null 2>&1; then
        print_check "PASS" "cargo-deny installed"
    else
        print_check "WARN" "cargo-deny not installed" "Install with: cargo install cargo-deny"
    fi
    
    # Check GitHub authentication
    if gh auth status >/dev/null 2>&1; then
        local username=$(gh api user --jq '.login' 2>/dev/null || echo "unknown")
        print_check "PASS" "GitHub authentication active" "User: $username"
    else
        print_check "FAIL" "GitHub authentication required" "Run: gh auth login"
    fi
    
    echo
}

verify_project_structure() {
    print_section "Project Structure"
    
    # Core files
    check_file_exists "Cargo.toml" "Workspace configuration"
    check_file_exists "Cargo.lock" "Dependency lock file"
    check_file_exists "README.md" "Project README"
    check_file_exists "LICENSE" "License file"
    check_file_exists "CHANGELOG.md" "Changelog"
    check_file_exists "CONTRIBUTING.md" "Contributing guidelines"
    check_file_exists "rustfmt.toml" "Rust formatting config"
    check_file_exists ".gitignore" "Git ignore file"
    
    # Crate structure
    local crates=("cli" "core" "output" "storage")
    for crate in "${crates[@]}"; do
        if [ -d "crates/$crate" ]; then
            print_check "PASS" "Crate '$crate' exists"
            check_file_exists "crates/$crate/Cargo.toml" "Crate '$crate' configuration"
            check_file_exists "crates/$crate/src/lib.rs" "Crate '$crate' library file"
        else
            print_check "FAIL" "Crate '$crate' missing" "crates/$crate"
        fi
    done
    
    # Documentation
    if [ -d "docs" ]; then
        print_check "PASS" "Documentation directory exists"
        check_file_exists "docs/BEST_PRACTICES.md" "Best practices documentation"
    else
        print_check "WARN" "Documentation directory missing"
    fi
    
    echo
}

verify_github_infrastructure() {
    print_section "GitHub Infrastructure"
    
    # GitHub templates
    check_file_exists ".github/ISSUE_TEMPLATE/bug_report.yml" "Bug report template"
    check_file_exists ".github/ISSUE_TEMPLATE/feature_request.yml" "Feature request template"
    check_file_exists ".github/ISSUE_TEMPLATE/config.yml" "Issue template config"
    check_file_exists ".github/PULL_REQUEST_TEMPLATE.md" "PR template"
    
    # Repository governance
    check_file_exists ".github/CODEOWNERS" "Code owners file"
    check_file_exists ".github/SECURITY.md" "Security policy"
    check_file_exists ".github/FUNDING.yml" "Funding configuration"
    check_file_exists ".github/dependabot.yml" "Dependabot configuration"
    
    # Workflows
    local workflows=("ci.yml" "docs.yml" "release.yml" "security.yml" "performance.yml" "stale.yml" "release-please.yml")
    for workflow in "${workflows[@]}"; do
        check_file_exists ".github/workflows/$workflow" "Workflow: $workflow"
    done
    
    # Additional configs
    check_file_exists ".github/repository-ruleset.json" "Repository ruleset"
    check_file_exists "deny.toml" "cargo-deny configuration"
    
    echo
}

verify_code_quality() {
    print_section "Code Quality"
    
    # Formatting check
    if cargo fmt --check >/dev/null 2>&1; then
        print_check "PASS" "Code formatting compliant"
    else
        print_check "FAIL" "Code formatting issues" "Run: cargo fmt"
    fi
    
    # Linting check
    if cargo clippy --all-targets --all-features -- -D warnings >/dev/null 2>&1; then
        print_check "PASS" "Clippy linting passed"
    else
        print_check "FAIL" "Clippy linting issues" "Run: cargo clippy"
    fi
    
    # Build check
    if cargo check >/dev/null 2>&1; then
        print_check "PASS" "Project builds successfully"
    else
        print_check "FAIL" "Build errors present" "Run: cargo check"
    fi
    
    # Test check
    local test_output=$(cargo test 2>&1)
    if echo "$test_output" | grep -q "test result: ok"; then
        local test_count=$(echo "$test_output" | grep -o "[0-9]\+ passed" | head -1 | grep -o "[0-9]\+")
        print_check "PASS" "Tests passing" "$test_count tests passed"
    else
        print_check "FAIL" "Test failures detected" "Run: cargo test"
    fi
    
    # Documentation test
    if cargo test --doc >/dev/null 2>&1; then
        print_check "PASS" "Documentation tests passed"
    else
        print_check "WARN" "Documentation test issues" "Run: cargo test --doc"
    fi
    
    echo
}

verify_security() {
    print_section "Security Configuration"
    
    # Security audit
    if command -v cargo-audit >/dev/null 2>&1; then
        if cargo audit >/dev/null 2>&1; then
            print_check "PASS" "Security audit clean"
        else
            print_check "FAIL" "Security vulnerabilities found" "Run: cargo audit"
        fi
    else
        print_check "WARN" "cargo-audit not available"
    fi
    
    # License compliance
    if command -v cargo-deny >/dev/null 2>&1; then
        if cargo deny check >/dev/null 2>&1; then
            print_check "PASS" "License compliance verified"
        else
            print_check "FAIL" "License or dependency issues" "Run: cargo deny check"
        fi
    else
        print_check "WARN" "cargo-deny not available"
    fi
    
    # Check for common security issues in code
    if grep -r "unwrap()" crates/ >/dev/null 2>&1; then
        local unwrap_count=$(grep -r "unwrap()" crates/ | wc -l)
        print_check "WARN" "Found $unwrap_count unwrap() calls" "Consider using proper error handling"
    else
        print_check "PASS" "No unwrap() calls found"
    fi
    
    if grep -r "panic!" crates/ >/dev/null 2>&1; then
        local panic_count=$(grep -r "panic!" crates/ | wc -l)
        print_check "WARN" "Found $panic_count panic! calls" "Consider graceful error handling"
    else
        print_check "PASS" "No panic! calls found"
    fi
    
    echo
}

verify_github_settings() {
    print_section "GitHub Repository Settings"
    
    # Check repository access
    if check_github_api "repos/d-oit/code-guardian" "Repository access"; then
        
        # Check default branch
        local default_branch=$(gh api repos/d-oit/code-guardian --jq '.default_branch')
        if [ "$default_branch" = "main" ]; then
            print_check "PASS" "Default branch is main"
        else
            print_check "WARN" "Default branch is not main" "Current: $default_branch"
        fi
        
        # Check branch protection
        if gh api repos/d-oit/code-guardian/branches/main/protection >/dev/null 2>&1; then
            local checks_count=$(gh api repos/d-oit/code-guardian/branches/main/protection --jq '.required_status_checks.checks | length')
            print_check "PASS" "Main branch protection active" "$checks_count required checks"
        else
            print_check "FAIL" "Main branch protection not configured" "Run branch protection setup"
        fi
        
        # Check develop branch protection
        if gh api repos/d-oit/code-guardian/branches/develop/protection >/dev/null 2>&1; then
            print_check "PASS" "Develop branch protection active"
        else
            print_check "WARN" "Develop branch protection not configured"
        fi
        
        # Check security features
        if gh api repos/d-oit/code-guardian/vulnerability-alerts >/dev/null 2>&1; then
            print_check "PASS" "Vulnerability alerts enabled"
        else
            print_check "WARN" "Vulnerability alerts not enabled"
        fi
        
        # Check repository visibility
        local visibility=$(gh api repos/d-oit/code-guardian --jq '.visibility')
        print_check "INFO" "Repository visibility: $visibility"
        
    fi
    
    echo
}

verify_documentation() {
    print_section "Documentation Quality"
    
    # Check API documentation builds
    if cargo doc --no-deps >/dev/null 2>&1; then
        print_check "PASS" "API documentation builds"
    else
        print_check "FAIL" "API documentation build errors" "Run: cargo doc"
    fi
    
    # Check for documentation coverage
    local doc_comments=$(find crates -name "*.rs" -exec grep -l "///" {} \; | wc -l)
    local rust_files=$(find crates -name "*.rs" | wc -l)
    local doc_coverage=$((doc_comments * 100 / rust_files))
    
    if [ $doc_coverage -ge 80 ]; then
        print_check "PASS" "Documentation coverage good" "$doc_coverage% of files documented"
    elif [ $doc_coverage -ge 60 ]; then
        print_check "WARN" "Documentation coverage moderate" "$doc_coverage% of files documented"
    else
        print_check "FAIL" "Documentation coverage low" "$doc_coverage% of files documented"
    fi
    
    # Check README completeness
    if grep -q "## Installation" README.md && grep -q "## Usage" README.md; then
        print_check "PASS" "README has required sections"
    else
        print_check "WARN" "README missing standard sections"
    fi
    
    echo
}

verify_performance() {
    print_section "Performance Configuration"
    
    # Check for release optimizations
    if grep -q "lto = true" Cargo.toml; then
        print_check "PASS" "Link-time optimization enabled"
    else
        print_check "WARN" "LTO not configured" "Add lto = true to [profile.release]"
    fi
    
    if grep -q "codegen-units = 1" Cargo.toml; then
        print_check "PASS" "Codegen units optimized"
    else
        print_check "WARN" "Codegen units not optimized"
    fi
    
    # Check for benchmarks
    if find crates -name "benches" -type d | grep -q .; then
        print_check "PASS" "Benchmark directories found"
    else
        print_check "WARN" "No benchmark directories found"
    fi
    
    # Check for performance-related dependencies
    if grep -q "rayon" Cargo.toml; then
        print_check "PASS" "Parallel processing library (rayon) included"
    else
        print_check "WARN" "No parallel processing library found"
    fi
    
    echo
}

verify_agent_integration() {
    print_section "Agent Integration"
    
    # Check OpenCode configuration
    if [ -d ".opencode" ]; then
        print_check "PASS" "OpenCode directory exists"
        
        # Check agent configurations
        local agent_count=$(find .opencode/agent -name "*.md" | wc -l)
        print_check "INFO" "Agent configurations found" "$agent_count agents configured"
        
        # Check command configurations  
        local command_count=$(find .opencode/command -name "*.md" | wc -l)
        print_check "INFO" "Command configurations found" "$command_count commands configured"
        
        # Check plugins
        if [ -d ".opencode/plugin" ]; then
            local plugin_count=$(find .opencode/plugin -name "*.js" | wc -l)
            print_check "PASS" "Plugin directory exists" "$plugin_count plugins found"
        else
            print_check "WARN" "No plugin directory found"
        fi
        
    else
        print_check "FAIL" "OpenCode configuration missing" ".opencode directory not found"
    fi
    
    echo
}

verify_git_workflow() {
    print_section "Git Workflow"
    
    # Check current branch
    local current_branch=$(git branch --show-current)
    print_check "INFO" "Current branch: $current_branch"
    
    # Check for conventional commit format in recent commits
    local recent_commits=$(git log --oneline -10 --pretty=format:"%s")
    local conventional_count=0
    while IFS= read -r commit; do
        if [[ $commit =~ ^(feat|fix|docs|style|refactor|perf|test|chore|ci|build|revert)(\(.+\))?: ]]; then
            conventional_count=$((conventional_count + 1))
        fi
    done <<< "$recent_commits"
    
    if [ $conventional_count -ge 8 ]; then
        print_check "PASS" "Recent commits follow conventional format" "$conventional_count/10 commits"
    elif [ $conventional_count -ge 5 ]; then
        print_check "WARN" "Some commits don't follow conventional format" "$conventional_count/10 commits"
    else
        print_check "FAIL" "Many commits don't follow conventional format" "$conventional_count/10 commits"
    fi
    
    # Check for clean working directory
    if git status --porcelain | grep -q .; then
        local changes=$(git status --porcelain | wc -l)
        print_check "INFO" "Working directory has changes" "$changes files modified"
    else
        print_check "PASS" "Working directory clean"
    fi
    
    echo
}

generate_summary() {
    print_header "Verification Summary"
    
    local pass_rate=$((PASSED_CHECKS * 100 / TOTAL_CHECKS))
    
    echo -e "${CYAN}📊 Results Summary:${NC}"
    echo -e "   Total Checks: $TOTAL_CHECKS"
    echo -e "   ✅ Passed: ${GREEN}$PASSED_CHECKS${NC}"
    echo -e "   ❌ Failed: ${RED}$FAILED_CHECKS${NC}"
    echo -e "   ⚠️  Warnings: ${YELLOW}$WARNING_CHECKS${NC}"
    echo -e "   📈 Pass Rate: ${CYAN}${pass_rate}%${NC}"
    echo
    
    if [ $pass_rate -ge 90 ]; then
        echo -e "${GREEN}🎉 Excellent! Code-Guardian follows best practices.${NC}"
        echo -e "${GREEN}Your repository is production-ready!${NC}"
    elif [ $pass_rate -ge 80 ]; then
        echo -e "${YELLOW}👍 Good! Minor improvements needed.${NC}"
        echo -e "${YELLOW}Address warnings to reach excellence.${NC}"
    elif [ $pass_rate -ge 70 ]; then
        echo -e "${YELLOW}⚠️  Fair. Several issues need attention.${NC}"
        echo -e "${YELLOW}Focus on failed checks first.${NC}"
    else
        echo -e "${RED}❌ Poor. Significant improvements needed.${NC}"
        echo -e "${RED}Please address critical issues.${NC}"
    fi
    
    echo
    
    if [ $FAILED_CHECKS -gt 0 ]; then
        echo -e "${RED}🚨 Action Required:${NC}"
        echo "   1. Address failed checks above"
        echo "   2. Run verification script again"
        echo "   3. Consider running setup scripts in ./scripts/"
        echo
    fi
    
    if [ $WARNING_CHECKS -gt 0 ]; then
        echo -e "${YELLOW}💡 Recommendations:${NC}"
        echo "   1. Install missing optional tools"
        echo "   2. Configure branch protection if not set"
        echo "   3. Improve documentation coverage"
        echo "   4. Follow conventional commit format"
        echo
    fi
    
    echo -e "${BLUE}📚 Resources:${NC}"
    echo "   • Best Practices Guide: docs/BEST_PRACTICES.md"
    echo "   • Setup Scripts: scripts/"
    echo "   • GitHub Documentation: .github/"
    echo
    
    print_header "Verification Complete"
}

# Main execution
main() {
    print_header "Code-Guardian Best Practices Verification"
    echo "Comprehensive verification of implementation status"
    echo
    
    verify_prerequisites
    verify_project_structure
    verify_github_infrastructure
    verify_code_quality
    verify_security
    verify_github_settings
    verify_documentation
    verify_performance
    verify_agent_integration
    verify_git_workflow
    
    generate_summary
}

# Handle script arguments
case "${1:-}" in
    --help|-h)
        echo "Usage: $0 [--help]"
        echo ""
        echo "Verifies Code-Guardian best practices implementation"
        echo ""
        echo "This script checks:"
        echo "  • Project structure and configuration"
        echo "  • GitHub infrastructure and templates"
        echo "  • Code quality and security"
        echo "  • CI/CD pipeline setup"
        echo "  • Documentation completeness"
        echo "  • Git workflow compliance"
        ;;
    *)
        main
        ;;
esac