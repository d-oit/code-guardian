# Testing Guidelines

## Overview

This document outlines testing best practices, standards, and guidelines for Code Guardian development to ensure high-quality, reliable code with comprehensive test coverage.

## Testing Philosophy

### Core Principles
1. **Test-Driven Development**: Write tests before implementation when possible
2. **Comprehensive Coverage**: Aim for 82%+ line coverage, 90%+ branch coverage for critical paths
3. **Fast Feedback**: Tests should run quickly to enable rapid iteration
4. **Reliable Tests**: Tests should be deterministic and not flaky
5. **Clear Intent**: Tests should clearly express what they're validating

### Testing Pyramid
```
    /\
   /  \     E2E Tests (5%)
  /    \    - End-to-end workflows
 /      \   - CLI integration tests
/________\  
   /\       Integration Tests (15%)
  /  \      - Cross-crate interactions
 /    \     - API contract tests
/______\    
   /\       Unit Tests (80%)
  /  \      - Individual functions
 /    \     - Pure logic validation
/______\    - Error conditions
```

## Test Organization

### Directory Structure
```
crates/
├── core/
│   ├── src/
│   ├── tests/           # Integration tests
│   └── benches/         # Performance benchmarks
├── cli/
│   ├── src/
│   │   └── lib.rs       # Unit tests in modules
│   └── tests/           # CLI integration tests
└── output/
    ├── src/
    └── tests/
```

### Test Categories

#### 1. Unit Tests
Located within source files using `#[cfg(test)]` modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_creation() {
        let detector = SqlInjectionDetector::new();
        assert!(detector.is_enabled());
    }

    #[test]
    fn test_pattern_matching() {
        let detector = SqlInjectionDetector::new();
        let content = "SELECT * FROM users WHERE id = " + user_input;
        
        let matches = detector.scan(content);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].severity, Severity::High);
    }
}
```

#### 2. Integration Tests
Located in `tests/` directories for cross-module testing:

```rust
// tests/workflow_tests.rs
use code_guardian_core::Scanner;
use code_guardian_output::formatters::JsonFormatter;

#[test]
fn test_scan_to_output_workflow() {
    let scanner = Scanner::new();
    let results = scanner.scan_directory("test_data/").unwrap();
    
    let formatter = JsonFormatter::new();
    let output = formatter.format(&results).unwrap();
    
    // Validate JSON structure
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert!(json["issues"].is_array());
}
```

#### 3. Property-Based Tests
Using `proptest` for generating test cases:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_formatter_handles_arbitrary_content(
        content in ".*",
        severity in prop::sample::select(vec![
            Severity::Low, Severity::Medium, Severity::High, Severity::Critical
        ])
    ) {
        let formatter = TextFormatter::new();
        let issue = Issue::new("test", content, severity);
        
        // Should never panic
        let result = formatter.format_issue(&issue);
        prop_assert!(result.is_ok());
        
        // Output should contain the content
        let output = result.unwrap();
        prop_assert!(output.contains(&content) || content.is_empty());
    }
}
```

## Test Writing Standards

### Naming Conventions
```rust
// Function: test_<what>_<condition>_<expected_result>
#[test]
fn test_sql_detector_with_injection_returns_high_severity() { }

#[test]
fn test_file_scanner_with_empty_file_returns_no_issues() { }

#[test]
fn test_config_parser_with_invalid_toml_returns_error() { }
```

### Test Structure (AAA Pattern)
```rust
#[test]
fn test_detector_finds_security_issue() {
    // Arrange
    let detector = SecurityDetector::new();
    let test_content = "eval(user_input)";
    
    // Act
    let results = detector.scan(test_content);
    
    // Assert
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::High);
    assert_eq!(results[0].message, "Dangerous eval usage detected");
}
```

### Error Testing
```rust
#[test]
fn test_scanner_handles_permission_denied() {
    let scanner = Scanner::new();
    
    // Create a file with no read permissions
    let temp_file = create_unreadable_file();
    
    let result = scanner.scan_file(&temp_file);
    
    match result {
        Err(ScanError::PermissionDenied { path }) => {
            assert_eq!(path, temp_file);
        }
        _ => panic!("Expected PermissionDenied error"),
    }
}
```

## Mock and Test Doubles

### Using mockall for Complex Dependencies
```rust
use mockall::{automock, predicate::*};

#[automock]
trait FileSystem {
    fn read_file(&self, path: &Path) -> Result<String>;
    fn list_files(&self, dir: &Path) -> Result<Vec<PathBuf>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_with_mock_filesystem() {
        let mut mock_fs = MockFileSystem::new();
        
        mock_fs
            .expect_read_file()
            .with(eq(Path::new("test.rs")))
            .times(1)
            .returning(|_| Ok("fn main() {}".to_string()));
        
        let scanner = Scanner::with_filesystem(Box::new(mock_fs));
        let result = scanner.scan_file(Path::new("test.rs"));
        
        assert!(result.is_ok());
    }
}
```

### Test Fixtures
```rust
// tests/fixtures/mod.rs
pub struct TestFixtures;

impl TestFixtures {
    pub fn create_temp_file_with_content(content: &str) -> TempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }
    
    pub fn sample_config() -> ScanConfig {
        ScanConfig {
            parallel: false, // Deterministic for tests
            max_file_size: Some(1024),
            detectors: DetectorConfig::default(),
        }
    }
}
```

## Performance Testing

### Benchmark Tests
```rust
// benches/scanner_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_scanner(c: &mut Criterion) {
    let test_data = include_str!("../test_data/large_file.rs");
    
    c.bench_function("scan_large_file", |b| {
        b.iter(|| {
            let scanner = Scanner::new();
            scanner.scan_content(black_box(test_data))
        })
    });
}

criterion_group!(benches, benchmark_scanner);
criterion_main!(benches);
```

### Memory Usage Tests
```rust
#[test]
fn test_memory_usage_bounded() {
    let initial_memory = get_memory_usage();
    
    // Scan a large directory
    let scanner = Scanner::new();
    let _results = scanner.scan_directory("large_test_data/").unwrap();
    
    let final_memory = get_memory_usage();
    let memory_increase = final_memory - initial_memory;
    
    // Should not use more than 100MB
    assert!(memory_increase < 100 * 1024 * 1024);
}
```

## Coverage Requirements

### Coverage Targets by Crate
- **Core**: 85%+ (critical business logic)
- **CLI**: 80%+ (user interface layer)
- **Storage**: 90%+ (data integrity critical)
- **Output**: 75%+ (formatting logic)

### Measuring Coverage
```bash
# Generate coverage report
cargo llvm-cov --workspace --html --open

# Generate LCOV for CI
cargo llvm-cov --workspace --lcov --output-path coverage.lcov

# Check coverage thresholds
cargo llvm-cov --workspace --fail-under-lines 82
```

### Coverage Exclusions
```rust
#[cfg(not(tarpaulin_include))]
fn debug_only_function() {
    // Exclude from coverage
}

// Or use attributes
#[coverage(off)]
fn unreachable_error_path() {
    panic!("This should never happen");
}
```

## Test Data Management

### Test Data Organization
```
tests/
├── fixtures/
│   ├── sample_code/         # Sample source files
│   ├── configs/             # Test configuration files
│   └── expected_outputs/    # Expected scan results
└── data/
    ├── large_files/         # Performance test data
    └── edge_cases/          # Corner case examples
```

### Environment Setup
```rust
// tests/common/mod.rs
use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        env_logger::init();
        // Initialize test database
        // Set up temp directories
    });
}
```

## CI/CD Integration

### Test Execution in CI
```yaml
# .github/workflows/test.yml
- name: Run tests
  run: |
    cargo test --workspace --verbose
    cargo test --workspace --release

- name: Run property tests
  run: cargo test --release -- --ignored

- name: Check coverage
  run: |
    cargo llvm-cov --workspace --lcov --output-path coverage.lcov
    cargo llvm-cov --workspace --fail-under-lines 82
```

### Parallel Test Execution
```bash
# Use cargo-nextest for faster parallel execution
cargo nextest run --workspace --verbose

# Run specific test categories
cargo nextest run --workspace unit
cargo nextest run --workspace integration
```

## LLM Detector Testing

### Specific LLM Test Patterns
```rust
#[test]
fn test_llm_sql_injection_detector() {
    let detector = LLMSQLInjectionDetector::new();
    
    // Test cases for LLM-generated SQL injection patterns
    let test_cases = vec![
        ("sql = \"SELECT * FROM users WHERE id = \" + user_id", true),
        ("query = f\"SELECT * FROM {table} WHERE id = {id}\"", true),
        ("sql = \"SELECT * FROM users WHERE id = ?\"", false),
    ];
    
    for (content, should_detect) in test_cases {
        let matches = detector.scan(content);
        assert_eq!(!matches.is_empty(), should_detect, 
                  "Failed for content: {}", content);
    }
}
```

### LLM Detection Performance Tests
```rust
#[test]
fn test_llm_detector_performance() {
    let detector = ComprehensiveLLMDetector::new();
    let large_content = generate_large_test_content(10_000); // 10k lines
    
    let start = Instant::now();
    let _results = detector.scan(&large_content);
    let duration = start.elapsed();
    
    // LLM detection should add < 10% overhead
    assert!(duration < Duration::from_millis(100), 
           "LLM detection too slow: {:?}", duration);
}
```

## Common Testing Patterns

### Testing Async Code
```rust
#[tokio::test]
async fn test_async_scanner() {
    let scanner = AsyncScanner::new();
    let result = scanner.scan_async("test_file.rs").await;
    assert!(result.is_ok());
}
```

### Testing with Temporary Files
```rust
use tempfile::TempDir;

#[test]
fn test_scanner_with_temp_directory() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    std::fs::write(&test_file, "fn main() {}").unwrap();
    
    let scanner = Scanner::new();
    let results = scanner.scan_directory(temp_dir.path()).unwrap();
    
    assert!(!results.is_empty());
}
```

### Testing Configuration
```rust
#[test]
fn test_config_from_environment() {
    env::set_var("CODE_GUARDIAN_PARALLEL", "false");
    env::set_var("CODE_GUARDIAN_OUTPUT_FORMAT", "json");
    
    let config = ScanConfig::from_env().unwrap();
    
    assert!(!config.parallel);
    assert_eq!(config.output_format, OutputFormat::Json);
    
    // Cleanup
    env::remove_var("CODE_GUARDIAN_PARALLEL");
    env::remove_var("CODE_GUARDIAN_OUTPUT_FORMAT");
}
```

## Debugging Tests

### Test Debugging Techniques
```rust
#[test]
fn test_with_debugging() {
    // Enable logging for this test
    let _ = env_logger::builder().is_test(true).try_init();
    
    // Use debug assertions
    debug_assert_eq!(expected, actual);
    
    // Print intermediate values
    eprintln!("Debug: intermediate_value = {:?}", intermediate_value);
}
```

### Test Isolation
```rust
#[test]
fn test_isolated_state() {
    // Each test should be independent
    let mut scanner = Scanner::new();
    scanner.configure(test_config());
    
    // Test should not depend on global state
    assert_eq!(scanner.get_detector_count(), 5);
}
```

## Best Practices Checklist

### Before Writing Tests
- [ ] Understand the requirements and edge cases
- [ ] Consider the testing pyramid (more unit tests, fewer E2E)
- [ ] Plan test data and fixtures needed

### While Writing Tests
- [ ] Follow AAA pattern (Arrange, Act, Assert)
- [ ] Use descriptive test names
- [ ] Test one thing per test
- [ ] Include both positive and negative cases
- [ ] Test error conditions

### After Writing Tests
- [ ] Verify tests fail when they should
- [ ] Check test coverage reports
- [ ] Ensure tests run quickly
- [ ] Document complex test scenarios

### Code Review
- [ ] Tests are easy to understand
- [ ] Tests don't duplicate implementation logic
- [ ] Mocks are used appropriately
- [ ] Performance implications considered

---

*Last updated: 2024-12-19*
*See also: [Coverage Analysis](../plans/02-test-coverage-analysis.md), [Performance Testing](../performance/latest.md)*