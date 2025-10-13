use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Severity;

/// Enhanced configuration for more flexible pattern detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedScanConfig {
    /// Enabled pattern detectors
    pub enabled_detectors: Vec<DetectorType>,
    /// File extensions to include in scanning
    pub include_extensions: Vec<String>,
    /// File extensions to exclude from scanning
    pub exclude_extensions: Vec<String>,
    /// Paths to exclude from scanning (glob patterns)
    pub exclude_paths: Vec<String>,
    /// Maximum file size to scan (in bytes)
    pub max_file_size: Option<usize>,
    /// Custom regex patterns
    pub custom_patterns: HashMap<String, String>,
    /// Severity levels for different pattern types
    pub severity_levels: HashMap<String, Severity>,
}

/// Types of available pattern detectors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DetectorType {
    // Comment-based patterns
    Todo,
    Fixme,
    Hack,
    Bug,
    Xxx,
    Note,
    Warning,

    // Rust-specific patterns
    Panic,
    Unwrap,
    Expect,
    Unimplemented,
    Unreachable,

    // Performance patterns
    Clone,
    ToString,

    // Security patterns
    Unsafe,

    // Development/Phase patterns
    Dev,
    Debug,
    Test,
    Phase,
    Staging,

    // Non-production code patterns
    ConsoleLog,
    Print,
    Alert,
    Debugger,
    UnusedVar,
    DeadCode,
    Experimental,

    // LLM-specific security patterns
    LLMHallucinatedApi,
    LLMSQLInjection,
    LLMInsecureRandom,
    LLMHardcodedCredentials,
    LLMRustMemorySafety,
    LLMCryptoAntipattern,
    LLMXSSInjection,
    LLMFilesystemSecurity,
    LLMContextConfusion,

    // LLM-specific quality patterns
    LLMAsyncAntipattern,
    LLMPerformanceIssue,
    LLMErrorHandling,
    LLMOverengineering,
    LLMConfigAntipattern,
    LLMDatabaseAntipattern,
    LLMJSIssues,
    LLMPythonIssues,
    LLMGeneratedComments,

    // Comprehensive LLM detector
    LLMComprehensive,

    // Custom pattern with name
    Custom(String),
}

impl Default for EnhancedScanConfig {
    fn default() -> Self {
        let mut severity_levels = HashMap::new();
        severity_levels.insert("TODO".to_string(), Severity::Low);
        severity_levels.insert("FIXME".to_string(), Severity::Medium);
        severity_levels.insert("HACK".to_string(), Severity::High);
        severity_levels.insert("BUG".to_string(), Severity::High);
        severity_levels.insert("XXX".to_string(), Severity::Critical);
        severity_levels.insert("PANIC".to_string(), Severity::High);
        severity_levels.insert("UNWRAP".to_string(), Severity::Medium);
        severity_levels.insert("UNSAFE".to_string(), Severity::High);

        // Development/Phase patterns
        severity_levels.insert("DEV".to_string(), Severity::High);
        severity_levels.insert("DEBUG".to_string(), Severity::Medium);
        severity_levels.insert("TEST".to_string(), Severity::Medium);
        severity_levels.insert("PHASE".to_string(), Severity::Medium);
        severity_levels.insert("STAGING".to_string(), Severity::High);

        // Non-production code patterns
        severity_levels.insert("CONSOLE_LOG".to_string(), Severity::High);
        severity_levels.insert("PRINT".to_string(), Severity::Medium);
        severity_levels.insert("ALERT".to_string(), Severity::High);
        severity_levels.insert("DEBUGGER".to_string(), Severity::Critical);
        severity_levels.insert("UNUSED_VAR".to_string(), Severity::Low);
        severity_levels.insert("DEAD_CODE".to_string(), Severity::Medium);
        severity_levels.insert("EXPERIMENTAL".to_string(), Severity::Medium);

        // LLM-specific security patterns (high priority)
        severity_levels.insert("LLM_HALLUCINATED_API".to_string(), Severity::High);
        severity_levels.insert("LLM_SQL_INJECTION".to_string(), Severity::Critical);
        severity_levels.insert("LLM_INSECURE_RANDOM".to_string(), Severity::High);
        severity_levels.insert("LLM_HARDCODED_CREDENTIALS".to_string(), Severity::Critical);
        severity_levels.insert("LLM_RUST_MEMORY_SAFETY".to_string(), Severity::High);
        severity_levels.insert("LLM_CRYPTO_ANTIPATTERN".to_string(), Severity::High);
        severity_levels.insert("LLM_XSS_INJECTION".to_string(), Severity::Critical);
        severity_levels.insert("LLM_FILESYSTEM_SECURITY".to_string(), Severity::High);
        severity_levels.insert("LLM_CONTEXT_CONFUSION".to_string(), Severity::High);

        // LLM-specific quality patterns (medium priority)
        severity_levels.insert("LLM_ASYNC_ANTIPATTERN".to_string(), Severity::Medium);
        severity_levels.insert("LLM_PERFORMANCE_ISSUE".to_string(), Severity::Medium);
        severity_levels.insert("LLM_ERROR_HANDLING".to_string(), Severity::Medium);
        severity_levels.insert("LLM_OVERENGINEERING".to_string(), Severity::Low);
        severity_levels.insert("LLM_CONFIG_ANTIPATTERN".to_string(), Severity::Medium);
        severity_levels.insert("LLM_DATABASE_ANTIPATTERN".to_string(), Severity::Medium);
        severity_levels.insert("LLM_JS_ISSUES".to_string(), Severity::Medium);
        severity_levels.insert("LLM_PYTHON_ISSUES".to_string(), Severity::High);
        severity_levels.insert("LLM_GENERATED_COMMENT".to_string(), Severity::Info);

        Self {
            enabled_detectors: vec![DetectorType::Todo, DetectorType::Fixme],
            include_extensions: vec![
                "rs".to_string(),
                "py".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "tsx".to_string(),
                "jsx".to_string(),
                "java".to_string(),
                "cs".to_string(),
                "cpp".to_string(),
                "cxx".to_string(),
                "c".to_string(),
                "h".to_string(),
                "hpp".to_string(),
                "go".to_string(),
                "php".to_string(),
                "rb".to_string(),
                "kt".to_string(),
                "swift".to_string(),
                "dart".to_string(),
                "scala".to_string(),
                "sh".to_string(),
                "ps1".to_string(),
                "sql".to_string(),
                "html".to_string(),
                "vue".to_string(),
                "svelte".to_string(),
                "md".to_string(),
                "txt".to_string(),
                "yml".to_string(),
                "yaml".to_string(),
                "json".to_string(),
                "toml".to_string(),
            ],
            exclude_extensions: vec![
                "exe".to_string(),
                "dll".to_string(),
                "so".to_string(),
                "bin".to_string(),
                "png".to_string(),
                "jpg".to_string(),
                "jpeg".to_string(),
                "gif".to_string(),
                "pdf".to_string(),
                "zip".to_string(),
            ],
            exclude_paths: vec![
                "target/*".to_string(),
                "node_modules/*".to_string(),
                ".git/*".to_string(),
                "*.lock".to_string(),
                "vendor/*".to_string(),
                "build/*".to_string(),
            ],
            max_file_size: Some(1024 * 1024), // 1MB default
            custom_patterns: HashMap::new(),
            severity_levels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_scan_config_default() {
        let config = EnhancedScanConfig::default();

        assert!(!config.enabled_detectors.is_empty());
        assert!(config.enabled_detectors.contains(&DetectorType::Todo));
        assert!(config.enabled_detectors.contains(&DetectorType::Fixme));
        assert!(config.include_extensions.contains(&"rs".to_string()));
        assert!(config.exclude_paths.contains(&"target/*".to_string()));
        assert_eq!(config.max_file_size, Some(1024 * 1024));
    }

    #[test]
    fn test_detector_type_equality() {
        assert_eq!(DetectorType::Todo, DetectorType::Todo);
        assert_ne!(DetectorType::Todo, DetectorType::Fixme);
        assert_eq!(
            DetectorType::Custom("test".to_string()),
            DetectorType::Custom("test".to_string())
        );
        assert_ne!(
            DetectorType::Custom("test1".to_string()),
            DetectorType::Custom("test2".to_string())
        );
    }

    #[test]
    fn test_detector_type_serialization() {
        let detector = DetectorType::Todo;
        let json = serde_json::to_string(&detector).unwrap();
        let deserialized: DetectorType = serde_json::from_str(&json).unwrap();
        assert_eq!(detector, deserialized);

        // Test custom detector
        let custom_detector = DetectorType::Custom("my_pattern".to_string());
        let json = serde_json::to_string(&custom_detector).unwrap();
        let deserialized: DetectorType = serde_json::from_str(&json).unwrap();
        assert_eq!(custom_detector, deserialized);
    }

    #[test]
    fn test_enhanced_config_serialization() {
        let config = EnhancedScanConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: EnhancedScanConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.enabled_detectors, deserialized.enabled_detectors);
        assert_eq!(config.include_extensions, deserialized.include_extensions);
        assert_eq!(config.exclude_paths, deserialized.exclude_paths);
        assert_eq!(config.max_file_size, deserialized.max_file_size);
    }

    #[test]
    fn test_severity_levels_defaults() {
        let config = EnhancedScanConfig::default();

        // Test basic severity levels
        assert_eq!(config.severity_levels.get("TODO"), Some(&Severity::Low));
        assert_eq!(config.severity_levels.get("FIXME"), Some(&Severity::Medium));
        assert_eq!(config.severity_levels.get("HACK"), Some(&Severity::High));
        assert_eq!(config.severity_levels.get("XXX"), Some(&Severity::Critical));

        // Test LLM-specific patterns
        assert_eq!(
            config.severity_levels.get("LLM_SQL_INJECTION"),
            Some(&Severity::Critical)
        );
        assert_eq!(
            config.severity_levels.get("LLM_HARDCODED_CREDENTIALS"),
            Some(&Severity::Critical)
        );
        assert_eq!(
            config.severity_levels.get("LLM_XSS_INJECTION"),
            Some(&Severity::Critical)
        );

        // Test development patterns
        assert_eq!(config.severity_levels.get("DEV"), Some(&Severity::High));
        assert_eq!(
            config.severity_levels.get("CONSOLE_LOG"),
            Some(&Severity::High)
        );
        assert_eq!(
            config.severity_levels.get("DEBUGGER"),
            Some(&Severity::Critical)
        );
    }

    #[test]
    fn test_all_detector_types_coverage() {
        // Test all basic detector types
        let basic_detectors = vec![
            DetectorType::Todo,
            DetectorType::Fixme,
            DetectorType::Hack,
            DetectorType::Bug,
            DetectorType::Xxx,
            DetectorType::Note,
            DetectorType::Warning,
        ];

        for detector in basic_detectors {
            let cloned = detector.clone();
            assert_eq!(detector, cloned);
        }

        // Test Rust-specific detectors
        let rust_detectors = vec![
            DetectorType::Panic,
            DetectorType::Unwrap,
            DetectorType::Expect,
            DetectorType::Unimplemented,
            DetectorType::Unreachable,
        ];

        for detector in rust_detectors {
            let cloned = detector.clone();
            assert_eq!(detector, cloned);
        }

        // Test LLM-specific detectors
        let llm_detectors = vec![
            DetectorType::LLMHallucinatedApi,
            DetectorType::LLMSQLInjection,
            DetectorType::LLMInsecureRandom,
            DetectorType::LLMHardcodedCredentials,
            DetectorType::LLMComprehensive,
        ];

        for detector in llm_detectors {
            let cloned = detector.clone();
            assert_eq!(detector, cloned);
        }
    }

    #[test]
    fn test_config_with_custom_patterns() {
        let mut config = EnhancedScanConfig::default();
        config
            .custom_patterns
            .insert("CUSTOM_PATTERN".to_string(), r"CUSTOM:\s*(.+)".to_string());
        config
            .severity_levels
            .insert("CUSTOM_PATTERN".to_string(), Severity::Medium);

        assert_eq!(config.custom_patterns.len(), 1);
        assert_eq!(
            config.severity_levels.get("CUSTOM_PATTERN"),
            Some(&Severity::Medium)
        );

        // Test that custom pattern is properly stored
        assert_eq!(
            config.custom_patterns.get("CUSTOM_PATTERN"),
            Some(&r"CUSTOM:\s*(.+)".to_string())
        );
    }

    #[test]
    fn test_file_extension_filters() {
        let config = EnhancedScanConfig::default();

        // Verify common programming languages are included
        assert!(config.include_extensions.contains(&"rs".to_string()));
        assert!(config.include_extensions.contains(&"py".to_string()));
        assert!(config.include_extensions.contains(&"js".to_string()));
        assert!(config.include_extensions.contains(&"java".to_string()));
        assert!(config.include_extensions.contains(&"go".to_string()));

        // Verify binary files are excluded
        assert!(config.exclude_extensions.contains(&"exe".to_string()));
        assert!(config.exclude_extensions.contains(&"dll".to_string()));
        assert!(config.exclude_extensions.contains(&"png".to_string()));
        assert!(config.exclude_extensions.contains(&"zip".to_string()));
    }

    #[test]
    fn test_path_exclusion_patterns() {
        let config = EnhancedScanConfig::default();

        // Verify common build directories are excluded
        assert!(config.exclude_paths.contains(&"target/*".to_string()));
        assert!(config.exclude_paths.contains(&"node_modules/*".to_string()));
        assert!(config.exclude_paths.contains(&".git/*".to_string()));
        assert!(config.exclude_paths.contains(&"build/*".to_string()));
        assert!(config.exclude_paths.contains(&"vendor/*".to_string()));
    }

    #[test]
    fn test_config_clone() {
        let config = EnhancedScanConfig::default();
        let cloned = config.clone();

        assert_eq!(config.enabled_detectors, cloned.enabled_detectors);
        assert_eq!(config.include_extensions, cloned.include_extensions);
        assert_eq!(config.exclude_extensions, cloned.exclude_extensions);
        assert_eq!(config.exclude_paths, cloned.exclude_paths);
        assert_eq!(config.max_file_size, cloned.max_file_size);
        assert_eq!(config.custom_patterns, cloned.custom_patterns);
        assert_eq!(config.severity_levels, cloned.severity_levels);
    }

    #[test]
    fn test_llm_security_patterns_comprehensive() {
        let config = EnhancedScanConfig::default();

        // Verify all critical security patterns are marked as Critical or High
        let critical_patterns = vec![
            "LLM_SQL_INJECTION",
            "LLM_HARDCODED_CREDENTIALS",
            "LLM_XSS_INJECTION",
            "DEBUGGER",
            "XXX",
        ];

        for pattern in critical_patterns {
            let severity = config.severity_levels.get(pattern);
            assert!(
                severity == Some(&Severity::Critical) || severity == Some(&Severity::High),
                "Pattern {} should be Critical or High severity, got: {:?}",
                pattern,
                severity
            );
        }
    }
}
