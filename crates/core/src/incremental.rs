use crate::{Match, PatternDetector};
use anyhow::Result;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// File metadata for incremental scanning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub modified_time: u64,
    pub size: u64,
    pub hash: Option<String>,
    pub last_scan_time: u64,
    pub match_count: usize,
}

/// Incremental scan state persistence
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncrementalState {
    pub last_full_scan: u64,
    pub file_metadata: HashMap<PathBuf, FileMetadata>,
    pub scan_history: Vec<IncrementalScanResult>,
}

/// Result of an incremental scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalScanResult {
    pub timestamp: u64,
    pub files_scanned: usize,
    pub files_skipped: usize,
    pub files_modified: usize,
    pub files_added: usize,
    pub files_removed: usize,
    pub total_matches: usize,
    pub scan_duration_ms: u64,
}

/// Incremental scanner that only scans changed files
pub struct IncrementalScanner {
    detectors: Vec<Box<dyn PatternDetector>>,
    state: IncrementalState,
    state_file: PathBuf,
    force_rescan_threshold: u64, // Days after which to force full rescan
}

impl IncrementalScanner {
    /// Create a new incremental scanner
    pub fn new(detectors: Vec<Box<dyn PatternDetector>>, state_file: PathBuf) -> Result<Self> {
        let state = if state_file.exists() {
            let content = std::fs::read_to_string(&state_file)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            IncrementalState::default()
        };

        Ok(Self {
            detectors,
            state,
            state_file,
            force_rescan_threshold: 7, // 7 days
        })
    }

    /// Perform incremental scan
    pub fn scan_incremental(&mut self, root: &Path) -> Result<(Vec<Match>, IncrementalScanResult)> {
        let start_time = std::time::Instant::now();
        let scan_timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let mut all_matches = Vec::new();
        let mut files_scanned = 0;
        let mut files_skipped = 0;
        let mut files_modified = 0;
        let mut files_added = 0;
        let mut files_removed = 0;

        // Check if we need a full rescan
        let days_since_full_scan = (scan_timestamp - self.state.last_full_scan) / (24 * 60 * 60);
        let force_full_scan = days_since_full_scan > self.force_rescan_threshold;

        if force_full_scan {
            println!(
                "🔄 Performing full rescan (last full scan: {} days ago)",
                days_since_full_scan
            );
            self.state.last_full_scan = scan_timestamp;
            self.state.file_metadata.clear();
        }

        // Collect current files
        let current_files = self.collect_files(root)?;
        let mut current_file_set = std::collections::HashSet::new();

        for file_path in current_files {
            current_file_set.insert(file_path.clone());

            if let Some(metadata) = self.get_file_metadata(&file_path)? {
                let existing_metadata = self.state.file_metadata.get(&file_path);

                let needs_scan = match existing_metadata {
                    Some(existing) => {
                        // Check if file has been modified
                        existing.modified_time != metadata.modified_time
                            || existing.size != metadata.size
                            || force_full_scan
                    }
                    None => {
                        // New file
                        files_added += 1;
                        true
                    }
                };

                if needs_scan {
                    if existing_metadata.is_some() {
                        files_modified += 1;
                    }

                    // Scan the file - skip if not valid UTF-8 (like binary files)
                    let content = match std::fs::read_to_string(&file_path) {
                        Ok(content) => content,
                        Err(_) => continue, // Skip files that can't be read as UTF-8
                    };
                    let file_matches: Vec<Match> = self
                        .detectors
                        .iter()
                        .flat_map(|detector| detector.detect(&content, &file_path))
                        .collect();

                    let updated_metadata = FileMetadata {
                        path: file_path.clone(),
                        modified_time: metadata.modified_time,
                        size: metadata.size,
                        hash: metadata.hash,
                        last_scan_time: scan_timestamp,
                        match_count: file_matches.len(),
                    };

                    self.state.file_metadata.insert(file_path, updated_metadata);
                    all_matches.extend(file_matches);
                    files_scanned += 1;
                } else {
                    // File unchanged, use cached results
                    files_skipped += 1;

                    // For complete results, we'd need to store and retrieve cached matches
                    // For now, we'll just note that the file was skipped
                }
            }
        }

        // Find removed files
        let existing_files: Vec<PathBuf> = self.state.file_metadata.keys().cloned().collect();
        for existing_file in existing_files {
            if !current_file_set.contains(&existing_file) {
                self.state.file_metadata.remove(&existing_file);
                files_removed += 1;
            }
        }

        let scan_duration = start_time.elapsed();
        let result = IncrementalScanResult {
            timestamp: scan_timestamp,
            files_scanned,
            files_skipped,
            files_modified,
            files_added,
            files_removed,
            total_matches: all_matches.len(),
            scan_duration_ms: scan_duration.as_millis() as u64,
        };

        // Save state
        self.save_state()?;

        // Update scan history
        self.state.scan_history.push(result.clone());
        if self.state.scan_history.len() > 100 {
            self.state.scan_history.remove(0); // Keep last 100 scans
        }

        println!("📊 Incremental scan completed:");
        println!(
            "   Files scanned: {} | Skipped: {} | Modified: {} | Added: {} | Removed: {}",
            files_scanned, files_skipped, files_modified, files_added, files_removed
        );
        println!(
            "   Speed improvement: {:.1}x faster than full scan",
            self.calculate_speedup(files_scanned, files_skipped)
        );

        Ok((all_matches, result))
    }

    /// Force a full rescan on next scan
    pub fn force_full_rescan(&mut self) {
        self.state.last_full_scan = 0;
        self.state.file_metadata.clear();
    }

    /// Get incremental scan statistics
    pub fn get_statistics(&self) -> IncrementalStats {
        let recent_scans = self
            .state
            .scan_history
            .iter()
            .rev()
            .take(10)
            .collect::<Vec<_>>();

        let avg_speedup = if !recent_scans.is_empty() {
            recent_scans
                .iter()
                .map(|scan| self.calculate_speedup(scan.files_scanned, scan.files_skipped))
                .sum::<f64>()
                / recent_scans.len() as f64
        } else {
            1.0
        };

        IncrementalStats {
            total_files_tracked: self.state.file_metadata.len(),
            last_scan_time: recent_scans.first().map(|s| s.timestamp),
            average_speedup: avg_speedup,
            cache_hit_rate: if !recent_scans.is_empty() {
                let total_files = recent_scans
                    .iter()
                    .map(|s| s.files_scanned + s.files_skipped)
                    .sum::<usize>();
                let total_skipped = recent_scans.iter().map(|s| s.files_skipped).sum::<usize>();
                if total_files > 0 {
                    total_skipped as f64 / total_files as f64
                } else {
                    0.0
                }
            } else {
                0.0
            },
            scan_history_count: self.state.scan_history.len(),
        }
    }

    fn collect_files(&self, root: &Path) -> Result<Vec<PathBuf>> {
        use ignore::WalkBuilder;

        let mut files = Vec::new();
        for entry in WalkBuilder::new(root).build() {
            let entry = entry?;
            if entry.file_type().is_some_and(|ft| ft.is_file()) {
                files.push(entry.path().to_path_buf());
            }
        }
        Ok(files)
    }

    fn get_file_metadata(&self, path: &Path) -> Result<Option<FileMetadata>> {
        if let Ok(metadata) = std::fs::metadata(path) {
            let modified_time = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs();

            // Optional: Calculate file hash for more accurate change detection
            let hash = if metadata.len() < 1024 * 1024 {
                // Only hash files < 1MB
                self.calculate_file_hash(path).ok()
            } else {
                None
            };

            Ok(Some(FileMetadata {
                path: path.to_path_buf(),
                modified_time,
                size: metadata.len(),
                hash,
                last_scan_time: 0,
                match_count: 0,
            }))
        } else {
            Ok(None)
        }
    }

    fn calculate_file_hash(&self, path: &Path) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let content = std::fs::read(path)?;
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }

    fn calculate_speedup(&self, files_scanned: usize, files_skipped: usize) -> f64 {
        let total_files = files_scanned + files_skipped;
        if total_files > 0 && files_scanned > 0 {
            total_files as f64 / files_scanned as f64
        } else {
            1.0
        }
    }

    fn save_state(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.state)?;
        std::fs::write(&self.state_file, content)?;
        Ok(())
    }
}

/// Statistics for incremental scanning
#[derive(Debug, Clone)]
pub struct IncrementalStats {
    pub total_files_tracked: usize,
    pub last_scan_time: Option<u64>,
    pub average_speedup: f64,
    pub cache_hit_rate: f64,
    pub scan_history_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detectors::TodoDetector;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_incremental_scanner_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let detectors: Vec<Box<dyn PatternDetector>> = vec![Box::new(TodoDetector)];

        let scanner = IncrementalScanner::new(detectors, temp_file.path().to_path_buf());
        assert!(scanner.is_ok());
    }

    #[test]
    fn test_file_metadata_tracking() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");
        std::fs::write(&test_file, "// TODO: test").unwrap();

        let temp_state = NamedTempFile::new().unwrap();
        let detectors: Vec<Box<dyn PatternDetector>> = vec![Box::new(TodoDetector)];
        let mut scanner =
            IncrementalScanner::new(detectors, temp_state.path().to_path_buf()).unwrap();

        // First scan
        let (matches1, result1) = scanner.scan_incremental(temp_dir.path()).unwrap();
        assert_eq!(result1.files_added, 1);
        assert_eq!(result1.files_scanned, 1);
        assert_eq!(matches1.len(), 1);

        // Second scan without changes - should skip file
        let (_matches2, result2) = scanner.scan_incremental(temp_dir.path()).unwrap();
        assert_eq!(result2.files_skipped, 1);
        assert_eq!(result2.files_scanned, 0);
    }
}
