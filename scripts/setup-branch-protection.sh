#!/bin/bash
# Setup Branch Protection Rules for code-guardian

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔒 Setting up branch protection rules for code-guardian${NC}"

# Check if gh CLI is available and authenticated
if ! command -v gh &> /dev/null; then
    echo -e "${RED}❌ GitHub CLI (gh) is not installed. Please install it first.${NC}"
    exit 1
fi

if ! gh auth status &> /dev/null; then
    echo -e "${RED}❌ GitHub CLI is not authenticated. Please run 'gh auth login' first.${NC}"
    exit 1
fi

# Function to apply branch protection
apply_protection() {
    local branch=$1
    local config_file=".github/branch-protection-config.json"
    
    echo -e "${YELLOW}📋 Applying protection rules for branch: ${branch}${NC}"
    
    if jq -e ".${branch}" "${config_file}" > /dev/null; then
        if gh api "repos/:owner/:repo/branches/${branch}/protection" \
           --method PUT \
           --input <(jq ".${branch}" "${config_file}") \
           > /dev/null 2>&1; then
            echo -e "${GREEN}✅ Successfully applied protection rules for ${branch}${NC}"
        else
            echo -e "${RED}❌ Failed to apply protection rules for ${branch}${NC}"
            echo -e "${YELLOW}💡 This might be due to insufficient permissions or missing branch${NC}"
            echo -e "${YELLOW}💡 Please apply the rules manually in GitHub Settings > Branches${NC}"
        fi
    else
        echo -e "${RED}❌ No configuration found for branch: ${branch}${NC}"
    fi
}

# Apply protection to main and develop branches
apply_protection "main"
apply_protection "develop"

echo -e "${BLUE}📖 Branch Protection Summary:${NC}"
echo -e "${GREEN}Main branch protection includes:${NC}"
echo -e "  • Required status checks: All CI/CD workflows must pass"
echo -e "  • Required PR reviews: 1 approving review required"
echo -e "  • Code owner reviews: Required"
echo -e "  • Admin enforcement: Enabled"
echo -e "  • Force pushes: Disabled"
echo -e "  • Conversation resolution: Required"

echo -e "${GREEN}Develop branch protection includes:${NC}"
echo -e "  • Required status checks: Core quality gates must pass"
echo -e "  • Required PR reviews: 1 approving review required"
echo -e "  • Admin enforcement: Disabled (for flexibility)"
echo -e "  • Force pushes: Disabled"

echo -e "${BLUE}🎯 Quality Gates Enforced:${NC}"
echo -e "  • ✅ Linting (cargo fmt + clippy)"
echo -e "  • ✅ Building (all crates)"
echo -e "  • ✅ Testing (all platforms)"
echo -e "  • ✅ Security auditing"
echo -e "  • ✅ Performance benchmarking"
echo -e "  • ✅ Code coverage"
echo -e "  • ✅ CodeQL analysis"

echo -e "${GREEN}🎉 Branch protection setup completed!${NC}"