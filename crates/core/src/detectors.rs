use crate::{Match, PatternDetector};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;

lazy_static! {
    pub static ref TODO_REGEX: Regex = Regex::new(r"\b(?i)todo\b").unwrap();
    pub static ref FIXME_REGEX: Regex = Regex::new(r"\b(?i)fixme\b").unwrap();
    pub static ref HACK_REGEX: Regex = Regex::new(r"\b(?i)hack\b").unwrap();
    pub static ref BUG_REGEX: Regex = Regex::new(r"\b(?i)bug\b").unwrap();
    pub static ref XXX_REGEX: Regex = Regex::new(r"\bXXX\b").unwrap();
    pub static ref NOTE_REGEX: Regex = Regex::new(r"\b(?i)note\b").unwrap();
    pub static ref WARNING_REGEX: Regex = Regex::new(r"\b(?i)warning\b").unwrap();
    // Rust-specific patterns
    pub static ref PANIC_REGEX: Regex = Regex::new(r"\bpanic!\s*\(").unwrap();
    pub static ref UNWRAP_REGEX: Regex = Regex::new(r"\.unwrap\s*\(\s*\)").unwrap();
    pub static ref EXPECT_REGEX: Regex = Regex::new(r"\.expect\s*\(").unwrap();
    pub static ref UNIMPLEMENTED_REGEX: Regex = Regex::new(r"\bunimplemented!\s*\(").unwrap();
    pub static ref UNREACHABLE_REGEX: Regex = Regex::new(r"\bunreachable!\s*\(").unwrap();
    // Performance patterns
    pub static ref CLONE_REGEX: Regex = Regex::new(r"\.clone\s*\(\s*\)").unwrap();
    pub static ref TO_STRING_REGEX: Regex = Regex::new(r"\.to_string\s*\(\s*\)").unwrap();
    // Security patterns
    pub static ref UNSAFE_REGEX: Regex = Regex::new(r"\bunsafe\s+\{").unwrap();
}

fn detect_pattern_with_context(
    content: &str, 
    file_path: &Path, 
    pattern_name: &str, 
    re: &Regex
) -> Vec<Match> {
    let mut matches = Vec::new();
    for (line_idx, line) in content.lines().enumerate() {
        for mat in re.find_iter(line) {
            // Extract more context around the match
            let context_start = mat.start().saturating_sub(10);
            let context_end = (mat.end() + 20).min(line.len());
            let context = &line[context_start..context_end];
            
            matches.push(Match {
                file_path: file_path.to_string_lossy().to_string(),
                line_number: line_idx + 1,
                column: mat.start() + 1,
                pattern: pattern_name.to_string(),
                message: format!("{}: {}", pattern_name, context.trim()),
            });
        }
    }
    matches
}

/// Default detector for TODO comments (case-insensitive)
pub struct TodoDetector;

impl PatternDetector for TodoDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "TODO", &TODO_REGEX)
    }
}

/// Default detector for FIXME comments (case-insensitive)
pub struct FixmeDetector;

impl PatternDetector for FixmeDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "FIXME", &FIXME_REGEX)
    }
}

/// Detector for HACK comments indicating temporary workarounds
pub struct HackDetector;

impl PatternDetector for HackDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "HACK", &HACK_REGEX)
    }
}

/// Detector for BUG comments indicating known issues
pub struct BugDetector;

impl PatternDetector for BugDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "BUG", &BUG_REGEX)
    }
}

/// Detector for XXX comments indicating urgent attention needed
pub struct XxxDetector;

impl PatternDetector for XxxDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "XXX", &XXX_REGEX)
    }
}

/// Detector for NOTE comments
pub struct NoteDetector;

impl PatternDetector for NoteDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "NOTE", &NOTE_REGEX)
    }
}

/// Detector for WARNING comments
pub struct WarningDetector;

impl PatternDetector for WarningDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "WARNING", &WARNING_REGEX)
    }
}

/// Detector for panic! macros in Rust code
pub struct PanicDetector;

impl PatternDetector for PanicDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        // Only detect in Rust files
        if let Some(ext) = file_path.extension() {
            if ext == "rs" {
                return detect_pattern_with_context(content, file_path, "PANIC", &PANIC_REGEX);
            }
        }
        Vec::new()
    }
}

/// Detector for .unwrap() calls in Rust code (potential panic points)
pub struct UnwrapDetector;

impl PatternDetector for UnwrapDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        // Only detect in Rust files
        if let Some(ext) = file_path.extension() {
            if ext == "rs" {
                return detect_pattern_with_context(content, file_path, "UNWRAP", &UNWRAP_REGEX);
            }
        }
        Vec::new()
    }
}

/// Detector for .expect() calls in Rust code
pub struct ExpectDetector;

impl PatternDetector for ExpectDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        // Only detect in Rust files
        if let Some(ext) = file_path.extension() {
            if ext == "rs" {
                return detect_pattern_with_context(content, file_path, "EXPECT", &EXPECT_REGEX);
            }
        }
        Vec::new()
    }
}

/// Detector for unimplemented! macros in Rust code
pub struct UnimplementedDetector;

impl PatternDetector for UnimplementedDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        // Only detect in Rust files
        if let Some(ext) = file_path.extension() {
            if ext == "rs" {
                return detect_pattern_with_context(content, file_path, "UNIMPLEMENTED", &UNIMPLEMENTED_REGEX);
            }
        }
        Vec::new()
    }
}

/// Detector for unreachable! macros in Rust code
pub struct UnreachableDetector;

impl PatternDetector for UnreachableDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        // Only detect in Rust files
        if let Some(ext) = file_path.extension() {
            if ext == "rs" {
                return detect_pattern_with_context(content, file_path, "UNREACHABLE", &UNREACHABLE_REGEX);
            }
        }
        Vec::new()
    }
}

/// Detector for excessive .clone() calls (potential performance issue)
pub struct CloneDetector;

impl PatternDetector for CloneDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        // Only detect in Rust files
        if let Some(ext) = file_path.extension() {
            if ext == "rs" {
                return detect_pattern_with_context(content, file_path, "CLONE", &CLONE_REGEX);
            }
        }
        Vec::new()
    }
}

/// Detector for .to_string() calls (potential performance issue)
pub struct ToStringDetector;

impl PatternDetector for ToStringDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        // Only detect in Rust files
        if let Some(ext) = file_path.extension() {
            if ext == "rs" {
                return detect_pattern_with_context(content, file_path, "TO_STRING", &TO_STRING_REGEX);
            }
        }
        Vec::new()
    }
}

/// Detector for unsafe blocks in Rust code (security concern)
pub struct UnsafeDetector;

impl PatternDetector for UnsafeDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        // Only detect in Rust files
        if let Some(ext) = file_path.extension() {
            if ext == "rs" {
                return detect_pattern_with_context(content, file_path, "UNSAFE", &UNSAFE_REGEX);
            }
        }
        Vec::new()
    }
}

/// Custom pattern detector that uses user-defined regex patterns
pub struct CustomPatternDetector {
    name: String,
    regex: Regex,
}

impl CustomPatternDetector {
    /// Creates a new custom pattern detector with the given name and regex pattern
    pub fn new(name: &str, pattern: &str) -> Result<Self> {
        let regex = Regex::new(pattern)?;
        Ok(Self {
            name: name.to_string(),
            regex,
        })
    }
}

impl PatternDetector for CustomPatternDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, &self.name, &self.regex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_hack_detector() {
        let detector = HackDetector;
        let content = "// HACK: temporary fix\nlet x = 1;";
        let path = PathBuf::from("test.rs");
        let matches = detector.detect(content, &path);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].pattern, "HACK");
    }

    #[test]
    fn test_panic_detector_rust_only() {
        let detector = PanicDetector;
        let rust_content = "panic!(\"error\");";
        let js_content = "panic!(\"error\");";
        
        let rust_path = PathBuf::from("test.rs");
        let js_path = PathBuf::from("test.js");
        
        let rust_matches = detector.detect(rust_content, &rust_path);
        let js_matches = detector.detect(js_content, &js_path);
        
        assert_eq!(rust_matches.len(), 1);
        assert_eq!(js_matches.len(), 0);
    }

    #[test]
    fn test_unwrap_detector() {
        let detector = UnwrapDetector;
        let content = "let value = some_option.unwrap();";
        let path = PathBuf::from("test.rs");
        let matches = detector.detect(content, &path);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].pattern, "UNWRAP");
    }

    #[test]
    fn test_case_insensitive_todo() {
        let detector = TodoDetector;
        let content = "todo: fix this\nTODO: another\nTodo: yet another";
        let path = PathBuf::from("test.rs");
        let matches = detector.detect(content, &path);
        assert_eq!(matches.len(), 3);
    }

    #[test]
    fn test_custom_pattern_detector() {
        let detector = CustomPatternDetector::new("TEST", r"test").unwrap();
        let content = "this is a test";
        let path = PathBuf::from("test.txt");
        let matches = detector.detect(content, &path);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].pattern, "TEST");
        assert_eq!(matches[0].line_number, 1);
        assert!(matches[0].message.contains("TEST"));
    }

    #[test]
    fn test_custom_pattern_detector_invalid_regex() {
        let result = CustomPatternDetector::new("TEST", r"[invalid");
        assert!(result.is_err());
    }
}