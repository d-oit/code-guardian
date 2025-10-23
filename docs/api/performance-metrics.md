# Performance Metrics API

## Overview

Code Guardian provides comprehensive performance metrics through the `AdvancedScanMetrics` struct and related APIs. This documentation covers how to access, interpret, and utilize performance data.

## Core Types

### AdvancedScanMetrics

```rust
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
    pub average_file_size_bytes: f64,
    pub cache_hit_rate: f64,
    pub throughput_files_per_second: f64,
    pub throughput_lines_per_second: f64,
}
```

### ScanMetrics (Internal)

```rust
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
```

## Scanner APIs

### Standard Scanner

```rust
use code_guardian_core::Scanner;

let scanner = Scanner::new();
let (matches, metrics) = scanner.scan_with_metrics(&path)?;

// Basic metrics available
println!("Files scanned: {}", metrics.total_files_scanned);
println!("Duration: {}ms", metrics.scan_duration_ms);
```

### Optimized Scanner

```rust
use code_guardian_core::OptimizedScanner;

let scanner = OptimizedScanner::new(detectors);
let (matches, metrics) = scanner.scan_optimized(&path)?;

// Enhanced metrics with caching information
println!("Cache hit rate: {:.2}%", metrics.cache_hit_rate);
println!("Throughput: {:.1} files/sec", metrics.throughput_files_per_second);
```

### Performance Optimized Scanner

```rust
use code_guardian_core::PerformanceOptimizedScanner;

let scanner = PerformanceOptimizedScanner::new(detectors);
let (matches, metrics) = scanner.scan_optimized(&path)?;

// Advanced metrics with detailed timing
println!("SIMD matches: {}", metrics.simd_matches);
println!("Regex matches: {}", metrics.regex_matches);
println!("File read time: {}ms", metrics.file_read_time_ms);
println!("Pattern search time: {}ms", metrics.pattern_search_time_ms);
```

## Metric Interpretation

### Performance Indicators

#### Throughput Metrics
```rust
// Files per second - overall scanning speed
let files_per_sec = metrics.throughput_files_per_second;

// Lines per second - content processing speed  
let lines_per_sec = metrics.throughput_lines_per_second;

// Efficiency ratio
let efficiency = lines_per_sec / files_per_sec; // Lines per file processed
```

#### Cache Performance
```rust
// Cache effectiveness
let cache_hit_rate = metrics.cache_hit_rate;
if cache_hit_rate < 50.0 {
    println!("⚠️ Low cache hit rate - consider tuning cache size");
} else if cache_hit_rate > 90.0 {
    println!("✅ Excellent cache performance");
}

// Cache utilization
let total_cache_ops = metrics.cache_hits + metrics.cache_misses;
let cache_utilization = total_cache_ops as f64 / metrics.total_files_scanned as f64;
```

#### Detection Method Distribution
```rust
// Pattern detection breakdown
let total_matches = metrics.simd_matches + metrics.regex_matches;
let simd_percentage = (metrics.simd_matches as f64 / total_matches as f64) * 100.0;

println!("SIMD detection: {:.1}%", simd_percentage);
println!("Regex detection: {:.1}%", 100.0 - simd_percentage);
```

#### Timing Analysis
```rust
// Time distribution analysis
let total_processing_time = metrics.file_read_time_ms + 
                           metrics.pattern_search_time_ms + 
                           metrics.result_processing_time_ms;

let read_percentage = (metrics.file_read_time_ms as f64 / total_processing_time as f64) * 100.0;
let search_percentage = (metrics.pattern_search_time_ms as f64 / total_processing_time as f64) * 100.0;
let process_percentage = (metrics.result_processing_time_ms as f64 / total_processing_time as f64) * 100.0;

println!("Time breakdown:");
println!("  File reading: {:.1}%", read_percentage);
println!("  Pattern search: {:.1}%", search_percentage);
println!("  Result processing: {:.1}%", process_percentage);
```

## Performance Monitoring

### Real-time Monitoring

```rust
use code_guardian_core::PerformanceMonitor;

let monitor = PerformanceMonitor::new();

// Record scan metrics
monitor.record_scan(&metrics);

// Get performance trends
let avg_duration = monitor.average_scan_duration();
let trend = monitor.performance_trend();

match trend {
    PerformanceTrend::Improving => println!("📈 Performance improving"),
    PerformanceTrend::Degrading => println!("📉 Performance degrading"),
    PerformanceTrend::Stable => println!("📊 Performance stable"),
}
```

### Alerting Integration

```rust
use code_guardian_core::alerts::PerformanceAlert;

let alert_config = PerformanceAlert::builder()
    .max_scan_duration_ms(30000)  // 30 seconds
    .min_cache_hit_rate(0.7)      // 70%
    .max_memory_usage_mb(1024)    // 1GB
    .build();

if let Some(alert) = alert_config.check_metrics(&metrics) {
    println!("🚨 Performance Alert: {}", alert.message());
    // Send to monitoring system
}
```

## Benchmark Integration

### Custom Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use code_guardian_core::*;

fn benchmark_scanner_performance(c: &mut Criterion) {
    let test_data = create_test_data();
    
    c.bench_function("optimized_scanner", |b| {
        let scanner = OptimizedScanner::new(create_detectors());
        b.iter(|| {
            let (matches, metrics) = scanner.scan_optimized(black_box(&test_data)).unwrap();
            black_box((matches, metrics));
        });
    });
}

criterion_group!(benches, benchmark_scanner_performance);
criterion_main!(benches);
```

### Performance Regression Testing

```rust
#[test]
fn test_performance_regression() {
    let scanner = OptimizedScanner::new(create_test_detectors());
    let test_path = create_medium_test_data(); // 100 files, 500 lines each
    
    let start = std::time::Instant::now();
    let (matches, metrics) = scanner.scan_optimized(&test_path).unwrap();
    let duration = start.elapsed();
    
    // Performance regression thresholds
    assert!(duration.as_millis() < 5000, "Scan took too long: {:?}", duration);
    assert!(metrics.cache_hit_rate > 0.5, "Cache hit rate too low: {}", metrics.cache_hit_rate);
    assert!(metrics.throughput_files_per_second > 20.0, "Throughput too low: {}", metrics.throughput_files_per_second);
}
```

## Configuration API

### Scanner Configuration

```rust
use code_guardian_core::config::ScannerConfig;

let config = ScannerConfig::builder()
    .batch_size(100)
    .parallel_workers(num_cpus::get())
    .cache_size_mb(256)
    .adaptive_threshold(25)
    .enable_simd(true)
    .enable_metrics_collection(true)
    .build();

let scanner = PerformanceOptimizedScanner::with_config(detectors, config);
```

### Metrics Configuration

```rust
use code_guardian_core::config::MetricsConfig;

let metrics_config = MetricsConfig::builder()
    .collect_timing_details(true)
    .collect_cache_metrics(true)
    .collect_memory_metrics(true)
    .enable_real_time_monitoring(true)
    .build();

let scanner = scanner.with_metrics_config(metrics_config);
```

## Export and Integration

### JSON Export

```rust
// Export metrics as JSON
let json = serde_json::to_string_pretty(&metrics)?;
std::fs::write("scan_metrics.json", json)?;
```

### Prometheus Integration

```rust
use code_guardian_core::metrics::PrometheusExporter;

let exporter = PrometheusExporter::new();
exporter.export_metrics(&metrics, "code_guardian")?;

// Metrics available at http://localhost:9090/metrics
```

### CSV Export

```rust
use code_guardian_core::metrics::CsvExporter;

let exporter = CsvExporter::new();
exporter.write_metrics(&metrics, "performance_log.csv")?;
```

## Error Handling

### Performance Errors

```rust
use code_guardian_core::errors::PerformanceError;

match scanner.scan_optimized(&path) {
    Ok((matches, metrics)) => {
        // Success case
    },
    Err(PerformanceError::TimeoutExceeded(duration)) => {
        println!("Scan timed out after {:?}", duration);
    },
    Err(PerformanceError::MemoryLimitExceeded(usage)) => {
        println!("Memory limit exceeded: {} MB", usage);
    },
    Err(PerformanceError::CacheCorruption) => {
        println!("Cache corruption detected, rebuilding...");
    },
    Err(e) => {
        println!("Performance error: {}", e);
    }
}
```

## Best Practices

### Metrics Collection
1. **Enable appropriate metrics** for your use case
2. **Monitor trends** rather than individual data points
3. **Set up alerting** for performance degradation
4. **Export data** for analysis and reporting

### Performance Optimization
1. **Use cache hit rate** to tune cache sizes
2. **Monitor timing breakdowns** to identify bottlenecks  
3. **Track throughput trends** to validate optimizations
4. **Benchmark regularly** to catch regressions

### Production Deployment
1. **Configure appropriate thresholds** for your environment
2. **Monitor memory usage** patterns over time
3. **Set up automated performance testing** in CI/CD
4. **Export metrics** to your monitoring infrastructure