# Reusable CI/CD Patterns for Code-Guardian

This directory contains reusable GitHub Actions workflows, composite actions, and templates to standardize CI/CD processes across the project.

## Structure

```
.github/
├── actions/                    # Composite actions
│   ├── setup-rust/            # Rust toolchain setup
│   ├── setup-cache/           # Cargo caching
│   ├── run-clippy/            # Clippy linting
│   ├── run-tests/             # Test execution
│   ├── generate-coverage/     # Coverage reports
│   ├── build-workspace/       # Workspace building
│   └── run-security-scan/     # Security scanning
├── workflows/
│   └── reusable/              # Reusable workflows
│       ├── _quality-checks.yml
│       ├── _test.yml
│       └── _security-scan.yml
├── workflow-templates/        # Workflow templates
│   ├── basic-ci.yml
│   └── comprehensive-ci.yml
├── config/                     # Shared configurations
│   └── test-matrix.json
└── README.md                  # This file
```

## Composite Actions

### setup-rust
Sets up Rust toolchain with sccache and optional components.

```yaml
- uses: ./.github/actions/setup-rust
  with:
    toolchain: 'stable'  # or 'beta', 'nightly', or specific version
    components: 'rustfmt,clippy'
    targets: 'x86_64-unknown-linux-gnu'
```

### setup-cache
Configures caching for Cargo registry and target directories.

```yaml
- uses: ./.github/actions/setup-cache
  with:
    cache-target: true
    cache-registry: true
    cache-key-suffix: 'optional-suffix'
```

### run-clippy
Runs cargo clippy with configurable options.

```yaml
- uses: ./.github/actions/run-clippy
  with:
    args: '--all-targets --all-features -- -D warnings'
    fix: false
    allow-dirty: false
```

### run-tests
Runs cargo tests with nextest support.

```yaml
- uses: ./.github/actions/run-tests
  with:
    package: 'code_guardian_core'  # optional
    features: '--all-features'
    nextest: true
```

### generate-coverage
Generates test coverage reports.

```yaml
- uses: ./.github/actions/generate-coverage
  with:
    format: 'lcov'  # or 'html', 'text'
    threshold: 82
```

### build-workspace
Builds the entire Cargo workspace.

```yaml
- uses: ./.github/actions/build-workspace
  with:
    release: false
    features: '--all-features'
    targets: '--all-targets'
```

### run-security-scan
Runs comprehensive security scanning.

```yaml
- uses: ./.github/actions/run-security-scan
  with:
    audit: true
    deny: true
    gitleaks: true
    clippy-security: true
```

## Reusable Workflows

### _quality-checks.yml
Runs formatting, clippy, and workspace checks.

```yaml
jobs:
  quality:
    uses: ./.github/workflows/reusable/_quality-checks.yml
    with:
      auto-fix: false
      fail-on-warnings: true
```

### _test.yml
Runs cross-platform testing with coverage.

```yaml
jobs:
  test:
    uses: ./.github/workflows/reusable/_test.yml
    with:
      os: '["ubuntu-latest", "windows-latest", "macos-latest"]'
      rust-version: '["stable"]'
      coverage: true
      coverage-threshold: 82
```

### _security-scan.yml
Runs security scanning tools.

```yaml
jobs:
  security:
    uses: ./.github/workflows/reusable/_security-scan.yml
    with:
      audit: true
      deny: true
      gitleaks: true
      clippy-security: true
```

## Workflow Templates

### Basic CI Template
For simple projects needing basic quality checks and testing.

```yaml
# Copy from .github/workflow-templates/basic-ci.yml
name: Basic CI
# ... rest of template
```

### Comprehensive CI Template
For production-ready projects with full CI/CD features.

```yaml
# Copy from .github/workflow-templates/comprehensive-ci.yml
name: Comprehensive CI
# ... rest of template
```

## Shared Configurations

### test-matrix.json
Contains predefined test matrices for different scenarios.

```json
{
  "os": ["ubuntu-latest", "windows-latest", "macos-latest"],
  "rust": ["stable"],
  "include": [
    {
      "os": "ubuntu-latest",
      "rust": "beta"
    }
  ]
}
```

## Usage Examples

### Simple CI Pipeline
```yaml
name: CI
on: [push, pull_request]

jobs:
  quality:
    uses: ./.github/workflows/reusable/_quality-checks.yml

  test:
    uses: ./.github/workflows/reusable/_test.yml
    with:
      os: '["ubuntu-latest"]'
      coverage: true
```

### Advanced CI Pipeline
```yaml
name: Advanced CI
on: [push, pull_request]

jobs:
  changes:
    # Change detection logic
    outputs:
      src: ${{ steps.filter.outputs.src }}

  quality:
    uses: ./.github/workflows/reusable/_quality-checks.yml
    with:
      auto-fix: ${{ github.ref == 'refs/heads/main' }}

  test:
    uses: ./.github/workflows/reusable/_test.yml
    needs: [changes, quality]
    if: needs.changes.outputs.src == 'true'

  security:
    uses: ./.github/workflows/reusable/_security-scan.yml
    needs: changes
    if: needs.changes.outputs.src == 'true'
```

## Best Practices

1. **Use reusable workflows** for common patterns to reduce duplication
2. **Leverage composite actions** for repeated setup steps
3. **Configure caching** to improve build times
4. **Use change detection** to skip unnecessary jobs
5. **Implement auto-fixing** only on protected branches
6. **Set appropriate permissions** with least privilege
7. **Use concurrency controls** to prevent overlapping runs

## Maintenance

- Keep actions and workflows updated with latest best practices
- Test changes in a separate branch before merging
- Document any breaking changes
- Review and update shared configurations regularly