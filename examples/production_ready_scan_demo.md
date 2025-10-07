# Code-Guardian Production Readiness Scanning

This example demonstrates Code-Guardian's comprehensive multi-language capabilities for detecting non-production code that shouldn't make it to production releases.

## Overview

Code-Guardian now includes **13 new detectors** specifically designed to find non-production code across **30+ programming languages** including TypeScript, Python, C#, Go, Java, PHP, Rust, and more.

## New Non-Production Code Detectors

### 🔴 Critical Severity
- **`Debugger`**: Debugger statements, breakpoints (`debugger`, `pdb.set_trace()`, `breakpoint()`)

### 🟠 High Severity  
- **`Dev`**: Development environment references
- **`Staging`**: Staging environment references
- **`ConsoleLog`**: JavaScript/TypeScript console statements
- **`Alert`**: JavaScript alert/prompt/confirm statements

### 🟡 Medium Severity
- **`Debug`**: Debug-related code
- **`Test`**: Test code in production files
- **`Phase`**: Phase markers (Phase 1, Phase 2, etc.)
- **`Print`**: Print statements across languages
- **`DeadCode`**: Dead code comments
- **`Experimental`**: Experimental/prototype code markers

### 🟢 Low Severity
- **`UnusedVar`**: Explicitly marked unused variables

## Language Support Examples

### JavaScript/TypeScript
```javascript
// These will be detected:
console.log("Debug info");           // CONSOLE_LOG - High
console.error("Error details");      // CONSOLE_LOG - High
alert("Debug message");              // ALERT - High
debugger;                           // DEBUGGER - Critical
let unused = 5; // unused           // UNUSED_VAR - Low
// experimental feature              // EXPERIMENTAL - Medium
```

### Python
```python
# These will be detected:
print("Debug output")               # PRINT - Medium
pdb.set_trace()                    # DEBUGGER - Critical
breakpoint()                       # DEBUGGER - Critical
# Phase 1 implementation           # PHASE - Medium
# dev environment setup           # DEV - High
```

### C#
```csharp
// These will be detected:
Console.WriteLine("Debug");         // Custom pattern
System.Diagnostics.Debug.Print();  // Custom pattern
// staging server config           // STAGING - High
// TODO: remove debug code         // TODO - Low
```

### Go
```go
// These will be detected:
fmt.Println("debug info")          // PRINT - Medium
// Phase 2 implementation          // PHASE - Medium
// This is a prototype             // EXPERIMENTAL - Medium
```

### PHP
```php
<?php
// These will be detected:
echo "Debug info";                 // PRINT - Medium
var_dump($data);                   // PRINT - Medium
// dev database connection         // DEV - High
?>
```

### Rust
```rust
// These will be detected:
println!("Debug: {:?}", data);     // PRINT - Medium
panic!("Should not happen");       // PANIC - High
value.unwrap()                     // UNWRAP - Medium
unsafe { /* code */ }              // UNSAFE - High
// experimental algorithm          // EXPERIMENTAL - Medium
```

## Usage Examples

### 1. Basic Production Readiness Scan
```bash
# Scan with production-ready profile
code-guardian scan /path/to/project --profile production-ready

# Or specify individual detectors
code-guardian scan /path/to/project --detectors ConsoleLog,Debugger,Dev,Phase
```

### 2. Using Configuration File
```bash
# Use the comprehensive production config
code-guardian scan /path/to/project --config examples/production_ready_config.toml
```

### 3. Language-Specific Scanning
```bash
# JavaScript/TypeScript projects
code-guardian scan /path/to/frontend --include-ext js,ts,jsx,tsx --detectors ConsoleLog,Alert,Debugger

# Python projects  
code-guardian scan /path/to/backend --include-ext py --detectors Print,Debugger,Dev

# Multi-language monorepo
code-guardian scan /path/to/monorepo --profile comprehensive
```

### 4. CI/CD Integration
```yaml
# GitHub Actions example
- name: Check Production Readiness
  run: |
    code-guardian scan . \
      --profile production-ready \
      --format json \
      --output production-check.json
    
    # Fail if critical issues found
    if grep -q "Critical" production-check.json; then
      echo "Critical production issues found!"
      exit 1
    fi
```

## Sample Output

```
🔍 Code-Guardian Production Readiness Report

📁 Scanning: /path/to/project (1,247 files)

🔴 Critical Issues (2):
├── src/auth/login.js:42:5 [DEBUGGER] debugger; // TODO: remove
└── src/api/config.ts:15:12 [DEBUGGER] breakpoint();

🟠 High Severity (5):  
├── src/utils/helpers.js:23:1 [CONSOLE_LOG] console.log("User data:", user)
├── src/config/env.js:8:20 [DEV] const DEV_SERVER = "dev.example.com"
├── src/components/Modal.tsx:67:5 [ALERT] alert("Debug: Modal opened")
├── src/services/api.py:34:5 [STAGING] # staging endpoint configuration
└── src/auth/oauth.cs:12:1 [CONSOLE_LOG] Console.WriteLine("OAuth debug")

🟡 Medium Severity (8):
├── src/data/processor.py:156:5 [PRINT] print(f"Processing {len(items)} items")
├── src/features/search.go:89:2 [PRINT] fmt.Println("Search query:", query)  
├── src/payment/stripe.php:45:1 [PRINT] echo "Payment debug: " . $amount;
├── src/core/engine.rs:203:9 [PRINT] println!("Engine state: {:?}", state);
├── src/ml/algorithm.py:78:1 [EXPERIMENTAL] # experimental ML algorithm
├── src/ui/prototype.js:12:1 [PHASE] // Phase 1: Basic implementation
├── src/utils/cache.ts:91:1 [DEAD_CODE] // dead code - never called
└── src/auth/session.java:134:5 [DEBUG] // debug session management

🟢 Low Severity (3):
├── src/helpers/utils.js:45:5 [UNUSED_VAR] let temp = data; // unused
├── src/models/user.py:23:1 [TODO] # TODO: add validation
└── src/config/settings.rb:67:1 [NOTE] # NOTE: legacy compatibility

📊 Summary:
• Total Issues: 18
• Files Scanned: 1,247  
• Languages: JavaScript, TypeScript, Python, C#, Go, PHP, Rust, Java, Ruby
• Scan Time: 2.3s

⚠️  Production readiness: FAILED
   Critical and high-severity issues must be resolved before production deployment.
```

## Benefits

1. **🌍 Universal Language Support**: Works with 30+ programming languages
2. **🎯 Production-Focused**: Specifically targets code that shouldn't be in production
3. **⚡ Fast Parallel Scanning**: Efficiently scans large multi-language codebases  
4. **🔧 Configurable**: Customize patterns and severity levels per project needs
5. **🔄 CI/CD Ready**: Easy integration with build pipelines and deployment gates
6. **📊 Multiple Output Formats**: JSON, CSV, HTML, Markdown, and text reports

## Best Practices

1. **Run Before Every Release**: Include production readiness checks in your CI/CD pipeline
2. **Block Critical Issues**: Fail builds when critical issues (debugger statements) are found
3. **Review High Severity**: Manually review high-severity issues before production deployment
4. **Custom Patterns**: Add project-specific patterns for your non-production code markers
5. **Team Education**: Use reports to educate developers about production-ready code practices

This makes Code-Guardian an essential tool for ensuring code quality and production readiness across any technology stack!