use code_guardian_core::detector_factory::DetectorFactory;
use code_guardian_core::Scanner;
use std::fs;
use std::time::Instant;
use tempfile::TempDir;

/// Performance regression tests to ensure code changes don't degrade performance
#[cfg(test)]
mod performance_regression_tests {
    use super::*;

    // Performance baselines - these should be updated when intentional performance improvements are made
    const BASELINE_SMALL_FILE_MS: u64 = 60; // 60ms for small files
    const BASELINE_MEDIUM_FILE_MS: u64 = 200; // 200ms for medium files
    const BASELINE_LARGE_FILE_MS: u64 = 1000; // 1000ms for large files
    const BASELINE_MANY_FILES_MS: u64 = 2000; // 2000ms for many files

    pub fn create_scanner() -> Scanner {
        let detectors = DetectorFactory::create_default_detectors();
        Scanner::new(detectors)
    }

    fn create_test_content(size: usize, pattern_density: f64) -> String {
        let mut content = String::new();
        let patterns = ["TODO", "FIXME", "HACK", "console.log", "unwrap()", "panic!"];

        for i in 0..size {
            if (i as f64 / size as f64) < pattern_density {
                let pattern = &patterns[i % patterns.len()];
                content.push_str(&format!("// {}: Line {}\n", pattern, i));
            } else {
                content.push_str(&format!(
                    "fn function_{}() {{ println!(\"Line {}\"); }}\n",
                    i, i
                ));
            }
        }
        content
    }

    #[test]
    fn test_small_file_performance() {
        // Test performance on small file (100 lines)
        let content = create_test_content(100, 0.1); // 10% pattern density
        let detectors = DetectorFactory::create_default_detectors();
        let scanner = Scanner::new(detectors);

        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("small.rs");
        fs::write(&file_path, &content).unwrap();

        let start = Instant::now();
        let matches = scanner.scan(temp_dir.path()).unwrap();
        let duration = start.elapsed();

        println!(
            "Small file scan took: {:?} (baseline: {}ms)",
            duration, BASELINE_SMALL_FILE_MS
        );

        // Should complete within baseline time
        assert!(
            duration.as_millis() <= BASELINE_SMALL_FILE_MS as u128,
            "Small file scan took {}ms, expected <= {}ms",
            duration.as_millis(),
            BASELINE_SMALL_FILE_MS
        );

        // Should find some matches
        assert!(
            !matches.is_empty(),
            "Should find some patterns in small file"
        );
    }

    #[test]
    fn test_medium_file_performance() {
        // Test performance on medium file (1000 lines)
        let content = create_test_content(1000, 0.05); // 5% pattern density
        let scanner = create_scanner();

        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("medium.rs");
        fs::write(&file_path, &content).unwrap();

        let start = Instant::now();
        let matches = scanner.scan(temp_dir.path()).unwrap();
        let duration = start.elapsed();

        println!(
            "Medium file scan took: {:?} (baseline: {}ms)",
            duration, BASELINE_MEDIUM_FILE_MS
        );

        assert!(
            duration.as_millis() <= BASELINE_MEDIUM_FILE_MS as u128,
            "Medium file scan took {}ms, expected <= {}ms",
            duration.as_millis(),
            BASELINE_MEDIUM_FILE_MS
        );

        assert!(
            !matches.is_empty(),
            "Should find some patterns in medium file"
        );
    }

    #[test]
    fn test_large_file_performance() {
        // Test performance on large file (10000 lines)
        let content = create_test_content(10000, 0.02); // 2% pattern density
        let scanner = create_scanner();

        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("large.rs");
        fs::write(&file_path, &content).unwrap();

        let start = Instant::now();
        let matches = scanner.scan(temp_dir.path()).unwrap();
        let duration = start.elapsed();

        println!(
            "Large file scan took: {:?} (baseline: {}ms)",
            duration, BASELINE_LARGE_FILE_MS
        );

        assert!(
            duration.as_millis() <= BASELINE_LARGE_FILE_MS as u128,
            "Large file scan took {}ms, expected <= {}ms",
            duration.as_millis(),
            BASELINE_LARGE_FILE_MS
        );

        assert!(
            !matches.is_empty(),
            "Should find some patterns in large file"
        );
    }

    #[test]
    fn test_many_files_performance() {
        // Test performance with many files (50 files)
        let scanner = create_scanner();
        let temp_dir = TempDir::new().unwrap();

        // Create 50 files with varying content
        for i in 0..50 {
            let content = create_test_content(200, 0.05);
            let file_path = temp_dir.path().join(format!("file_{}.rs", i));
            fs::write(&file_path, &content).unwrap();
        }

        let start = Instant::now();
        let matches = scanner.scan(temp_dir.path()).unwrap();
        let duration = start.elapsed();

        println!(
            "Many files scan took: {:?} (baseline: {}ms)",
            duration, BASELINE_MANY_FILES_MS
        );

        assert!(
            duration.as_millis() <= BASELINE_MANY_FILES_MS as u128,
            "Many files scan took {}ms, expected <= {}ms",
            duration.as_millis(),
            BASELINE_MANY_FILES_MS
        );

        assert!(
            !matches.is_empty(),
            "Should find some patterns across many files"
        );
    }

    #[test]
    fn test_memory_usage_bounded() {
        // Test that memory usage doesn't grow unbounded with file size
        let scanner = create_scanner();

        // Test with progressively larger content
        for size in [1000, 5000, 10000].iter() {
            let content = create_test_content(*size, 0.01);
            let temp_dir = TempDir::new().unwrap();
            let file_path = temp_dir.path().join(format!("memory_test_{}.rs", size));
            fs::write(&file_path, &content).unwrap();

            // Memory usage should be roughly linear, not exponential
            let _matches = scanner.scan(temp_dir.path()).unwrap();

            // This is a basic test - in a real scenario you'd want more sophisticated memory monitoring
            println!("Completed scan for {} lines", size);
        }
    }

    #[test]
    fn test_performance_scaling() {
        // Test that performance scales reasonably with input size
        let scanner = create_scanner();
        let mut timings = Vec::new();

        for size in [100, 500, 1000, 2000].iter() {
            let content = create_test_content(*size, 0.05);
            let temp_dir = TempDir::new().unwrap();
            let file_path = temp_dir.path().join(format!("scaling_{}.rs", size));
            fs::write(&file_path, &content).unwrap();

            let start = Instant::now();
            let _matches = scanner.scan(temp_dir.path()).unwrap();
            let duration = start.elapsed();

            timings.push((*size, duration));
            println!("Size {}: {:?}", size, duration);
        }

        // Performance should scale sub-quadratically
        // (This is a simple heuristic - real performance analysis would be more sophisticated)
        for i in 1..timings.len() {
            let (prev_size, prev_time) = timings[i - 1];
            let (curr_size, curr_time) = timings[i];

            let size_ratio = curr_size as f64 / prev_size as f64;
            let time_ratio = curr_time.as_nanos() as f64 / prev_time.as_nanos() as f64;

            // Time should not grow faster than size^2 (quadratic)
            assert!(
                time_ratio <= size_ratio * size_ratio * 2.0, // Allow 2x buffer for variance
                "Performance degraded: {}x size increase led to {}x time increase",
                size_ratio,
                time_ratio
            );
        }
    }
}
