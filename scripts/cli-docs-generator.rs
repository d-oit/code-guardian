use clap::Command;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This would be a separate binary crate for generating CLI docs
    // For now, this is a placeholder

    let app = code_guardian_cli::build_cli_app();
    let help_text = get_help_text(&app);

    fs::write("docs/api/cli-commands.md", help_text)?;
    Ok(())
}

fn get_help_text(_app: &Command) -> String {
    // Generate comprehensive CLI documentation
    "# CLI Commands Reference

This document provides detailed reference for all Code-Guardian CLI commands.

## Global Options

- `--help`: Show help information
- `--version`: Show version information
- `--verbose`: Enable verbose output
- `--quiet`: Suppress non-error output

## Commands

### scan

Scan files and directories for security issues.

```bash
code-guardian scan [OPTIONS] <PATH>
```

**Arguments:**
- `<PATH>`: Path to scan (file or directory)

**Options:**
- `--format <FORMAT>`: Output format [default: text] [possible values: text, json, html, markdown, csv]
- `--output <FILE>`: Output file path
- `--profile <PROFILE>`: Scanning profile [default: security] [possible values: basic, security, performance, comprehensive]
- `--config <FILE>`: Configuration file path
- `--exclude <PATTERN>`: Exclude pattern (can be used multiple times)
- `--include <PATTERN>`: Include pattern (can be used multiple times)
- `--max-threads <NUM>`: Maximum number of threads [default: CPU cores]
- `--max-file-size <BYTES>`: Maximum file size to scan [default: 10485760]
- `--fail-on-critical`: Exit with error on critical issues
- `--fail-on-high`: Exit with error on high severity issues
- `--incremental`: Only scan changed files
- `--cache-size <NUM>`: Result cache size [default: 1000]
- `--progress`: Show progress bar
- `--metrics`: Show performance metrics

### report

Generate reports from scan results.

```bash
code-guardian report [OPTIONS] <SCAN_ID>
```

**Arguments:**
- `<SCAN_ID>`: Scan ID to report on

**Options:**
- `--format <FORMAT>`: Output format [possible values: text, json, html, markdown]
- `--output <FILE>`: Output file path

### history

Show scan history.

```bash
code-guardian history [OPTIONS]
```

**Options:**
- `--limit <NUM>`: Maximum number of results [default: 10]
- `--format <FORMAT>`: Output format [default: text]

### compare

Compare two scans.

```bash
code-guardian compare [OPTIONS] <SCAN_ID_1> <SCAN_ID_2>
```

**Arguments:**
- `<SCAN_ID_1>`: First scan ID
- `<SCAN_ID_2>`: Second scan ID

**Options:**
- `--format <FORMAT>`: Output format [default: text]
- `--output <FILE>`: Output file path

### config

Configuration management.

```bash
code-guardian config <SUBCOMMAND>
```

**Subcommands:**
- `validate`: Validate configuration file
- `show`: Show current configuration
- `init`: Create default configuration file

### benchmark

Run performance benchmarks.

```bash
code-guardian benchmark [OPTIONS] <PATH>
```

**Arguments:**
- `<PATH>`: Path to benchmark

**Options:**
- `--quick`: Run quick benchmark
- `--iterations <NUM>`: Number of iterations [default: 10]
- `--format <FORMAT>`: Output format [default: text]

## Examples

### Basic Scan
```bash
code-guardian scan src/
```

### Security-Focused Scan with JSON Output
```bash
code-guardian scan . --profile security --format json --output report.json
```

### CI/CD Scan with Quality Gates
```bash
code-guardian scan . --fail-on-critical --fail-on-high --format junit --output results.xml
```

### Incremental Scan
```bash
code-guardian scan . --incremental --cache-size 2000
```

### Performance Benchmark
```bash
code-guardian benchmark src/ --iterations 5 --format json
```

## Exit Codes

- `0`: Success
- `1`: General error
- `2`: Security issues found (when using --fail-on-* flags)
- `3`: Configuration error
".to_string()
}