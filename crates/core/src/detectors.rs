use crate::{Match, PatternDetector};
use aho_corasick::AhoCorasick;
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

    // Development/Phase patterns
    pub static ref DEV_REGEX: Regex = Regex::new(r"\b(?i)(dev|development)\b").unwrap();
    pub static ref DEBUG_REGEX: Regex = Regex::new(r"\b(?i)debug\b").unwrap();
    pub static ref TEST_REGEX: Regex = Regex::new(r"\b(?i)(test|testing)\b").unwrap();
    pub static ref PHASE_REGEX: Regex = Regex::new(r"\b(?i)phase\s*[0-9]+\b").unwrap();
    pub static ref STAGING_REGEX: Regex = Regex::new(r"\b(?i)staging\b").unwrap();

    // Non-production code patterns
    pub static ref CONSOLE_LOG_REGEX: Regex = Regex::new(r"console\.(log|debug|info|warn|error)\s*\(").unwrap();
    pub static ref PRINT_REGEX: Regex = Regex::new(r"\b(print|printf|println!?|var_dump)\s*\(|console\.log\s*\(|\becho\s+").unwrap();
    pub static ref ALERT_REGEX: Regex = Regex::new(r"\b(alert|confirm|prompt)\s*\(").unwrap();
    pub static ref DEBUGGER_REGEX: Regex = Regex::new(r"\b(debugger|pdb\.set_trace|breakpoint|__debugbreak)\b").unwrap();
    pub static ref UNUSED_VAR_REGEX: Regex = Regex::new(r"\b(let|var|const)\s+(\w+)\s*[=;].*?\/\/\s*(?i)(unused|not\s+used)").unwrap();
    pub static ref DEAD_CODE_REGEX: Regex = Regex::new(r"\/\/\s*(?i)(dead\s*code|unreachable|never\s+called)").unwrap();
    pub static ref EXPERIMENTAL_REGEX: Regex = Regex::new(r"\b(?i)(experimental|prototype|poc|proof[\s-]of[\s-]concept)\b").unwrap();
}

fn detect_pattern_with_context(
    content: &str,
    file_path: &Path,
    pattern_name: &str,
    re: &Regex,
) -> Vec<Match> {
    let mut matches = smallvec::SmallVec::<[Match; 4]>::new();
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
    matches.into_vec()
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
                return detect_pattern_with_context(
                    content,
                    file_path,
                    "UNIMPLEMENTED",
                    &UNIMPLEMENTED_REGEX,
                );
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
                return detect_pattern_with_context(
                    content,
                    file_path,
                    "UNREACHABLE",
                    &UNREACHABLE_REGEX,
                );
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
                return detect_pattern_with_context(
                    content,
                    file_path,
                    "TO_STRING",
                    &TO_STRING_REGEX,
                );
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

/// Detector for development/dev environment references
pub struct DevDetector;

impl PatternDetector for DevDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "DEV", &DEV_REGEX)
    }
}

/// Detector for debug-related code
pub struct DebugDetector;

impl PatternDetector for DebugDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "DEBUG", &DEBUG_REGEX)
    }
}

/// Detector for test-related code in production files
pub struct TestDetector;

impl PatternDetector for TestDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        // Skip actual test files
        if let Some(path_str) = file_path.to_str() {
            if path_str.contains("test") || path_str.contains("spec") {
                return Vec::new();
            }
        }
        detect_pattern_with_context(content, file_path, "TEST", &TEST_REGEX)
    }
}

/// Detector for phase markers in code
pub struct PhaseDetector;

impl PatternDetector for PhaseDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "PHASE", &PHASE_REGEX)
    }
}

/// Detector for staging environment references
pub struct StagingDetector;

impl PatternDetector for StagingDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "STAGING", &STAGING_REGEX)
    }
}

/// Detector for console.log statements (JavaScript/TypeScript)
pub struct ConsoleLogDetector;

impl PatternDetector for ConsoleLogDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        // Detect in JavaScript/TypeScript files
        if let Some(ext) = file_path.extension() {
            let ext_str = ext.to_string_lossy();
            if matches!(
                ext_str.as_ref(),
                "js" | "ts" | "jsx" | "tsx" | "vue" | "svelte"
            ) {
                return detect_pattern_with_context(
                    content,
                    file_path,
                    "CONSOLE_LOG",
                    &CONSOLE_LOG_REGEX,
                );
            }
        }
        Vec::new()
    }
}

/// Detector for print statements in various languages
pub struct PrintDetector;

impl PatternDetector for PrintDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "PRINT", &PRINT_REGEX)
    }
}

/// Detector for alert/prompt statements (JavaScript)
pub struct AlertDetector;

impl PatternDetector for AlertDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        // Detect in JavaScript/TypeScript files
        if let Some(ext) = file_path.extension() {
            let ext_str = ext.to_string_lossy();
            if matches!(
                ext_str.as_ref(),
                "js" | "ts" | "jsx" | "tsx" | "html" | "vue" | "svelte"
            ) {
                return detect_pattern_with_context(content, file_path, "ALERT", &ALERT_REGEX);
            }
        }
        Vec::new()
    }
}

/// Detector for debugger statements and breakpoints
pub struct DebuggerDetector;

impl PatternDetector for DebuggerDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "DEBUGGER", &DEBUGGER_REGEX)
    }
}

/// Detector for explicitly marked unused variables
pub struct UnusedVarDetector;

impl PatternDetector for UnusedVarDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "UNUSED_VAR", &UNUSED_VAR_REGEX)
    }
}

/// Detector for dead code comments
pub struct DeadCodeDetector;

impl PatternDetector for DeadCodeDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "DEAD_CODE", &DEAD_CODE_REGEX)
    }
}

/// Detector for experimental/prototype code
pub struct ExperimentalDetector;

impl PatternDetector for ExperimentalDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        detect_pattern_with_context(content, file_path, "EXPERIMENTAL", &EXPERIMENTAL_REGEX)
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

/// High-performance detector using Aho-Corasick algorithm for multiple pattern matching
pub struct HighPerformanceDetector {
    pattern_names: Vec<String>,
    ac: AhoCorasick,
}

impl HighPerformanceDetector {
    /// Creates a new high-performance detector with the given patterns
    pub fn new(patterns: Vec<(&str, &str)>) -> Result<Self> {
        let (pattern_names, pattern_strings): (Vec<String>, Vec<String>) = patterns
            .into_iter()
            .map(|(name, pattern)| (name.to_string(), pattern.to_string()))
            .unzip();

        let ac = AhoCorasick::new(&pattern_strings)?;

        Ok(Self { pattern_names, ac })
    }

    /// Creates a detector for common TODO/FIXME patterns
    pub fn for_common_patterns() -> Self {
        let patterns = vec![
            ("TODO", r"(?i)todo"),
            ("FIXME", r"(?i)fixme"),
            ("HACK", r"(?i)hack"),
            ("BUG", r"(?i)bug"),
            ("XXX", r"XXX"),
            ("NOTE", r"(?i)note"),
            ("WARNING", r"(?i)warning"),
            ("PANIC", r"panic!"),
            ("UNWRAP", r"\.unwrap\(\)"),
            ("UNSAFE", r"unsafe\s+\{"),
            ("DEBUG", r"(?i)debug"),
            ("TEST", r"(?i)test"),
            ("PHASE", r"(?i)phase\s*[0-9]+"),
            ("CONSOLE_LOG", r"console\.(log|debug|info|warn|error)"),
            ("PRINT", r"print|println|echo"),
            ("ALERT", r"alert\(|confirm\(|prompt\("),
            ("DEBUGGER", r"debugger|pdb\.set_trace"),
        ];

        Self::new(patterns).unwrap()
    }
}

impl PatternDetector for HighPerformanceDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        let mut matches = Vec::new();

        for mat in self.ac.find_iter(content) {
            let pattern_id = mat.pattern();
            let pattern_name = &self.pattern_names[pattern_id.as_usize()];

            // Extract context around the match
            let start = mat.start().saturating_sub(15);
            let end = (mat.end() + 25).min(content.len());
            let context = &content[start..end];

            // Find the line number
            let line_start = content[..mat.start()]
                .rfind('\n')
                .map(|pos| pos + 1)
                .unwrap_or(0);
            let line_number = content[..line_start].lines().count() + 1;
            let column = mat.start() - line_start + 1;

            matches.push(Match {
                file_path: file_path.to_string_lossy().to_string(),
                line_number,
                column,
                pattern: pattern_name.clone(),
                message: format!("{}: {}", pattern_name, context.trim()),
            });
        }

        matches
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

    #[test]
    fn test_console_log_detector() {
        let detector = ConsoleLogDetector;
        let js_content = "console.log('debug info');";
        let py_content = "console.log('debug info');";

        let js_path = PathBuf::from("test.js");
        let py_path = PathBuf::from("test.py");

        let js_matches = detector.detect(js_content, &js_path);
        let py_matches = detector.detect(py_content, &py_path);

        assert_eq!(js_matches.len(), 1);
        assert_eq!(py_matches.len(), 0); // Should only detect in JS/TS files
        assert_eq!(js_matches[0].pattern, "CONSOLE_LOG");
    }

    #[test]
    fn test_debugger_detector() {
        let detector = DebuggerDetector;
        let content = "function test() {\n    debugger;\n    pdb.set_trace();\n}";
        let path = PathBuf::from("test.js");
        let matches = detector.detect(content, &path);
        assert_eq!(matches.len(), 2);
        assert!(matches.iter().all(|m| m.pattern == "DEBUGGER"));
    }

    #[test]
    fn test_phase_detector() {
        let detector = PhaseDetector;
        let content = "// Phase 1 implementation\nlet phase2_code = 'todo';";
        let path = PathBuf::from("test.js");
        let matches = detector.detect(content, &path);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].pattern, "PHASE");
        assert!(matches[0].message.contains("Phase 1"));
    }

    #[test]
    fn test_print_detector_multi_language() {
        let detector = PrintDetector;
        let content = "print('debug')\nprintf('test')\necho 'hello'\nconsole.log('js')";
        let path = PathBuf::from("test.py");
        let matches = detector.detect(content, &path);

        // Should find all 4 print statements
        assert_eq!(matches.len(), 4);
        assert!(matches.iter().all(|m| m.pattern == "PRINT"));

        // Check specific patterns are found
        let messages: Vec<String> = matches.iter().map(|m| m.message.clone()).collect();
        assert!(messages.iter().any(|m| m.contains("print(")));
        assert!(messages.iter().any(|m| m.contains("printf(")));
        assert!(messages.iter().any(|m| m.contains("console.log(")));
    }

    #[test]
    fn test_unused_var_detector() {
        let detector = UnusedVarDetector;
        let content =
            "let unusedVar = 5; // unused\nvar used = 10;\nconst another = 2; // not used";
        let path = PathBuf::from("test.js");
        let matches = detector.detect(content, &path);
        assert_eq!(matches.len(), 2);
        assert!(matches.iter().all(|m| m.pattern == "UNUSED_VAR"));
    }

    #[test]
    fn test_experimental_detector() {
        let detector = ExperimentalDetector;
        let content = "// experimental feature\n// This is a prototype\n// POC implementation";
        let path = PathBuf::from("test.rs");
        let matches = detector.detect(content, &path);
        assert_eq!(matches.len(), 3);
        assert!(matches.iter().all(|m| m.pattern == "EXPERIMENTAL"));
    }

    #[test]
    fn test_test_detector_skips_test_files() {
        let detector = TestDetector;
        let content = "// test implementation";

        let test_path = PathBuf::from("src/test/test_module.rs");
        let prod_path = PathBuf::from("src/main.rs");

        let test_matches = detector.detect(content, &test_path);
        let prod_matches = detector.detect(content, &prod_path);

        assert_eq!(test_matches.len(), 0); // Should skip test files
        assert_eq!(prod_matches.len(), 1); // Should detect in production files
    }
}
