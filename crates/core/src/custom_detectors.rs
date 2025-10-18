use crate::{Match, PatternDetector, Severity};
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Configuration for a custom detector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomDetectorConfig {
    pub name: String,
    pub description: String,
    pub pattern: String,
    pub file_extensions: Vec<String>, // Empty = all files
    pub case_sensitive: bool,
    pub multiline: bool,
    pub capture_groups: Vec<String>, // Named capture groups
    pub severity: Severity,
    pub category: DetectorCategory,
    pub examples: Vec<String>,
    pub enabled: bool,
}

/// Categories for organizing custom detectors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DetectorCategory {
    CodeQuality,
    Security,
    Performance,
    Documentation,
    Testing,
    Deprecated,
    Custom(String),
}

/// A custom pattern detector built from configuration
pub struct CustomDetector {
    config: CustomDetectorConfig,
    regex: Regex,
}

impl Clone for CustomDetector {
    fn clone(&self) -> Self {
        Self::new(self.config.clone()).unwrap()
    }
}

impl CustomDetector {
    /// Create a new custom detector from configuration
    pub fn new(config: CustomDetectorConfig) -> Result<Self> {
        let pattern = config.pattern.clone();

        // Build regex flags
        let mut regex_flags = regex::RegexBuilder::new(&pattern);
        regex_flags.case_insensitive(!config.case_sensitive);
        regex_flags.multi_line(config.multiline);

        let regex = regex_flags
            .build()
            .map_err(|e| anyhow::anyhow!("Invalid regex pattern '{}': {}", pattern, e))?;

        Ok(Self { config, regex })
    }

    /// Get detector configuration
    pub fn config(&self) -> &CustomDetectorConfig {
        &self.config
    }

    /// Check if this detector should process the given file
    fn should_process_file(&self, file_path: &Path) -> bool {
        if self.config.file_extensions.is_empty() {
            return true; // Process all files
        }

        if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
            self.config
                .file_extensions
                .iter()
                .any(|allowed_ext| allowed_ext.eq_ignore_ascii_case(ext))
        } else {
            false
        }
    }
}

impl PatternDetector for CustomDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        if !self.config.enabled || !self.should_process_file(file_path) {
            return Vec::new();
        }

        let mut matches = Vec::new();

        for cap in self.regex.captures_iter(content) {
            if let Some(full_match) = cap.get(0) {
                // Find line and column
                let (line_number, column) = find_line_column(content, full_match.start());

                // Extract message from capture groups or use full match
                let message = if !self.config.capture_groups.is_empty() {
                    self.extract_message_from_groups(&cap)
                } else {
                    full_match.as_str().trim().to_string()
                };

                matches.push(Match {
                    file_path: file_path.to_string_lossy().to_string(),
                    line_number,
                    column,
                    pattern: self.config.name.clone(),
                    message: format!("{}: {}", self.config.name, message),
                });
            }
        }

        matches
    }
}

impl CustomDetector {
    fn extract_message_from_groups(&self, cap: &regex::Captures) -> String {
        let mut parts = Vec::new();

        for group_name in &self.config.capture_groups {
            if let Some(group_match) = cap.name(group_name) {
                parts.push(format!("{}={}", group_name, group_match.as_str()));
            }
        }

        if parts.is_empty() {
            cap.get(0)
                .map_or("".to_string(), |m| m.as_str().to_string())
        } else {
            parts.join(", ")
        }
    }
}

/// Manager for custom detectors
pub struct CustomDetectorManager {
    detectors: HashMap<String, CustomDetector>,
    config_file: Option<std::path::PathBuf>,
}

impl CustomDetectorManager {
    pub fn new() -> Self {
        Self {
            detectors: HashMap::new(),
            config_file: None,
        }
    }

    /// Load detectors from configuration file
    pub fn load_from_file<P: AsRef<Path>>(&mut self, config_file: P) -> Result<()> {
        let config_file = config_file.as_ref();
        let content = std::fs::read_to_string(config_file)?;

        let configs: Vec<CustomDetectorConfig> =
            match config_file.extension().and_then(|s| s.to_str()) {
                Some("json") => serde_json::from_str(&content)?,
                Some("yaml" | "yml") => serde_yaml::from_str(&content)?,
                Some("toml") => toml::from_str(&content)?,
                _ => return Err(anyhow::anyhow!("Unsupported config file format")),
            };

        for config in configs {
            let detector = CustomDetector::new(config.clone())?;
            self.detectors.insert(config.name.clone(), detector);
        }

        self.config_file = Some(config_file.to_path_buf());
        println!(
            "📁 Loaded {} custom detectors from {}",
            self.detectors.len(),
            config_file.display()
        );

        Ok(())
    }

    /// Save detectors to configuration file
    pub fn save_to_file<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let configs: Vec<CustomDetectorConfig> = self
            .detectors
            .values()
            .map(|d| d.config().clone())
            .collect();

        let config_file = config_file.as_ref();
        let content = match config_file.extension().and_then(|s| s.to_str()) {
            Some("json") => serde_json::to_string_pretty(&configs)?,
            Some("yaml" | "yml") => serde_yaml::to_string(&configs)?,
            Some("toml") => toml::to_string_pretty(&configs)?,
            _ => return Err(anyhow::anyhow!("Unsupported config file format")),
        };

        std::fs::write(config_file, content)?;
        println!(
            "💾 Saved {} custom detectors to {}",
            configs.len(),
            config_file.display()
        );

        Ok(())
    }

    /// Add a new custom detector
    pub fn add_detector(&mut self, config: CustomDetectorConfig) -> Result<()> {
        let name = config.name.clone();
        let detector = CustomDetector::new(config)?;
        self.detectors.insert(name.clone(), detector);
        println!("➕ Added custom detector: {}", name);
        Ok(())
    }

    /// Remove a custom detector
    pub fn remove_detector(&mut self, name: &str) -> bool {
        if self.detectors.remove(name).is_some() {
            println!("➖ Removed custom detector: {}", name);
            true
        } else {
            false
        }
    }

    /// Get all custom detectors as PatternDetector trait objects
    pub fn get_detectors(&self) -> Vec<Box<dyn PatternDetector>> {
        self.detectors
            .values()
            .filter(|d| d.config().enabled)
            .map(|d| Box::new(d.clone()) as Box<dyn PatternDetector>)
            .collect()
    }

    /// List all detector configurations
    pub fn list_detectors(&self) -> Vec<&CustomDetectorConfig> {
        self.detectors.values().map(|d| d.config()).collect()
    }

    /// Enable/disable a detector
    pub fn set_detector_enabled(&mut self, name: &str, enabled: bool) -> Result<()> {
        if let Some(detector) = self.detectors.get_mut(name) {
            // Note: We'd need to modify CustomDetector to allow config mutation
            // For now, we'll recreate the detector with updated config
            let mut config = detector.config().clone();
            config.enabled = enabled;
            let new_detector = CustomDetector::new(config)?;
            self.detectors.insert(name.to_string(), new_detector);
            println!(
                "🔄 {} detector: {}",
                if enabled { "Enabled" } else { "Disabled" },
                name
            );
            Ok(())
        } else {
            Err(anyhow::anyhow!("Detector '{}' not found", name))
        }
    }

    /// Create some example detectors
    pub fn create_examples(&mut self) -> Result<()> {
        let examples = vec![
            CustomDetectorConfig {
                name: "SQL_INJECTION".to_string(),
                description: "Detect potential SQL injection vulnerabilities".to_string(),
                pattern: r#"(?i)(query|execute)\s*\(\s*["']\s*SELECT.*\+.*["']\s*\)"#.to_string(),
                file_extensions: vec!["py".to_string(), "js".to_string(), "php".to_string()],
                case_sensitive: false,
                multiline: false,
                capture_groups: vec![],
                severity: Severity::Critical,
                category: DetectorCategory::Security,
                examples: vec![r#"query("SELECT * FROM users WHERE id = " + user_id)"#.to_string()],
                enabled: true,
            },
            CustomDetectorConfig {
                name: "HARDCODED_PASSWORD".to_string(),
                description: "Detect hardcoded passwords and secrets".to_string(),
                pattern: r#"(?i)(password|secret|key|token)\s*[=:]\s*["'][^"']{8,}["']"#
                    .to_string(),
                file_extensions: vec![],
                case_sensitive: false,
                multiline: false,
                capture_groups: vec![],
                severity: Severity::High,
                category: DetectorCategory::Security,
                examples: vec![r#"password = "secretpassword123""#.to_string()],
                enabled: true,
            },
            CustomDetectorConfig {
                name: "LARGE_FUNCTION".to_string(),
                description: "Detect functions that might be too large".to_string(),
                pattern: r"fn\s+\w+[^{]*\{(?:[^{}]*\{[^{}]*\})*[^{}]{500,}\}".to_string(),
                file_extensions: vec!["rs".to_string()],
                case_sensitive: true,
                multiline: true,
                capture_groups: vec![],
                severity: Severity::Medium,
                category: DetectorCategory::CodeQuality,
                examples: vec!["Functions with more than 500 characters in body".to_string()],
                enabled: true,
            },
        ];

        for config in examples {
            self.add_detector(config)?;
        }

        Ok(())
    }
}

impl Default for CustomDetectorManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to find line and column from byte offset
fn find_line_column(content: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;

    for (i, ch) in content.char_indices() {
        if i >= offset {
            break;
        }

        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }

    (line, column)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_detector_creation() {
        let config = CustomDetectorConfig {
            name: "TEST".to_string(),
            description: "Test detector".to_string(),
            pattern: r"test".to_string(),
            file_extensions: vec!["rs".to_string()],
            case_sensitive: true,
            multiline: false,
            capture_groups: vec![],
            severity: Severity::Low,
            category: DetectorCategory::Testing,
            examples: vec![],
            enabled: true,
        };

        let detector = CustomDetector::new(config);
        assert!(detector.is_ok());
    }

    #[test]
    fn test_custom_detector_matching() {
        let config = CustomDetectorConfig {
            name: "TODO_CUSTOM".to_string(),
            description: "Custom TODO detector".to_string(),
            pattern: r"TODO:.*".to_string(),
            file_extensions: vec![],
            case_sensitive: false,
            multiline: false,
            capture_groups: vec![],
            severity: Severity::Low,
            category: DetectorCategory::Documentation,
            examples: vec![],
            enabled: true,
        };

        let detector = CustomDetector::new(config).unwrap();
        let content = "// TODO: implement this\nsome code";
        let matches = detector.detect(content, Path::new("test.rs"));

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line_number, 1);
    }

    #[test]
    fn test_detector_manager() {
        let mut manager = CustomDetectorManager::new();
        assert_eq!(manager.list_detectors().len(), 0);

        manager.create_examples().unwrap();
        assert!(!manager.list_detectors().is_empty());

        let detectors = manager.get_detectors();
        assert!(!detectors.is_empty());
    }

    #[test]
    fn test_empty_pattern() {
        let config = CustomDetectorConfig {
            name: "EMPTY".to_string(),
            description: "Empty pattern test".to_string(),
            pattern: "".to_string(),
            file_extensions: vec![],
            case_sensitive: true,
            multiline: false,
            capture_groups: vec![],
            severity: Severity::Low,
            category: DetectorCategory::Testing,
            examples: vec![],
            enabled: true,
        };

        let detector = CustomDetector::new(config);
        // Empty pattern is actually valid in regex (matches empty string)
        assert!(detector.is_ok());
    }

    #[test]
    fn test_complex_regex() {
        let config = CustomDetectorConfig {
            name: "COMPLEX".to_string(),
            description: "Complex regex with word boundaries".to_string(),
            pattern: r"\bclass\s+\w+\s+extends\s+\w+\s*\{".to_string(),
            file_extensions: vec!["js".to_string()],
            case_sensitive: true,
            multiline: false,
            capture_groups: vec![],
            severity: Severity::Medium,
            category: DetectorCategory::CodeQuality,
            examples: vec![],
            enabled: true,
        };

        let detector = CustomDetector::new(config).unwrap();
        let content = "class MyClass extends Base {\n  constructor() {}\n}";
        let matches = detector.detect(content, Path::new("test.js"));
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_large_content() {
        let config = CustomDetectorConfig {
            name: "LARGE_TEST".to_string(),
            description: "Test with large content".to_string(),
            pattern: r"TODO".to_string(),
            file_extensions: vec![],
            case_sensitive: false,
            multiline: false,
            capture_groups: vec![],
            severity: Severity::Low,
            category: DetectorCategory::Testing,
            examples: vec![],
            enabled: true,
        };

        let detector = CustomDetector::new(config).unwrap();
        let large_content = "some code\n".repeat(10000)
            + "// TODO: large file test\n"
            + &"more code\n".repeat(10000);
        let matches = detector.detect(&large_content, Path::new("large.rs"));
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line_number, 10001);
    }

    #[test]
    fn test_multiline_pattern() {
        let config = CustomDetectorConfig {
            name: "MULTILINE".to_string(),
            description: "Multiline pattern test".to_string(),
            pattern: r"function\s+\w+\([^)]*\)\s*\{[^}]*\}".to_string(),
            file_extensions: vec!["js".to_string()],
            case_sensitive: true,
            multiline: true,
            capture_groups: vec![],
            severity: Severity::Low,
            category: DetectorCategory::Testing,
            examples: vec![],
            enabled: true,
        };

        let detector = CustomDetector::new(config).unwrap();
        let content = "function test() {\n  return true;\n}\nother code";
        let matches = detector.detect(content, Path::new("test.js"));
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_case_insensitive() {
        let config = CustomDetectorConfig {
            name: "CASE_TEST".to_string(),
            description: "Case insensitive test".to_string(),
            pattern: r"todo".to_string(),
            file_extensions: vec![],
            case_sensitive: false,
            multiline: false,
            capture_groups: vec![],
            severity: Severity::Low,
            category: DetectorCategory::Testing,
            examples: vec![],
            enabled: true,
        };

        let detector = CustomDetector::new(config).unwrap();
        let content = "// TODO: case test\n// todo: another";
        let matches = detector.detect(content, Path::new("test.rs"));
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_file_extension_filtering() {
        let config = CustomDetectorConfig {
            name: "EXT_TEST".to_string(),
            description: "File extension test".to_string(),
            pattern: r"test".to_string(),
            file_extensions: vec!["rs".to_string()],
            case_sensitive: true,
            multiline: false,
            capture_groups: vec![],
            severity: Severity::Low,
            category: DetectorCategory::Testing,
            examples: vec![],
            enabled: true,
        };

        let detector = CustomDetector::new(config).unwrap();
        let content = "test content";

        // Should match .rs file
        let matches_rs = detector.detect(content, Path::new("test.rs"));
        assert_eq!(matches_rs.len(), 1);

        // Should not match .js file
        let js_matches = detector.detect(content, Path::new("test.js"));
        assert_eq!(js_matches.len(), 0);
    }

    #[test]
    fn test_capture_groups() {
        let config = CustomDetectorConfig {
            name: "CAPTURE".to_string(),
            description: "Capture groups test".to_string(),
            pattern: r"let\s+(?P<var>\w+)\s*=\s*(?P<value>\w+);".to_string(),
            file_extensions: vec![],
            case_sensitive: true,
            multiline: false,
            capture_groups: vec!["var".to_string(), "value".to_string()],
            severity: Severity::Low,
            category: DetectorCategory::Testing,
            examples: vec![],
            enabled: true,
        };

        let detector = CustomDetector::new(config).unwrap();
        let content = "let x = 42;";
        let matches = detector.detect(content, Path::new("test.rs"));
        assert_eq!(matches.len(), 1);
        assert!(matches[0].message.contains("var=x"));
        assert!(matches[0].message.contains("value=42"));
    }

    #[test]
    fn test_disabled_detector() {
        let config = CustomDetectorConfig {
            name: "DISABLED".to_string(),
            description: "Disabled detector test".to_string(),
            pattern: r"test".to_string(),
            file_extensions: vec![],
            case_sensitive: true,
            multiline: false,
            capture_groups: vec![],
            severity: Severity::Low,
            category: DetectorCategory::Testing,
            examples: vec![],
            enabled: false,
        };

        let detector = CustomDetector::new(config).unwrap();
        let content = "test content";
        let matches = detector.detect(content, Path::new("test.rs"));
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_invalid_regex() {
        let config = CustomDetectorConfig {
            name: "INVALID".to_string(),
            description: "Invalid regex test".to_string(),
            pattern: r"[unclosed".to_string(),
            file_extensions: vec![],
            case_sensitive: true,
            multiline: false,
            capture_groups: vec![],
            severity: Severity::Low,
            category: DetectorCategory::Testing,
            examples: vec![],
            enabled: true,
        };

        let detector = CustomDetector::new(config);
        assert!(detector.is_err());
    }
}
