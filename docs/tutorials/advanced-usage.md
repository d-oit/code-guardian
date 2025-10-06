# Advanced Usage Tutorial

This tutorial covers advanced features of Code-Guardian, including custom databases, automation, and CI/CD integration.

## Custom Database Locations

By default, Code-Guardian uses `data/code-guardian.db`. You can specify custom locations:

```bash
# Scan with custom database
./code-guardian scan /path/to/project --db ~/my-scans.db

# View history from custom database
./code-guardian history --db ~/my-scans.db

# Generate report from custom database
./code-guardian report 1 --db ~/my-scans.db --format json
```

## Comparing Scans

Track progress by comparing different scans:

```bash
# Run initial scan
./code-guardian scan . --db project-scans.db

# Make some changes (fix TODOs), then scan again
./code-guardian scan . --db project-scans.db

# Compare the two scans
./code-guardian compare 1 2 --db project-scans.db --format markdown
```

The comparison shows new matches that appeared in the second scan.

## Automating Scans with Scripts

Create a script for regular scanning:

```bash
#!/bin/bash
# File: daily-scan.sh

PROJECT_DIR="/path/to/your/project"
DB_PATH="$HOME/code-guardian-scans.db"
OUTPUT_DIR="$HOME/scan-reports"

mkdir -p "$OUTPUT_DIR"

echo "Running daily code scan..."

# Run scan
./code-guardian scan "$PROJECT_DIR" --db "$DB_PATH"

# Get latest scan ID
SCAN_ID=$(./code-guardian history --db "$DB_PATH" | tail -1 | awk '{print $2}' | tr -d ',')

echo "Scan ID: $SCAN_ID"

# Generate reports
./code-guardian report "$SCAN_ID" --db "$DB_PATH" --format html > "$OUTPUT_DIR/scan-$(date +%Y%m%d).html"
./code-guardian report "$SCAN_ID" --db "$DB_PATH" --format json > "$OUTPUT_DIR/scan-$(date +%Y%m%d).json"

echo "Reports saved to $OUTPUT_DIR"
```

Make it executable and run daily with cron:

```bash
chmod +x daily-scan.sh
# Add to crontab: 0 9 * * 1-5 /path/to/daily-scan.sh
```

## CI/CD Integration

### GitHub Actions

Add Code-Guardian to your CI pipeline:

```yaml
# .github/workflows/code-quality.yml
name: Code Quality

on: [push, pull_request]

jobs:
  scan-todos:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4

    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Build Code-Guardian
      run: cargo build --release --bin code-guardian-cli

    - name: Scan for TODOs
      run: |
        ./target/release/code-guardian-cli scan . --db /tmp/scans.db

    - name: Check TODO count
      run: |
        SCAN_ID=$(./target/release/code-guardian-cli history --db /tmp/scans.db | tail -1 | awk '{print $2}' | tr -d ',')
        COUNT=$(./target/release/code-guardian-cli report "$SCAN_ID" --db /tmp/scans.db --format json | jq '.matches | length')
        echo "Found $COUNT TODO/FIXME items"
        ./target/release/code-guardian-cli report "$SCAN_ID" --db /tmp/scans.db --format markdown >> $GITHUB_STEP_SUMMARY

        # Optional: fail if too many TODOs
        if [ "$COUNT" -gt 50 ]; then
          echo "Too many TODOs found: $COUNT"
          exit 1
        fi
```

### Jenkins Pipeline

```groovy
pipeline {
    agent any

    stages {
        stage('Scan Code') {
            steps {
                sh '''
                    # Build Code-Guardian
                    cargo build --release --bin code-guardian-cli

                    # Run scan
                    ./target/release/code-guardian-cli scan . --db scans.db

                    # Get scan ID and generate report
                    SCAN_ID=$(./target/release/code-guardian-cli history --db scans.db | tail -1 | awk '{print $2}' | tr -d ',')
                    ./target/release/code-guardian-cli report $SCAN_ID --db scans.db --format html > scan-report.html
                '''

                publishHTML(target: [
                    allowMissing: false,
                    alwaysLinkToLastBuild: true,
                    keepAll: true,
                    reportDir: '.',
                    reportFiles: 'scan-report.html',
                    reportName: 'Code-Guardian Scan Report'
                ])
            }
        }
    }
}
```

## Custom Detectors

Create custom pattern detectors for security vulnerabilities, code quality issues, or project-specific patterns:

```bash
# Create example custom detectors
./code-guardian custom-detectors create-examples --output security_detectors.json

# Scan with custom detectors
./code-guardian scan . --custom-detectors security_detectors.json --db custom.db

# Test detectors on a specific file
./code-guardian custom-detectors test security_detectors.json problematic_file.rs
```

See the [Custom Detectors Guide](custom-detectors.md) for detailed information on creating and configuring custom detectors.

## Processing Reports Programmatically

Use JSON output for programmatic processing:

```python
#!/usr/bin/env python3
import json
import subprocess
import sys

def run_scan_and_analyze():
    # Run scan
    result = subprocess.run([
        './code-guardian', 'scan', '.', '--db', 'analysis.db'
    ], capture_output=True, text=True)

    # Get latest scan ID
    result = subprocess.run([
        './code-guardian', 'history', '--db', 'analysis.db'
    ], capture_output=True, text=True)

    scan_id = result.stdout.strip().split('\n')[-1].split()[1].rstrip(',')

    # Get JSON report
    result = subprocess.run([
        './code-guardian', 'report', scan_id, '--format', 'json', '--db', 'analysis.db'
    ], capture_output=True, text=True)

    data = json.loads(result.stdout)

    # Analyze results
    todos_by_file = {}
    for match in data['matches']:
        file_path = match['file_path']
        if file_path not in todos_by_file:
            todos_by_file[file_path] = []
        todos_by_file[file_path].append(match)

    # Print summary
    print(f"Total TODO/FIXME items: {len(data['matches'])}")
    print("\nBy file:")
    for file_path, matches in sorted(todos_by_file.items()):
        print(f"  {file_path}: {len(matches)} items")

    return data

if __name__ == '__main__':
    run_scan_and_analyze()
```

## Custom Output Processing

Pipe outputs to other tools:

```bash
# Filter only TODO items
./code-guardian report 1 --format json | jq '.matches[] | select(.pattern == "TODO")'

# Count by pattern type
./code-guardian report 1 --format json | jq '.matches | group_by(.pattern) | map({pattern: .[0].pattern, count: length})'

# Find files with most matches
./code-guardian report 1 --format json | jq '.matches | group_by(.file_path) | map({file: .[0].file_path, count: length}) | sort_by(.count) | reverse | .[0:5]'
```

## Database Management

Code-Guardian uses SQLite, so you can query the database directly:

```sql
-- Connect to database
sqlite3 data/code-guardian.db

-- View all scans
SELECT * FROM scans;

-- View matches for a specific scan
SELECT * FROM matches WHERE scan_id = 1;

-- Count matches by pattern
SELECT pattern, COUNT(*) as count FROM matches WHERE scan_id = 1 GROUP BY pattern;
```

This gives you full flexibility for custom analysis and reporting.