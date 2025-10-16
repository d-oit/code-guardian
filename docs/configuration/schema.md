# Configuration Schema Documentation

This document describes the configuration options available in Code-Guardian.

## Configuration File Format

Code-Guardian supports multiple configuration formats:
- TOML (recommended): `config.toml`
- JSON: `config.json`
- YAML: `config.yaml`

## Schema Overview

```toml
[scanning]
# Scanning behavior configuration
max_file_size = 10485760  # 10MB in bytes
max_threads = 4
batch_size = 100
cache_size = 1000
streaming = false
incremental = true

[detectors]
# Detector configuration
enabled = ["security", "performance", "maintainability"]
custom_detectors_path = "./custom_detectors.json"

[security]
# Security-specific settings
fail_on_critical = true
fail_on_high = false
severity_threshold = "Medium"

[performance]
# Performance optimization settings
enable_parallel_scanning = true
memory_limit_mb = 512
timeout_seconds = 300

[output]
# Output formatting options
default_format = "text"
include_metrics = true
show_progress = true

[logging]
# Logging configuration
level = "info"
format = "json"
correlation_ids = true

[database]
# Database settings
path = "./code_guardian.db"
enable_migrations = true
backup_before_migration = true

[git]
# Git integration settings
pre_commit_enabled = true
staged_files_only = true
fast_mode = false

[llm_detection]
# LLM-specific vulnerability detection
enabled = true
profiles = ["security", "quality", "comprehensive"]
```

## Configuration Sections

### Scanning Configuration

Controls how files are scanned and processed.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `max_file_size` | integer | 10485760 | Maximum file size to scan (bytes) |
| `max_threads` | integer | CPU cores | Maximum number of scanning threads |
| `batch_size` | integer | 100 | Number of files to process in each batch |
| `cache_size` | integer | 1000 | Number of scan results to cache |
| `streaming` | boolean | false | Enable streaming scan results |
| `incremental` | boolean | true | Only scan changed files |

### Detector Configuration

Manages which detectors are enabled and their settings.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | array | ["comprehensive"] | List of enabled detector profiles |
| `custom_detectors_path` | string | null | Path to custom detector definitions |

Available detector profiles:
- `security`: Security-focused detectors
- `performance`: Performance issue detectors  
- `maintainability`: Code quality detectors
- `comprehensive`: All built-in detectors
- `llm-security`: LLM-specific security detectors
- `llm-quality`: LLM-specific quality detectors
- `production-ready-llm`: Production + LLM detectors

### Security Configuration

Security scanning behavior and thresholds.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `fail_on_critical` | boolean | true | Exit with error on critical issues |
| `fail_on_high` | boolean | false | Exit with error on high severity issues |
| `severity_threshold` | string | "Medium" | Minimum severity to report |

Severity levels: `Critical`, `High`, `Medium`, `Low`, `Info`

### Performance Configuration

Performance and resource usage settings.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enable_parallel_scanning` | boolean | true | Use multiple threads for scanning |
| `memory_limit_mb` | integer | 512 | Maximum memory usage (MB) |
| `timeout_seconds` | integer | 300 | Scan timeout (seconds) |

### Output Configuration

Output formatting and display options.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `default_format` | string | "text" | Default output format |
| `include_metrics` | boolean | true | Include performance metrics |
| `show_progress` | boolean | true | Show progress indicators |

Supported formats: `text`, `json`, `html`, `markdown`, `csv`

### Logging Configuration

Logging behavior and format settings.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `level` | string | "info" | Log level |
| `format` | string | "json" | Log format |
| `correlation_ids` | boolean | true | Include correlation IDs |

Log levels: `error`, `warn`, `info`, `debug`, `trace`

### Database Configuration

Database connection and migration settings.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `path` | string | "./code_guardian.db" | Database file path |
| `enable_migrations` | boolean | true | Run migrations automatically |
| `backup_before_migration` | boolean | true | Backup before schema changes |

### Git Integration Configuration

Git workflow integration settings.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `pre_commit_enabled` | boolean | true | Enable pre-commit hooks |
| `staged_files_only` | boolean | true | Only scan staged files |
| `fast_mode` | boolean | false | Skip expensive checks |

### LLM Detection Configuration

LLM-specific vulnerability detection settings.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | boolean | true | Enable LLM detection |
| `profiles` | array | ["comprehensive"] | LLM detector profiles to use |

## Environment Variable Override

All configuration options can be overridden using environment variables with the prefix `CODE_GUARDIAN_`:

```bash
export CODE_GUARDIAN_SCANNING_MAX_THREADS=8
export CODE_GUARDIAN_SECURITY_FAIL_ON_HIGH=true
export CODE_GUARDIAN_OUTPUT_DEFAULT_FORMAT=json
```

## Configuration Validation

Code-Guardian validates configuration on startup and will report errors for:
- Invalid option values
- Unknown configuration keys
- Type mismatches
- Missing required dependencies

Use `code-guardian config validate` to check configuration without running a scan.

## Migration Guide

### From v0.1.x to v0.2.x
- `detector_profiles` renamed to `detectors.enabled`
- `output_format` moved to `output.default_format`
- Added LLM detection configuration section

### Best Practices
1. Use TOML format for human-readable configs
2. Store sensitive values in environment variables
3. Use incremental scanning for large codebases
4. Enable streaming for real-time feedback
5. Configure appropriate memory limits for your environment