# Code-Guardian Git Integration Demo

This document demonstrates the new git integration features in Code-Guardian.

## Features

### 1. Pre-commit Hook Installation

Install the pre-commit hook to automatically run Code-Guardian before each commit:

```bash
# Install the hook
code-guardian git install-hook

# The hook will now run automatically on every commit
git commit -m "Your commit message"
```

### 2. Staged Files Scanning

Scan only staged files instead of the entire repository:

```bash
# Scan only staged files
code-guardian pre-commit --staged-only

# Fast scan of staged files (critical issues only)
code-guardian pre-commit --staged-only --fast
```

### 3. Git Integration Commands

```bash
# List currently staged files
code-guardian git staged

# Install pre-commit hook
code-guardian git install-hook

# Uninstall pre-commit hook  
code-guardian git uninstall-hook
```

## Workflow Integration

### Typical Developer Workflow

1. **One-time setup**: Install the pre-commit hook
   ```bash
   code-guardian git install-hook
   ```

2. **Daily development**: Work normally - the hook runs automatically
   ```bash
   # Make changes
   vim src/main.rs
   
   # Stage changes
   git add src/main.rs
   
   # Commit (hook runs automatically)
   git commit -m "Add new feature"
   ```

3. **Manual scanning**: Check staged files before committing
   ```bash
   # See what files are staged
   code-guardian git staged
   
   # Run manual pre-commit check
   code-guardian pre-commit --staged-only --fast
   ```

### CI/CD Integration

The git integration works seamlessly with CI/CD pipelines:

```yaml
# .github/workflows/code-quality.yml
- name: Run Code-Guardian on changed files
  run: |
    # Get list of changed files in PR
    git fetch origin main
    git diff --name-only origin/main... > changed_files.txt
    
    # Run Code-Guardian on changed files
    code-guardian scan --files-from changed_files.txt
```

## Benefits

- **Faster feedback**: Catch issues at commit time, not in CI
- **Focused scanning**: Only scan files you're actually changing
- **Team adoption**: Automatic enforcement through git hooks
- **Flexible**: Works with both individual commits and CI/CD pipelines

## Technical Implementation

The git integration uses:
- `git diff --cached --name-only` to get staged files
- `git rev-parse --show-toplevel` to find repository root
- Native git commands for maximum compatibility
- Graceful fallback to directory scanning when not in a git repo

## Configuration

The pre-commit hook uses these default settings:
- `--staged-only`: Only scan staged files
- `--fast`: Quick scan mode (critical issues only)

You can customize by editing the hook file directly or using a configuration file.