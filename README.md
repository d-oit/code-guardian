# Code-Guardian

[![CI](https://github.com/d-oit/code-guardian/actions/workflows/enhanced-ci.yml/badge.svg)](https://github.com/d-oit/code-guardian/actions/workflows/enhanced-ci.yml)
[![Crates.io](https://img.shields.io/crates/v/code-guardian-cli.svg)](https://crates.io/crates/code-guardian-cli)
[![License](https://img.shields.io/crates/l/code-guardian-cli.svg)](https://github.com/d-oit/code-guardian/blob/main/LICENSE)

A fast, modular CLI tool for scanning codebases to detect non-productive code.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [System Requirements](#system-requirements)
- [Performance Benchmarks](#performance-benchmarks)
- [Usage](#usage)
- [Advanced Usage](#advanced-usage)
- [Supported Patterns](#supported-patterns)
- [Output Formats](#output-formats)
- [Architecture](#architecture)
- [Development](#development)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Branch Protection](#branch-protection)
- [License](#license)

## Features

- 🔍 **Pattern Detection**: Scan for TODO, FIXME, and other customizable patterns
- 📊 **Multiple Output Formats**: Support for text, JSON, CSV, Markdown, and HTML
- 💾 **Persistent Storage**: SQLite-based scan history and comparison
- ⚡ **High Performance**: Parallel processing with Rust and Rayon
- 🏗️ **Modular Architecture**: Clean separation of concerns across crates
- 🌐 **Distributed Scanning**: Handle large codebases with distributed processing
- 🔄 **Incremental Scanning**: Efficient rescanning of changed files only
- 📈 **Performance Benchmarking**: Built-in benchmarks and optimization recommendations
- 🚀 **Production Readiness**: Checks and CI/CD integration for production environments
- 🛠️ **Custom Detectors**: JSON-configurable custom pattern detectors
- ⚙️ **Advanced Scanning Options**: Streaming, optimized, and metrics-based scanning
- 🏷️ **Technology Stack Presets**: Presets for web, backend, fullstack, mobile, and systems
- 🌍 **Multi-Language Support**: Scanning for Rust, JavaScript, TypeScript, Python, Go, Java, C#, PHP and 20+ other programming languages

## Installation

### From Source

```bash
git clone https://github.com/d-oit/code-guardian
cd code-guardian
cargo build --release
```

The binary will be available at `target/release/code-guardian`.

### Using Cargo Install

```bash
cargo install code-guardian
```

This will download, compile, and install the binary to your Cargo bin directory (usually `~/.cargo/bin/`).

### System Requirements

- **Minimum Rust Version**: 1.70.0 (Rust 2021 edition)
- **Supported Platforms**: Linux, macOS, Windows
- **Memory**: 50MB+ recommended for large codebases

### Performance Benchmarks

Code-Guardian is optimized for speed and efficiency. Here are typical performance metrics:

| Metric | Small Project (1k files) | Medium Project (10k files) | Large Project (100k files) |
|--------|--------------------------|----------------------------|----------------------------|
| Scan Duration | ~2.3 seconds | ~18.7 seconds | ~2.6 minutes |
| Memory Usage | ~45MB | ~67MB | ~87MB |
| Throughput | ~434 files/second | ~535 files/second | ~641 files/second |

For detailed performance data and optimization recommendations, see [Performance Benchmarks](docs/performance/latest.md).

## Usage

### Scan a Directory

```bash
code-guardian scan /path/to/your/project
```

### View Scan History

```bash
code-guardian history
```

### Generate Reports

```bash
# Text format (default)
code-guardian report 1

# JSON format
code-guardian report 1 --format json

# HTML format
code-guardian report 1 --format html
```

### Compare Scans

```bash
code-guardian compare 1 2 --format markdown
```

## Advanced Usage

### Custom Database Location

By default, scans are stored in `data/code-guardian.db`. You can specify a custom database path:

```bash
code-guardian scan /path/to/project --db /custom/path/my-scans.db
code-guardian history --db /custom/path/my-scans.db
code-guardian report 1 --db /custom/path/my-scans.db --format json
```

### Piping and Redirecting Output

Redirect reports to files for further processing:

```bash
# Save HTML report to file
code-guardian report 1 --format html > scan-report.html

# Pipe JSON output to jq for filtering
code-guardian report 1 --format json | jq '.matches[] | select(.pattern == "TODO")'

# Export CSV for spreadsheet analysis
code-guardian report 1 --format csv > scan-results.csv
```

### Automating Scans with Scripts

Create a bash script for regular scanning:

```bash
#!/bin/bash
# daily-scan.sh
PROJECT_DIR="/path/to/your/project"
DB_PATH="$HOME/code-guardian-scans.db"

echo "Running daily code scan..."
code-guardian scan "$PROJECT_DIR" --db "$DB_PATH"
SCAN_ID=$(code-guardian history --db "$DB_PATH" | tail -1 | awk '{print $2}' | tr -d ',')

echo "Generating reports..."
code-guardian report "$SCAN_ID" --db "$DB_PATH" --format html > "scan-$(date +%Y%m%d).html"
code-guardian report "$SCAN_ID" --db "$DB_PATH" --format json > "scan-$(date +%Y%m%d).json"

echo "Scan complete. Reports saved."
```

### Comparing Scan Results Over Time

Track progress by comparing scans:

```bash
# Compare last two scans
LATEST_ID=$(code-guardian history | tail -1 | awk '{print $2}' | tr -d ',')
PREVIOUS_ID=$(code-guardian history | tail -2 | head -1 | awk '{print $2}' | tr -d ',')

code-guardian compare "$PREVIOUS_ID" "$LATEST_ID" --format markdown
```

### Integrating with CI/CD

The project includes an enhanced CI/CD pipeline that combines the best features from multiple workflows:

- **Enhanced CI/CD Workflow** (`enhanced-ci.yml`): Combines features from `optimized-ci.yml`, `security.yml`, `performance.yml`, and `auto-fix.yml`
- **Concurrency Controls**: Prevents overlapping runs
- **Least Privilege Permissions**: Enhanced security
- **Auto-fix Capabilities**: Automatically fixes formatting and clippy issues
- **Comprehensive Testing**: Cross-platform testing with incremental builds
- **Security Scanning**: Cargo audit, deny, and security-focused clippy
- **Performance Benchmarking**: Build time and binary size optimization
- **Coverage Thresholds**: Enforces 82%+ test coverage

Example integration for scanning TODOs in CI:

```yaml
# .github/workflows/enhanced-ci.yml
- name: Scan for TODOs
  run: |
    ./code-guardian scan . --db /tmp/scans.db
    SCAN_ID=$(./code-guardian history --db /tmp/scans.db | tail -1 | awk '{print $2}' | tr -d ',')
    COUNT=$(./code-guardian report "$SCAN_ID" --db /tmp/scans.db --format json | jq '.matches | length')
    if [ "$COUNT" -gt 10 ]; then
      echo "Too many TODOs found: $COUNT"
      exit 1
    fi
```

### Benchmarking

Run performance benchmarks to assess scanning speed and receive optimization recommendations:

```bash
code-guardian benchmark --quick
```

### Production Readiness Checks

Perform production readiness checks with configurable severity levels:

```bash
code-guardian production-check --severity high
```

### Incremental Scanning

Efficiently rescan only changed files for faster subsequent scans:

```bash
code-guardian scan /path --incremental
```

### Distributed Scanning

Distribute scanning across multiple processes for large codebases:

```bash
code-guardian scan /path --distributed
```

## Supported Patterns

- **TODO**: Tasks that need to be completed
- **FIXME**: Code that needs to be fixed
- **HACK**: Temporary workarounds
- **BUG**: Known bugs
- **XXX**: Critical issues
- **PANIC**: Rust panic calls
- **UNWRAP**: Rust unwrap calls
- **UNSAFE**: Rust unsafe blocks
- **Custom Patterns**: Define your own patterns via configuration files

### Custom Detectors

Code-Guardian supports custom pattern detectors for detecting project-specific issues:

```bash
# Create example custom detectors
code-guardian custom-detectors create-examples

# Scan with custom detectors
code-guardian scan /path/to/project --custom-detectors custom_detectors.json

# List available custom detectors
code-guardian custom-detectors list
```

Custom detectors can detect security vulnerabilities, code quality issues, and more. See the [Custom Detectors Guide](docs/tutorials/custom-detectors.md) for details.

## Output Formats

- **text**: Human-readable console output
- **json**: Machine-readable JSON format
- **csv**: Spreadsheet-compatible CSV format
- **markdown**: Documentation-friendly Markdown tables
- **html**: Web-friendly HTML tables

## Architecture

The project follows a modular architecture with separate crates:

- **`core`**: Scanning logic, pattern detection, custom detectors, distributed scanning, incremental scanning, performance optimization, enhanced configuration
- **`storage`**: SQLite database operations, scan persistence, and migrations
- **`output`**: Multiple output format support (text, json, csv, markdown, html)
- **`cli`**: Command-line interface with handlers for scanning, reporting, comparisons, benchmarks, production usage, advanced features

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

## Documentation

- [Full Documentation](docs/README.md)
- [Getting Started Tutorial](docs/tutorials/getting-started.md)
- [Advanced Usage](docs/tutorials/advanced-usage.md)
- [Custom Detectors Guide](docs/tutorials/custom-detectors.md)
- [Automation Guide](docs/tutorials/automation.md)
- [API Docs](https://d-oit.github.io/code-guardian/) (GitHub Pages)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed contribution guidelines.

Quick checklist:
1. Follow the guidelines in `AGENTS.md`
2. Keep modules under 500 lines of code
3. Maintain 82%+ test coverage
4. Use conventional commit messages

## Branch Protection

To ensure code quality and security, this repository employs branch protection rules aligned with 2025 best practices. These include requiring 2 approvals for pull requests, signed commits, and passing all status checks (such as CI/CD, linting, and tests).

For detailed setup instructions, refer to [BRANCH_PROTECTION_SETUP.md](BRANCH_PROTECTION_SETUP.md).

## License

[MIT](LICENSE)

