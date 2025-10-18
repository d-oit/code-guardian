# Release Management System

This document describes the enhanced release management system for Code Guardian, which provides automated, professional release descriptions and synchronized changelog management.

## 🎯 Overview

The release management system consists of several integrated components that work together to ensure consistent, professional releases:

1. **Release Template** - Standardized format for all releases
2. **Enhanced Release Workflow** - Automatic professional formatting
3. **Changelog Sync** - Bidirectional sync between releases and changelog
4. **Release-Please Integration** - Automated version management
5. **Management Scripts** - Tools for manual operations

## 📁 Components

### 🔧 Configuration Files

- **`.github/RELEASE_TEMPLATE.md`** - Template and guidelines for release formatting
- **`.github/release-please-config.json`** - Enhanced release-please configuration
- **`.github/.release-please-manifest.json`** - Version tracking

### ⚙️ Workflows

- **`.github/workflows/enhanced-release.yml`** - Automatically enhances release descriptions
- **`.github/workflows/changelog-sync.yml`** - Syncs changelog with releases
- **`.github/workflows/release-please.yml`** - Updated to trigger enhancements

### 🛠️ Scripts

- **`scripts/release-management.sh`** - Command-line tool for release operations

## 🚀 How It Works

### Automated Release Flow

1. **Developer commits** using conventional commit format
2. **Release-please** creates a PR with version bump and changelog
3. **PR merge** triggers release creation
4. **Enhanced release workflow** automatically formats the release description
5. **Changelog sync** ensures consistency between changelog and releases

### Manual Release Flow

1. **Create tag** manually or using the management script
2. **Enhanced release workflow** triggers automatically
3. **Professional formatting** applied to release description

## 📝 Release Format

All releases follow this professional format:

```markdown
## Code Guardian v{VERSION} {EMOJI}

### ✨ Added
- New features and capabilities

### 🐛 Fixed
- Bug fixes and improvements

### 🔄 Changed
- Modifications and updates

### 📦 Assets
- Pre-built binaries for Linux (x86_64), macOS (Intel & Apple Silicon), and Windows
- Full source code archives

### 🚀 Installation
```bash
# Download and extract the appropriate binary for your platform
# Or install from source:
cargo install --git https://github.com/d-oit/code-guardian
```

### 🔗 Links
- [Installation Guide](https://github.com/d-oit/code-guardian#installation)
- [Documentation](https://github.com/d-oit/code-guardian/tree/main/docs)
- [Changelog](https://github.com/d-oit/code-guardian/blob/main/CHANGELOG.md)
```

## 🎨 Section Mapping

| Commit Type | Emoji | Section Name |
|-------------|-------|--------------|
| feat | ✨ | Added |
| fix | 🐛 | Fixed |
| perf | ⚡ | Performance |
| docs | 📚 | Documentation |
| style | 🎨 | Style |
| refactor | ♻️ | Refactor |
| test | 🧪 | Tests |
| chore | 🔧 | Maintenance |
| breaking | ⚠️ | Breaking Changes |

## 🔧 Using the Management Script

The `scripts/release-management.sh` script provides convenient commands:

### List Releases
```bash
./scripts/release-management.sh list
```

### Check Status
```bash
./scripts/release-management.sh status
```

### Enhance a Release
```bash
./scripts/release-management.sh enhance v0.1.5
```

### Sync Changelog
```bash
./scripts/release-management.sh sync-changelog
```

### Test Workflows
```bash
./scripts/release-management.sh test-workflows
```

### Create Manual Release
```bash
./scripts/release-management.sh create-manual v0.1.7
```

### Validate Release Format
```bash
./scripts/release-management.sh validate v0.1.6
```

## 🔄 Workflow Triggers

### Automatic Triggers

- **On tag push** - Enhanced release workflow runs
- **On release publish/edit** - Changelog sync runs
- **On main branch push** - Release-please checks for new release

### Manual Triggers

- **Enhanced Release**: `gh workflow run enhanced-release.yml -f tag=v0.1.5`
- **Changelog Sync**: `gh workflow run changelog-sync.yml -f sync_all=true`

## 📋 Conventional Commits

Use conventional commit format for automatic changelog generation:

```
feat: add new scanning feature
fix: resolve memory leak in scanner
docs: update installation guide
chore: update dependencies
```

### Commit Types

- **feat**: New features → ✨ Added
- **fix**: Bug fixes → 🐛 Fixed
- **perf**: Performance improvements → ⚡ Performance
- **docs**: Documentation → 📚 Documentation
- **style**: Code style → 🎨 Style
- **refactor**: Code refactoring → ♻️ Refactor
- **test**: Tests → 🧪 Tests
- **chore**: Maintenance → 🔧 Maintenance

## 🛡️ Best Practices

### For Developers

1. **Use conventional commits** for automatic changelog generation
2. **Review release PRs** created by release-please
3. **Test releases** before merging release PRs
4. **Monitor workflows** after releases are created

### For Maintainers

1. **Validate releases** using the management script
2. **Enhance old releases** to maintain consistency
3. **Sync changelog** when making manual changes
4. **Monitor workflow failures** and fix issues promptly

## 🔍 Troubleshooting

### Common Issues

**Release description not enhanced**
- Check if the enhanced-release workflow ran successfully
- Manually trigger: `./scripts/release-management.sh enhance v0.1.5`

**Changelog out of sync**
- Run: `./scripts/release-management.sh sync-changelog`
- Check the changelog-sync workflow logs

**Workflow failures**
- Check workflow permissions
- Verify GitHub token has necessary scopes
- Review workflow logs for specific errors

### Manual Fixes

**Fix a release description**
```bash
./scripts/release-management.sh enhance v0.1.5
```

**Sync all releases to changelog**
```bash
./scripts/release-management.sh sync-changelog
```

**Validate release format**
```bash
./scripts/release-management.sh validate v0.1.5
```

## 📊 Monitoring

### Workflow Status

Monitor workflow runs at:
- https://github.com/d-oit/code-guardian/actions

### Key Workflows to Watch

- **Enhanced Release** - Should run on every tag push
- **Changelog Sync** - Should run on release events
- **Release Please** - Should run on main branch pushes

### Success Indicators

- ✅ Professional release descriptions with all sections
- ✅ Synchronized changelog entries
- ✅ Consistent formatting across all releases
- ✅ Working installation links and documentation

## 🔮 Future Enhancements

Potential improvements to consider:

1. **Release notes templates** for different types of releases
2. **Automated testing** of release artifacts
3. **Integration with project boards** for tracking
4. **Slack/Discord notifications** for releases
5. **Automated security scanning** of release binaries

## 📚 Related Documentation

- [GitHub Actions Workflows](../.github/workflows/)
- [Contributing Guidelines](../CONTRIBUTING.md)
- [Changelog](../CHANGELOG.md)
- [Release Template](../.github/RELEASE_TEMPLATE.md)