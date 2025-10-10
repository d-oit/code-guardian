#!/bin/bash

# Setup Branch Protection Rules for Code-Guardian Repository
# This script configures branch protection rules via GitHub API

set -euo pipefail

# Configuration
REPO_OWNER="d-oit"
REPO_NAME="code-guardian"
GITHUB_TOKEN="${GITHUB_TOKEN:-}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if GitHub CLI is installed
    if ! command -v gh &> /dev/null; then
        print_error "GitHub CLI (gh) is not installed. Please install it first."
        echo "Visit: https://cli.github.com/"
        exit 1
    fi
    
    # Check if curl is installed
    if ! command -v curl &> /dev/null; then
        print_error "curl is not installed. Please install it first."
        exit 1
    fi
    
    # Check if jq is installed
    if ! command -v jq &> /dev/null; then
        print_error "jq is not installed. Please install it first."
        exit 1
    fi
    
    # Check GitHub authentication
    if [ -z "$GITHUB_TOKEN" ]; then
        print_status "No GITHUB_TOKEN provided. Checking gh auth status..."
        if ! gh auth status &> /dev/null; then
            print_error "Not authenticated with GitHub. Please run 'gh auth login' or set GITHUB_TOKEN."
            exit 1
        fi
        # Get token from gh cli
        GITHUB_TOKEN=$(gh auth token)
    fi
    
    print_success "All prerequisites met!"
}

# Function to make GitHub API calls
github_api() {
    local method="$1"
    local endpoint="$2"
    local data="${3:-}"
    
    local url="https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/${endpoint}"
    
    if [ -n "$data" ]; then
        curl -s -X "$method" \
             -H "Authorization: token $GITHUB_TOKEN" \
             -H "Accept: application/vnd.github.v3+json" \
             -H "Content-Type: application/json" \
             -d "$data" \
             "$url"
    else
        curl -s -X "$method" \
             -H "Authorization: token $GITHUB_TOKEN" \
             -H "Accept: application/vnd.github.v3+json" \
             "$url"
    fi
}

# Function to setup branch protection for main branch
setup_main_branch_protection() {
    print_status "Setting up branch protection for 'main' branch..."
    
    local protection_config='{
        "required_status_checks": {
            "strict": true,
            "checks": [
                {"context": "Test (ubuntu-latest, stable)"},
                {"context": "Test (windows-latest, stable)"},
                {"context": "Test (macos-latest, stable)"},
                {"context": "Coverage"},
                {"context": "Security Audit"},
                {"context": "CodeQL / Analyze (rust)"},
                {"context": "CodeQL / Analyze (javascript)"}
            ]
        },
        "enforce_admins": false,
        "required_pull_request_reviews": {
            "required_approving_review_count": 1,
            "dismiss_stale_reviews": true,
            "require_code_owner_reviews": true,
            "require_last_push_approval": true
        },
        "restrictions": null,
        "required_linear_history": true,
        "allow_force_pushes": false,
        "allow_deletions": false,
        "block_creations": false,
        "required_conversation_resolution": true,
        "lock_branch": false,
        "allow_fork_syncing": false
    }'
    
    local response
    response=$(github_api "PUT" "branches/main/protection" "$protection_config")
    
    if echo "$response" | jq -e '.required_status_checks' > /dev/null 2>&1; then
        print_success "Main branch protection configured successfully!"
    else
        print_error "Failed to configure main branch protection."
        echo "Response: $response"
        return 1
    fi
}

# Function to setup branch protection for develop branch
setup_develop_branch_protection() {
    print_status "Setting up branch protection for 'develop' branch..."
    
    local protection_config='{
        "required_status_checks": {
            "strict": true,
            "checks": [
                {"context": "Test (ubuntu-latest, stable)"},
                {"context": "Coverage"},
                {"context": "Security Audit"}
            ]
        },
        "enforce_admins": false,
        "required_pull_request_reviews": {
            "required_approving_review_count": 1,
            "dismiss_stale_reviews": true,
            "require_code_owner_reviews": false,
            "require_last_push_approval": false
        },
        "restrictions": null,
        "required_linear_history": false,
        "allow_force_pushes": false,
        "allow_deletions": false,
        "block_creations": false,
        "required_conversation_resolution": true,
        "lock_branch": false,
        "allow_fork_syncing": true
    }'
    
    local response
    response=$(github_api "PUT" "branches/develop/protection" "$protection_config")
    
    if echo "$response" | jq -e '.required_status_checks' > /dev/null 2>&1; then
        print_success "Develop branch protection configured successfully!"
    else
        print_error "Failed to configure develop branch protection."
        echo "Response: $response"
        return 1
    fi
}

# Function to create a tag protection rule
setup_tag_protection() {
    print_status "Setting up tag protection rules..."
    
    local tag_protection_config='{
        "pattern": "v*",
        "required_status_checks": {
            "strict": true,
            "checks": [
                {"context": "Test (ubuntu-latest, stable)"},
                {"context": "Test (windows-latest, stable)"},
                {"context": "Test (macos-latest, stable)"},
                {"context": "Security Audit"}
            ]
        }
    }'
    
    # Note: Tag protection is currently in beta and may require special permissions
    local response
    response=$(github_api "POST" "tags/protection" "$tag_protection_config" 2>/dev/null || echo '{"message": "Tag protection not available"}')
    
    if echo "$response" | jq -e '.pattern' > /dev/null 2>&1; then
        print_success "Tag protection configured successfully!"
    else
        print_warning "Tag protection could not be configured (may require organization settings)."
    fi
}

# Function to verify current protection settings
verify_protection_settings() {
    print_status "Verifying current protection settings..."
    
    # Check main branch
    local main_protection
    main_protection=$(github_api "GET" "branches/main/protection" 2>/dev/null || echo '{}')
    
    if echo "$main_protection" | jq -e '.required_status_checks' > /dev/null 2>&1; then
        print_success "Main branch protection is active"
        
        # Show summary
        local required_checks
        required_checks=$(echo "$main_protection" | jq -r '.required_status_checks.checks[]?.context // empty' | wc -l)
        print_status "  - Required status checks: $required_checks"
        
        local review_count
        review_count=$(echo "$main_protection" | jq -r '.required_pull_request_reviews.required_approving_review_count // 0')
        print_status "  - Required approving reviews: $review_count"
        
        local linear_history
        linear_history=$(echo "$main_protection" | jq -r '.required_linear_history // false')
        print_status "  - Linear history required: $linear_history"
    else
        print_warning "Main branch protection is not configured"
    fi
    
    # Check develop branch
    local develop_protection
    develop_protection=$(github_api "GET" "branches/develop/protection" 2>/dev/null || echo '{}')
    
    if echo "$develop_protection" | jq -e '.required_status_checks' > /dev/null 2>&1; then
        print_success "Develop branch protection is active"
    else
        print_warning "Develop branch protection is not configured"
    fi
}

# Function to show current repository settings
show_repository_info() {
    print_status "Repository Information:"
    
    local repo_info
    repo_info=$(github_api "GET" "")
    
    local default_branch
    default_branch=$(echo "$repo_info" | jq -r '.default_branch')
    print_status "  - Default branch: $default_branch"
    
    local private_repo
    private_repo=$(echo "$repo_info" | jq -r '.private')
    print_status "  - Private repository: $private_repo"
    
    local security_features
    security_features=$(echo "$repo_info" | jq -r '.security_and_analysis // {}')
    print_status "  - Security features configured: $(echo "$security_features" | jq 'keys | length')"
}

# Function to setup repository security features
setup_security_features() {
    print_status "Configuring repository security features..."
    
    # Enable vulnerability alerts
    github_api "PUT" "vulnerability-alerts" '{}' > /dev/null 2>&1
    
    # Enable automated security fixes
    github_api "PUT" "automated-security-fixes" '{}' > /dev/null 2>&1
    
    # Note: Some security features require organization-level settings
    print_success "Security features configured (where permissions allow)"
}

# Main execution
main() {
    echo "=================================="
    echo "🛡️  Code-Guardian Branch Protection Setup"
    echo "=================================="
    echo
    
    check_prerequisites
    echo
    
    show_repository_info
    echo
    
    # Verify current settings first
    verify_protection_settings
    echo
    
    # Ask for confirmation
    read -p "Do you want to proceed with setting up branch protection rules? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_status "Setup cancelled by user."
        exit 0
    fi
    echo
    
    # Setup branch protection
    setup_main_branch_protection
    echo
    
    setup_develop_branch_protection
    echo
    
    setup_tag_protection
    echo
    
    setup_security_features
    echo
    
    # Final verification
    print_status "Final verification..."
    verify_protection_settings
    echo
    
    print_success "Branch protection setup completed!"
    echo
    echo "🎉 Your repository now has enterprise-grade protection rules!"
    echo
    echo "Next steps:"
    echo "1. Verify the settings in GitHub web interface"
    echo "2. Test by creating a PR to main branch"
    echo "3. Configure any additional organization-level settings"
    echo "4. Set up repository rulesets for additional protection"
}

# Handle script arguments
case "${1:-}" in
    --verify)
        check_prerequisites
        verify_protection_settings
        ;;
    --help|-h)
        echo "Usage: $0 [--verify|--help]"
        echo ""
        echo "Options:"
        echo "  --verify    Only verify current protection settings"
        echo "  --help      Show this help message"
        ;;
    *)
        main
        ;;
esac