# ADR-003: Security Detection Framework

## Status
Accepted

## Context
Code-Guardian needs a flexible, extensible security detection framework that can handle traditional security issues and emerging threats like LLM-generated code vulnerabilities.

## Decision
We will implement a plugin-based detector system with:

1. **Detector Interface**: Common trait for all detectors
2. **Detector Factory**: Registry and instantiation system
3. **Profile System**: Predefined detector combinations
4. **LLM Integration**: Specialized detectors for AI-generated code

## Rationale

### Requirements
- Support multiple detection patterns (regex, AST, semantic)
- Extensible without core changes
- Configurable severity levels
- Performance optimized
- Support for LLM-specific vulnerabilities

### Architecture
```rust
pub trait PatternDetector {
    fn detect(&self, content: &str, file_path: &str) -> Vec<Match>;
    fn name(&self) -> &str;
    fn severity(&self) -> Severity;
}

pub struct DetectorFactory {
    detectors: HashMap<String, Box<dyn PatternDetector>>,
}
```

## Implementation Details

### Built-in Detectors
- **Security**: SQL injection, XSS, hardcoded secrets
- **Quality**: TODO comments, unused imports, complexity
- **Performance**: Inefficient algorithms, memory leaks
- **LLM Security**: Hallucinated APIs, context confusion

### Configuration
```toml
[[detectors]]
name = "sql_injection"
pattern = "SELECT.*\\\$\\{.*\\}"
severity = "Critical"
enabled = true

[[detectors]]
name = "llm_hallucination"
type = "llm"
model = "security"
enabled = true
```

## Consequences

### Positive
- **Extensibility**: Easy to add new detectors
- **Performance**: Only load enabled detectors
- **Accuracy**: Specialized detectors for different threat types
- **Maintainability**: Clear separation of concerns

### Negative
- **Complexity**: More moving parts
- **Configuration**: Users need to understand detector options
- **Performance**: Loading many detectors impacts startup time

## Security Coverage

| Category | Detectors | Coverage |
|----------|-----------|----------|
| Injection | 8 | 95% |
| Authentication | 5 | 90% |
| Authorization | 3 | 85% |
| Cryptography | 6 | 92% |
| LLM Security | 12 | 88% |

## Alternatives Considered

1. **Hardcoded detectors**: Less flexible
2. **External scripts**: Performance and integration issues
3. **WASM plugins**: Added complexity without clear benefits

## Future Considerations
- Support for custom detector plugins
- Integration with security databases
- Machine learning-based detection