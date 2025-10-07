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
