# Automated Version Management System

## Overview

The Code Guardian project uses a comprehensive automated version management system that ensures consistency across all workspace crates and integrates seamlessly with the existing CI/CD infrastructure.

## Components

### 1. GitHub Workflow (`.github/workflows/version-sync.yml`)
- **Automatic Detection**: Detects version inconsistencies across workspace crates
- **Smart Synchronization**: Aligns all crates to the highest or specified version
- **Integration**: Works with existing release-please and CI/CD workflows
- **Validation**: Ensures workspace builds after version changes

### 2. Local Script (`scripts/version-manager.sh`)
- **Manual Control**: Provides local version management capabilities
- **Development Support**: Helps developers manage versions during development
- **Release Preparation**: Streamlines release preparation process
- **Validation**: Local workspace validation after changes

### 3. Release-Please Integration
- **Automated Releases**: Existing release-please configuration enhanced
- **Changelog Generation**: Automatic changelog updates for version syncs
- **Semantic Versioning**: Follows semantic versioning standards

## Usage

### Automatic Version Management (GitHub Actions)

The workflow triggers automatically on:
- Changes to any `Cargo.toml` file
- Changes to release-please manifest
- Changes to `CHANGELOG.md`

**Manual Trigger:**
```bash
# Via GitHub web interface or CLI
gh workflow run version-sync.yml -f target_version=0.3.0 -f sync_type=manual
```

### Local Version Management

**Check Status:**
```bash
./scripts/version-manager.sh status
```

**Synchronize Versions:**
```bash
# Sync all crates to specific version
./scripts/version-manager.sh sync 0.3.0

# Preview changes without applying
./scripts/version-manager.sh sync 0.3.0 --dry-run
```

**Bump Versions:**
```bash
# Bump patch version (0.2.2 → 0.2.3)
./scripts/version-manager.sh bump patch

# Bump minor version (0.2.2 → 0.3.0)
./scripts/version-manager.sh bump minor

# Bump major version (0.2.2 → 1.0.0)
./scripts/version-manager.sh bump major
```

**Validate Workspace:**
```bash
./scripts/version-manager.sh validate
```

**Prepare Release:**
```bash
# Complete release preparation
./scripts/version-manager.sh prepare-release 0.3.0

# Preview release preparation
./scripts/version-manager.sh prepare-release 0.3.0 --dry-run
```

## Integration with Existing Workflows

### 1. Enhanced CI Workflow
- Automatically detects version changes
- Runs full test suite after version sync
- Validates workspace consistency

### 2. Release-Please Workflow
- Triggers on version synchronization commits
- Generates appropriate changelog entries
- Creates releases when versions are bumped

### 3. Auto-fix Workflow
- Applies code quality fixes after version changes
- Ensures formatting consistency
- Maintains code standards

### 4. Security & Performance Workflows
- Run comprehensive checks after version updates
- Validate that version changes don't introduce issues
- Maintain security and performance standards

## Best Practices

### Development Workflow

1. **Feature Development:**
   ```bash
   # Start feature development
   git checkout -b feature/new-feature
   
   # Check version status
   ./scripts/version-manager.sh status
   
   # Develop features...
   
   # Prepare for PR
   ./scripts/version-manager.sh validate
   ```

2. **Release Preparation:**
   ```bash
   # Prepare release locally
   ./scripts/version-manager.sh prepare-release 0.3.0 --dry-run
   
   # If satisfied, apply changes
   ./scripts/version-manager.sh prepare-release 0.3.0
   
   # Commit and push
   git add .
   git commit -m "chore: prepare release v0.3.0"
   git push
   ```

3. **Hotfix Workflow:**
   ```bash
   # Quick patch version bump
   ./scripts/version-manager.sh bump patch
   
   # Validate and push
   ./scripts/version-manager.sh validate
   git push
   ```

### CI/CD Integration

1. **Automated Sync:**
   - Version inconsistencies are automatically detected and fixed
   - Commits include `[skip ci]` to prevent infinite loops
   - Changes trigger appropriate downstream workflows

2. **Release Automation:**
   - Version bumps automatically trigger release-please
   - Changelog is updated with version sync information
   - Releases include all workspace crates at consistent versions

3. **Quality Assurance:**
   - All version changes are validated through CI/CD
   - Code quality standards are maintained
   - Security and performance checks run automatically

## Configuration

### Release-Please Configuration (`.github/release-please-config.json`)
```json
{
  "release-type": "rust",
  "extra-files": [
    "crates/cli/Cargo.toml",
    "crates/core/Cargo.toml", 
    "crates/output/Cargo.toml",
    "crates/storage/Cargo.toml"
  ],
  "include-component-in-tag": false,
  "separate-pull-requests": false
}
```

### Workflow Triggers
- **Push to main**: Automatic detection and sync
- **Pull requests**: Validation and preview
- **Manual dispatch**: Custom version management
- **Schedule**: Optional periodic consistency checks

## Troubleshooting

### Common Issues

1. **Version Inconsistencies:**
   ```bash
   # Check current state
   ./scripts/version-manager.sh check
   
   # Fix inconsistencies
   ./scripts/version-manager.sh sync $(./scripts/version-manager.sh status | grep -E "core|cli|output|storage" | head -1 | awk '{print $2}')
   ```

2. **Build Failures After Sync:**
   ```bash
   # Validate workspace
   ./scripts/version-manager.sh validate
   
   # Check for dependency issues
   cargo update
   cargo check --workspace
   ```

3. **Merge Conflicts:**
   ```bash
   # Resolve conflicts in Cargo.toml files
   git checkout --theirs crates/*/Cargo.toml
   
   # Re-sync versions
   ./scripts/version-manager.sh sync <target-version>
   ```

### Debugging

**Enable verbose output:**
```bash
# Set debug mode
export RUST_LOG=debug

# Run with verbose cargo output
./scripts/version-manager.sh status --verbose
```

**Check workflow logs:**
```bash
# View recent workflow runs
gh run list --workflow=version-sync.yml

# View specific run logs
gh run view <run-id> --log
```

## Advanced Usage

### Custom Version Strategies

1. **Alpha/Beta Releases:**
   ```bash
   ./scripts/version-manager.sh sync 0.3.0-alpha.1
   ./scripts/version-manager.sh sync 0.3.0-beta.1
   ```

2. **Release Candidates:**
   ```bash
   ./scripts/version-manager.sh sync 0.3.0-rc.1
   ```

3. **Build Metadata:**
   ```bash
   ./scripts/version-manager.sh sync 0.3.0+build.123
   ```

### Integration with External Tools

1. **Semantic Release:**
   - Compatible with conventional commits
   - Integrates with existing changelog generation
   - Supports automated version determination

2. **Cargo Workspaces:**
   - Maintains workspace consistency
   - Handles inter-crate dependencies
   - Supports workspace-wide operations

3. **Release Automation:**
   - GitHub Release creation
   - Asset building and uploading
   - Documentation deployment

## Monitoring and Maintenance

### Health Checks
- Daily workflow monitoring detects version drift
- Automated issues created for inconsistencies
- Integration with existing monitoring system

### Maintenance Tasks
- Regular review of version management logs
- Periodic validation of automation effectiveness
- Updates to scripts and workflows as needed

### Metrics
- Version sync frequency
- Time to resolve inconsistencies
- Release preparation efficiency
- CI/CD integration success rate