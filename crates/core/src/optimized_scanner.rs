use crate::{Match, PatternDetector};
use anyhow::Result;
use dashmap::DashMap;
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

/// Performance metrics for scanning operations
#[derive(Debug, Clone)]
pub struct ScanMetrics {
    pub total_files_scanned: usize,
    pub total_lines_processed: usize,
    pub total_matches_found: usize,
    pub scan_duration_ms: u64,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

/// Optimized scanner with performance enhancements
pub struct OptimizedScanner {
    detectors: Vec<Box<dyn PatternDetector>>,
    cache: DashMap<String, Vec<Match>>,
    file_cache: DashMap<String, (u64, Vec<Match>)>, // (modified_time, matches)
    max_cache_size: usize,
}

impl OptimizedScanner {
    /// Creates a new optimized scanner with the given pattern detectors
    pub fn new(detectors: Vec<Box<dyn PatternDetector>>) -> Self {
        Self {
            detectors,
            cache: DashMap::new(),
            file_cache: DashMap::new(),
            max_cache_size: 10000, // Maximum number of cached file results
        }
    }

    /// Set maximum cache size
    pub fn with_cache_size(mut self, size: usize) -> Self {
        self.max_cache_size = size;
        self
    }

    /// Optimized scan with performance improvements
    pub fn scan_optimized(&self, root: &Path) -> Result<(Vec<Match>, ScanMetrics)> {
        let start_time = Instant::now();
        let files_processed = AtomicUsize::new(0);
        let lines_processed = AtomicUsize::new(0);
        let cache_hits = AtomicUsize::new(0);
        let cache_misses = AtomicUsize::new(0);

        // Pre-compile regex patterns and optimize file filtering
        let matches: Vec<Match> = WalkBuilder::new(root)
            .standard_filters(true) // Use gitignore, etc.
            .build()
            .par_bridge()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_type = entry.file_type()?;

                if !file_type.is_file() {
                    return None;
                }

                let path = entry.path();

                // Skip binary files and large files early
                if !self.should_scan_file(path) {
                    return None;
                }

                files_processed.fetch_add(1, Ordering::Relaxed);

                let path_str = path.to_string_lossy().to_string();

                // Check file-based cache with modification time
                if let Some(cached_result) = self.get_cached_result(path, &path_str) {
                    cache_hits.fetch_add(1, Ordering::Relaxed);
                    return Some(cached_result);
                }

                cache_misses.fetch_add(1, Ordering::Relaxed);

                // Read and process file
                let content = std::fs::read_to_string(path).ok()?;
                lines_processed.fetch_add(content.lines().count(), Ordering::Relaxed);

                // Use optimized parallel processing for detectors
                let file_matches: Vec<Match> = if self.detectors.len() > 3 {
                    // For many detectors, use parallel processing
                    self.detectors
                        .par_iter()
                        .flat_map(|detector| detector.detect(&content, path))
                        .collect()
                } else {
                    // For few detectors, sequential is faster (less overhead)
                    self.detectors
                        .iter()
                        .flat_map(|detector| detector.detect(&content, path))
                        .collect()
                };

                // Cache the result with file modification time
                self.cache_result(path, &path_str, &file_matches);

                Some(file_matches)
            })
            .flatten()
            .collect();

        let duration = start_time.elapsed();

        let metrics = ScanMetrics {
            total_files_scanned: files_processed.load(Ordering::Relaxed),
            total_lines_processed: lines_processed.load(Ordering::Relaxed),
            total_matches_found: matches.len(),
            scan_duration_ms: duration.as_millis() as u64,
            cache_hits: cache_hits.load(Ordering::Relaxed),
            cache_misses: cache_misses.load(Ordering::Relaxed),
        };

        Ok((matches, metrics))
    }

    /// Check if a file should be scanned based on size and type
    fn should_scan_file(&self, path: &Path) -> bool {
        // Check file extension
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

        // Check file size (skip files larger than 5MB)
        if let Ok(metadata) = std::fs::metadata(path) {
            if metadata.len() > 5 * 1024 * 1024 {
                return false;
            }
        }

        true
    }

    /// Get cached result if file hasn't been modified
    fn get_cached_result(&self, path: &Path, path_str: &str) -> Option<Vec<Match>> {
        if let Ok(metadata) = std::fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                if let Some(cached_entry) = self.file_cache.get(path_str) {
                    let (cached_time, cached_matches) = cached_entry.value();
                    let modified_timestamp = modified
                        .duration_since(std::time::UNIX_EPOCH)
                        .ok()?
                        .as_secs();

                    if modified_timestamp == *cached_time {
                        return Some(cached_matches.clone());
                    }
                }
            }
        }
        None
    }

    /// Cache result with file modification time
    fn cache_result(&self, path: &Path, path_str: &str, matches: &[Match]) {
        // Manage cache size
        if self.file_cache.len() >= self.max_cache_size {
            // Remove some old entries (simple LRU-like behavior)
            let keys_to_remove: Vec<String> = self
                .file_cache
                .iter()
                .take(self.max_cache_size / 4)
                .map(|entry| entry.key().clone())
                .collect();

            for key in keys_to_remove {
                self.file_cache.remove(&key);
            }
        }

        if let Ok(metadata) = std::fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                let modified_timestamp = modified
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                self.file_cache
                    .insert(path_str.to_string(), (modified_timestamp, matches.to_vec()));
            }
        }
    }

    /// Clear all caches
    pub fn clear_cache(&self) {
        self.cache.clear();
        self.file_cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.cache.len(), self.file_cache.len())
    }
}

/// Memory-efficient streaming scanner for very large codebases
pub struct StreamingScanner {
    detectors: Vec<Box<dyn PatternDetector>>,
    batch_size: usize,
}

impl StreamingScanner {
    pub fn new(detectors: Vec<Box<dyn PatternDetector>>) -> Self {
        Self {
            detectors,
            batch_size: 100, // Process files in batches
        }
    }

    /// Scan with memory-efficient streaming
    pub fn scan_streaming<F>(&self, root: &Path, mut callback: F) -> Result<ScanMetrics>
    where
        F: FnMut(Vec<Match>) -> Result<()>,
    {
        let start_time = Instant::now();
        let mut total_files = 0;
        let mut total_lines = 0;
        let mut total_matches = 0;

        let walker = WalkBuilder::new(root).standard_filters(true).build();

        let mut file_batch = Vec::new();

        for entry in walker {
            let entry = entry?;
            if entry.file_type().is_some_and(|ft| ft.is_file()) {
                file_batch.push(entry.path().to_path_buf());

                if file_batch.len() >= self.batch_size {
                    let (batch_matches, batch_lines) = self.process_batch(&file_batch)?;
                    total_files += file_batch.len();
                    total_lines += batch_lines;
                    total_matches += batch_matches.len();

                    callback(batch_matches)?;
                    file_batch.clear();
                }
            }
        }

        // Process remaining files
        if !file_batch.is_empty() {
            let (batch_matches, batch_lines) = self.process_batch(&file_batch)?;
            total_files += file_batch.len();
            total_lines += batch_lines;
            total_matches += batch_matches.len();

            callback(batch_matches)?;
        }

        let duration = start_time.elapsed();

        Ok(ScanMetrics {
            total_files_scanned: total_files,
            total_lines_processed: total_lines,
            total_matches_found: total_matches,
            scan_duration_ms: duration.as_millis() as u64,
            cache_hits: 0,
            cache_misses: 0,
        })
    }

    fn process_batch(&self, files: &[std::path::PathBuf]) -> Result<(Vec<Match>, usize)> {
        let results: Vec<(Vec<Match>, usize)> = files
            .par_iter()
            .filter_map(|path| {
                let content = std::fs::read_to_string(path).ok()?;
                let line_count = content.lines().count();

                let matches: Vec<Match> = self
                    .detectors
                    .iter()
                    .flat_map(|detector| detector.detect(&content, path))
                    .collect();

                Some((matches, line_count))
            })
            .collect();

        let all_matches: Vec<Match> = results.iter().flat_map(|(m, _)| m.clone()).collect();
        let total_lines: usize = results.iter().map(|(_, l)| *l).sum();

        Ok((all_matches, total_lines))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detectors::*;
    use tempfile::TempDir;

    #[test]
    fn test_optimized_scanner() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.rs");
        std::fs::write(&file_path, "// TODO: test\n// FIXME: another").unwrap();

        let detectors: Vec<Box<dyn PatternDetector>> =
            vec![Box::new(TodoDetector), Box::new(FixmeDetector)];

        let scanner = OptimizedScanner::new(detectors);
        let (matches, metrics) = scanner.scan_optimized(temp_dir.path()).unwrap();

        assert_eq!(matches.len(), 2);
        assert_eq!(metrics.total_files_scanned, 1);
        assert!(metrics.scan_duration_ms > 0);
    }

    #[test]
    fn test_caching() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.rs");
        std::fs::write(&file_path, "// TODO: test").unwrap();

        let detectors: Vec<Box<dyn PatternDetector>> = vec![Box::new(TodoDetector)];
        let scanner = OptimizedScanner::new(detectors);

        // First scan
        let (matches1, _metrics1) = scanner.scan_optimized(temp_dir.path()).unwrap();

        // Second scan should use cache
        let (matches2, metrics2) = scanner.scan_optimized(temp_dir.path()).unwrap();

        assert_eq!(matches1.len(), matches2.len());
        assert!(metrics2.cache_hits > 0);
    }
}
