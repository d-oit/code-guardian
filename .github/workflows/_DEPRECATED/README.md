# Deprecated Workflows

These workflows have been consolidated following GitHub Actions best practices.

## Migration Guide

### Security Workflows
- ❌ `security.yml` → ✅ `security-consolidated.yml`
- ❌ `security-config.yml` → ✅ `security-consolidated.yml` 
- ❌ `security-enhancements.yml` → ✅ `security-consolidated.yml`

**New Features:**
- Configurable security levels (minimal/standard/strict)
- Auto-issue creation on failures
- Better artifact management
- Security-focused clippy (no code style enforcement)

### Release Workflows
- ❌ `release.yml` → ✅ `release-consolidated.yml`
- ❌ `enhanced-release.yml` → ✅ `release-consolidated.yml`
- ✅ `release-please.yml` → Keep (automated releases)

**New Features:**
- Multi-platform builds (Linux, Windows, macOS Intel/ARM)
- Enhanced release notes with installation instructions
- Automatic archive creation with docs/examples
- Better error handling and summaries

## Benefits

1. **Single Source of Truth** - One workflow per concern
2. **Reduced Maintenance** - 758 lines → ~300 lines
3. **Better UX** - Clear parameterization and reporting
4. **Consistency** - Standardized patterns across workflows
5. **Reliability** - Eliminated conflicting configurations

## Timeline

- **Phase 1**: New consolidated workflows active
- **Phase 2**: Move old workflows to `_DEPRECATED/` (safe backup)
- **Phase 3**: Remove deprecated workflows after 30 days

## Testing

Run the new workflows with:
```bash
# Test security workflow
gh workflow run security-consolidated.yml -f security_level=standard

# Test release workflow  
gh workflow run release-consolidated.yml -f tag=v0.1.8 -f prerelease=true
```