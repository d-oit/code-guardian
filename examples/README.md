# Examples

This directory contains examples of how to use the Code-Guardian CLI tool, including configuration files and custom detector definitions.

## Basic Usage

### Scan a directory
```bash
cargo run --bin code_guardian_cli -- scan ./src
```

### View scan history
```bash
cargo run --bin code_guardian_cli -- history
```

### Generate a report
```bash
cargo run --bin code_guardian_cli -- report 1 --format json
```

### Compare two scans
```bash
cargo run --bin code_guardian_cli -- compare 1 2 --format markdown
```

## Sample Output

When you run a scan, you'll see output like:

```
Scan completed and saved with ID: 1
Found 3 matches:
src/main.rs:42:5 - TODO: Implement error handling
src/lib.rs:15:1 - FIXME: This function needs optimization
src/utils.rs:8:9 - TODO: Add documentation
```

## Using Different Output Formats

### JSON Output
```bash
cargo run --bin code_guardian_cli -- report 1 --format json
```

### HTML Output
```bash
cargo run --bin code_guardian_cli -- report 1 --format html > report.html
```

### CSV Output
```bash
cargo run --bin code_guardian_cli -- report 1 --format csv > report.csv
```

## Custom Detectors

### Using Custom Detectors

```bash
# Create example custom detectors
cargo run --bin code_guardian_cli -- custom-detectors create-examples --output custom_detectors.json

# Scan with custom detectors
cargo run --bin code_guardian_cli -- scan ./src --custom-detectors custom_detectors.json

# List available custom detectors
cargo run --bin code_guardian_cli -- custom-detectors list
```

### Configuration Files

#### Custom Config with Simple Patterns

Use `custom_config.toml` for basic custom patterns:

```bash
cargo run --bin code_guardian_cli -- scan ./src --config custom_config.toml
```

#### Advanced Custom Detectors

Use `custom_detectors.json` for full-featured custom detectors with severity levels, categories, and file filtering:

```bash
cargo run --bin code_guardian_cli -- scan ./src --custom-detectors custom_detectors.json
```

### Example Files

- `custom_config.toml`: Basic configuration with simple custom patterns
- `custom_detectors.json`: Advanced custom detector definitions

See the [Custom Detectors Guide](../docs/tutorials/custom-detectors.md) for detailed documentation.