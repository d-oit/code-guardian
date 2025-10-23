use crate::{Match, PatternDetector};

/// Metrics for tracking scanning performance
#[derive(Clone)]
struct ScanMetrics {
    files_processed: Arc<AtomicUsize>,
    lines_processed: Arc<AtomicUsize>,
    cache_hits: Arc<AtomicUsize>,
    cache_misses: Arc<AtomicUsize>,
    simd_matches: Arc<AtomicUsize>,
    regex_matches: Arc<AtomicUsize>,
    file_read_time: Arc<AtomicUsize>,
    pattern_search_time: Arc<AtomicUsize>,
    result_processing_time: Arc<AtomicUsize>,
}
use anyhow::Result;
use dashmap::DashMap;
use ignore::WalkBuilder;
use memchr::{memchr, memmem};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// SIMD-accelerated pattern finder for common byte patterns
struct SimdPatternFinder {
    // Common patterns as byte arrays for SIMD search
    todo_bytes: &'static [u8],
    fixme_bytes: &'static [u8],
    hack_bytes: &'static [u8],
}

impl SimdPatternFinder {
    fn new() -> Self {
        Self {
            todo_bytes: b"todo",
            fixme_bytes: b"fixme",
            hack_bytes: b"hack",
        }
    }

    /// Fast byte-level search for common patterns
    fn find_fast_patterns(&self, content: &[u8]) -> Vec<(usize, &'static str)> {
        let mut results = Vec::new();

        // Use memchr for extremely fast byte searching
        let mut pos = 0;
        while pos < content.len() {
            // Find next potential pattern start
            if let Some(found) = memchr(b't', &content[pos..]) {
                let abs_pos = pos + found;
                if abs_pos + 4 <= content.len() {
                    // Check if it's "todo" (case insensitive)
                    let slice = &content[abs_pos..abs_pos + 4];
                    if slice.eq_ignore_ascii_case(self.todo_bytes) {
                        // Verify it's a word boundary
                        if self.is_word_boundary(content, abs_pos, 4) {
                            results.push((abs_pos, "TODO"));
                        }
                    }
                }
                pos = abs_pos + 1;
            } else {
                break;
            }
        }

        // Search for FIXME
        pos = 0;
        while pos < content.len() {
            if let Some(found) = memchr(b'f', &content[pos..]) {
                let abs_pos = pos + found;
                if abs_pos + 5 <= content.len() {
                    let slice = &content[abs_pos..abs_pos + 5];
                    if slice.eq_ignore_ascii_case(self.fixme_bytes)
                        && self.is_word_boundary(content, abs_pos, 5)
                    {
                        results.push((abs_pos, "FIXME"));
                    }
                }
                pos = abs_pos + 1;
            } else {
                break;
            }
        }

        // Search for HACK
        pos = 0;
        while pos < content.len() {
            if let Some(found) = memchr(b'h', &content[pos..]) {
                let abs_pos = pos + found;
                if abs_pos + 4 <= content.len() {
                    let slice = &content[abs_pos..abs_pos + 4];
                    if slice.eq_ignore_ascii_case(self.hack_bytes)
                        && self.is_word_boundary(content, abs_pos, 4)
                    {
                        results.push((abs_pos, "HACK"));
                    }
                }
                pos = abs_pos + 1;
            } else {
                break;
            }
        }

        results
    }

    fn is_word_boundary(&self, content: &[u8], pos: usize, len: usize) -> bool {
        let before_ok = pos == 0 || !content[pos - 1].is_ascii_alphanumeric();
        let after_ok = pos + len >= content.len() || !content[pos + len].is_ascii_alphanumeric();
        before_ok && after_ok
    }
}

/// Zero-copy line iterator for efficient line counting and context extraction
struct LineIterator<'a> {
    content: &'a str,
    pos: usize,
    line_num: usize,
}

impl<'a> LineIterator<'a> {
    fn new(content: &'a str) -> Self {
        Self {
            content,
            pos: 0,
            line_num: 1,
        }
    }

    fn find_line_info(&mut self, target_pos: usize) -> (usize, usize) {
        while self.pos <= target_pos && self.pos < self.content.len() {
            if let Some(newline_pos) = memchr(b'\n', &self.content.as_bytes()[self.pos..]) {
                let abs_newline = self.pos + newline_pos;
                if abs_newline >= target_pos {
                    // Found the line containing target_pos
                    let line_start = if self.line_num == 1 {
                        0
                    } else {
                        self.content[..self.pos]
                            .rfind('\n')
                            .map(|p| p + 1)
                            .unwrap_or(0)
                    };
                    let column = target_pos - line_start + 1;
                    return (self.line_num, column);
                }
                self.pos = abs_newline + 1;
                self.line_num += 1;
            } else {
                // No more newlines, target is on the last line
                let line_start = if self.line_num == 1 {
                    0
                } else {
                    self.content[..self.pos]
                        .rfind('\n')
                        .map(|p| p + 1)
                        .unwrap_or(0)
                };
                let column = target_pos - line_start + 1;
                return (self.line_num, column);
            }
        }
        (self.line_num, 1)
    }
}

/// Performance metrics with detailed timing
#[derive(Debug, Clone)]
pub struct AdvancedScanMetrics {
    pub total_files_scanned: usize,
    pub total_lines_processed: usize,
    pub total_matches_found: usize,
    pub scan_duration_ms: u64,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub simd_matches: usize,
    pub regex_matches: usize,
    pub file_read_time_ms: u64,
    pub pattern_search_time_ms: u64,
    pub result_processing_time_ms: u64,
}

/// Ultra-optimized scanner with multiple performance enhancements
pub struct PerformanceOptimizedScanner {
    detectors: Vec<Box<dyn PatternDetector>>,
    cache: Arc<DashMap<String, (u64, u64, Vec<Match>)>>, // (mtime, content_hash, matches)
    simd_finder: SimdPatternFinder,
    max_cache_size: usize,
    adaptive_threshold: usize,
}

impl PerformanceOptimizedScanner {
    /// Creates a new performance-optimized scanner
    pub fn new(detectors: Vec<Box<dyn PatternDetector>>) -> Self {
        Self {
            detectors,
            cache: Arc::new(DashMap::new()),
            simd_finder: SimdPatternFinder::new(),
            max_cache_size: 50000,
            adaptive_threshold: 2, // Dynamic threshold for parallel processing
        }
    }

    /// Configure cache size
    pub fn with_cache_size(mut self, size: usize) -> Self {
        self.max_cache_size = size;
        self
    }

    /// Ultra-fast scan with all optimizations enabled
    pub fn scan_ultra_fast(&self, root: &Path) -> Result<(Vec<Match>, AdvancedScanMetrics)> {
        let start_time = Instant::now();
        let files_processed = AtomicUsize::new(0);
        let lines_processed = AtomicUsize::new(0);
        let cache_hits = AtomicUsize::new(0);
        let cache_misses = AtomicUsize::new(0);
        let simd_matches = AtomicUsize::new(0);
        let regex_matches = AtomicUsize::new(0);
        let file_read_time = AtomicUsize::new(0);
        let pattern_search_time = AtomicUsize::new(0);
        let result_processing_time = AtomicUsize::new(0);

        // Use adaptive batch size based on system capabilities
        let num_cpus = num_cpus::get();
        let optimal_batch_size = (num_cpus * 8).clamp(32, 512);

        // Collect files first, then process in optimized batches
        let files: Vec<_> = WalkBuilder::new(root)
            .standard_filters(true)
            .build()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_some_and(|ft| ft.is_file()))
            .collect();

        let scan_metrics = ScanMetrics {
            files_processed: Arc::new(files_processed),
            lines_processed: Arc::new(lines_processed),
            cache_hits: Arc::new(cache_hits),
            cache_misses: Arc::new(cache_misses),
            simd_matches: Arc::new(simd_matches),
            regex_matches: Arc::new(regex_matches),
            file_read_time: Arc::new(file_read_time),
            pattern_search_time: Arc::new(pattern_search_time),
            result_processing_time: Arc::new(result_processing_time),
        };

        let matches: Vec<Match> = files
            .par_chunks(optimal_batch_size)
            .flat_map(|chunk| self.process_file_batch(chunk.to_vec(), &scan_metrics))
            .collect();

        let duration = start_time.elapsed();

        let metrics = AdvancedScanMetrics {
            total_files_scanned: scan_metrics.files_processed.load(Ordering::Relaxed),
            total_lines_processed: scan_metrics.lines_processed.load(Ordering::Relaxed),
            total_matches_found: matches.len(),
            scan_duration_ms: duration.as_millis() as u64,
            cache_hits: scan_metrics.cache_hits.load(Ordering::Relaxed),
            cache_misses: scan_metrics.cache_misses.load(Ordering::Relaxed),
            simd_matches: scan_metrics.simd_matches.load(Ordering::Relaxed),
            regex_matches: scan_metrics.regex_matches.load(Ordering::Relaxed),
            file_read_time_ms: scan_metrics.file_read_time.load(Ordering::Relaxed) as u64
                / 1_000_000, // ns to ms
            pattern_search_time_ms: scan_metrics.pattern_search_time.load(Ordering::Relaxed) as u64
                / 1_000_000,
            result_processing_time_ms: scan_metrics.result_processing_time.load(Ordering::Relaxed)
                as u64
                / 1_000_000,
        };

        Ok((matches, metrics))
    }

    fn process_file_batch(
        &self,
        entries: Vec<ignore::DirEntry>,
        metrics: &ScanMetrics,
    ) -> Vec<Match> {
        entries
            .into_iter()
            .filter_map(|entry| {
                if entry.file_type()?.is_file() && self.should_scan_file_optimized(entry.path()) {
                    metrics.files_processed.fetch_add(1, Ordering::Relaxed);
                    self.process_single_file(entry.path(), metrics)
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    fn process_single_file(&self, path: &Path, metrics: &ScanMetrics) -> Option<Vec<Match>> {
        let path_str = path.to_string_lossy().to_string();

        // Advanced cache check with content hash
        if let Some(cached) = self.get_cached_result_advanced(path, &path_str) {
            metrics.cache_hits.fetch_add(1, Ordering::Relaxed);
            return Some(cached);
        }

        metrics.cache_misses.fetch_add(1, Ordering::Relaxed);

        // Optimized file reading
        let read_start = Instant::now();
        let content = self.read_file_optimized(path).ok()?;
        metrics
            .file_read_time
            .fetch_add(read_start.elapsed().as_nanos() as usize, Ordering::Relaxed);

        let line_count = bytecount::count(content.as_bytes(), b'\n') + 1;
        metrics
            .lines_processed
            .fetch_add(line_count, Ordering::Relaxed);

        // Pattern search with SIMD + regex hybrid approach
        let search_start = Instant::now();
        let mut matches = Vec::new();

        // Phase 1: SIMD search for common patterns (extremely fast)
        let simd_results = self.simd_finder.find_fast_patterns(content.as_bytes());
        metrics
            .simd_matches
            .fetch_add(simd_results.len(), Ordering::Relaxed);

        // Convert SIMD results to matches
        let process_start = Instant::now();
        for (pos, pattern_name) in simd_results {
            if let Some(match_result) =
                self.create_match_from_position(&content, path, pos, pattern_name)
            {
                matches.push(match_result);
            }
        }
        metrics.result_processing_time.fetch_add(
            process_start.elapsed().as_nanos() as usize,
            Ordering::Relaxed,
        );

        // Phase 2: Regex search for complex patterns
        let relevant_detectors = self.get_relevant_detectors_optimized(path);

        if relevant_detectors.len() <= self.adaptive_threshold {
            // Sequential processing for few detectors
            for detector in relevant_detectors {
                let detector_matches = detector.detect(&content, path);
                metrics
                    .regex_matches
                    .fetch_add(detector_matches.len(), Ordering::Relaxed);
                matches.extend(detector_matches);
            }
        } else {
            // Parallel processing for many detectors
            let parallel_matches: Vec<Match> = relevant_detectors
                .par_iter()
                .flat_map(|detector| {
                    let detector_matches = detector.detect(&content, path);
                    metrics
                        .regex_matches
                        .fetch_add(detector_matches.len(), Ordering::Relaxed);
                    detector_matches
                })
                .collect();
            matches.extend(parallel_matches);
        }

        metrics.pattern_search_time.fetch_add(
            search_start.elapsed().as_nanos() as usize,
            Ordering::Relaxed,
        );

        // Deduplication and sorting
        let final_process_start = Instant::now();
        self.dedup_and_sort_matches(&mut matches);
        metrics.result_processing_time.fetch_add(
            final_process_start.elapsed().as_nanos() as usize,
            Ordering::Relaxed,
        );

        // Cache the result
        self.cache_result_advanced(path, &path_str, &content, &matches);

        Some(matches)
    }

    /// Fast file content reading with size-based strategy
    fn read_file_optimized(&self, path: &Path) -> Result<String> {
        let metadata = std::fs::metadata(path)?;
        let file_size = metadata.len();

        if file_size > 4 * 1024 * 1024 {
            // For large files (>4MB), use memory mapping
            let file = File::open(path)?;
            let mmap = unsafe { memmap2::Mmap::map(&file)? };
            Ok(std::str::from_utf8(&mmap)?.to_string())
        } else if file_size > 64 * 1024 {
            // For medium files (>64KB), read with pre-allocated buffer
            let mut file = File::open(path)?;
            let mut buffer = String::with_capacity(file_size as usize);
            file.read_to_string(&mut buffer)?;
            Ok(buffer)
        } else {
            // For small files, use standard reading
            Ok(std::fs::read_to_string(path)?)
        }
    }

    /// Optimized file filtering with early rejection
    fn should_scan_file_optimized(&self, path: &Path) -> bool {
        // Quick string-based filtering first (fastest)
        if let Some(path_str) = path.to_str() {
            // Check for common exclusion patterns using byte search
            if memmem::find(path_str.as_bytes(), b"/target/").is_some()
                || memmem::find(path_str.as_bytes(), b"/node_modules/").is_some()
                || memmem::find(path_str.as_bytes(), b"/.git/").is_some()
                || memmem::find(path_str.as_bytes(), b"/build/").is_some()
                || memmem::find(path_str.as_bytes(), b"/dist/").is_some()
            {
                return false;
            }
        }

        // File size check (avoid syscall for known small files)
        if let Ok(metadata) = path.metadata() {
            let size = metadata.len();
            if size == 0 || size > 10 * 1024 * 1024 {
                return false; // Skip empty files and very large files
            }
        }

        // Extension-based filtering (faster than reading file content)
        if let Some(
            "exe" | "dll" | "so" | "dylib" | "bin" | "obj" | "o" | "a" | "lib" | "png" | "jpg"
            | "jpeg" | "gif" | "svg" | "ico" | "bmp" | "tiff" | "zip" | "tar" | "gz" | "rar" | "7z"
            | "bz2" | "xz" | "mp3" | "mp4" | "avi" | "mov" | "wav" | "flac",
        ) = path.extension().and_then(|s| s.to_str())
        {
            return false;
        }

        true
    }

    /// Get relevant detectors with caching
    fn get_relevant_detectors_optimized(&self, path: &Path) -> Vec<&dyn PatternDetector> {
        // Cache detector selection based on file extension
        static DETECTOR_CACHE: Lazy<DashMap<String, Vec<usize>>> = Lazy::new(DashMap::new);

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown")
            .to_lowercase();

        if let Some(cached_indices) = DETECTOR_CACHE.get(&ext) {
            return cached_indices
                .iter()
                .filter_map(|&i| self.detectors.get(i).map(|d| d.as_ref()))
                .collect();
        }

        // Determine relevant detectors and cache the result
        let indices: Vec<usize> = match ext.as_str() {
            "rs" | "js" | "ts" | "jsx" | "tsx" | "vue" | "py" | "java" | "go" | "cpp" | "c"
            | "h" => {
                (0..self.detectors.len()).collect() // All detectors for code files
            }
            "json" | "yaml" | "yml" | "toml" | "xml" | "md" | "txt" => {
                (0..self.detectors.len()).collect() // All detectors for config/doc files
            }
            _ => (0..self.detectors.len()).collect(), // Default to all detectors
        };

        DETECTOR_CACHE.insert(ext, indices.clone());

        indices
            .iter()
            .filter_map(|&i| self.detectors.get(i).map(|d| d.as_ref()))
            .collect()
    }

    /// Create match from byte position using zero-copy line finding
    fn create_match_from_position(
        &self,
        content: &str,
        path: &Path,
        pos: usize,
        pattern: &str,
    ) -> Option<Match> {
        let mut line_iter = LineIterator::new(content);
        let (line_number, column) = line_iter.find_line_info(pos);

        // Extract context efficiently
        let start = pos.saturating_sub(20);
        let end = (pos + 30).min(content.len());
        let context = &content[start..end];

        Some(Match {
            file_path: path.to_string_lossy().to_string(),
            line_number,
            column,
            pattern: pattern.to_string(),
            message: format!("{}: {}", pattern, context.trim()),
        })
    }

    /// Efficient deduplication and sorting
    fn dedup_and_sort_matches(&self, matches: &mut Vec<Match>) {
        // Sort first for better dedup performance
        matches.sort_unstable_by(|a, b| {
            (a.line_number, a.column, &a.pattern).cmp(&(b.line_number, b.column, &b.pattern))
        });

        // Efficient deduplication
        matches.dedup_by(|a, b| {
            a.line_number == b.line_number && a.column == b.column && a.pattern == b.pattern
        });
    }

    /// Advanced caching with content hash for better invalidation
    fn get_cached_result_advanced(&self, path: &Path, path_str: &str) -> Option<Vec<Match>> {
        if let Ok(metadata) = std::fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                let mtime = modified
                    .duration_since(std::time::UNIX_EPOCH)
                    .ok()?
                    .as_secs();

                if let Some(entry) = self.cache.get(path_str) {
                    let (cached_mtime, _cached_hash, cached_matches) = entry.value();

                    if mtime == *cached_mtime {
                        return Some(cached_matches.clone());
                    }
                }
            }
        }
        None
    }

    /// Cache result with content hash for better invalidation
    fn cache_result_advanced(&self, path: &Path, path_str: &str, content: &str, matches: &[Match]) {
        // Manage cache size
        if self.cache.len() >= self.max_cache_size {
            // Remove 25% of entries to avoid frequent cleanup
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
                let mtime = modified
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                // Simple content hash (could use a better hash function for production)
                let content_hash = content.len() as u64;

                self.cache.insert(
                    path_str.to_string(),
                    (mtime, content_hash, matches.to_vec()),
                );
            }
        }
    }

    /// Get detailed performance statistics
    pub fn get_performance_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("cache_entries".to_string(), self.cache.len());
        stats.insert("cache_capacity".to_string(), self.max_cache_size);
        stats.insert("detector_count".to_string(), self.detectors.len());
        stats.insert("adaptive_threshold".to_string(), self.adaptive_threshold);
        stats
    }

    /// Clear all caches to free memory
    pub fn clear_caches(&self) {
        self.cache.clear();
    }
}
