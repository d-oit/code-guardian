# Code Guardian API Documentation

This document provides comprehensive API documentation for the Code Guardian library and CLI tool.

## Table of Contents

- [Core Library API](#core-library-api)
- [CLI Interface](#cli-interface)
- [Configuration API](#configuration-api)
- [Detector API](#detector-api)
- [Performance Monitoring](#performance-monitoring)
- [Health Checks](#health-checks)
- [Examples](#examples)

## Core Library API

### Scanner Module

The main scanning functionality is provided through the core scanner:

```rust
use code_guardian_core::{Scanner, Match, Severity};
use std::path::Path;

// Basic usage
let scanner = Scanner::new();
let matches = scanner.scan_path(Path::new("src/")).unwrap();

for m in matches {
    println!("Found {} at {}:{}", m.pattern, m.file_path, m.line_number);
}
```

#### Scanner Methods

| Method | Description | Parameters | Returns |
|--------|-------------|------------|---------|
| `new()` | Create a new scanner with default configuration | None | `Scanner` |
| `with_config(config)` | Create scanner with custom configuration | `Config` | `Scanner` |
| `scan_path(path)` | Scan a directory or file | `&Path` | `Result<Vec<Match>>` |
| `scan_content(content, filename)` | Scan string content | `&str, &str` | `Result<Vec<Match>>` |

### Match Structure

```rust
pub struct Match {
    pub file_path: String,     // Path to the file
    pub line_number: usize,    // Line number (1-based)
    pub column: usize,         // Column number (1-based)
    pub pattern: String,       // Pattern type (e.g., "TODO")
    pub message: String,       // Match description
}
```

### Severity Levels

```rust
pub enum Severity {
    Info,      // Informational
    Low,       // Low priority
    Medium,    // Medium priority
    High,      // High priority
    Critical,  // Critical issue
}
```

## CLI Interface

### Basic Commands

#### Scan Command

```bash
# Basic scan
code-guardian scan <path>

# Scan with options
code-guardian scan src/ \
  --profile security \
  --format json \
  --progress \
  --metrics
```

**Parameters:**
- `path`: Directory or file to scan (required)
- `--db`: Database path for storing results
- `--config`: Path to configuration file
- `--profile`: Scanning profile (basic, security, performance, comprehensive)
- `--format`: Output format (text, json, csv, html, markdown)
- `--progress`: Show progress bar
- `--optimize`: Enable performance optimizations
- `--streaming`: Stream results as they're found
- `--metrics`: Show performance metrics
- `--incremental`: Only scan changed files
- `--distributed`: Enable distributed scanning
- `--custom-detectors`: Path to custom detector file
- `--cache-size`: Size of result cache
- `--batch-size`: Number of files to process in batch
- `--max-file-size`: Maximum file size to scan (bytes)
- `--max-threads`: Maximum number of threads to use

#### Production Commands

```bash
# Production readiness check
code-guardian production-check src/ \
  --fail-on-critical \
  --fail-on-high \
  --format json \
  --output report.json

# Pre-commit hook
code-guardian pre-commit src/ \
  --staged-only \
  --fast

# CI/CD gate
code-guardian ci-gate src/ \
  --max-critical 0 \
  --max-high 5 \
  --output ci-report.json
```

#### Language-Specific Scanning

```bash
# Scan specific languages
code-guardian lang \
  --languages rust,javascript,python \
  src/ \
  --production \
  --format json
```

#### Monitoring Commands

```bash
# Benchmark performance
code-guardian benchmark src/ --quick

# Watch for changes
code-guardian watch src/ \
  --include "*.rs" \
  --exclude "target" \
  --delay 2

# Git integration
code-guardian git install    # Install pre-commit hook
code-guardian git uninstall  # Remove pre-commit hook
```

### Report Commands

```bash
# View scan history
code-guardian history

# Generate report from scan ID
code-guardian report <scan-id> --format html

# Compare two scans
code-guardian compare <scan-id-1> <scan-id-2> --format json
```

### Stack Presets

```bash
# Web development stack
code-guardian stack web src/ --production

# Backend development stack
code-guardian stack backend src/

# Full-stack development
code-guardian stack fullstack .

# Mobile development
code-guardian stack mobile mobile-app/

# Systems programming
code-guardian stack systems kernel/
```

## Configuration API

### TOML Configuration

```toml
# config.toml
[scan]
max_file_size = 1048576  # 1MB
max_threads = 4
batch_size = 100
exclude_patterns = ["target/", "node_modules/", ".git/"]

[[detectors]]
name = "todo"
pattern = "TODO|FIXME|HACK"
severity = "Medium"
enabled = true
file_extensions = [".rs", ".js", ".py"]

[[detectors]]
name = "security"
pattern = "password|secret|api_key"
severity = "High"
enabled = true
case_sensitive = false
```

### JSON Configuration

```json
{
  "scan": {
    "max_file_size": 1048576,
    "max_threads": 4,
    "batch_size": 100,
    "exclude_patterns": ["target/", "node_modules/", ".git/"]
  },
  "detectors": [
    {
      "name": "todo",
      "pattern": "TODO|FIXME|HACK",
      "severity": "Medium",
      "enabled": true,
      "file_extensions": [".rs", ".js", ".py"]
    }
  ]
}
```

### Programmatic Configuration

```rust
use code_guardian_core::{Config, DetectorConfig, Severity};

let mut config = Config::default();

// Add custom detector
config.detectors.push(DetectorConfig {
    name: "custom".to_string(),
    pattern: "CUSTOM_PATTERN".to_string(),
    severity: Severity::High,
    enabled: true,
    file_extensions: Some(vec![".rs".to_string()]),
    case_sensitive: Some(true),
    description: Some("Custom pattern detector".to_string()),
});

// Configure scan settings
config.scan.max_threads = Some(8);
config.scan.batch_size = Some(50);
```

## Detector API

### Built-in Detectors

| Detector | Pattern | Severity | Description |
|----------|---------|----------|-------------|
| `todo` | `TODO\|FIXME\|HACK` | Medium | Development notes |
| `console_log` | `console\.log` | Low | Debug statements |
| `unwrap` | `\.unwrap\(\)` | Medium | Unsafe unwrap calls |
| `panic` | `panic!` | High | Panic statements |
| `debugger` | `debugger` | Medium | Debugger statements |
| `experimental` | `#\[experimental\]` | Info | Experimental features |

### LLM Security Detectors

| Detector | Description | Severity |
|----------|-------------|----------|
| `hallucinated_api` | Non-existent API calls | High |
| `sql_injection` | SQL injection vulnerabilities | Critical |
| `insecure_random` | Non-cryptographic random usage | High |
| `hardcoded_credentials` | Embedded secrets | Critical |
| `memory_safety` | Unsafe memory operations | High |
| `crypto_antipattern` | Weak cryptography | High |
| `xss_injection` | XSS vulnerabilities | Critical |
| `filesystem_security` | Path traversal issues | High |

### Custom Detectors

```rust
use code_guardian_core::{PatternDetector, Match};

struct CustomDetector {
    pattern: String,
}

impl PatternDetector for CustomDetector {
    fn detect(&self, content: &str, file_path: &str) -> Vec<Match> {
        // Implementation
        vec![]
    }
    
    fn name(&self) -> &str {
        "custom"
    }
    
    fn severity(&self) -> Severity {
        Severity::Medium
    }
}
```

### Detector Profiles

```rust
use code_guardian_core::detector_factory::{DetectorFactory, DetectorProfile};

// Security-focused profile
let detectors = DetectorFactory::create_profile_detectors(DetectorProfile::Security)?;

// Performance-focused profile
let detectors = DetectorFactory::create_profile_detectors(DetectorProfile::Performance)?;

// Comprehensive profile (all detectors)
let detectors = DetectorFactory::create_profile_detectors(DetectorProfile::Comprehensive)?;
```

## Performance Monitoring

### Performance Dashboard

```rust
use code_guardian_core::performance_dashboard::{PerformanceDashboard, DashboardConfig};
use std::time::Duration;

// Create dashboard
let mut dashboard = PerformanceDashboard::new_with_defaults();

// Record scan metrics
dashboard.record_scan_metrics(
    Duration::from_millis(1500),  // scan duration
    250,                          // files processed
    15000,                        // lines processed
    42,                           // matches found
)?;

// Generate HTML dashboard
let html = dashboard.generate_html_dashboard();
std::fs::write("dashboard.html", html)?;

// Export metrics as JSON
let json = dashboard.export_metrics_json()?;
```

### Performance Metrics

```rust
pub struct PerformanceMetrics {
    pub timestamp: u64,
    pub scan_duration_ms: u64,
    pub files_processed: usize,
    pub lines_processed: usize,
    pub matches_found: usize,
    pub memory_usage_mb: u64,
    pub cpu_usage_percent: f64,
    pub throughput_files_per_second: f64,
    pub throughput_lines_per_second: f64,
}
```

## Health Checks

### Health Check API

```rust
use code_guardian_core::health_checks::{HealthChecker, HealthCheckConfig};

// Create health checker
let checker = HealthChecker::new_with_defaults();

// Perform health check
let health_report = checker.check_health().await?;

println!("Overall status: {:?}", health_report.overall_status);
for check in health_report.checks {
    println!("  {}: {:?} - {}", check.name, check.status, check.message);
}

// Readiness probe (for load balancers)
let is_ready = checker.check_readiness().await?;

// Liveness probe (for container orchestrators)
let is_alive = checker.check_liveness().await?;
```

### Health Check Configuration

```rust
use code_guardian_core::health_checks::HealthCheckConfig;
use std::collections::HashMap;

let mut config = HealthCheckConfig::default();
config.enabled_checks = vec![
    "database".to_string(),
    "memory".to_string(),
    "detectors".to_string(),
];

// Configure alert thresholds
config.alert_thresholds.insert("max_memory_usage_mb".to_string(), 512.0);
config.alert_thresholds.insert("max_scan_duration_ms".to_string(), 10000.0);
```

## Examples

### Basic Library Usage

```rust
use code_guardian_core::{Scanner, Config};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = Config::load(Some("config.toml"))?;
    
    // Create scanner
    let scanner = Scanner::with_config(config);
    
    // Scan directory
    let matches = scanner.scan_path(Path::new("src/"))?;
    
    // Process results
    for m in matches {
        println!("{}:{}:{} - {} ({})", 
            m.file_path, 
            m.line_number, 
            m.column, 
            m.message, 
            m.pattern
        );
    }
    
    Ok(())
}
```

### Custom Integration

```rust
use code_guardian_core::{Scanner, Match, Severity};
use code_guardian_output::formatters::JsonFormatter;

async fn scan_and_report(path: &str) -> anyhow::Result<String> {
    // Scan
    let scanner = Scanner::new();
    let matches = scanner.scan_path(Path::new(path))?;
    
    // Filter high-severity matches
    let high_severity: Vec<Match> = matches
        .into_iter()
        .filter(|m| matches_severity(m) >= Severity::High)
        .collect();
    
    // Format as JSON
    let formatter = JsonFormatter::new();
    let json_output = formatter.format(&high_severity)?;
    
    Ok(json_output)
}

fn matches_severity(m: &Match) -> Severity {
    match m.pattern.as_str() {
        "panic" => Severity::Critical,
        "unwrap" => Severity::High,
        "todo" => Severity::Medium,
        _ => Severity::Low,
    }
}
```

### CI/CD Integration

```bash
#!/bin/bash
# CI script example

# Install Code Guardian
cargo install code-guardian-cli

# Run production check
code-guardian production-check . \
  --fail-on-critical \
  --format json \
  --output security-report.json

# Exit with error if critical issues found
if [ $? -ne 0 ]; then
    echo "Critical security issues found!"
    exit 1
fi

# Generate dashboard
code-guardian scan . --metrics --format html --output dashboard.html

echo "Security scan completed successfully"
```

### Pre-commit Hook Integration

```bash
#!/bin/sh
# .git/hooks/pre-commit

# Run Code Guardian on staged files
code-guardian pre-commit . --staged-only --fast

if [ $? -ne 0 ]; then
    echo "Code quality issues found. Please fix before committing."
    exit 1
fi
```

## Error Handling

All API functions return `Result<T, E>` types for proper error handling:

```rust
use anyhow::Result;

fn example() -> Result<()> {
    let scanner = Scanner::new();
    let matches = scanner.scan_path(Path::new("nonexistent/"))?;
    Ok(())
}
```

Common error types:
- `ConfigError`: Configuration file issues
- `ScanError`: Scanning failures
- `DetectorError`: Detector creation/execution errors
- `FormatError`: Output formatting errors

## Performance Tips

1. **Use incremental scanning** for large codebases:
   ```bash
   code-guardian scan . --incremental
   ```

2. **Configure thread limits** for resource-constrained environments:
   ```bash
   code-guardian scan . --max-threads 2
   ```

3. **Use appropriate profiles** for your use case:
   ```bash
   code-guardian scan . --profile security  # Faster, security-focused
   ```

4. **Enable caching** for repeated scans:
   ```bash
   code-guardian scan . --cache-size 1000
   ```

5. **Filter file sizes** to avoid processing large files:
   ```bash
   code-guardian scan . --max-file-size 1048576  # 1MB limit
   ```

## Support

- **Documentation**: [GitHub Wiki](https://github.com/user/code-guardian/wiki)
- **Issues**: [GitHub Issues](https://github.com/user/code-guardian/issues)
- **Discussions**: [GitHub Discussions](https://github.com/user/code-guardian/discussions)
- **CLI Help**: `code-guardian --help`