# GitHub Repository Setup Scripts

This directory contains automated scripts to configure GitHub repository protection rules and security features for Code-Guardian.

## 🚀 Quick Start

### Prerequisites

1. **GitHub CLI** (recommended):
   ```bash
   # Install GitHub CLI
   brew install gh  # macOS
   # or
   sudo apt install gh  # Ubuntu
   # or 
   winget install GitHub.cli  # Windows
   
   # Authenticate
   gh auth login
   ```

2. **Alternative - Personal Access Token**:
   ```bash
   export GITHUB_TOKEN="your_github_token_here"
   ```

3. **Install dependencies**:
   ```bash
   cd scripts
   npm install
   pip3 install requests
   ```

## 📜 Available Scripts

### 1. **setup-branch-protection.sh** - Bash Script (Recommended)
Complete branch protection setup with interactive prompts.

```bash
# Run full setup
./setup-branch-protection.sh

# Verify current settings only
./setup-branch-protection.sh --verify

# Show help
./setup-branch-protection.sh --help
```

**Features:**
- ✅ Interactive setup with confirmations
- ✅ Comprehensive error handling
- ✅ Colored output and progress indicators
- ✅ Verification of existing settings
- ✅ Compatible with GitHub CLI and tokens

### 2. **github-api-setup.py** - Python Script (Advanced)
Python-based setup with detailed configuration options.

```bash
# Run setup
python3 github-api-setup.py
```

**Features:**
- ✅ Object-oriented design
- ✅ Detailed branch configuration
- ✅ Rate limiting and error handling
- ✅ Comprehensive status reporting
- ✅ Modular configuration system

### 3. **setup-repository-ruleset.js** - Node.js Script (Cutting Edge)
Modern repository rulesets using GitHub's latest APIs.

```bash
# Install dependencies first
npm install

# Run setup  
node setup-repository-ruleset.js
```

**Features:**
- ✅ Repository rulesets (GitHub's newest feature)
- ✅ Advanced rule configuration
- ✅ Tag protection rules
- ✅ Conventional commit enforcement
- ✅ File path restrictions

## 🛡️ Protection Rules Configuration

### Main Branch Protection
- **Required Status Checks**: All CI workflows must pass
- **Required Reviews**: 1 approving review from code owners
- **Linear History**: Enforced for clean commit history
- **Force Push**: Disabled
- **Admin Enforcement**: Enabled

### Develop Branch Protection  
- **Required Status Checks**: Core tests and security audit
- **Required Reviews**: 1 approving review
- **Linear History**: Flexible for development
- **Fork Syncing**: Allowed

### Repository Rulesets (Advanced)
- **Conventional Commits**: Enforced pattern matching
- **Email Validation**: Valid email addresses required
- **File Protection**: Critical files require additional review
- **Path Length**: Maximum 255 characters
- **Tag Protection**: Version tags protected from deletion

## 🔧 Configuration Options

### Branch Protection Settings

```bash
# Main branch checks
MAIN_REQUIRED_CHECKS=(
    "Test (ubuntu-latest, stable)"
    "Test (windows-latest, stable)" 
    "Test (macos-latest, stable)"
    "Coverage"
    "Security Audit"
    "CodeQL / Analyze (rust)"
    "CodeQL / Analyze (javascript)"
)

# Develop branch checks  
DEVELOP_REQUIRED_CHECKS=(
    "Test (ubuntu-latest, stable)"
    "Coverage"
    "Security Audit"
)
```

### Security Features Enabled

- 🔍 **Vulnerability Alerts**: Automatic dependency scanning
- 🤖 **Automated Security Fixes**: Dependabot security updates
- 🛡️ **Code Scanning**: CodeQL analysis
- 📋 **Supply Chain**: cargo-deny license and security checks

## 🚨 Troubleshooting

### Common Issues

**1. Authentication Errors**
```bash
# Check GitHub CLI auth
gh auth status

# Re-authenticate if needed
gh auth login --scopes repo,admin:repo_hook,admin:org
```

**2. Permission Errors**
```bash
# Repository rulesets require admin permissions
# Use branch protection as alternative
./setup-branch-protection.sh
```

**3. Rate Limiting**
```bash
# Scripts include automatic rate limiting
# If issues persist, add delays between requests
```

**4. API Endpoint Errors**
```bash
# Some features require GitHub Enterprise or organization settings
# Check GitHub documentation for feature availability
```

### Debug Mode

Enable verbose output for debugging:

```bash
# Bash script
export DEBUG=1
./setup-branch-protection.sh

# Python script  
export PYTHONPATH=/usr/local/lib/python3.*/site-packages
python3 -v github-api-setup.py

# Node.js script
DEBUG=* node setup-repository-ruleset.js
```

## 📝 Manual Verification

After running scripts, verify settings manually:

### Via GitHub Web Interface
1. Go to **Settings** → **Branches**
2. Check protection rules for `main` and `develop`
3. Verify required status checks
4. Confirm review requirements

### Via GitHub CLI
```bash
# Check branch protection
gh api repos/d-oit/code-guardian/branches/main/protection

# Check repository rulesets
gh api repos/d-oit/code-guardian/rulesets
```

### Via API
```bash
# Using curl
curl -H "Authorization: token $GITHUB_TOKEN" \
     -H "Accept: application/vnd.github.v3+json" \
     https://api.github.com/repos/d-oit/code-guardian/branches/main/protection
```

## 🔄 Updating Rules

To modify protection rules:

1. **Edit script configuration** sections
2. **Re-run the setup script**
3. **Verify changes** in GitHub interface
4. **Test with a test PR**

## 📋 Testing Your Setup

Create a test branch to verify rules:

```bash
# Create test branch
git checkout -b test-protection-rules

# Make a test change
echo "# Test" >> test-file.md
git add test-file.md
git commit -m "test: verify protection rules"

# Push and create PR
git push -u origin test-protection-rules
gh pr create --title "Test: Protection Rules" --body "Testing branch protection"

# This should trigger all required checks
```

## 🆘 Support

If you encounter issues:

1. **Check Prerequisites**: Ensure all tools are installed
2. **Verify Permissions**: Confirm you have admin access to the repository
3. **Review Logs**: Check script output for specific error messages
4. **Manual Fallback**: Configure rules manually via GitHub web interface
5. **GitHub Documentation**: Refer to official GitHub API documentation

## 📚 Additional Resources

- [GitHub Branch Protection](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/about-protected-branches)
- [Repository Rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets)
- [GitHub Security Features](https://docs.github.com/en/code-security)
- [Conventional Commits](https://www.conventionalcommits.org/)

---

**🛡️ Secure your repository with enterprise-grade protection rules!**