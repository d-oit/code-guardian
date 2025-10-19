# Branch Protection Setup Guide

This guide provides instructions for setting up GitHub Rulesets to enforce the quality gates in the code-guardian project.

> **Note**: The configuration now uses the modern GitHub Rulesets API format, providing more flexible and powerful branch protection capabilities compared to the legacy branch protection rules.

## 🎯 Overview

GitHub Rulesets ensure that all code changes go through proper quality gates before being merged into protected branches. This maintains code quality, security, and stability by enforcing requirements such as required signatures, multiple approvals, and status checks.

## 🔧 Automated Setup

### Prerequisites
- GitHub CLI (`gh`) installed and authenticated
- Repository admin permissions
- All workflows successfully running

### Quick Setup
```bash
# Run the automated setup script
./scripts/setup-branch-protection.sh
```

## 📱 Manual Setup (GitHub Web Interface)

If the automated script fails due to permissions, follow these manual steps to create a Ruleset:

### 1. Navigate to Rulesets
1. Go to your repository on GitHub
2. Click **Settings** tab
3. Click **Rules** in the left sidebar
4. Click **Rulesets** tab

### 2. Create Main Branch Ruleset

Click **New ruleset** and select **Branch ruleset**:

- **Name**: `main-protection`
- **Enforcement status**: Active
- **Target branches**: Include `main`

#### Branch Rules
- ✅ **Restrict deletions**: Prevent deletion of the branch
- ✅ **Restrict non-fast-forward updates**: Prevent force pushes
- ✅ **Require linear history**: Enforce merge commits or rebases
- ✅ **Require signed commits**: All commits must be signed

#### Pull Request Rules
- ✅ **Require pull request before merging**
- ✅ **Required approvals: 2**
- ✅ **Dismiss stale reviews when new commits are pushed**
- ✅ **Require approval of the most recent reviewable push**
- ✅ **Require review from code owners**
- ✅ **Require conversation resolution before merging**

#### Status Checks
- ✅ **Require status checks to pass before merging**
- ✅ **Require branches to be up to date before merging**

**Required checks:**
- `Test (ubuntu-latest, stable)`
- `Test (windows-latest, stable)`
- `Test (macos-latest, stable)`
- `Coverage`
- `Security Audit`
- `Performance Benchmark`
- `CodeQL / Analyze (rust)`
- `CodeQL / Analyze (javascript)`
- `Lint`
- `Build`

#### Bypass Permissions
- Allow repository administrators to bypass rules for pull requests

### 3. Create Develop Branch Ruleset (Optional)

Create a separate ruleset for `develop` branch with relaxed requirements:

- **Name**: `develop-protection`
- **Enforcement status**: Active
- **Target branches**: Include `develop`

#### Branch Rules
- ✅ **Restrict deletions**
- ✅ **Restrict non-fast-forward updates**
- ✅ **Require linear history**
- ✅ **Require signed commits**

#### Pull Request Rules
- ✅ **Require pull request before merging**
- ✅ **Required approvals: 2**
- ✅ **Dismiss stale reviews when new commits are pushed**
- ✅ **Require approval of the most recent reviewable push**
- ✅ **Require review from code owners**
- ✅ **Require conversation resolution before merging**

#### Status Checks
- ✅ **Require status checks to pass before merging**
- ✅ **Require branches to be up to date before merging**

**Required checks:**
- `Test (ubuntu-latest, stable)`
- `Coverage`
- `Security Audit`
- `Lint`
- `Build`

#### Bypass Permissions
- Allow repository administrators to bypass rules for pull requests

## 🛡️ Quality Gates Enforced

### Code Quality
- **Linting**: `cargo fmt` and `cargo clippy` must pass
- **Building**: All workspace crates must build successfully
- **Testing**: All tests must pass on multiple platforms

### Security
- **Security Audit**: No known vulnerabilities in dependencies
- **CodeQL Analysis**: Static analysis for security issues
- **License Compliance**: All dependencies have approved licenses

### Performance
- **Benchmarks**: Performance regressions are detected
- **Coverage**: Code coverage requirements are met

## 🔍 Verification

After setup, verify the rulesets:

```bash
# List all rulesets
gh api repos/:owner/:repo/rulesets

# Check specific ruleset details
gh api repos/:owner/:repo/rulesets/{ruleset_id}
```

## 🚫 Common Issues

### Permission Errors
- Ensure you have admin permissions on the repository
- Check that your GitHub token has the `repo` scope

### Missing Status Checks
- Ensure all workflows have run at least once
- Status check names must match exactly (case-sensitive)

### Code Owner Issues
- Verify `CODEOWNERS` file is in the repository root
- Check that specified users/teams exist and have access

## 📋 Maintenance

### Adding New Status Checks
When adding new workflows:
1. Update `.github/branch-protection-config.json` (now in Rulesets format)
2. Run `./scripts/setup-branch-protection.sh` or update manually
3. Test with a test PR

### Updating Code Owners
Edit the `CODEOWNERS` file in the repository root to modify review requirements.

## 🎉 Best Practices

1. **Start with develop branch** - Test protection rules on develop before applying to main
2. **Gradual rollout** - Add protection rules incrementally
3. **Monitor impact** - Watch for blocked PRs and adjust as needed
4. **Regular review** - Periodically review and update protection rules
5. **Team communication** - Ensure all team members understand the quality gates

## 🚀 2025 Best Practices

As we move into 2025, several advanced features and practices are becoming essential for maintaining high-quality, secure, and efficient software development workflows:

### Merge Queues
- **Enable merge queues** on protected branches to automatically merge PRs that pass all checks
- Reduces manual intervention and ensures continuous integration
- Configure queue rules to require additional approvals for complex changes

### Deployment Requirements
- **Require deployment previews** for PRs affecting production systems
- Implement **environment-specific protections** (e.g., stricter rules for production deployments)
- Use **deployment gates** to ensure successful staging deployments before merging to main

### Regular Audits
- **Quarterly branch protection audits** to review and update rules based on team feedback and security best practices
- **Access reviews** to ensure only authorized team members can bypass protections
- **Compliance checks** to align with industry standards (e.g., SOC 2, ISO 27001)
- Monitor and analyze **protection rule effectiveness** through GitHub Insights and custom dashboards

## 📞 Support

If you encounter issues with branch protection setup:
1. Check the GitHub documentation on branch protection
2. Verify your permissions and token scopes
3. Test with a simple PR to identify specific failing checks
4. Review workflow logs for detailed error messages