use crate::{Match, PatternDetector};
use anyhow::Result;
use dashmap::DashMap;
use ignore::WalkBuilder;
use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;
use std::io::Read;
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
            max_cache_size: 1000, // Maximum number of cached file results
        }
    }

    /// Set maximum cache size
    pub fn with_cache_size(mut self, size: usize) -> Self {
        self.max_cache_size = size;
        self
    }

    /// Gets the relevant detectors for a specific file extension
    /// This optimizes performance by only running detectors that are likely to match
    fn get_relevant_detectors(&self, path: &Path) -> Vec<&dyn PatternDetector> {
        let ext = path.extension().and_then(|e| e.to_str());

        match ext {
            Some("rs") => {
                // For Rust files, prioritize Rust-specific detectors but include general ones
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("js") | Some("ts") | Some("jsx") | Some("tsx") | Some("vue") | Some("svelte") => {
                // For JS/TS files, include all detectors
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("py") | Some("pyw") | Some("pyx") => {
                // For Python files, include all detectors
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("java") => {
                // For Java files, include all detectors
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("c") | Some("cpp") | Some("cc") | Some("cxx") | Some("h") | Some("hpp") => {
                // For C/C++ files, include all detectors
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("go") => {
                // For Go files, include all detectors
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("php") => {
                // For PHP files, include all detectors
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("rb") => {
                // For Ruby files, include all detectors
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("sh") | Some("bash") | Some("zsh") => {
                // For shell scripts, include all detectors
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("json") | Some("yaml") | Some("yml") | Some("toml") | Some("xml")
            | Some("ini") | Some("cfg") => {
                // For config files, include general detectors (TODO, FIXME, etc.)
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            _ => {
                // For unknown extensions, include all detectors
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
        }
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
                let content = self.read_file_content(path).ok()?;
                lines_processed.fetch_add(content.lines().count(), Ordering::Relaxed);

                // Use optimized parallel processing for detectors
                let relevant_detectors = self.get_relevant_detectors(path);
                let file_matches: Vec<Match> = if relevant_detectors.len() > 3 {
                    // For many detectors, use parallel processing
                    relevant_detectors
                        .par_iter()
                        .flat_map(|detector| detector.detect(&content, path))
                        .collect()
                } else {
                    // For few detectors, sequential is faster (less overhead)
                    relevant_detectors
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

    /// Gets the relevant detectors for a specific file extension
    fn get_relevant_detectors(&self, path: &Path) -> Vec<&dyn PatternDetector> {
        let ext = path.extension().and_then(|e| e.to_str());

        match ext {
            Some("rs") => self.detectors.iter().map(|d| d.as_ref()).collect(),
            Some("js") | Some("ts") | Some("jsx") | Some("tsx") | Some("vue") | Some("svelte") => {
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("py") | Some("pyw") | Some("pyx") => {
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("java") => self.detectors.iter().map(|d| d.as_ref()).collect(),
            Some("c") | Some("cpp") | Some("cc") | Some("cxx") | Some("h") | Some("hpp") => {
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("go") => self.detectors.iter().map(|d| d.as_ref()).collect(),
            Some("php") => self.detectors.iter().map(|d| d.as_ref()).collect(),
            Some("rb") => self.detectors.iter().map(|d| d.as_ref()).collect(),
            Some("sh") | Some("bash") | Some("zsh") => {
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("json") | Some("yaml") | Some("yml") | Some("toml") | Some("xml")
            | Some("ini") | Some("cfg") => self.detectors.iter().map(|d| d.as_ref()).collect(),
            _ => self.detectors.iter().map(|d| d.as_ref()).collect(),
        }
    }

    /// Check if a file should be scanned based on size and type
    fn should_scan_file_streaming(&self, path: &Path) -> bool {
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
            if entry.file_type().is_some_and(|ft| ft.is_file())
                && self.should_scan_file_streaming(entry.path())
            {
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
                // Use memory-mapped files for large files (>1MB) for better performance
                let (content, line_count) = if let Ok(metadata) = std::fs::metadata(path) {
                    if metadata.len() > 1024 * 1024 {
                        // Use memory mapping for large files
                        if let Ok(file) = File::open(path) {
                            if let Ok(mmap) = unsafe { Mmap::map(&file) } {
                                let content = std::str::from_utf8(&mmap).ok()?;
                                let line_count = content.lines().count();
                                (content.to_string(), line_count)
                            } else {
                                // Fallback to regular reading
                                let content = std::fs::read_to_string(path).ok()?;
                                let line_count = content.lines().count();
                                (content, line_count)
                            }
                        } else {
                            return None;
                        }
                    } else {
                        // Use regular reading for smaller files
                        let content = std::fs::read_to_string(path).ok()?;
                        let line_count = content.lines().count();
                        (content, line_count)
                    }
                } else {
                    return None;
                };

                let relevant_detectors = self.get_relevant_detectors(path);
                let matches: Vec<Match> = if relevant_detectors.len() <= 3 {
                    // For few detectors, sequential is faster (less overhead)
                    relevant_detectors
                        .iter()
                        .flat_map(|detector| detector.detect(&content, path))
                        .collect()
                } else {
                    // For many detectors, use parallel processing
                    relevant_detectors
                        .par_iter()
                        .flat_map(|detector| detector.detect(&content, path))
                        .collect()
                };

                Some((matches, line_count))
            })
            .collect();

        let all_matches: Vec<Match> = results.iter().flat_map(|(m, _)| m.clone()).collect();
        let total_lines: usize = results.iter().map(|(_, l)| *l).sum();

        Ok((all_matches, total_lines))
    }
}

/// Advanced scanner combining multiple optimization techniques
pub struct AdvancedScanner {
    detectors: Vec<Box<dyn PatternDetector>>,
    high_perf_detector: crate::detectors::HighPerformanceDetector,
    cache: DashMap<String, (u64, Vec<Match>)>,
    max_cache_size: usize,
    use_memory_mapping: bool,
}

impl AdvancedScanner {
    /// Creates a new advanced scanner with optimized detectors
    pub fn new(detectors: Vec<Box<dyn PatternDetector>>) -> Self {
        let high_perf_detector = crate::detectors::HighPerformanceDetector::for_common_patterns();

        Self {
            detectors,
            high_perf_detector,
            cache: DashMap::new(),
            max_cache_size: 20000,
            use_memory_mapping: true,
        }
    }

    /// Gets the relevant detectors for a specific file extension
    fn get_relevant_detectors(&self, path: &Path) -> Vec<&dyn PatternDetector> {
        let ext = path.extension().and_then(|e| e.to_str());

        match ext {
            Some("rs") => self.detectors.iter().map(|d| d.as_ref()).collect(),
            Some("js") | Some("ts") | Some("jsx") | Some("tsx") | Some("vue") | Some("svelte") => {
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("py") | Some("pyw") | Some("pyx") => {
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("java") => self.detectors.iter().map(|d| d.as_ref()).collect(),
            Some("c") | Some("cpp") | Some("cc") | Some("cxx") | Some("h") | Some("hpp") => {
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("go") => self.detectors.iter().map(|d| d.as_ref()).collect(),
            Some("php") => self.detectors.iter().map(|d| d.as_ref()).collect(),
            Some("rb") => self.detectors.iter().map(|d| d.as_ref()).collect(),
            Some("sh") | Some("bash") | Some("zsh") => {
                self.detectors.iter().map(|d| d.as_ref()).collect()
            }
            Some("json") | Some("yaml") | Some("yml") | Some("toml") | Some("xml")
            | Some("ini") | Some("cfg") => self.detectors.iter().map(|d| d.as_ref()).collect(),
            _ => self.detectors.iter().map(|d| d.as_ref()).collect(),
        }
    }

    /// Advanced scan with multiple optimization layers
    pub fn scan_advanced(&self, root: &Path) -> Result<(Vec<Match>, ScanMetrics)> {
        let start_time = Instant::now();
        let files_processed = AtomicUsize::new(0);
        let lines_processed = AtomicUsize::new(0);
        let cache_hits = AtomicUsize::new(0);
        let cache_misses = AtomicUsize::new(0);

        let matches: Vec<Match> = WalkBuilder::new(root)
            .standard_filters(true)
            .build()
            .par_bridge()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_type = entry.file_type()?;

                if !file_type.is_file() {
                    return None;
                }

                let path = entry.path();

                // Skip inappropriate files early
                if !self.should_scan_file_advanced(path) {
                    return None;
                }

                files_processed.fetch_add(1, Ordering::Relaxed);
                let path_str = path.to_string_lossy().to_string();

                // Check cache
                if let Some(cached_result) = self.get_cached_result_advanced(path, &path_str) {
                    cache_hits.fetch_add(1, Ordering::Relaxed);
                    return Some(cached_result);
                }

                cache_misses.fetch_add(1, Ordering::Relaxed);

                // Read content with optimizations
                let content = self.read_file_content_advanced(path).ok()?;
                lines_processed.fetch_add(content.lines().count(), Ordering::Relaxed);

                // Use high-performance detector for common patterns
                let mut file_matches = self.high_perf_detector.detect(&content, path);

                // Use specialized detectors for remaining patterns
                let relevant_detectors = self.get_relevant_detectors(path);
                if relevant_detectors.len() <= 3 {
                    // For few detectors, sequential is faster (less overhead)
                    for detector in relevant_detectors {
                        file_matches.extend(detector.detect(&content, path));
                    }
                } else {
                    // For many detectors, use parallel processing
                    let additional_matches: Vec<Match> = relevant_detectors
                        .par_iter()
                        .flat_map(|detector| detector.detect(&content, path))
                        .collect();
                    file_matches.extend(additional_matches);
                }

                // Remove duplicates (patterns might overlap)
                file_matches.sort_by(|a, b| {
                    (a.line_number, a.column, a.pattern.clone()).cmp(&(
                        b.line_number,
                        b.column,
                        b.pattern.clone(),
                    ))
                });
                file_matches.dedup_by(|a, b| {
                    a.line_number == b.line_number && a.column == b.column && a.pattern == b.pattern
                });

                // Cache result
                self.cache_result_advanced(path, &path_str, &file_matches);

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

    /// Advanced file filtering with better heuristics
    fn should_scan_file_advanced(&self, path: &Path) -> bool {
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
                "exe" | "dll" | "so" | "dylib" | "bin" | "obj" | "o" | "a" | "lib" | "png"
                | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "bmp" | "tiff" | "zip" | "tar"
                | "gz" | "rar" | "7z" | "bz2" | "xz" | "mp3" | "mp4" | "avi" | "mov" | "wav"
                | "flac" | "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => {
                    return false
                }
                _ => {}
            }
        }

        true
    }

    /// Advanced file reading with memory mapping for large files
    fn read_file_content_advanced(&self, path: &Path) -> Result<String> {
        if !self.use_memory_mapping {
            return Ok(std::fs::read_to_string(path)?);
        }

        let metadata = std::fs::metadata(path)?;

        if metadata.len() > 1024 * 1024 {
            // 1MB threshold
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

    /// Advanced caching with better invalidation
    fn get_cached_result_advanced(&self, path: &Path, path_str: &str) -> Option<Vec<Match>> {
        if let Ok(metadata) = std::fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                if let Some(cached_entry) = self.cache.get(path_str) {
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

    /// Cache result with LRU-style eviction
    fn cache_result_advanced(&self, path: &Path, path_str: &str, matches: &[Match]) {
        // Manage cache size
        if self.cache.len() >= self.max_cache_size {
            let keys_to_remove: Vec<String> = self
                .cache
                .iter()
                .take(self.max_cache_size / 4)
                .map(|entry| entry.key().clone())
                .collect();

            for key in keys_to_remove {
                self.cache.remove(&key);
            }
        }

        if let Ok(metadata) = std::fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                let modified_timestamp = modified
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                self.cache
                    .insert(path_str.to_string(), (modified_timestamp, matches.to_vec()));
            }
        }
    }

    /// Configure memory mapping usage
    pub fn with_memory_mapping(mut self, enabled: bool) -> Self {
        self.use_memory_mapping = enabled;
        self
    }

    /// Set cache size
    pub fn with_cache_size(mut self, size: usize) -> Self {
        self.max_cache_size = size;
        self
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
