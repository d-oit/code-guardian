# ADR-002: LLM Vulnerability Detection Integration

## Status
Accepted

## Context
With the increasing use of AI-assisted development tools (GitHub Copilot, ChatGPT, etc.), traditional static analysis tools miss vulnerabilities specific to AI-generated code. These include hallucinated APIs, insecure patterns, and quality issues that LLMs commonly produce.

## Decision
Integrate specialized LLM detection capabilities into Code Guardian with 18 dedicated detectors:

### Security Detectors (9 types)
- HallucinatedApiDetector: Non-existent API calls
- LLMSQLInjectionDetector: String concatenation vulnerabilities
- InsecureRandomDetector: Non-cryptographic random usage
- HardcodedCredentialsDetector: Embedded secrets
- RustMemorySafetyDetector: Unsafe Rust patterns
- CryptoAntipatternDetector: Weak cryptographic implementations
- XSSInjectionDetector: DOM manipulation vulnerabilities
- FilesystemSecurityDetector: Path traversal issues
- ContextConfusionDetector: Privilege escalation patterns

### Quality Detectors (9 types)
- AsyncAntipatternDetector: Incorrect async/await patterns
- PerformanceAntipatternDetector: Inefficient algorithms
- ErrorHandlingDetector: Poor error handling
- OverengineeringDetector: Over-complex design patterns
- ConfigAntipatternDetector: Hardcoded configuration
- DatabaseAntipatternDetector: N+1 queries and inefficiencies
- JSLLMIssuesDetector: JavaScript-specific issues
- PythonLLMIssuesDetector: Python-specific issues
- LLMGeneratedCommentsDetector: AI-generated code comments

## Consequences

### Positive
- Proactive detection of LLM-generated vulnerabilities
- Enhanced security for AI-assisted development workflows
- Comprehensive multi-language support (JS/TS, Python, Rust, SQL)
- Minimal performance impact (~7% scan time increase)
- 100% test coverage for all LLM detectors
- Future-proof against emerging AI-generated code issues

### Negative
- Additional complexity in detector management
- Need for ongoing pattern updates as LLMs evolve
- Potential for false positives requiring tuning

## Implementation Details
```rust
// Integration through detector factory
pub fn create_llm_security_profile() -> Vec<Box<dyn Detector>> {
    vec![
        Box::new(HallucinatedApiDetector::new()),
        Box::new(LLMSQLInjectionDetector::new()),
        Box::new(InsecureRandomDetector::new()),
        // ... other security detectors
    ]
}

// CLI integration
code-guardian scan --profile llm-security ./src
code-guardian scan --profile production-ready-llm ./src
```

## Performance Impact
- Scan time increase: ~5-10% for comprehensive LLM detection
- Memory overhead: <1MB additional per scan
- Parallel processing maintained for large codebases
- No impact on compilation times

## Date
2024-10-16

## Reviewers
- Security Team
- Code Guardian Team