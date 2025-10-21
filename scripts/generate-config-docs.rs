#!/usr/bin/env cargo +nightly -Zscript

//! Configuration Documentation Generator
//! 
//! This script generates comprehensive documentation for Code Guardian's
//! configuration schema from the actual configuration structs.

use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating configuration documentation...");
    
    let config_docs = generate_config_documentation();
    
    // Ensure docs/configuration directory exists
    fs::create_dir_all("docs/src/configuration")?;
    
    // Write the generated documentation
    fs::write("docs/src/configuration/schema.md", config_docs)?;
    
    println!("Configuration documentation generated successfully!");
    println!("Location: docs/src/configuration/schema.md");
    
    Ok(())
}

fn generate_config_documentation() -> String {
    format!(r#"# Configuration Schema

Code Guardian uses a flexible configuration system that supports multiple formats (TOML, JSON, YAML) and environment-based overrides.

## Configuration File Locations

Code Guardian searches for configuration files in the following order:

1. `./code-guardian.toml` (current directory)
2. `./config/default.toml` 
3. `./config/{{environment}}.toml` (where environment = `$ENVIRONMENT` or "development")
4. Environment variables with `CODE_GUARDIAN_` prefix

## Complete Configuration Reference

### Root Configuration

```toml
# code-guardian.toml

[scanning]
# Enable parallel processing (default: true)
parallel = true

# Maximum file size to scan (default: "10MB")
max_file_size = "10MB"

# File patterns to include (default: all supported file types)
include_patterns = ["**/*.rs", "**/*.js", "**/*.py", "**/*.sql"]

# File patterns to exclude (default: common ignore patterns)
exclude_patterns = ["**/target/**", "**/node_modules/**", "**/.git/**"]

# Maximum depth for directory recursion (default: 100)
max_depth = 100

# Timeout for individual file scanning in seconds (default: 30)
scan_timeout = 30
```

### Detector Configuration

```toml
[detectors]
# Enable security detectors (default: true)
security = true

# Enable performance detectors (default: true)
performance = true

# Enable LLM-specific detectors (default: true)
llm_detection = true

# Custom detector configuration files
custom_detectors = ["./custom-rules.json"]

# Detector-specific settings
[detectors.security]
# SQL injection detection sensitivity (low/medium/high)
sql_injection_sensitivity = "medium"

# Hardcoded credentials detection patterns
credentials_patterns = ["password", "api_key", "secret", "token"]

[detectors.llm]
# Enable hallucinated API detection (default: true)
hallucinated_apis = true

# Enable LLM SQL injection detection (default: true)
llm_sql_injection = true

# Enable async anti-pattern detection (default: true)
async_antipatterns = true

# Enable over-engineering detection (default: true)
overengineering = true
```

### Output Configuration

```toml
[output]
# Default output format (json/csv/html/markdown/text)
format = "json"

# Output file path (default: stdout)
file = "./scan-results.json"

# Include source code snippets in output (default: false)
include_snippets = false

# Maximum lines of context around issues (default: 3)
context_lines = 3

# Group results by file or severity (file/severity/detector)
group_by = "severity"
```

### Storage Configuration

```toml
[storage]
# Database URL (default: "data/code-guardian.db")
database_url = "data/code-guardian.db"

# Enable result caching (default: true)
enable_caching = true

# Cache TTL in seconds (default: 3600 = 1 hour)
cache_ttl = 3600

# Maximum cache size in MB (default: 100)
max_cache_size = 100
```

### Performance Configuration

```toml
[performance]
# Number of worker threads (default: num_cpus)
worker_threads = 8

# Memory limit in MB (default: 1024)
memory_limit = 1024

# Enable memory usage monitoring (default: true)
monitor_memory = true

# Enable performance profiling (default: false)
enable_profiling = false
```

### Logging Configuration

```toml
[logging]
# Log level (trace/debug/info/warn/error)
level = "info"

# Log format (json/text)
format = "json"

# Log output file (default: stderr)
file = "./code-guardian.log"

# Enable correlation IDs (default: true)
correlation_ids = true
```

### CI/CD Integration

```toml
[ci_cd]
# Fail build on issues above this severity (low/medium/high/critical)
fail_threshold = "high"

# Generate reports for CI systems (default: true)
generate_reports = true

# Upload results to external systems (default: false)
upload_results = false

# Maximum scan time in minutes before timeout (default: 30)
max_scan_time = 30
```

## Environment Variables

All configuration options can be overridden using environment variables with the `CODE_GUARDIAN_` prefix:

```bash
# Override scanning parallel setting
export CODE_GUARDIAN_SCANNING__PARALLEL=false

# Override detector security setting  
export CODE_GUARDIAN_DETECTORS__SECURITY=true

# Override output format
export CODE_GUARDIAN_OUTPUT__FORMAT=html

# Override logging level
export CODE_GUARDIAN_LOGGING__LEVEL=debug
```

## Scanning Profiles

Code Guardian includes predefined scanning profiles for different use cases:

### Development Profile
```toml
[profiles.development]
# Fast scanning for development workflow
detectors = ["security", "basic_quality"]
output_format = "text"
fail_threshold = "critical"
```

### Production Profile  
```toml
[profiles.production]
# Comprehensive scanning for production readiness
detectors = ["security", "performance", "llm_detection", "compliance"]
output_format = "json"
fail_threshold = "medium"
include_snippets = true
```

### LLM Security Profile
```toml
[profiles.llm_security]
# Focus on LLM-generated code vulnerabilities
detectors = ["llm_detection", "security"]
llm_detection_mode = "comprehensive"
output_format = "html"
fail_threshold = "high"
```

## Example Configurations

### Basic Development Setup
```toml
# code-guardian.toml
[scanning]
parallel = true
max_file_size = "5MB"

[detectors]
security = true
performance = false
llm_detection = true

[output]
format = "text"
```

### Production CI/CD Setup
```toml
# config/production.toml
[scanning]
parallel = true
max_file_size = "20MB"
include_patterns = ["src/**/*.rs", "web/**/*.js", "api/**/*.py"]

[detectors]
security = true
performance = true
llm_detection = true

[detectors.security]
sql_injection_sensitivity = "high"

[output]
format = "json"
file = "./security-report.json"
include_snippets = true

[ci_cd]
fail_threshold = "high"
max_scan_time = 15
```

### Compliance Scanning
```toml
# config/compliance.toml
[scanning]
parallel = true

[detectors]
security = true
performance = true
llm_detection = true
compliance = true

[detectors.compliance]
standards = ["SOC2", "PCI_DSS", "GDPR"]

[output]
format = "html"
file = "./compliance-report.html"
group_by = "severity"

[storage]
enable_caching = false  # Disable caching for audit trails
```

## Configuration Validation

Code Guardian validates all configuration at startup and provides helpful error messages:

```bash
# Invalid configuration example
$ code-guardian scan ./src
Error: Invalid configuration
  - detectors.sql_injection_sensitivity must be one of: low, medium, high
  - performance.worker_threads must be between 1 and 64
  - output.format must be one of: json, csv, html, markdown, text
```

## Migration Guide

### From v0.1.x to v0.2.x

- `detector_config` → `detectors`
- `output_config` → `output`  
- `scan_config` → `scanning`

Example migration:
```toml
# Old format (v0.1.x)
[detector_config]
enable_security = true

# New format (v0.2.x)
[detectors]
security = true
```

## Advanced Configuration

### Custom Detector Integration
```toml
[detectors.custom]
# Path to custom detector definitions
config_files = ["./detectors/custom-security.json", "./detectors/custom-quality.json"]

# Custom detector priority (1-10, higher = more important)
priority = 5
```

### Performance Tuning
```toml
[performance]
# Optimize for memory usage vs speed
optimization_mode = "memory"  # or "speed" or "balanced"

# Custom memory allocator settings
[performance.memory]
pool_size = "256MB"
gc_threshold = 0.8
```

### Distributed Scanning
```toml
[distributed]
# Enable distributed scanning
enabled = true

# Coordinator node address
coordinator = "http://coordinator:8080"

# Worker node settings
[distributed.worker]
max_concurrent_scans = 4
heartbeat_interval = 30
```
"#)
}