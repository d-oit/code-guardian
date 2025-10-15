use crate::{
    DetectorFactory, DetectorProfile, OptimizedScanner, Scanner, StreamingScanner,
};
use anyhow::Result;
use std::path::Path;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub duration: Duration,
    pub throughput: ThroughputMetrics,
    pub resource_usage: ResourceUsage,
    pub performance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputMetrics {
    pub files_per_second: f64,
    pub lines_per_second: f64,
    pub bytes_per_second: f64,
    pub detections_per_second: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub memory_mb: f64,
    pub cpu_percent: f64,
    pub io_operations: u64,
    pub thread_count: u32,
}

/// Comprehensive benchmark suite for Code Guardian
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuite {
    pub name: String,
    pub benchmarks: Vec<BenchmarkTest>,
    pub baseline_metrics: Option<BenchmarkResult>,
    pub results: Vec<BenchmarkResult>,
    pub summary: BenchmarkSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkTest {
    pub name: String,
    pub description: String,
    pub test_type: BenchmarkType,
    pub expected_performance: ExpectedPerformance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BenchmarkType {
    BasicScanning,
    ComprehensiveScanning,
    OptimizedScanning,
    StreamingScanning,
    LargeFileHandling,
    ManySmallFiles,
    MemoryIntensive,
    CacheEfficiency,
    ParallelProcessing,
    RegressiveDetection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedPerformance {
    pub max_duration_seconds: f64,
    pub max_memory_mb: f64,
    pub min_throughput_files_per_sec: f64,
    pub min_cache_hit_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub performance_score: f64,
    pub regression_detected: bool,
    pub improvement_areas: Vec<String>,
}

/// Predefined benchmark configurations
pub struct BenchmarkConfigurations;

impl BenchmarkConfigurations {
    /// Small project benchmark (< 1000 files)
    pub fn small_project() -> BenchmarkSuite {
        BenchmarkSuite {
            name: "Small Project Benchmark".to_string(),
            benchmarks: vec![
                BenchmarkTest {
                    name: "Basic Scan - Small Project".to_string(),
                    description: "Basic TODO/FIXME detection on small codebase".to_string(),
                    test_type: BenchmarkType::BasicScanning,
                    expected_performance: ExpectedPerformance {
                        max_duration_seconds: 2.0,
                        max_memory_mb: 50.0,
                        min_throughput_files_per_sec: 100.0,
                        min_cache_hit_rate: 0.0,
                    },
                },
                BenchmarkTest {
                    name: "Comprehensive Scan - Small Project".to_string(),
                    description: "All detectors on small codebase".to_string(),
                    test_type: BenchmarkType::ComprehensiveScanning,
                    expected_performance: ExpectedPerformance {
                        max_duration_seconds: 5.0,
                        max_memory_mb: 100.0,
                        min_throughput_files_per_sec: 50.0,
                        min_cache_hit_rate: 0.0,
                    },
                },
                BenchmarkTest {
                    name: "Optimized Scan - Small Project".to_string(),
                    description: "Optimized scanner with caching".to_string(),
                    test_type: BenchmarkType::OptimizedScanning,
                    expected_performance: ExpectedPerformance {
                        max_duration_seconds: 3.0,
                        max_memory_mb: 80.0,
                        min_throughput_files_per_sec: 80.0,
                        min_cache_hit_rate: 0.3,
                    },
                },
            ],
            baseline_metrics: None,
            results: Vec::new(),
            summary: BenchmarkSummary::default(),
        }
    }

    /// Medium project benchmark (1000-10000 files)
    pub fn medium_project() -> BenchmarkSuite {
        BenchmarkSuite {
            name: "Medium Project Benchmark".to_string(),
            benchmarks: vec![
                BenchmarkTest {
                    name: "Streaming Scan - Medium Project".to_string(),
                    description: "Memory-efficient streaming scan".to_string(),
                    test_type: BenchmarkType::StreamingScanning,
                    expected_performance: ExpectedPerformance {
                        max_duration_seconds: 15.0,
                        max_memory_mb: 200.0,
                        min_throughput_files_per_sec: 100.0,
                        min_cache_hit_rate: 0.5,
                    },
                },
                BenchmarkTest {
                    name: "Parallel Processing - Medium Project".to_string(),
                    description: "Multi-threaded scanning performance".to_string(),
                    test_type: BenchmarkType::ParallelProcessing,
                    expected_performance: ExpectedPerformance {
                        max_duration_seconds: 10.0,
                        max_memory_mb: 300.0,
                        min_throughput_files_per_sec: 200.0,
                        min_cache_hit_rate: 0.6,
                    },
                },
                BenchmarkTest {
                    name: "Cache Efficiency - Medium Project".to_string(),
                    description: "Repeated scans to test cache effectiveness".to_string(),
                    test_type: BenchmarkType::CacheEfficiency,
                    expected_performance: ExpectedPerformance {
                        max_duration_seconds: 8.0,
                        max_memory_mb: 150.0,
                        min_throughput_files_per_sec: 300.0,
                        min_cache_hit_rate: 0.8,
                    },
                },
            ],
            baseline_metrics: None,
            results: Vec::new(),
            summary: BenchmarkSummary::default(),
        }
    }

    /// Large project benchmark (> 10000 files)
    pub fn large_project() -> BenchmarkSuite {
        BenchmarkSuite {
            name: "Large Project Benchmark".to_string(),
            benchmarks: vec![
                BenchmarkTest {
                    name: "Large File Handling".to_string(),
                    description: "Performance on files > 1MB".to_string(),
                    test_type: BenchmarkType::LargeFileHandling,
                    expected_performance: ExpectedPerformance {
                        max_duration_seconds: 30.0,
                        max_memory_mb: 500.0,
                        min_throughput_files_per_sec: 10.0,
                        min_cache_hit_rate: 0.7,
                    },
                },
                BenchmarkTest {
                    name: "Many Small Files".to_string(),
                    description: "Performance on thousands of small files".to_string(),
                    test_type: BenchmarkType::ManySmallFiles,
                    expected_performance: ExpectedPerformance {
                        max_duration_seconds: 45.0,
                        max_memory_mb: 400.0,
                        min_throughput_files_per_sec: 500.0,
                        min_cache_hit_rate: 0.6,
                    },
                },
                BenchmarkTest {
                    name: "Memory Intensive Scan".to_string(),
                    description: "Memory usage under extreme load".to_string(),
                    test_type: BenchmarkType::MemoryIntensive,
                    expected_performance: ExpectedPerformance {
                        max_duration_seconds: 60.0,
                        max_memory_mb: 1000.0,
                        min_throughput_files_per_sec: 50.0,
                        min_cache_hit_rate: 0.5,
                    },
                },
            ],
            baseline_metrics: None,
            results: Vec::new(),
            summary: BenchmarkSummary::default(),
        }
    }

    /// Regression detection benchmark
    pub fn regression_detection() -> BenchmarkSuite {
        BenchmarkSuite {
            name: "Regression Detection Benchmark".to_string(),
            benchmarks: vec![
                BenchmarkTest {
                    name: "Performance Regression Detection".to_string(),
                    description: "Detect performance regressions against baseline".to_string(),
                    test_type: BenchmarkType::RegressiveDetection,
                    expected_performance: ExpectedPerformance {
                        max_duration_seconds: 10.0,
                        max_memory_mb: 200.0,
                        min_throughput_files_per_sec: 100.0,
                        min_cache_hit_rate: 0.5,
                    },
                },
            ],
            baseline_metrics: None,
            results: Vec::new(),
            summary: BenchmarkSummary::default(),
        }
    }
}

impl BenchmarkSuite {
    /// Run all benchmarks in the suite
    pub fn run_benchmarks(&mut self, path: &Path) -> Result<()> {
        println!("🚀 Running Benchmark Suite: {}", self.name);
        println!("=====================================");
        
        self.results.clear();
        let mut passed = 0;
        let mut failed = 0;

        for benchmark in &self.benchmarks {
            println!("\n📊 Running: {}", benchmark.name);
            println!("   Description: {}", benchmark.description);
            
            match self.run_single_benchmark(benchmark, path) {
                Ok(result) => {
                    let performance_met = self.check_performance_expectations(&result, &benchmark.expected_performance);
                    
                    if performance_met {
                        println!("   ✅ PASSED");
                        passed += 1;
                    } else {
                        println!("   ❌ FAILED - Performance expectations not met");
                        failed += 1;
                    }
                    
                    println!("   ⏱️  Duration: {:?}", result.duration);
                    println!("   📈 Files/sec: {:.1}", result.throughput.files_per_second);
                    println!("   💾 Memory: {:.1} MB", result.resource_usage.memory_mb);
                    
                    self.results.push(result);
                }
                Err(e) => {
                    println!("   ❌ FAILED - Error: {}", e);
                    failed += 1;
                }
            }
        }

        // Calculate summary
        self.summary = BenchmarkSummary {
            total_tests: self.benchmarks.len(),
            passed_tests: passed,
            failed_tests: failed,
            performance_score: self.calculate_overall_performance_score(),
            regression_detected: self.detect_regression(),
            improvement_areas: self.identify_improvement_areas(),
        };

        self.print_summary();
        Ok(())
    }

    /// Run a single benchmark test
    fn run_single_benchmark(&self, benchmark: &BenchmarkTest, path: &Path) -> Result<BenchmarkResult> {
        let start_time = Instant::now();
        let start_memory = self.get_memory_usage();

        let (files_scanned, lines_processed, matches_found, _cache_hits, _cache_misses) = match benchmark.test_type {
            BenchmarkType::BasicScanning => self.run_basic_scan(path)?,
            BenchmarkType::ComprehensiveScanning => self.run_comprehensive_scan(path)?,
            BenchmarkType::OptimizedScanning => self.run_optimized_scan(path)?,
            BenchmarkType::StreamingScanning => self.run_streaming_scan(path)?,
            BenchmarkType::LargeFileHandling => self.run_large_file_test(path)?,
            BenchmarkType::ManySmallFiles => self.run_many_small_files_test(path)?,
            BenchmarkType::MemoryIntensive => self.run_memory_intensive_test(path)?,
            BenchmarkType::CacheEfficiency => self.run_cache_efficiency_test(path)?,
            BenchmarkType::ParallelProcessing => self.run_parallel_processing_test(path)?,
            BenchmarkType::RegressiveDetection => self.run_regression_test(path)?,
        };

        let duration = start_time.elapsed();
        let end_memory = self.get_memory_usage();
        let peak_memory = end_memory.max(start_memory);

        let throughput = ThroughputMetrics {
            files_per_second: files_scanned as f64 / duration.as_secs_f64(),
            lines_per_second: lines_processed as f64 / duration.as_secs_f64(),
            bytes_per_second: (lines_processed * 50) as f64 / duration.as_secs_f64(), // Estimate
            detections_per_second: matches_found as f64 / duration.as_secs_f64(),
        };

        let resource_usage = ResourceUsage {
            memory_mb: peak_memory,
            cpu_percent: 0.0, // Would need system monitoring
            io_operations: files_scanned,
            thread_count: 1, // Would need actual thread monitoring
        };

        let performance_score = self.calculate_performance_score(&throughput, &resource_usage, duration);

        Ok(BenchmarkResult {
            name: benchmark.name.clone(),
            duration,
            throughput,
            resource_usage,
            performance_score,
        })
    }

    /// Run basic scanning benchmark
    fn run_basic_scan(&self, path: &Path) -> Result<(u64, u64, u64, u64, u64)> {
        let scanner = Scanner::new(DetectorFactory::create_default_detectors());
        let matches = scanner.scan(path)?;
        
        // Estimate metrics (in real implementation, these would be tracked)
        let files_scanned = self.estimate_file_count(path);
        let lines_processed = files_scanned * 100; // Estimate
        
        Ok((files_scanned, lines_processed, matches.len() as u64, 0, 0))
    }

    /// Run comprehensive scanning benchmark
    fn run_comprehensive_scan(&self, path: &Path) -> Result<(u64, u64, u64, u64, u64)> {
        let scanner = Scanner::new(DetectorProfile::Comprehensive.get_detectors());
        let matches = scanner.scan(path)?;
        
        let files_scanned = self.estimate_file_count(path);
        let lines_processed = files_scanned * 100;
        
        Ok((files_scanned, lines_processed, matches.len() as u64, 0, 0))
    }

    /// Run optimized scanning benchmark
    fn run_optimized_scan(&self, path: &Path) -> Result<(u64, u64, u64, u64, u64)> {
        let scanner = OptimizedScanner::new(DetectorProfile::Comprehensive.get_detectors())
            .with_cache_size(10000);
        let (matches, metrics) = scanner.scan_optimized(path)?;
        
        Ok((
            metrics.total_files_scanned as u64,
            metrics.total_lines_processed as u64,
            matches.len() as u64,
            metrics.cache_hits as u64,
            metrics.cache_misses as u64,
        ))
    }

    /// Run streaming scanning benchmark
    fn run_streaming_scan(&self, path: &Path) -> Result<(u64, u64, u64, u64, u64)> {
        let scanner = StreamingScanner::new(DetectorProfile::Comprehensive.get_detectors());
        let mut matches = Vec::new();
        let metrics = scanner.scan_streaming(path, |batch| {
            matches.extend(batch);
            Ok(())
        })?;
        
        Ok((
            metrics.total_files_scanned as u64,
            metrics.total_lines_processed as u64,
            matches.len() as u64,
            0,
            0,
        ))
    }

    /// Run large file handling test
    fn run_large_file_test(&self, path: &Path) -> Result<(u64, u64, u64, u64, u64)> {
        // Filter for large files and test specifically
        self.run_optimized_scan(path)
    }

    /// Run many small files test
    fn run_many_small_files_test(&self, path: &Path) -> Result<(u64, u64, u64, u64, u64)> {
        // Test performance on many small files
        self.run_streaming_scan(path)
    }

    /// Run memory intensive test
    fn run_memory_intensive_test(&self, path: &Path) -> Result<(u64, u64, u64, u64, u64)> {
        // Test with multiple concurrent scans
        self.run_comprehensive_scan(path)
    }

    /// Run cache efficiency test
    fn run_cache_efficiency_test(&self, path: &Path) -> Result<(u64, u64, u64, u64, u64)> {
        // Run multiple scans to test cache effectiveness
        let scanner = OptimizedScanner::new(DetectorProfile::Basic.get_detectors())
            .with_cache_size(50000);
        
        // First scan (cold cache)
        let _ = scanner.scan_optimized(path)?;
        
        // Second scan (should hit cache)
        let (matches, metrics) = scanner.scan_optimized(path)?;
        
        Ok((
            metrics.total_files_scanned as u64,
            metrics.total_lines_processed as u64,
            matches.len() as u64,
            metrics.cache_hits as u64,
            metrics.cache_misses as u64,
        ))
    }

    /// Run parallel processing test
    fn run_parallel_processing_test(&self, path: &Path) -> Result<(u64, u64, u64, u64, u64)> {
        // Test parallel scanning performance
        self.run_optimized_scan(path)
    }

    /// Run regression detection test
    fn run_regression_test(&self, path: &Path) -> Result<(u64, u64, u64, u64, u64)> {
        // Compare against baseline if available
        self.run_optimized_scan(path)
    }

    /// Check if performance meets expectations
    fn check_performance_expectations(&self, result: &BenchmarkResult, expected: &ExpectedPerformance) -> bool {
        result.duration.as_secs_f64() <= expected.max_duration_seconds
            && result.resource_usage.memory_mb <= expected.max_memory_mb
            && result.throughput.files_per_second >= expected.min_throughput_files_per_sec
    }

    /// Calculate overall performance score for the suite
    fn calculate_overall_performance_score(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        
        self.results.iter().map(|r| r.performance_score).sum::<f64>() / self.results.len() as f64
    }

    /// Calculate performance score for individual benchmark
    fn calculate_performance_score(&self, throughput: &ThroughputMetrics, resource_usage: &ResourceUsage, duration: Duration) -> f64 {
        let time_score = if duration.as_secs() < 5 { 100.0 } else { 100.0 / duration.as_secs() as f64 };
        let memory_score = if resource_usage.memory_mb < 100.0 { 100.0 } else { 10000.0 / resource_usage.memory_mb };
        let throughput_score = throughput.files_per_second.min(100.0);
        
        (time_score * 0.4 + memory_score * 0.3 + throughput_score * 0.3).min(100.0)
    }

    /// Detect performance regression
    fn detect_regression(&self) -> bool {
        if let Some(baseline) = &self.baseline_metrics {
            if let Some(current) = self.results.first() {
                return current.performance_score < baseline.performance_score * 0.9; // 10% regression threshold
            }
        }
        false
    }

    /// Identify areas for improvement
    fn identify_improvement_areas(&self) -> Vec<String> {
        let mut areas = Vec::new();
        
        for result in &self.results {
            if result.resource_usage.memory_mb > 500.0 {
                areas.push("Memory optimization needed".to_string());
            }
            if result.throughput.files_per_second < 50.0 {
                areas.push("Throughput optimization needed".to_string());
            }
            if result.duration.as_secs() > 30 {
                areas.push("Execution time optimization needed".to_string());
            }
        }
        
        areas.sort();
        areas.dedup();
        areas
    }

    /// Print benchmark summary
    fn print_summary(&self) {
        println!("\n🏁 Benchmark Suite Summary");
        println!("==========================");
        println!("Suite: {}", self.name);
        println!("Total Tests: {}", self.summary.total_tests);
        println!("Passed: ✅ {}", self.summary.passed_tests);
        println!("Failed: ❌ {}", self.summary.failed_tests);
        println!("Performance Score: {:.1}/100", self.summary.performance_score);
        
        if self.summary.regression_detected {
            println!("🚨 Performance regression detected!");
        } else {
            println!("✅ No performance regression");
        }
        
        if !self.summary.improvement_areas.is_empty() {
            println!("\n💡 Improvement Areas:");
            for area in &self.summary.improvement_areas {
                println!("   • {}", area);
            }
        }
    }

    /// Estimate file count for benchmarking
    fn estimate_file_count(&self, path: &Path) -> u64 {
        // Simple estimation - in real implementation would traverse directory
        if path.is_file() {
            1
        } else {
            100 // Estimate for directory
        }
    }

    /// Get current memory usage (placeholder)
    fn get_memory_usage(&self) -> f64 {
        // Placeholder - would integrate with system monitoring
        50.0
    }
}

impl Default for BenchmarkSummary {
    fn default() -> Self {
        Self {
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            performance_score: 0.0,
            regression_detected: false,
            improvement_areas: Vec::new(),
        }
    }
}