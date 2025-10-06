use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance profiler for tracking operation timings
#[derive(Debug, Clone)]
pub struct PerformanceProfiler {
    timings: HashMap<String, Vec<Duration>>,
    start_times: HashMap<String, Instant>,
}

impl PerformanceProfiler {
    pub fn new() -> Self {
        Self {
            timings: HashMap::new(),
            start_times: HashMap::new(),
        }
    }
    
    /// Start timing an operation
    pub fn start(&mut self, operation: &str) {
        self.start_times.insert(operation.to_string(), Instant::now());
    }
    
    /// End timing an operation
    pub fn end(&mut self, operation: &str) {
        if let Some(start_time) = self.start_times.remove(operation) {
            let duration = start_time.elapsed();
            self.timings
                .entry(operation.to_string())
                .or_default()
                .push(duration);
        }
    }
    
    /// Get average duration for an operation
    pub fn average_duration(&self, operation: &str) -> Option<Duration> {
        let durations = self.timings.get(operation)?;
        if durations.is_empty() {
            return None;
        }
        
        let total: Duration = durations.iter().sum();
        Some(total / durations.len() as u32)
    }
    
    /// Get total duration for an operation
    pub fn total_duration(&self, operation: &str) -> Option<Duration> {
        let durations = self.timings.get(operation)?;
        Some(durations.iter().sum())
    }
    
    /// Get operation count
    pub fn operation_count(&self, operation: &str) -> usize {
        self.timings.get(operation).map_or(0, |d| d.len())
    }
    
    /// Generate performance report
    pub fn report(&self) -> String {
        let mut report = String::from("Performance Report:\n");
        report.push_str("==================\n\n");
        
        for (operation, durations) in &self.timings {
            if durations.is_empty() {
                continue;
            }
            
            let total: Duration = durations.iter().sum();
            let average = total / durations.len() as u32;
            let min = *durations.iter().min().unwrap();
            let max = *durations.iter().max().unwrap();
            
            report.push_str(&format!(
                "{}: {} calls\n  Total: {:?}\n  Average: {:?}\n  Min: {:?}\n  Max: {:?}\n\n",
                operation,
                durations.len(),
                total,
                average,
                min,
                max
            ));
        }
        
        report
    }
    
    /// Clear all timings
    pub fn clear(&mut self) {
        self.timings.clear();
        self.start_times.clear();
    }
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory usage tracker
#[derive(Debug, Clone)]
pub struct MemoryTracker {
    peak_memory: usize,
    current_memory: usize,
}

impl MemoryTracker {
    pub fn new() -> Self {
        Self {
            peak_memory: 0,
            current_memory: 0,
        }
    }
    
    /// Track memory allocation
    pub fn allocate(&mut self, size: usize) {
        self.current_memory += size;
        if self.current_memory > self.peak_memory {
            self.peak_memory = self.current_memory;
        }
    }
    
    /// Track memory deallocation
    pub fn deallocate(&mut self, size: usize) {
        self.current_memory = self.current_memory.saturating_sub(size);
    }
    
    /// Get current memory usage
    pub fn current_usage(&self) -> usize {
        self.current_memory
    }
    
    /// Get peak memory usage
    pub fn peak_usage(&self) -> usize {
        self.peak_memory
    }
    
    /// Reset tracking
    pub fn reset(&mut self) {
        self.current_memory = 0;
        self.peak_memory = 0;
    }
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Input stats for performance calculation
#[derive(Debug, Clone)]
pub struct ScanStats {
    pub scan_duration: Duration,
    pub total_files: usize,
    pub total_lines: usize,
    pub total_matches: usize,
    pub cache_hits: usize,
    pub cache_total: usize,
    pub memory_usage_bytes: usize,
    pub thread_count: usize,
}

/// Comprehensive performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub scan_duration: Duration,
    pub files_per_second: f64,
    pub lines_per_second: f64,
    pub matches_per_second: f64,
    pub cache_hit_rate: f64,
    pub memory_usage_mb: f64,
    pub parallelism_efficiency: f64,
}

impl PerformanceMetrics {
    pub fn calculate(stats: ScanStats) -> Self {
        let duration_secs = stats.scan_duration.as_secs_f64();

        let files_per_second = if duration_secs > 0.0 {
            stats.total_files as f64 / duration_secs
        } else {
            0.0
        };

        let lines_per_second = if duration_secs > 0.0 {
            stats.total_lines as f64 / duration_secs
        } else {
            0.0
        };

        let matches_per_second = if duration_secs > 0.0 {
            stats.total_matches as f64 / duration_secs
        } else {
            0.0
        };

        let cache_hit_rate = if stats.cache_total > 0 {
            stats.cache_hits as f64 / stats.cache_total as f64
        } else {
            0.0
        };

        let memory_usage_mb = stats.memory_usage_bytes as f64 / (1024.0 * 1024.0);

        // Simple parallelism efficiency metric
        let ideal_duration = duration_secs * stats.thread_count as f64;
        let parallelism_efficiency = if ideal_duration > 0.0 {
            (duration_secs / ideal_duration).min(1.0)
        } else {
            0.0
        };

        Self {
            scan_duration: stats.scan_duration,
            files_per_second,
            lines_per_second,
            matches_per_second,
            cache_hit_rate,
            memory_usage_mb,
            parallelism_efficiency,
        }
    }
    
    pub fn report(&self) -> String {
        format!(
            "Performance Metrics:\n\
             ===================\n\
             Scan Duration: {:?}\n\
             Files/sec: {:.2}\n\
             Lines/sec: {:.2}\n\
             Matches/sec: {:.2}\n\
             Cache Hit Rate: {:.2}%\n\
             Memory Usage: {:.2} MB\n\
             Parallelism Efficiency: {:.2}%\n",
            self.scan_duration,
            self.files_per_second,
            self.lines_per_second,
            self.matches_per_second,
            self.cache_hit_rate * 100.0,
            self.memory_usage_mb,
            self.parallelism_efficiency * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    #[test]
    fn test_performance_profiler() {
        let mut profiler = PerformanceProfiler::new();
        
        profiler.start("test_operation");
        thread::sleep(Duration::from_millis(10));
        profiler.end("test_operation");
        
        assert!(profiler.average_duration("test_operation").is_some());
        assert_eq!(profiler.operation_count("test_operation"), 1);
    }
    
    #[test]
    fn test_memory_tracker() {
        let mut tracker = MemoryTracker::new();
        
        tracker.allocate(1024);
        assert_eq!(tracker.current_usage(), 1024);
        assert_eq!(tracker.peak_usage(), 1024);
        
        tracker.allocate(512);
        assert_eq!(tracker.current_usage(), 1536);
        assert_eq!(tracker.peak_usage(), 1536);
        
        tracker.deallocate(1024);
        assert_eq!(tracker.current_usage(), 512);
        assert_eq!(tracker.peak_usage(), 1536); // Peak should remain
    }
    
    #[test]
    fn test_performance_metrics() {
        let stats = ScanStats {
            scan_duration: Duration::from_secs(2),
            total_files: 100,
            total_lines: 10000,
            total_matches: 50,
            cache_hits: 80,
            cache_total: 100,
            memory_usage_bytes: 1024 * 1024,
            thread_count: 4,
        };
        let metrics = PerformanceMetrics::calculate(stats);
        
        assert_eq!(metrics.files_per_second, 50.0);
        assert_eq!(metrics.lines_per_second, 5000.0);
        assert_eq!(metrics.matches_per_second, 25.0);
        assert_eq!(metrics.cache_hit_rate, 0.8);
        assert_eq!(metrics.memory_usage_mb, 1.0);
    }
}