# Automation Tutorial

Learn how to automate Code-Guardian scans for continuous monitoring of your codebase.

## Cron Jobs for Regular Scanning

Set up daily scans using cron:

```bash
# Edit crontab
crontab -e

# Add this line for daily scans at 9 AM weekdays
0 9 * * 1-5 /path/to/code-guardian scan /path/to/project --db /path/to/scans.db
```

## Git Hooks for Pre-Commit Checks

Add a pre-commit hook to prevent commits with too many new TODOs:

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Build code-guardian if needed
# cargo build --release --bin code_guardian_cli

DB_PATH=".code-guardian.db"
SCAN_DIR="."

# Run scan
./target/release/code_guardian_cli scan "$SCAN_DIR" --db "$DB_PATH" > /dev/null 2>&1

# Get latest scan ID
SCAN_ID=$(./target/release/code_guardian_cli history --db "$DB_PATH" 2>/dev/null | tail -1 | awk '{print $2}' | tr -d ',')

if [ -n "$SCAN_ID" ]; then
    # Count matches
    COUNT=$(./target/release/code_guardian_cli report "$SCAN_ID" --db "$DB_PATH" --format json 2>/dev/null | jq '.matches | length' 2>/dev/null || echo "0")

    echo "Found $COUNT TODO/FIXME items in this commit"

    # Optional: set a threshold
    if [ "$COUNT" -gt 10 ]; then
        echo "Warning: High number of TODO/FIXME items detected"
        # exit 1  # Uncomment to block commits
    fi
fi
```

Make it executable:

```bash
chmod +x .git/hooks/pre-commit
```

## GitHub Actions Workflows

### Basic Quality Gate

```yaml
# .github/workflows/quality-gate.yml
name: Quality Gate

on: [push, pull_request]

jobs:
  code-guardian:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4

    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Build Code-Guardian
      run: cargo build --release --bin code_guardian_cli

    - name: Scan codebase
      run: ./target/release/code_guardian_cli scan . --db /tmp/scans.db

    - name: Generate report
      run: |
        SCAN_ID=$(./target/release/code_guardian_cli history --db /tmp/scans.db | tail -1 | awk '{print $2}' | tr -d ',')
        ./target/release/code_guardian_cli report $SCAN_ID --db /tmp/scans.db --format markdown >> $GITHUB_STEP_SUMMARY

    - name: Check thresholds
      run: |
        SCAN_ID=$(./target/release/code_guardian_cli history --db /tmp/scans.db | tail -1 | awk '{print $2}' | tr -d ',')
        COUNT=$(./target/release/code_guardian_cli report $SCAN_ID --db /tmp/scans.db --format json | jq '.matches | length')

        if [ "$COUNT" -gt 100 ]; then
          echo "❌ Too many TODO/FIXME items: $COUNT"
          exit 1
        else
          echo "✅ Acceptable TODO/FIXME count: $COUNT"
        fi
```

### Trend Analysis

Track TODO trends over time:

```yaml
# .github/workflows/trend-analysis.yml
name: Trend Analysis

on:
  schedule:
    # Run weekly on Mondays
    - cron: '0 9 * * 1'
  workflow_dispatch:

jobs:
  analyze-trends:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4

    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Build Code-Guardian
      run: cargo build --release --bin code_guardian_cli

    - name: Scan and analyze
      run: |
        # Use persistent database
        ./target/release/code_guardian_cli scan . --db scans.db

        # Get all scan IDs
        SCAN_IDS=$(./target/release/code_guardian_cli history --db scans.db | awk '{print $2}' | tr -d ',' | tail -10)

        echo "# Code-Guardian Trend Report" >> trend-report.md
        echo "" >> trend-report.md
        echo "| Scan ID | Date | TODO Count | FIXME Count | Total |" >> trend-report.md
        echo "|---------|------|------------|-------------|-------|" >> trend-report.md

        for id in $SCAN_IDS; do
          JSON=$(./target/release/code_guardian_cli report $id --db scans.db --format json)
          DATE=$(echo "$JSON" | jq -r '.timestamp | strftime("%Y-%m-%d")')
          TODO_COUNT=$(echo "$JSON" | jq '.matches | map(select(.pattern == "TODO")) | length')
          FIXME_COUNT=$(echo "$JSON" | jq '.matches | map(select(.pattern == "FIXME")) | length')
          TOTAL=$(echo "$JSON" | jq '.matches | length')

          echo "| $id | $DATE | $TODO_COUNT | $FIXME_COUNT | $TOTAL |" >> trend-report.md
        done

    - name: Upload trend report
      uses: actions/upload-artifact@v4
      with:
        name: trend-report
        path: trend-report.md
```

## Jenkins Integration

```groovy
// Jenkinsfile
pipeline {
    agent any

    triggers {
        cron('H 9 * * 1-5')  // Weekdays at 9 AM
    }

    stages {
        stage('Code Scan') {
            steps {
                sh '''
                    # Build tool
                    cargo build --release --bin code_guardian_cli

                    # Run scan
                    ./target/release/code_guardian_cli scan . --db scans.db

                    # Generate reports
                    SCAN_ID=$(./target/release/code_guardian_cli history --db scans.db | tail -1 | awk '{print $2}' | tr -d ',')
                    ./target/release/code_guardian_cli report $SCAN_ID --db scans.db --format html > scan-report.html
                    ./target/release/code_guardian_cli report $SCAN_ID --db scans.db --format json > scan-data.json
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

        stage('Trend Analysis') {
            steps {
                sh '''
                    # Analyze trends (simplified)
                    COUNT=$(cat scan-data.json | jq '.matches | length')
                    PREV_COUNT=$(cat previous-scan.json | jq '.matches | length' 2>/dev/null || echo "$COUNT")

                    if [ "$COUNT" -gt "$PREV_COUNT" ]; then
                        echo "⚠️  TODO/FIXME count increased from $PREV_COUNT to $COUNT"
                    else
                        echo "✅ TODO/FIXME count: $COUNT (previous: $PREV_COUNT)"
                    fi

                    # Save for next run
                    cp scan-data.json previous-scan.json
                '''
            }
        }
    }

    post {
        always {
            archiveArtifacts artifacts: 'scan-*.json,scan-report.html', allowEmptyArchive: true
        }
    }
}
```

## Docker Integration

Create a Docker image for easy deployment:

```dockerfile
# Dockerfile
FROM rust:1.70-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release --bin code_guardian_cli

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/code_guardian_cli /usr/local/bin/code-guardian

CMD ["code-guardian", "--help"]
```

Build and run:

```bash
docker build -t code-guardian .
docker run -v $(pwd):/workspace code-guardian scan /workspace
```

## Monitoring and Alerting

Set up alerts when TODO counts exceed thresholds:

```bash
#!/bin/bash
# alert-script.sh

DB_PATH="/path/to/scans.db"
THRESHOLD=50
EMAIL="team@example.com"

# Run scan
/code-guardian scan /path/to/project --db "$DB_PATH"

# Get latest count
SCAN_ID=$(/code-guardian history --db "$DB_PATH" | tail -1 | awk '{print $2}' | tr -d ',')
COUNT=$(/code-guardian report "$SCAN_ID" --db "$DB_PATH" --format json | jq '.matches | length')

if [ "$COUNT" -gt "$THRESHOLD" ]; then
    echo "High TODO count detected: $COUNT" | mail -s "Code-Guardian Alert" "$EMAIL"

    # Or send to Slack
    curl -X POST -H 'Content-type: application/json' \
         --data "{\"text\":\"High TODO count: $COUNT\"}" \
         YOUR_SLACK_WEBHOOK_URL
fi
```

These automation techniques help maintain code quality and track technical debt over time.