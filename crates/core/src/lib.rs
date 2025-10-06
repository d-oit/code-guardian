use anyhow::Result;
use ignore::WalkBuilder;
use rayon::prelude::*;
use regex::Regex;
use std::path::Path;

/// Represents a detected pattern match in a file.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Match {
    /// The path to the file where the match was found.
    pub file_path: String,
    /// The line number (1-based) where the match starts.
    pub line_number: usize,
    /// The column number (1-based) where the match starts.
    pub column: usize,
    /// The type of pattern detected (e.g., "TODO", "FIXME").
    pub pattern: String,
    /// The matched text or a descriptive message.
    pub message: String,
}

/// Trait for detecting patterns in code content.
/// Implementors should define how to find specific patterns like TODO or FIXME.
pub trait PatternDetector: Send + Sync {
    /// Detects patterns in the given content and returns a list of matches.
    /// The file_path is provided for context, such as filtering by file type.
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match>;
}

/// A scanner that uses parallel processing to scan codebases for patterns.
pub struct Scanner {
    detectors: Vec<Box<dyn PatternDetector>>,
}

impl Scanner {
    /// Creates a new scanner with the given pattern detectors.
    pub fn new(detectors: Vec<Box<dyn PatternDetector>>) -> Self {
        Self { detectors }
    }

    /// Scans the directory tree starting from the given root path.
    /// Returns all matches found by the detectors.
    /// Uses parallel processing for performance.
    pub fn scan(&self, root: &Path) -> Result<Vec<Match>> {
        let walker = WalkBuilder::new(root)
            .build()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_some_and(|ft| ft.is_file()))
            .collect::<Vec<_>>();

        let matches: Vec<Match> = walker
            .par_iter()
            .filter_map(|entry| {
                let path = entry.path();
                match std::fs::read_to_string(path) {
                    Ok(content) => {
                        let file_matches: Vec<Match> = self
                            .detectors
                            .par_iter()
                            .flat_map(|detector| detector.detect(&content, path))
                            .collect();
                        Some(file_matches)
                    }
                    Err(_) => None, // Skip files that can't be read
                }
            })
            .flatten()
            .collect();

        Ok(matches)
    }
}

/// Default detector for TODO comments.
pub struct TodoDetector;

impl PatternDetector for TodoDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        self.detect_pattern(content, file_path, "TODO", r"\bTODO\b")
    }
}

/// Default detector for FIXME comments.
pub struct FixmeDetector;

impl PatternDetector for FixmeDetector {
    fn detect(&self, content: &str, file_path: &Path) -> Vec<Match> {
        self.detect_pattern(content, file_path, "FIXME", r"\bFIXME\b")
    }
}

impl TodoDetector {
    fn detect_pattern(
        &self,
        content: &str,
        file_path: &Path,
        pattern_name: &str,
        regex_pattern: &str,
    ) -> Vec<Match> {
        let re = Regex::new(regex_pattern).unwrap();
        let mut matches = Vec::new();
        for (line_idx, line) in content.lines().enumerate() {
            for mat in re.find_iter(line) {
                matches.push(Match {
                    file_path: file_path.to_string_lossy().to_string(),
                    line_number: line_idx + 1,
                    column: mat.start() + 1,
                    pattern: pattern_name.to_string(),
                    message: mat.as_str().to_string(),
                });
            }
        }
        matches
    }
}

impl FixmeDetector {
    fn detect_pattern(
        &self,
        content: &str,
        file_path: &Path,
        pattern_name: &str,
        regex_pattern: &str,
    ) -> Vec<Match> {
        let re = Regex::new(regex_pattern).unwrap();
        let mut matches = Vec::new();
        for (line_idx, line) in content.lines().enumerate() {
            for mat in re.find_iter(line) {
                matches.push(Match {
                    file_path: file_path.to_string_lossy().to_string(),
                    line_number: line_idx + 1,
                    column: mat.start() + 1,
                    pattern: pattern_name.to_string(),
                    message: mat.as_str().to_string(),
                });
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_todo_detector() {
        let detector = TodoDetector;
        let content = "Some code\n// TODO: fix this\nMore code";
        let path = PathBuf::from("test.rs");
        let matches = detector.detect(content, &path);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].pattern, "TODO");
        assert_eq!(matches[0].line_number, 2);
        assert_eq!(matches[0].column, 4); // "// " is 3 chars, then TODO
        assert_eq!(matches[0].message, "TODO");
    }

    #[test]
    fn test_fixme_detector() {
        let detector = FixmeDetector;
        let content = "Code\nFIXME: issue here\nEnd";
        let path = PathBuf::from("test.js");
        let matches = detector.detect(content, &path);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].pattern, "FIXME");
        assert_eq!(matches[0].line_number, 2);
        assert_eq!(matches[0].column, 1);
        assert_eq!(matches[0].message, "FIXME");
    }

    #[test]
    fn test_no_matches() {
        let detector = TodoDetector;
        let content = "No todos here";
        let path = PathBuf::from("test.txt");
        let matches = detector.detect(content, &path);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_multiple_matches() {
        let detector = TodoDetector;
        let content = "TODO\n// TODO again";
        let path = PathBuf::from("test.rs");
        let matches = detector.detect(content, &path);
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_scanner_with_detectors() {
        let detectors: Vec<Box<dyn PatternDetector>> =
            vec![Box::new(TodoDetector), Box::new(FixmeDetector)];
        let scanner = Scanner::new(detectors);
        // For testing, we can create a temp dir, but for simplicity, assume a test file exists.
        // Since it's hard to create files in test, perhaps mock or use a known path.
        // For now, skip integration test or use a string-based approach.
        // Actually, since scan reads files, for unit test, perhaps test the logic separately.
        // But to have coverage, perhaps create a temp file in test.
        use tempfile::TempDir;
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.rs");
        std::fs::write(&file_path, "TODO: test\nFIXME: another").unwrap();
        let matches = scanner.scan(temp_dir.path()).unwrap();
        assert_eq!(matches.len(), 2);
        // Sort by pattern for deterministic test
        let mut sorted = matches;
        sorted.sort_by(|a, b| a.pattern.cmp(&b.pattern));
        assert_eq!(sorted[0].pattern, "FIXME");
        assert_eq!(sorted[1].pattern, "TODO");
    }
}
