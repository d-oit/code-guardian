# LLM Code Quality Detection Demo

This document demonstrates the LLM-specific code quality and security detection capabilities of Code-Guardian.

## Overview

Code-Guardian now includes specialized detectors for identifying security vulnerabilities and quality issues commonly found in LLM-generated code. These detectors are based on 2025 research into AI-generated code patterns.

## Quick Start

### 1. Basic LLM Security Scan

```bash
# Scan for LLM security vulnerabilities only
code-guardian scan --profile llm-security /path/to/project

# Scan for LLM quality issues only  
code-guardian scan --profile llm-quality /path/to/project

# Comprehensive LLM detection (all patterns)
code-guardian scan --profile llm-comprehensive /path/to/project
```

### 2. Production-Ready Scan with LLM Detection

```bash
# Production readiness scan including LLM issues
code-guardian scan --profile production-ready-llm /path/to/project

# Fail CI/CD pipeline on critical LLM security issues
code-guardian scan --profile llm-security --max-critical 0 /path/to/project
```

## LLM-Specific Patterns Detected

### Security Vulnerabilities (Critical/High Severity)

1. **Hallucinated APIs**: Non-existent function calls
   ```javascript
   // ❌ Detected: LLM_HALLUCINATED_API
   user.authenticate(); // This API doesn't exist
   data.validateInput(); // Common LLM hallucination
   ```

2. **SQL Injection**: String concatenation in queries
   ```javascript
   // ❌ Detected: LLM_SQL_INJECTION  
   const query = "SELECT * FROM users WHERE id = " + userId;
   db.execute(query);
   ```

3. **Hardcoded Credentials**: Security secrets in code
   ```javascript
   // ❌ Detected: LLM_HARDCODED_CREDENTIALS
   const apiKey = "sk-1234567890abcdef";
   const password = "mySecretPass123";
   ```

4. **Insecure Random**: Non-crypto random for security
   ```javascript
   // ❌ Detected: LLM_INSECURE_RANDOM
   const token = Math.random().toString(36); // Not cryptographically secure
   ```

5. **XSS Injection**: Direct HTML manipulation
   ```javascript
   // ❌ Detected: LLM_XSS_INJECTION
   element.innerHTML = userInput + "<br>";
   document.write("Hello " + userName);
   ```

### Memory Safety Issues (Rust)

```rust
// ❌ Detected: LLM_RUST_MEMORY_SAFETY
let value = ptr.unwrap(); // safe because we checked
unsafe { transmute(data) } // Unjustified unsafe
```

### Quality Issues (Medium Severity)

1. **Async Anti-patterns**: Incorrect async/await usage
   ```javascript
   // ❌ Detected: LLM_ASYNC_ANTIPATTERN
   await someVariable; // await without function call
   data.then(await processData); // mixing .then with await
   ```

2. **Performance Issues**: Inefficient patterns
   ```rust
   // ❌ Detected: LLM_PERFORMANCE_ISSUE
   for i in 0..n {
       for j in 0..n {
           for k in 0..n {
               for l in 0..n { // O(n^4) complexity
   ```

3. **Error Handling**: Poor error management
   ```javascript
   // ❌ Detected: LLM_ERROR_HANDLING
   try {
       riskyOperation();
   } catch (e) {
       // Empty catch block
   }
   ```

## Language-Specific Detection

### JavaScript/TypeScript Issues

```javascript
// ❌ Detected: LLM_JS_ISSUES
if (value == null) { } // Should use ===
parseInt(str); // Missing radix parameter
JSON.parse(data); // No error handling
```

### Python Security Issues

```python
# ❌ Detected: LLM_PYTHON_ISSUES  
exec(input('Enter code: ')) # Code injection
eval(user_input) # Dangerous eval
pickle.loads(request.data) # Unsafe deserialization
```

## Configuration Examples

### Custom LLM Detection Config

```toml
# custom_llm_config.toml
[scan]
enabled_detectors = [
    "LLMSQLInjection",
    "LLMHardcodedCredentials", 
    "LLMXSSInjection",
    "LLMAsyncAntipattern"
]

[severity_levels]
LLM_SQL_INJECTION = "Critical"
LLM_HARDCODED_CREDENTIALS = "Critical"
LLM_XSS_INJECTION = "Critical"
LLM_ASYNC_ANTIPATTERN = "Medium"
```

### CI/CD Integration

```yaml
# .github/workflows/llm-security.yml
name: LLM Security Check
on: [push, pull_request]

jobs:
  llm-security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run LLM Security Scan
        run: |
          code-guardian scan \
            --profile llm-security \
            --max-critical 0 \
            --max-high 5 \
            --format json \
            --output llm-security-report.json \
            .
      - name: Upload Security Report
        uses: actions/upload-artifact@v3
        with:
          name: llm-security-report
          path: llm-security-report.json
```

## Severity Guidelines

| Pattern | Severity | Description |
|---------|----------|-------------|
| SQL Injection | Critical | Immediate security risk |
| Hardcoded Credentials | Critical | Credential exposure |
| XSS Injection | Critical | Code injection vulnerability |
| Insecure Random | High | Weak cryptography |
| Memory Safety | High | Potential crashes/exploits |
| Hallucinated APIs | High | Runtime failures |
| Async Anti-patterns | Medium | Performance/correctness |
| Over-engineering | Low | Maintainability issues |

## Best Practices

1. **Run LLM scans on all AI-generated code**
2. **Integrate into pre-commit hooks**
3. **Fail CI/CD on critical LLM security issues**
4. **Review LLM-generated comments for accuracy**
5. **Combine with traditional static analysis**

## Example Output

```
🔍 LLM Security Scan Results
━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 Summary:
  • Critical: 2 issues
  • High: 5 issues  
  • Medium: 12 issues
  • Total: 19 LLM-related issues

🚨 Critical Issues:
  src/auth.js:15 [LLM_SQL_INJECTION] 
    "SELECT * FROM users WHERE id = " + userId
  
  src/config.js:8 [LLM_HARDCODED_CREDENTIALS]
    const apiKey = "sk-1234567890abcdef"

⚠️  High Priority:
  src/utils.js:23 [LLM_HALLUCINATED_API]
    user.authenticate() // Non-existent API
    
  src/random.js:12 [LLM_INSECURE_RANDOM] 
    Math.random() used for security token

❌ Production Readiness: FAILED
   Critical LLM security issues must be resolved
```

## Advanced Usage

### Custom Pattern Detection

```rust
// Add custom LLM patterns
let custom_detector = CustomPatternDetector::new(
    "LLM_CUSTOM_ANTIPATTERN",
    r"(?i)//.*(?:ai generated|gpt|claude)"
)?;
```

### Integration with IDEs

The LLM detectors can be integrated with VS Code, IntelliJ, and other IDEs to provide real-time feedback on AI-generated code quality.

## Research Sources

This implementation is based on:
- 2024-2025 LLM security research
- Industry reports on AI code quality
- Analysis of common LLM failure patterns
- Production incidents from AI-generated code

For more information, see the [LLM Vulnerabilities Research Document](../tmp_rovodev_llm_vulnerabilities_research.md).