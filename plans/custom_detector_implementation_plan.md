# Custom Pattern Detector Implementation Plan

## Overview
This GOAP (Goal-Oriented Action Planning) plan outlines the implementation of custom pattern detector support in the code-guardian detector factory. The goal is to enable users to define custom regex patterns in the `EnhancedScanConfig.custom_patterns` HashMap and enable them via `DetectorType::Custom(String)` in the `enabled_detectors` list.

## Current State Analysis
- **DetectorFactory**: Currently handles predefined detector types but returns `None` for `DetectorType::Custom(_)`
- **EnhancedScanConfig**: Contains `custom_patterns: HashMap<String, String>` and `severity_levels: HashMap<String, Severity>` but these are unused for detector creation
- **PatternDetector Trait**: Well-defined interface for implementing custom detectors
- **Existing Infrastructure**: `detect_pattern_with_context` helper function and regex compilation patterns already exist

## GOAP Plan Structure

### Initial State
- Custom patterns defined in config but not accessible to detector factory
- `DetectorType::Custom` returns `None`
- No custom detector implementation in core
- CLI loads custom detectors from separate files via `CustomDetectorManager`

### Goal State
- `DetectorType::Custom(name)` creates functional detectors from config patterns
- Custom detectors integrate seamlessly with existing detector profiles
- Error handling for invalid patterns and missing configurations
- Full test coverage for custom detector functionality
- Documentation and examples for custom pattern usage

### Preconditions
1. Access to `EnhancedScanConfig` instance during detector creation
2. Valid regex patterns in `custom_patterns` HashMap
3. Existing `PatternDetector` trait implementation patterns
4. Test infrastructure for detector validation

### Actions and Effects

#### Action 1: Create CustomPatternDetector Struct
**Preconditions**: PatternDetector trait available, regex crate accessible
**Effects**: 
- New `CustomPatternDetector` struct implementing `PatternDetector`
- Uses `detect_pattern_with_context` for consistent matching behavior
- Stores name and compiled regex
**Implementation**:
```rust
pub struct CustomPatternDetector {
    name: String,
    regex: Regex,
}

impl CustomPatternDetector {
    pub fn new(name: &str, pattern: &str) -> Result<Self> {
        let regex = Regex::new(pattern)?;
        Ok(Self { name: name.to_string(), regex })
    }
}

impl PatternDetector for CustomPatternDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, &self.name, &self.regex)
    }
}
```

#### Action 2: Modify create_detector Method Signature
**Preconditions**: Action 1 completed
**Effects**:
- Method accepts `Option<&EnhancedScanConfig>` parameter
- Returns `Result<Option<Box<dyn PatternDetector>>>` for error handling
- Maintains backward compatibility for existing calls
**Implementation**:
```rust
fn create_detector(detector_type: &DetectorType, config: Option<&EnhancedScanConfig>) -> Result<Option<Box<dyn PatternDetector>>> {
    // ... existing cases ...
    DetectorType::Custom(name) => {
        if let Some(config) = config {
            if let Some(pattern) = config.custom_patterns.get(name) {
                let detector = CustomPatternDetector::new(name, pattern)?;
                Ok(Some(Box::new(detector)))
            } else {
                Ok(None) // Pattern not found in config
            }
        } else {
            Ok(None) // No config provided
        }
    }
}
```

#### Action 3: Update create_detectors Method
**Preconditions**: Action 2 completed
**Effects**:
- Passes config to `create_detector` calls
- Handles `Result` from `create_detector`
- Logs warnings for failed custom detector creation
**Implementation**:
```rust
pub fn create_detectors(config: &EnhancedScanConfig) -> Vec<Box<dyn PatternDetector>> {
    let mut detectors = Vec::new();
    for detector_type in &config.enabled_detectors {
        match Self::create_detector(detector_type, Some(config)) {
            Ok(Some(detector)) => detectors.push(detector),
            Ok(None) => {} // Detector type not supported or disabled
            Err(e) => eprintln!("Warning: Failed to create detector for {:?}: {}", detector_type, e),
        }
    }
    detectors
}
```

#### Action 4: Update Profile Methods
**Preconditions**: Action 2 completed
**Effects**:
- Profile methods pass `None` for config (no custom detectors in profiles)
- Maintains existing behavior for predefined profiles
**Implementation**:
- No changes needed to method signatures
- `create_detector` calls remain with `None` config

#### Action 5: Add CustomPatternDetector to detectors.rs
**Preconditions**: Action 1 completed
**Effects**:
- `CustomPatternDetector` added to detectors module
- Proper imports and module exports
**Implementation**:
- Add struct and impl blocks to `detectors.rs`
- Update `mod.rs` if needed for exports

#### Action 6: Add Comprehensive Tests
**Preconditions**: Actions 1-5 completed
**Effects**:
- Unit tests for `CustomPatternDetector` creation and detection
- Integration tests for factory with custom detectors
- Error handling tests for invalid patterns
**Implementation**:
```rust
#[test]
fn test_custom_pattern_detector() {
    let detector = CustomPatternDetector::new("TEST", r"test").unwrap();
    let matches = detector.detect("this is a test", Path::new("test.txt"));
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].pattern, "TEST");
}

#[test]
fn test_factory_with_custom_detectors() {
    let mut config = EnhancedScanConfig::default();
    config.custom_patterns.insert("MY_PATTERN".to_string(), r"custom".to_string());
    config.enabled_detectors.push(DetectorType::Custom("MY_PATTERN".to_string()));
    
    let detectors = DetectorFactory::create_detectors(&config);
    assert!(detectors.len() >= 1);
}
```

#### Action 7: Update Error Handling and Logging
**Preconditions**: Actions 1-6 completed
**Effects**:
- Graceful handling of regex compilation errors
- Informative error messages for debugging
- Logging for successful custom detector creation
**Implementation**:
- Use `anyhow::Result` for detailed error context
- Add debug logging in factory methods

#### Action 8: Update Documentation and Examples
**Preconditions**: Actions 1-7 completed
**Effects**:
- README updates with custom pattern usage examples
- Code comments explaining custom detector integration
- Example configuration files
**Implementation**:
- Add section to README.md about custom patterns
- Document config file format for custom_patterns

#### Action 9: Integration Testing with CLI
**Preconditions**: Actions 1-8 completed
**Effects**:
- CLI can load configs with custom patterns
- Scan commands work with custom detectors
- Output formatting handles custom pattern names
**Implementation**:
- Test CLI with config files containing custom_patterns
- Verify scan results include custom detector matches

#### Action 10: Performance and Edge Case Testing
**Preconditions**: Actions 1-9 completed
**Effects**:
- Performance benchmarks for custom detectors
- Edge case handling (empty patterns, special regex, large files)
- Memory usage validation
**Implementation**:
- Add benchmark tests for custom detector performance
- Test with complex regex patterns and large codebases

### Action Sequencing
1. **Parallel**: Actions 1 (CustomPatternDetector) and 5 (add to detectors.rs)
2. **Sequential**: Action 2 → Action 3 → Action 4
3. **Sequential**: Actions 1-5 → Action 6 (tests)
4. **Sequential**: Actions 1-6 → Actions 7-10

### Success Criteria
- All tests pass (unit, integration, CLI)
- Custom detectors appear in scan results
- Invalid patterns don't crash the application
- Performance impact is minimal (<5% overhead)
- Documentation is complete and accurate
- Backward compatibility maintained

### Risk Mitigation
- **Regex Errors**: Wrap regex compilation in Result with descriptive errors
- **Missing Patterns**: Return None silently for missing custom patterns
- **Performance**: Limit regex complexity through validation
- **Backward Compatibility**: Keep existing APIs unchanged

### Estimated Effort
- **Development**: 4-6 hours
- **Testing**: 2-3 hours  
- **Documentation**: 1-2 hours
- **Total**: 7-11 hours

This plan provides a complete roadmap for implementing custom pattern detector support while maintaining code quality and system stability.