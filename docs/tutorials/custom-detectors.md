# Custom Detectors Guide

Code-Guardian supports custom pattern detectors that allow you to define your own rules for detecting specific code patterns, security vulnerabilities, or code quality issues.

## Overview

There are two ways to define custom patterns:

1. **Simple Custom Patterns**: Basic regex patterns defined in configuration
2. **Advanced Custom Detectors**: Full-featured detectors with rich configuration options

## Simple Custom Patterns

Simple custom patterns can be defined in a configuration file using the `custom_patterns` field:

### Configuration File Format

Create a `config.toml`, `config.json`, or `config.yaml` file:

**TOML format:**
```toml
[custom_patterns]
SQL_INJECTION = "(?i)SELECT.*\\+.*FROM"
HARDCODED_SECRET = "(?i)(password|secret|key)\\s*=\\s*['\"][^'\"]{8,}['\"]"
```

**JSON format:**
```json
{
  "custom_patterns": {
    "SQL_INJECTION": "(?i)SELECT.*\\+.*FROM",
    "HARDCODED_SECRET": "(?i)(password|secret|key)\\s*=\\s*['\"][^'\"]{8,}['\"]"
  }
}
```

**YAML format:**
```yaml
custom_patterns:
  SQL_INJECTION: "(?i)SELECT.*\\+.*FROM"
  HARDCODED_SECRET: "(?i)(password|secret|key)\\s*=\\s*['\"][^'\"]{8,}['\"]"
```

### Usage

```bash
# Scan with custom config
code-guardian scan /path/to/project --config config.toml

# Or use enhanced config for more options
code-guardian scan /path/to/project --enhanced-config enhanced_config.json
```

## Advanced Custom Detectors

Advanced custom detectors provide full control over pattern detection with features like:

- File extension filtering
- Case sensitivity control
- Multi-line pattern matching
- Named capture groups
- Severity levels
- Categories
- Examples and descriptions

### Creating Custom Detectors

Custom detectors are defined in JSON, YAML, or TOML files. Here's an example:

```json
[
  {
    "name": "SQL_INJECTION",
    "description": "Detect potential SQL injection vulnerabilities",
    "pattern": "(?i)(query|execute)\\s*\\(\\s*['\"]\\s*SELECT.*\\+.*['\"]\\s*\\)",
    "file_extensions": ["py", "js", "php"],
    "case_sensitive": false,
    "multiline": false,
    "capture_groups": [],
    "severity": "Critical",
    "category": "Security",
    "examples": [
      "query(\"SELECT * FROM users WHERE id = \" + user_id)"
    ],
    "enabled": true
  },
  {
    "name": "HARDCODED_PASSWORD",
    "description": "Detect hardcoded passwords and secrets",
    "pattern": "(?i)(password|secret|key|token)\\s*[=:]\\s*['\"][^'\"]{8,}['\"]",
    "file_extensions": [],
    "case_sensitive": false,
    "multiline": false,
    "capture_groups": [],
    "severity": "High",
    "category": "Security",
    "examples": [
      "password = \"secretpassword123\""
    ],
    "enabled": true
  }
]
```

### Detector Configuration Fields

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Unique identifier for the detector |
| `description` | string | Human-readable description |
| `pattern` | string | Regular expression pattern to match |
| `file_extensions` | array | File extensions to scan (empty = all files) |
| `case_sensitive` | boolean | Whether pattern matching is case-sensitive |
| `multiline` | boolean | Whether to use multiline regex mode |
| `capture_groups` | array | Named capture groups for extracting data |
| `severity` | enum | Severity level: Low, Medium, High, Critical |
| `category` | enum | Category: CodeQuality, Security, Performance, Documentation, Testing, Deprecated, Custom |
| `examples` | array | Example code snippets that should match |
| `enabled` | boolean | Whether this detector is active |

### Creating Example Detectors

Code-Guardian can generate example custom detectors for you:

```bash
# Create examples in JSON format (default)
code-guardian custom-detectors create-examples

# Create examples in a specific file
code-guardian custom-detectors create-examples --output my_detectors.yaml
```

This creates detectors for common patterns like:
- SQL injection vulnerabilities
- Hardcoded passwords
- Large functions
- And more

### Managing Custom Detectors

```bash
# List all custom detectors
code-guardian custom-detectors list

# Load detectors from file
code-guardian custom-detectors load my_detectors.json

# Test detectors on a specific file
code-guardian custom-detectors test detectors.json test_file.rs
```

### Using Custom Detectors in Scans

```bash
# Scan with custom detectors
code-guardian scan /path/to/project --custom-detectors custom_detectors.json

# Combine with other options
code-guardian scan /path/to/project \
  --custom-detectors security_detectors.json \
  --db security_scans.db \
  --format json
```

### Advanced Pattern Examples

#### Named Capture Groups

Extract specific information from matches:

```json
{
  "name": "DEPRECATED_FUNCTION",
  "description": "Detect usage of deprecated functions",
  "pattern": "old_function\\((?P<args>.*)\\)",
  "capture_groups": ["args"],
  "severity": "Medium",
  "category": "Deprecated"
}
```

#### Multi-line Patterns

Detect patterns spanning multiple lines:

```json
{
  "name": "LARGE_FUNCTION",
  "description": "Detect functions that might be too large",
  "pattern": "fn\\s+\\w+[^{]*\\{[^}]{500,}\\}",  "multiline": true,
  "severity": "Medium",
  "category": "CodeQuality"
}
```

#### File-Specific Patterns

Only scan certain file types:

```json
{
  "name": "PYTHON_SQL_INJECTION",
  "description": "Python-specific SQL injection detection",
  "pattern": "cursor\\.execute\\([^,]+\\+",
  "file_extensions": ["py"],
  "severity": "Critical",
  "category": "Security"
}
```

### Integration with CI/CD

Add custom detectors to your CI pipeline:

```yaml
# .github/workflows/security-scan.yml
name: Security Scan

on: [push, pull_request]

jobs:
  security-scan:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4

    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Build Code-Guardian
      run: cargo build --release --bin code_guardian_cli

    - name: Run Security Scan
      run: |
        ./target/release/code_guardian_cli scan . \
          --custom-detectors security_detectors.json \
          --db /tmp/security.db

    - name: Check for Security Issues
      run: |
        SCAN_ID=$(./target/release/code_guardian_cli history --db /tmp/security.db | tail -1 | awk '{print $2}' | tr -d ',')
        COUNT=$(./target/release/code_guardian_cli report "$SCAN_ID" --db /tmp/security.db --format json | jq '.matches | length')

        if [ "$COUNT" -gt 0 ]; then
          echo "🚨 Security issues found: $COUNT"
          ./target/release/code_guardian_cli report "$SCAN_ID" --db /tmp/security.db --format markdown >> $GITHUB_STEP_SUMMARY
          exit 1
        else
          echo "✅ No security issues found"
        fi
```

### Best Practices

1. **Test Your Patterns**: Use the `test` command to verify patterns work as expected
2. **Start Simple**: Begin with basic patterns and add complexity as needed
3. **Use Appropriate Severity**: Critical for security issues, Medium for code quality
4. **Document Examples**: Include clear examples of what should match
5. **Regular Updates**: Review and update custom detectors as your codebase evolves
6. **Performance**: Test patterns on large codebases to ensure they don't slow down scans

### Troubleshooting

#### Pattern Not Matching

- Check regex syntax with an online regex tester
- Verify case sensitivity settings
- Test with the `custom-detectors test` command

#### Performance Issues

- Avoid overly complex regex patterns
- Use file extension filters to limit scan scope
- Consider multiline mode only when necessary

#### False Positives

- Refine patterns to be more specific
- Use negative lookbehinds/lookaheads in regex
- Add exclusion patterns in configuration

This guide covers the basics of custom detectors. For more advanced usage, check the API documentation or create an issue on GitHub.