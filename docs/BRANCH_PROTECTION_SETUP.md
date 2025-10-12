# Branch Protection Setup Guide

This guide provides instructions for setting up branch protection rules to enforce the quality gates in the code-guardian project.

## 🎯 Overview

Branch protection rules ensure that all code changes go through proper quality gates before being merged into protected branches. This maintains code quality, security, and stability.

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

If the automated script fails due to permissions, follow these manual steps:

### 1. Navigate to Branch Settings
1. Go to your repository on GitHub
2. Click **Settings** tab
3. Click **Branches** in the left sidebar

### 2. Configure Main Branch Protection

Click **Add rule** or edit existing rule for `main` branch:

#### Required Status Checks
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

#### Pull Request Reviews
- ✅ **Require a pull request before merging**
- ✅ **Require approvals: 1**
- ✅ **Dismiss stale reviews when new commits are pushed**
- ✅ **Require review from code owners**

#### Additional Settings
- ✅ **Restrict pushes that create files**
- ✅ **Require conversation resolution before merging**
- ✅ **Include administrators**
- ❌ **Allow force pushes**
- ❌ **Allow deletions**

### 3. Configure Develop Branch Protection

Create a similar rule for `develop` branch with these differences:
- Fewer required status checks (core quality gates only)
- ❌ **Include administrators** (disabled for flexibility)
- Same review requirements

#### Required Status Checks for Develop
- `Test (ubuntu-latest, stable)`
- `Coverage`
- `Security Audit`
- `Lint`
- `Build`

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

After setup, verify the protection rules:

```bash
# Check current protection status
gh api repos/:owner/:repo/branches/main/protection

# List required status checks
gh api repos/:owner/:repo/branches/main/protection/required_status_checks
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
1. Update `.github/branch-protection-config.json`
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

## 📞 Support

If you encounter issues with branch protection setup:
1. Check the GitHub documentation on branch protection
2. Verify your permissions and token scopes
3. Test with a simple PR to identify specific failing checks
4. Review workflow logs for detailed error messages