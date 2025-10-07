# 🚀 Code-Guardian Enhanced CLI Demo

The new CLI enhancements make Code-Guardian incredibly easy to use for production readiness and multi-language code analysis.

## 🎯 **Quick Production Check**

```bash
# Basic production readiness scan
code-guardian production-check .

# Production check with JSON output
code-guardian production-check . --format json --output production-report.json

# Fail CI/CD if critical issues found
code-guardian production-check . --fail-on-critical

# Only show critical and high severity issues
code-guardian production-check . --severity critical,high
```

**Sample Output:**
```
🔍 Code-Guardian Production Readiness Check
📁 Scanning: ./src

🔴 Critical Issues (1):
├── src/auth/login.js:42:5 [DEBUGGER] debugger; // TODO: remove

🟠 High Severity (3):  
├── src/utils/api.js:23:1 [CONSOLE_LOG] console.log("User data:", user)
├── src/config/env.py:8:20 [DEV] DEV_SERVER = "dev.example.com"
└── src/components/Modal.tsx:67:5 [ALERT] alert("Debug: Modal opened")

📊 Summary:
• Total Issues: 12
• Critical: 1
• High: 3
• Medium: 6
• Low: 2

⚠️ Production readiness: NEEDS ATTENTION - Address critical and high severity issues
```

## 🔧 **Pre-commit Integration**

```bash
# Pre-commit hook (fails on critical issues)
code-guardian pre-commit .

# Fast mode - only check critical issues
code-guardian pre-commit . --fast

# Check only staged files (coming soon)
code-guardian pre-commit . --staged-only
```

## 🚦 **CI/CD Gate**

```bash
# CI/CD gate with thresholds
code-guardian ci-gate . --max-critical 0 --max-high 5

# Generate JSON report for CI systems
code-guardian ci-gate . --output ci-report.json

# Custom configuration
code-guardian ci-gate . --config ci-config.toml
```

**CI Integration Example:**
```yaml
# .github/workflows/code-quality.yml
- name: Code Quality Gate
  run: |
    code-guardian ci-gate . \
      --max-critical 0 \
      --max-high 3 \
      --output code-quality-report.json
    
    # Upload report as artifact
    if [ $? -ne 0 ]; then
      echo "Code quality gate failed!"
      cat code-quality-report.json
      exit 1
    fi
```

## 🌍 **Language-Specific Scanning**

```bash
# Scan specific languages
code-guardian lang js,ts,py .

# Include production readiness checks
code-guardian lang js,ts --production .

# Multiple languages with JSON output
code-guardian lang py,go,rs . --format json
```

## 📱 **Technology Stack Presets**

```bash
# Web frontend scanning
code-guardian stack web .

# Backend services scanning  
code-guardian stack backend . --production

# Full-stack monorepo
code-guardian stack fullstack .

# Mobile development
code-guardian stack mobile .

# Systems programming
code-guardian stack systems . --production
```

**Stack Presets Include:**
- **Web**: JavaScript, TypeScript, JSX, TSX, Vue, Svelte
- **Backend**: Python, Java, Go, C#, PHP, Ruby
- **Fullstack**: JavaScript, TypeScript, Python, Java, Go, Rust
- **Mobile**: JavaScript, TypeScript, Swift, Kotlin, Dart
- **Systems**: Rust, C++, C, Go

## 👁️ **File Watching** (Coming Soon)

```bash
# Live scanning during development
code-guardian watch .

# Watch specific file patterns
code-guardian watch . --include "*.js,*.ts" --exclude "node_modules/*"

# Custom debounce delay
code-guardian watch . --delay 1000
```

## 🎨 **Developer-Friendly Features**

### **Smart Output Formatting**
```bash
# Summary view for quick overview
code-guardian production-check . --format summary

# JSON for CI/CD integration
code-guardian production-check . --format json

# Detailed text report (default)
code-guardian production-check . --format text
```

### **Severity Filtering**
```bash
# Only critical issues
code-guardian production-check . --severity critical

# Critical and high severity
code-guardian production-check . --severity critical,high

# All except low severity
code-guardian production-check . --severity critical,high,medium
```

### **Exit Code Integration**
```bash
# Fail on critical issues (exit code 1)
code-guardian production-check . --fail-on-critical

# Fail on high severity issues (exit code 1)  
code-guardian production-check . --fail-on-high

# Both flags can be combined
code-guardian production-check . --fail-on-critical --fail-on-high
```

## 🔗 **Integration Examples**

### **Git Pre-commit Hook**
```bash
#!/bin/sh
# .git/hooks/pre-commit
code-guardian pre-commit . --fast
exit $?
```

### **GitHub Actions**
```yaml
# .github/workflows/production-ready.yml
name: Production Readiness Check
on: [push, pull_request]

jobs:
  code-quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Code-Guardian
        run: cargo install --path .
      
      - name: Production Check
        run: |
          code-guardian production-check . \
            --format json \
            --output production-report.json \
            --fail-on-critical
            
      - name: Upload Report
        uses: actions/upload-artifact@v4
        with:
          name: production-report
          path: production-report.json
```

### **GitLab CI**
```yaml
# .gitlab-ci.yml
code-quality:
  stage: test
  script:
    - code-guardian ci-gate . --max-critical 0 --max-high 5
  artifacts:
    reports:
      junit: ci-report.json
  only:
    - merge_requests
    - main
```

### **Jenkins Pipeline**
```groovy
// Jenkinsfile
pipeline {
    agent any
    stages {
        stage('Code Quality') {
            steps {
                sh '''
                    code-guardian production-check . \
                        --format json \
                        --output production-report.json \
                        --fail-on-critical
                '''
                archiveArtifacts artifacts: 'production-report.json'
                publishHTML([
                    allowMissing: false,
                    alwaysLinkToLastBuild: true,
                    keepAll: true,
                    reportDir: '.',
                    reportFiles: 'production-report.json',
                    reportName: 'Code Quality Report'
                ])
            }
        }
    }
}
```

## 📊 **Comparison: Before vs After**

### **Before (Basic Scan):**
```bash
# Old way - complex and not production-focused
code-guardian scan . --profile comprehensive --config custom.toml --format json > report.json
```

### **After (Enhanced CLI):**
```bash
# New way - simple and production-focused
code-guardian production-check . --fail-on-critical

# Language-specific
code-guardian lang js,ts . --production

# Stack-based
code-guardian stack web . --production

# CI/CD ready
code-guardian ci-gate . --max-critical 0
```

## 🎯 **Benefits**

1. **🚀 Faster Adoption**: Intuitive commands reduce learning curve
2. **🎯 Production-Focused**: Specialized commands for production readiness
3. **🌍 Language-Aware**: Smart presets for different tech stacks
4. **🔗 CI/CD Ready**: Proper exit codes and JSON output for automation
5. **👥 Developer-Friendly**: Clear output with actionable insights
6. **⚡ Efficient**: Fast modes for pre-commit hooks and quick checks

The enhanced CLI transforms Code-Guardian from a basic scanning tool into a comprehensive, production-ready code quality platform that developers actually want to use! 🎉