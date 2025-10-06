# Getting Started with Code-Guardian

Welcome to Code-Guardian! This tutorial will guide you through your first scan and report generation.

## Prerequisites

- Rust installed (cargo available)
- A codebase to scan (we'll use this repository as an example)

## Step 1: Build Code-Guardian

First, let's build the tool:

```bash
git clone <repository-url>
cd code-guardian
cargo build --release
```

The binary will be at `target/release/code-guardian-cli` (or just `code-guardian` if installed).

## Step 2: Run Your First Scan

Let's scan the current directory for TODO and FIXME comments:

```bash
./target/release/code-guardian-cli scan .
```

You should see output like:

```
Scan completed and saved with ID: 1
Found 3 matches:
src/main.rs:42:5 - TODO: Implement error handling
src/lib.rs:15:1 - FIXME: This function needs optimization
src/utils.rs:8:9 - TODO: Add documentation
```

## Step 3: View Scan History

Check what scans you've run:

```bash
./target/release/code-guardian-cli history
```

Output:
```
Scan History:
ID: 1, Timestamp: 2023-10-06 12:34:56, Path: .
```

## Step 4: Generate a Report

Create a detailed report in different formats:

```bash
# Text format (default)
./target/release/code-guardian-cli report 1

# JSON format
./target/release/code-guardian-cli report 1 --format json

# HTML format (save to file)
./target/release/code-guardian-cli report 1 --format html > report.html
```

## Step 5: Open the HTML Report

If you generated an HTML report, open `report.html` in your browser to see a nicely formatted table of all matches.

## What's Next?

- Try scanning a different project
- Learn about [advanced usage](../advanced-usage.md)
- Create [custom detectors](../custom-detectors.md) for your specific needs
- Set up [automated scanning](../automation.md)

Congratulations! You've completed your first Code-Guardian scan.