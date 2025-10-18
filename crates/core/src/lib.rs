use anyhow::Result;
use dashmap::DashMap;
use ignore::WalkBuilder;
use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::SystemTime;

pub mod cache;
pub mod config;
pub mod custom_detectors;
pub mod detector_factory;
pub mod detectors;
pub mod distributed;
pub mod enhanced_config;
pub mod health_server;
pub mod incremental;
pub mod llm_detectors;
pub mod metrics;
pub mod monitoring;
pub mod observability;
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
    cache: DashMap<String, (SystemTime, Vec<Match>)>,
}

impl Scanner {
    /// Creates a new scanner with the given pattern detectors.
    pub fn new(detectors: Vec<Box<dyn PatternDetector>>) -> Self {
        Self {
            detectors,
            cache: DashMap::new(),
        }
    }

    /// Check if a file should be scanned based on size and type
    fn should_scan_file(&self, path: &Path) -> bool {
        // Skip files in common build/dependency directories
        if let Some(path_str) = path.to_str() {
            if path_str.contains("/target/")
                || path_str.contains("/node_modules/")
                || path_str.contains("/.git/")
                || path_str.contains("/build/")
                || path_str.contains("/dist/")
                || path_str.contains("/.next/")
                || path_str.contains("/.nuxt/")
            {
                return false;
            }
        }

        // Check file size (skip files larger than 5MB)
        if let Ok(metadata) = std::fs::metadata(path) {
            if metadata.len() > 5 * 1024 * 1024 {
                return false;
            }
        }

        // Check if file is binary by trying to read first 1024 bytes as UTF-8
        if let Ok(mut file) = File::open(path) {
            let mut buffer = [0; 1024];
            if let Ok(bytes_read) = file.read(&mut buffer) {
                if bytes_read > 0 && std::str::from_utf8(&buffer[..bytes_read]).is_err() {
                    return false;
                }
            }
        }

        // Check file extension for known binary types (fallback)
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            match ext.to_lowercase().as_str() {
                // Skip binary files
                "exe" | "dll" | "so" | "dylib" | "bin" | "obj" | "o" | "a" | "lib" => return false,
                // Skip image files
                "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "bmp" | "tiff" => return false,
                // Skip compressed files
                "zip" | "tar" | "gz" | "rar" | "7z" | "bz2" | "xz" => return false,
                // Skip media files
                "mp3" | "mp4" | "avi" | "mov" | "wav" | "flac" => return false,
                _ => {}
            }
        }

        true
    }

    /// Reads file content with memory mapping for large files
    fn read_file_content(&self, path: &Path) -> Result<String> {
        let metadata = std::fs::metadata(path)?;

        if metadata.len() > 1024 * 1024 {
            // Use memory mapping for large files
            let file = File::open(path)?;
            let mmap = unsafe { Mmap::map(&file)? };
            let content = std::str::from_utf8(&mmap)?;
            Ok(content.to_string())
        } else {
            // Regular reading for smaller files
            Ok(std::fs::read_to_string(path)?)
        }
    }

    /// Scans the directory tree starting from the given root path.
    /// Returns all matches found by the detectors.
    /// Uses parallel processing for performance with improved load balancing and caching.
    pub fn scan(&self, root: &Path) -> Result<Vec<Match>> {
        let matches: Vec<Match> = WalkBuilder::new(root)
            .build()
            .par_bridge()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_type = entry.file_type()?;
                if file_type.is_file() {
                    let path = entry.path();
                    if !self.should_scan_file(path) {
                        return None;
                    }
                    let path_str = path.to_string_lossy().to_string();
                    let metadata = std::fs::metadata(path).ok()?;
                    let mtime = metadata.modified().ok()?;
                    if let Some(cached) = self.cache.get(&path_str) {
                        let (cached_mtime, cached_matches) = &*cached;
                        if cached_mtime == &mtime {
                            Some(cached_matches.clone())
                        } else {
                            let content = self.read_file_content(path).ok()?;
                            let file_matches: Vec<Match> = if self.detectors.len() <= 3 {
                                // For few detectors, sequential is faster (less overhead)
                                self.detectors
                                    .iter()
                                    .flat_map(|detector| detector.detect(&content, path))
                                    .collect()
                            } else {
                                // For many detectors, use parallel processing
                                self.detectors
                                    .par_iter()
                                    .flat_map(|detector| detector.detect(&content, path))
                                    .collect()
                            };
                            self.cache.insert(path_str, (mtime, file_matches.clone()));
                            Some(file_matches)
                        }
                    } else {
                        let content = self.read_file_content(path).ok()?;
                        let file_matches: Vec<Match> = if self.detectors.len() <= 3 {
                            // For few detectors, sequential is faster (less overhead)
                            self.detectors
                                .iter()
                                .flat_map(|detector| detector.detect(&content, path))
                                .collect()
                        } else {
                            // For many detectors, use parallel processing
                            self.detectors
                                .par_iter()
                                .flat_map(|detector| detector.detect(&content, path))
                                .collect()
                        };
                        self.cache.insert(path_str, (mtime, file_matches.clone()));
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
pub use cache::*;
pub use custom_detectors::*;
pub use detector_factory::*;
pub use detectors::*;
pub use distributed::*;
pub use enhanced_config::*;
pub use incremental::*;
pub use llm_detectors::*;
pub use monitoring::*;
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
