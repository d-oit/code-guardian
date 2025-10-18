# Your First Code-Guardian Scan

Let's get you scanning code quickly! This tutorial assumes you've already installed Code-Guardian.

## Quick Start

### Step 1: Create a Test File

Let's create a simple file with some common issues:

```bash
# Create a test directory
mkdir code-guardian-test
cd code-guardian-test

# Create a JavaScript file with some issues
cat > app.js << 'EOF'
function login(username, password) {
    // TODO: Implement proper authentication
    if (username === 'admin' && password === 'password123') {
        console.log('Login successful!');
        return true;
    }
    return false;
}

function getUserData(userId) {
    // SQL injection vulnerability
    const query = `SELECT * FROM users WHERE id = ${userId}`;
    return database.query(query);
}

function generateToken() {
    // Insecure random generation
    return Math.random().toString(36);
}

// Unused import
const fs = require('fs');

// Debugger statement left in code
debugger;
EOF
```

### Step 2: Run Your First Scan

```bash
# Basic scan
code-guardian scan app.js
```

You should see output like:
```
Scan completed and saved with ID: 1
Found 5 matches:
app.js:3:5 - TODO: Implement proper authentication (TODO)
app.js:10:19 - Potential SQL injection vulnerability (sql_injection)
app.js:15:12 - Insecure random number generation (insecure_random)
app.js:18:1 - Unused import 'fs' (unused_import)
app.js:21:1 - Debugger statement found (debugger)
```

### Step 3: Try Different Output Formats

```bash
# JSON output
code-guardian scan app.js --format json

# HTML report
code-guardian scan app.js --format html --output report.html
# Then open report.html in your browser

# Markdown table
code-guardian scan app.js --format markdown
```

## Understanding the Results

### Match Structure
Each match shows:
- **File path and line number**: `app.js:3:5`
- **Description**: What was found
- **Pattern type**: The detector that found it

### Severity Levels
Code-Guardian uses these severity levels:
- **Info**: Informational (unused imports)
- **Low**: Minor issues (debugger statements)
- **Medium**: Moderate issues (TODO comments)
- **High**: Serious issues (hardcoded secrets)
- **Critical**: Severe security vulnerabilities (SQL injection)

## Scanning Different Languages

### Python Example
```python
# Create Python test file
cat > app.py << 'EOF'
import os

def authenticate(password):
    # Hardcoded password
    if password == "admin123":
        return True
    return False

def read_file(filename):
    # Path traversal vulnerability
    with open(filename, 'r') as f:
        return f.read()

# Unused variable
unused_var = "never used"

# TODO comment
# TODO: Add error handling
EOF
```

```bash
code-guardian scan app.py
```

### Rust Example
```rust
// Create Rust test file
cat > lib.rs << 'EOF'
use std::fs;

fn main() {
    // Unwrap without error handling
    let data = fs::read_to_string("file.txt").unwrap();

    // Panic in production code
    if data.is_empty() {
        panic!("File is empty!");
    }

    println!("{}", data);
}

// TODO: Implement proper error handling
// FIXME: This function is too long
EOF
```

```bash
code-guardian scan lib.rs
```

## Using Profiles

Code-Guardian has different scanning profiles for different use cases:

```bash
# Security-focused scan (default)
code-guardian scan . --profile security

# Comprehensive scan (all detectors)
code-guardian scan . --profile comprehensive

# Performance-focused scan
code-guardian scan . --profile performance

# Quick scan for development
code-guardian scan . --profile basic
```

## Configuration Basics

### Create a Basic Config File

```bash
# Create config.toml
cat > config.toml << 'EOF'
[scan]
max_file_size = 1048576  # 1MB
max_threads = 4

[[detectors]]
name = "todo"
enabled = true
severity = "Medium"

[[detectors]]
name = "security"
enabled = true
severity = "High"
EOF
```

```bash
# Use the config
code-guardian scan . --config config.toml
```

## Next Steps

Now that you know the basics:

1. **Scan your own project**: `code-guardian scan /path/to/your/project`
2. **Learn advanced features**: Check out the [Advanced Usage](advanced-usage.md) tutorial
3. **Set up automation**: See the [Automation](automation.md) guide
4. **Create custom detectors**: Follow the [Custom Detectors](custom-detectors.md) tutorial

## Troubleshooting

### No matches found?
- Check if the file has content Code-Guardian can analyze
- Try different file extensions (.js, .py, .rs, etc.)
- Use `--verbose` for more detailed output

### Scan is slow?
- Use `--max-threads 2` to limit CPU usage
- Try `--incremental` for subsequent scans
- Exclude large directories: `--exclude "node_modules/"`

### Permission errors?
- Ensure you have read access to the files
- On Windows, run as Administrator if needed
- Check file permissions with `ls -la` (Linux/macOS)

## Clean Up

```bash
# Remove test files
cd ..
rm -rf code-guardian-test
```

Congratulations! You've completed your first Code-Guardian scan. 🎉