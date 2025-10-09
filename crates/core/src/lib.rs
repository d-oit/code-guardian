use anyhow::Result;
use dashmap::DashMap;
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::path::Path;

pub mod config;
pub mod custom_detectors;
pub mod detector_factory;
pub mod detectors;
pub mod distributed;
pub mod enhanced_config;
pub mod incremental;
pub mod optimized_scanner;
pub mod performance;

/// Represents a detected pattern match in a file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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

/// Severity levels for detected patterns.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
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
    cache: DashMap<String, Vec<Match>>,
}

impl Scanner {
    /// Creates a new scanner with the given pattern detectors.
    pub fn new(detectors: Vec<Box<dyn PatternDetector>>) -> Self {
        Self {
            detectors,
            cache: DashMap::new(),
        }
    }

    /// Scans the directory tree starting from the given root path.
    /// Returns all matches found by the detectors.
    /// Uses parallel processing for performance with improved load balancing and caching.
    ///
    /// # Examples
    ///
    /// ```
    /// use code_guardian_core::{Scanner, PatternDetector, Match};
    /// use std::path::Path;
    ///
    /// struct MockDetector;
    /// impl PatternDetector for MockDetector {
    ///     fn detect(&self, content: &str, _file_path: &Path) -> Vec<Match> {
    ///         if content.contains("TODO") {
    ///             vec![Match {
    ///                 file_path: "test.rs".to_string(),
    ///                 line_number: 1,
    ///                 column: 1,
    ///                 pattern: "TODO".to_string(),
    ///                 message: "TODO found".to_string(),
    ///             }]
    ///         } else {
    ///             vec![]
    ///         }
    ///     }
    /// }
    ///
    /// let scanner = Scanner::new(vec![Box::new(MockDetector)]);
    /// // Note: This would scan actual files; in doctest, we can't create temp files easily
    /// ```
    pub fn scan(&self, root: &Path) -> Result<Vec<Match>> {
        let matches: Vec<Match> = WalkBuilder::new(root)
            .build()
            .par_bridge()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_type = entry.file_type()?;
                if file_type.is_file() {
                    let path = entry.path();
                    let path_str = path.to_string_lossy().to_string();
                    if let Some(cached) = self.cache.get(&path_str) {
                        Some(cached.clone())
                    } else {
                        let content = std::fs::read_to_string(path).ok()?;
                        let file_matches: Vec<Match> = self
                            .detectors
                            .par_iter()
                            .flat_map(|detector| detector.detect(&content, path))
                            .collect();
                        self.cache.insert(path_str, file_matches.clone());
                        Some(file_matches)
                    }
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        Ok(matches)
    }
}

// Re-export detectors and factory for convenience
pub use custom_detectors::*;
pub use detector_factory::*;
pub use detectors::*;
pub use distributed::*;
pub use enhanced_config::*;
pub use incremental::*;
pub use optimized_scanner::*;
pub use performance::*;

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
        assert!(matches[0].message.contains("TODO"));
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
        assert!(matches[0].message.contains("FIXME"));
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

    #[test]
    fn test_production_readiness_multi_language_scan() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();

        // Create test files with non-production code in different languages

        // JavaScript with console.log and debugger
        std::fs::write(temp_dir.path().join("app.js"), 
            "function login(user) {\n    console.log('User:', user);\n    debugger;\n    return true;\n}")
            .unwrap();

        // TypeScript with alert
        std::fs::write(
            temp_dir.path().join("utils.ts"),
            "export function debug() {\n    alert('Debug mode');\n    // Phase 1 implementation\n}",
        )
        .unwrap();

        // Python with print and experimental code
        std::fs::write(temp_dir.path().join("main.py"), 
            "def process_data():\n    print('Processing...')  # dev output\n    # experimental algorithm\n    pass")
            .unwrap();

        // Rust with println! and unwrap
        std::fs::write(temp_dir.path().join("lib.rs"), 
            "fn main() {\n    println!(\"Debug info\");\n    // TODO: remove debug\n    let value = result.unwrap();\n}")
            .unwrap();

        // Create production-ready detectors
        let detectors = crate::DetectorFactory::create_production_ready_detectors();
        let scanner = Scanner::new(detectors);

        // Scan the test directory
        let matches = scanner.scan(temp_dir.path()).unwrap();

        // Verify we found issues across languages
        assert!(
            matches.len() >= 6,
            "Should find multiple non-production patterns, found: {}",
            matches.len()
        );

        // Verify specific patterns were detected across languages
        let patterns: Vec<&str> = matches.iter().map(|m| m.pattern.as_str()).collect();

        // Verify critical non-production patterns are detected
        assert!(
            patterns.contains(&"CONSOLE_LOG"),
            "Should detect console.log in JavaScript"
        );
        assert!(
            patterns.contains(&"DEBUGGER"),
            "Should detect debugger statements"
        );
        assert!(
            patterns.contains(&"ALERT"),
            "Should detect alert in TypeScript"
        );
        assert!(
            patterns.contains(&"PRINT"),
            "Should detect print statements"
        );
        assert!(
            patterns.contains(&"DEV"),
            "Should detect dev environment references"
        );
        assert!(
            patterns.contains(&"EXPERIMENTAL"),
            "Should detect experimental code"
        );
        assert!(patterns.contains(&"PHASE"), "Should detect phase markers");
        assert!(
            patterns.contains(&"UNWRAP"),
            "Should detect Rust unwrap calls"
        );

        println!(
            "✅ Production readiness scan found {} issues across multiple languages",
            matches.len()
        );
        for m in &matches {
            println!("  {} [{}] {}", m.file_path, m.pattern, m.message);
        }
    }
}
