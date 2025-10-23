use code_guardian_core::{
    optimized_scanner::OptimizedScanner,
    performance_optimized_scanner::PerformanceOptimizedScanner, ConsoleLogDetector,
    DebuggerDetector, DetectorFactory, DetectorProfile, FixmeDetector, HackDetector,
    PatternDetector, Scanner, TodoDetector,
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs;
use tempfile::TempDir;

/// Create test files with various patterns for benchmarking
fn create_test_detectors() -> Vec<Box<dyn PatternDetector>> {
    vec![
        Box::new(TodoDetector),
        Box::new(FixmeDetector),
        Box::new(HackDetector),
        Box::new(ConsoleLogDetector),
        Box::new(DebuggerDetector),
    ]
}

fn create_test_files(dir: &TempDir, file_count: usize, lines_per_file: usize) {
    let patterns = [
        "// TODO: implement this feature",
        "// FIXME: this is broken",
        "// HACK: temporary workaround",
        "// BUG: needs investigation",
        "panic!(\"something went wrong\");",
        "value.unwrap()",
        "console.log(\"debug info\");",
        "print(\"debug\")",
        "debugger;",
        "let unused_var = 42; // unused",
    ];

    for i in 0..file_count {
        let mut content = String::new();
        for line in 0..lines_per_file {
            if line % 10 == 0 {
                // Add a pattern every 10 lines
                let pattern = patterns[line % patterns.len()];
                content.push_str(&format!("{}  // Line {}\n", pattern, line));
            } else {
                // Add regular code
                content.push_str(&format!("fn function_{}() {{ let x = {}; }}\n", line, line));
            }
        }

        let file_path = dir.path().join(format!("test_file_{}.rs", i));
        fs::write(file_path, content).unwrap();
    }
}

/// Benchmark small codebase (10 files, 100 lines each)
fn bench_small_codebase(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    create_test_files(&temp_dir, 10, 100);

    let _detectors = DetectorProfile::Comprehensive.get_detectors();

    let mut group = c.benchmark_group("small_codebase");

    // Standard scanner
    group.bench_with_input(
        BenchmarkId::new("standard_scanner", "10_files_100_lines"),
        &temp_dir,
        |b, dir| {
            let scanner = Scanner::new(DetectorFactory::create_default_detectors());
            b.iter(|| {
                let matches = scanner.scan(black_box(dir.path())).unwrap();
                black_box(matches);
            });
        },
    );

    // Optimized scanner
    group.bench_with_input(
        BenchmarkId::new("optimized_scanner", "10_files_100_lines"),
        &temp_dir,
        |b, dir| {
            let detectors = create_test_detectors();
            let scanner = OptimizedScanner::new(detectors);
            b.iter(|| {
                let (matches, _metrics) = scanner.scan_optimized(black_box(dir.path())).unwrap();
                black_box(matches);
            });
        },
    );

    // Performance optimized scanner
    group.bench_with_input(
        BenchmarkId::new("performance_optimized_scanner", "10_files_100_lines"),
        &temp_dir,
        |b, dir| {
            let scanner = PerformanceOptimizedScanner::new(create_test_detectors());
            b.iter(|| {
                let (matches, _metrics) = scanner.scan_ultra_fast(black_box(dir.path())).unwrap();
                black_box(matches);
            });
        },
    );

    group.finish();
}

/// Benchmark medium codebase (100 files, 500 lines each)
fn bench_medium_codebase(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    create_test_files(&temp_dir, 100, 500);

    let _detectors = DetectorProfile::Comprehensive.get_detectors();

    let mut group = c.benchmark_group("medium_codebase");
    group.sample_size(10); // Fewer samples for larger tests

    // Standard scanner
    group.bench_with_input(
        BenchmarkId::new("standard_scanner", "100_files_500_lines"),
        &temp_dir,
        |b, dir| {
            let scanner = Scanner::new(DetectorFactory::create_default_detectors());
            b.iter(|| {
                let matches = scanner.scan(black_box(dir.path())).unwrap();
                black_box(matches);
            });
        },
    );

    // Optimized scanner
    group.bench_with_input(
        BenchmarkId::new("optimized_scanner", "100_files_500_lines"),
        &temp_dir,
        |b, dir| {
            let scanner = OptimizedScanner::new(create_test_detectors());
            b.iter(|| {
                let (matches, _metrics) = scanner.scan_optimized(black_box(dir.path())).unwrap();
                black_box(matches);
            });
        },
    );

    // Performance optimized scanner
    group.bench_with_input(
        BenchmarkId::new("performance_optimized_scanner", "100_files_500_lines"),
        &temp_dir,
        |b, dir| {
            let scanner = PerformanceOptimizedScanner::new(create_test_detectors());
            b.iter(|| {
                let (matches, _metrics) = scanner.scan_ultra_fast(black_box(dir.path())).unwrap();
                black_box(matches);
            });
        },
    );

    group.finish();
}

/// Benchmark large codebase (500 files, 1000 lines each)
fn bench_large_codebase(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    create_test_files(&temp_dir, 500, 1000);

    let _detectors = DetectorProfile::Comprehensive.get_detectors();

    let mut group = c.benchmark_group("large_codebase");
    group.sample_size(5); // Even fewer samples for largest tests

    // Skip standard scanner for large tests (too slow)

    // Optimized scanner
    group.bench_with_input(
        BenchmarkId::new("optimized_scanner", "500_files_1000_lines"),
        &temp_dir,
        |b, dir| {
            let scanner = OptimizedScanner::new(create_test_detectors());
            b.iter(|| {
                let (matches, _metrics) = scanner.scan_optimized(black_box(dir.path())).unwrap();
                black_box(matches);
            });
        },
    );

    // Performance optimized scanner
    group.bench_with_input(
        BenchmarkId::new("performance_optimized_scanner", "500_files_1000_lines"),
        &temp_dir,
        |b, dir| {
            let scanner = PerformanceOptimizedScanner::new(create_test_detectors());
            b.iter(|| {
                let (matches, _metrics) = scanner.scan_ultra_fast(black_box(dir.path())).unwrap();
                black_box(matches);
            });
        },
    );

    group.finish();
}

/// Benchmark cache performance with repeated scans
fn bench_cache_performance(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    create_test_files(&temp_dir, 50, 200);

    let _detectors = DetectorProfile::Comprehensive.get_detectors();

    let mut group = c.benchmark_group("cache_performance");

    // Optimized scanner with cache
    group.bench_with_input(
        BenchmarkId::new("optimized_scanner_cached", "repeated_scans"),
        &temp_dir,
        |b, dir| {
            let scanner = OptimizedScanner::new(create_test_detectors());
            // Prime the cache
            let _ = scanner.scan_optimized(dir.path()).unwrap();

            b.iter(|| {
                let (matches, _metrics) = scanner.scan_optimized(black_box(dir.path())).unwrap();
                black_box(matches);
            });
        },
    );

    // Performance optimized scanner with cache
    group.bench_with_input(
        BenchmarkId::new("performance_optimized_scanner_cached", "repeated_scans"),
        &temp_dir,
        |b, dir| {
            let scanner = PerformanceOptimizedScanner::new(create_test_detectors());
            // Prime the cache
            let _ = scanner.scan_ultra_fast(dir.path()).unwrap();

            b.iter(|| {
                let (matches, _metrics) = scanner.scan_ultra_fast(black_box(dir.path())).unwrap();
                black_box(matches);
            });
        },
    );

    group.finish();
}

/// Benchmark SIMD vs Regex performance for pattern detection
fn bench_pattern_detection_methods(c: &mut Criterion) {
    let content = "// TODO: implement this feature\n".repeat(1000)
        + &"// FIXME: this is broken\n".repeat(800)
        + &"// HACK: temporary workaround\n".repeat(600)
        + &"regular code line\n".repeat(2000);

    let mut group = c.benchmark_group("pattern_detection_methods");

    // Pure regex approach
    group.bench_function("regex_only", |b| {
        use regex::Regex;
        let todo_regex = Regex::new(r"(?i)\btodo\b").unwrap();
        let fixme_regex = Regex::new(r"(?i)\bfixme\b").unwrap();
        let hack_regex = Regex::new(r"(?i)\bhack\b").unwrap();

        b.iter(|| {
            let mut count = 0;
            count += todo_regex.find_iter(black_box(&content)).count();
            count += fixme_regex.find_iter(black_box(&content)).count();
            count += hack_regex.find_iter(black_box(&content)).count();
            black_box(count);
        });
    });

    // SIMD approach (simulated with memchr)
    group.bench_function("simd_memchr", |b| {
        use memchr::memchr;

        b.iter(|| {
            let content_bytes = black_box(content.as_bytes());
            let mut count = 0;
            let mut pos = 0;

            // Count 't' characters (simplified SIMD simulation)
            while let Some(found) = memchr(b't', &content_bytes[pos..]) {
                count += 1;
                pos += found + 1;
                if pos >= content_bytes.len() {
                    break;
                }
            }

            black_box(count);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_small_codebase,
    bench_medium_codebase,
    bench_large_codebase,
    bench_cache_performance,
    bench_pattern_detection_methods
);
criterion_main!(benches);
