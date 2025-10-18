# Release Template for Code Guardian

This template ensures consistent, professional release descriptions across all versions.

## Template Structure

```markdown
## Code Guardian v{VERSION} {EMOJI}

### {SECTION_EMOJI} {SECTION_NAME}
- {CHANGE_DESCRIPTION}

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

## Section Mapping

| Change Type | Emoji | Section Name |
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

## Special Release Types

### Initial Release (v0.1.0)
- Use 🎉 emoji in title
- Include "Initial Release" section with feature overview
- Add celebration language

### Alpha/Beta Releases
- Include ⚠️ Note section explaining the pre-release nature
- Add testing and feedback encouragement

### Major Releases
- Include migration guide if needed
- Highlight breaking changes prominently
- Add upgrade instructions

## Examples

### Standard Release
```
## Code Guardian v1.2.3

### ✨ Added
- New feature X for enhanced scanning
- Support for additional file formats

### 🐛 Fixed
- Memory leak in scanner engine
- CLI argument parsing edge case
```

### Pre-release
```
## Code Guardian v1.3.0-alpha

### ⚠️ Note
This is an alpha release for testing new features. Please report any issues.

### ✨ Added
- Experimental AI-powered detection
```