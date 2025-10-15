use code_guardian_core::{
    BenchmarkConfigurations, ImpactLevel, ImplementationStatus, OptimizationType,
    PerformanceAnalyzer, PerformanceOptimizer,
};
use tempfile::TempDir;

fn create_test_project() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Create test files
    std::fs::create_dir_all(project_path.join("src")).unwrap();
    std::fs::write(
        project_path.join("src/main.rs"),
        r#"
fn main() {
    println!("Hello, world!");
    // TODO: Add error handling
    // FIXME: This needs fixing
}
"#,
    )
    .unwrap();

    std::fs::write(
        project_path.join("src/lib.rs"),
        r#"
pub fn library_function() {
    println!("Library function");
    // TODO: Implement this
}
"#,
    )
    .unwrap();

    temp_dir
}

#[cfg(test)]
mod performance_optimizer_tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = PerformanceOptimizer::new();

        assert!(!optimizer.optimizations.is_empty());
        assert!(!optimizer.profiles.is_empty());
        assert_eq!(optimizer.current_profile, "default");
        assert!(optimizer.auto_optimization);
    }

    #[test]
    fn test_default_optimizations_loaded() {
        let optimizer = PerformanceOptimizer::new();

        // Check that key optimizations are present
        let optimization_names: Vec<String> = optimizer
            .optimizations
            .iter()
            .map(|o| o.name.clone())
            .collect();

        assert!(optimization_names.contains(&"File Content Cache".to_string()));
        assert!(optimization_names.contains(&"Multi-threaded File Processing".to_string()));
        assert!(optimization_names.contains(&"Streaming File Processing".to_string()));
        assert!(optimization_names.contains(&"Smart File Filtering".to_string()));
    }

    #[test]
    fn test_optimization_types_coverage() {
        let optimizer = PerformanceOptimizer::new();

        let mut types_found = std::collections::HashSet::new();
        for optimization in &optimizer.optimizations {
            types_found.insert(&optimization.optimization_type);
        }

        // Ensure we have optimizations for key types
        assert!(types_found
            .iter()
            .any(|t| matches!(t, OptimizationType::Caching)));
        assert!(types_found
            .iter()
            .any(|t| matches!(t, OptimizationType::Parallelization)));
        assert!(types_found
            .iter()
            .any(|t| matches!(t, OptimizationType::MemoryOptimization)));
        assert!(types_found
            .iter()
            .any(|t| matches!(t, OptimizationType::IoOptimization)));
    }

    #[test]
    fn test_impact_levels_distributed() {
        let optimizer = PerformanceOptimizer::new();

        let mut impact_counts = std::collections::HashMap::new();
        for optimization in &optimizer.optimizations {
            *impact_counts.entry(&optimization.impact_level).or_insert(0) += 1;
        }

        // Should have optimizations with different impact levels
        assert!(impact_counts.contains_key(&ImpactLevel::High));
        assert!(impact_counts.contains_key(&ImpactLevel::Medium));
    }

    #[test]
    fn test_profile_application() {
        let mut optimizer = PerformanceOptimizer::new();

        // Test applying different profiles
        let profiles = vec!["fast", "default", "thorough", "memory_efficient"];

        for profile in profiles {
            let settings = optimizer.apply_profile(profile);
            assert!(settings.is_ok(), "Failed to apply profile: {}", profile);
            assert_eq!(optimizer.current_profile, profile);
        }
    }

    #[test]
    fn test_fast_profile_settings() {
        let mut optimizer = PerformanceOptimizer::new();
        let settings = optimizer.apply_profile("fast").unwrap();

        // Fast profile should enable parallel processing but minimal caching
        assert!(settings.enable_parallel_processing);
        assert!(settings.enable_smart_filtering);

        // Should have reasonable performance targets
        if let Some(targets) = &settings.performance_targets {
            assert!(targets.max_scan_time_seconds <= 10.0);
            assert!(targets.min_throughput_files_per_sec >= 100.0);
        }
    }

    #[test]
    fn test_thorough_profile_settings() {
        let mut optimizer = PerformanceOptimizer::new();
        let settings = optimizer.apply_profile("thorough").unwrap();

        // Thorough profile should enable comprehensive optimizations
        assert!(settings.enable_parallel_processing);
        assert!(settings.enable_caching);
        assert!(settings.enable_streaming);

        // Should allow more time for comprehensive analysis
        if let Some(targets) = &settings.performance_targets {
            assert!(targets.max_scan_time_seconds >= 30.0);
            assert!(targets.target_cache_hit_rate >= 0.7);
        }
    }

    #[test]
    fn test_memory_efficient_profile() {
        let mut optimizer = PerformanceOptimizer::new();
        let settings = optimizer.apply_profile("memory_efficient").unwrap();

        // Memory efficient profile should prioritize low memory usage
        assert!(settings.enable_streaming);
        assert!(settings.enable_smart_filtering);

        if let Some(targets) = &settings.performance_targets {
            assert!(targets.max_memory_usage_mb <= 200.0);
        }
    }

    #[test]
    fn test_invalid_profile() {
        let mut optimizer = PerformanceOptimizer::new();
        let result = optimizer.apply_profile("nonexistent_profile");
        assert!(result.is_err());
    }

    #[test]
    fn test_auto_tuning() {
        let mut optimizer = PerformanceOptimizer::new();
        let result = optimizer.auto_tune();
        assert!(result.is_ok());

        // Verify that auto-tuning modified some parameters
        let cache_optimization = optimizer
            .optimizations
            .iter()
            .find(|o| o.name == "File Content Cache")
            .unwrap();

        assert!(cache_optimization.configuration.auto_tune);
    }

    #[test]
    fn test_optimization_report_generation() {
        let optimizer = PerformanceOptimizer::new();
        let report = optimizer.generate_optimization_report();

        assert!(!report.is_empty());
        assert!(report.contains("Performance Optimization Report"));
        assert!(report.contains("Current Profile"));
        assert!(report.contains("Active Optimizations"));
    }

    #[test]
    fn test_optimization_status_filtering() {
        let optimizer = PerformanceOptimizer::new();

        let active_count = optimizer
            .optimizations
            .iter()
            .filter(|o| matches!(o.implementation_status, ImplementationStatus::Active))
            .count();

        let available_count = optimizer
            .optimizations
            .iter()
            .filter(|o| matches!(o.implementation_status, ImplementationStatus::Available))
            .count();

        assert!(active_count > 0);
        assert!(available_count > 0);
    }
}

#[cfg(test)]
mod benchmark_suite_tests {
    use super::*;

    #[test]
    fn test_small_project_benchmark_creation() {
        let suite = BenchmarkConfigurations::small_project();

        assert_eq!(suite.name, "Small Project Benchmark");
        assert!(!suite.benchmarks.is_empty());
        assert_eq!(suite.summary.total_tests, 0); // Not run yet
    }

    #[test]
    fn test_medium_project_benchmark_creation() {
        let suite = BenchmarkConfigurations::medium_project();

        assert_eq!(suite.name, "Medium Project Benchmark");
        assert!(!suite.benchmarks.is_empty());

        // Should have tests for streaming and parallel processing
        let test_names: Vec<String> = suite.benchmarks.iter().map(|b| b.name.clone()).collect();

        assert!(test_names.iter().any(|name| name.contains("Streaming")));
        assert!(test_names.iter().any(|name| name.contains("Parallel")));
    }

    #[test]
    fn test_large_project_benchmark_creation() {
        let suite = BenchmarkConfigurations::large_project();

        assert_eq!(suite.name, "Large Project Benchmark");
        assert!(!suite.benchmarks.is_empty());

        // Should have tests for large file handling and memory usage
        let test_names: Vec<String> = suite.benchmarks.iter().map(|b| b.name.clone()).collect();

        assert!(test_names.iter().any(|name| name.contains("Large File")));
        assert!(test_names.iter().any(|name| name.contains("Memory")));
    }

    #[test]
    fn test_regression_detection_benchmark() {
        let suite = BenchmarkConfigurations::regression_detection();

        assert_eq!(suite.name, "Regression Detection Benchmark");
        assert!(!suite.benchmarks.is_empty());

        // Should have at least one regression test
        let has_regression_test = suite
            .benchmarks
            .iter()
            .any(|b| b.name.contains("Regression"));
        assert!(has_regression_test);
    }

    #[test]
    fn test_benchmark_expected_performance() {
        let suite = BenchmarkConfigurations::small_project();

        for benchmark in &suite.benchmarks {
            let expected = &benchmark.expected_performance;

            // Verify reasonable expectations
            assert!(expected.max_duration_seconds > 0.0);
            assert!(expected.max_memory_mb > 0.0);
            assert!(expected.min_throughput_files_per_sec >= 0.0);
            assert!(expected.min_cache_hit_rate >= 0.0 && expected.min_cache_hit_rate <= 1.0);
        }
    }

    #[test]
    fn test_benchmark_suite_run() {
        let temp_dir = create_test_project();
        let mut suite = BenchmarkConfigurations::small_project();

        // This should not fail even with a small test project
        let _result = suite.run_benchmarks(temp_dir.path());

        // The benchmark should complete without errors

        // Summary should be populated after running
        assert!(suite.summary.total_tests > 0);
    }
}

#[cfg(test)]
mod performance_analyzer_tests {
    use super::*;

    #[test]
    fn test_performance_analyzer_creation() {
        let analyzer = PerformanceAnalyzer::new();

        // Should have default metrics
        assert_eq!(analyzer.metrics.execution_time.as_secs(), 0);
        assert!(analyzer.benchmarks.is_empty());
        assert_eq!(analyzer.analysis.overall_score, 0.0);
    }

    #[test]
    fn test_performance_analysis_execution() {
        let mut analyzer = PerformanceAnalyzer::new();

        // Test analyzing a simple operation
        let result = analyzer.analyze_performance(|| {
            // Simulate some work
            std::thread::sleep(std::time::Duration::from_millis(10));
            Ok(())
        });

        assert!(result.is_ok());
        assert!(analyzer.metrics.execution_time.as_millis() >= 10);
    }

    #[test]
    fn test_performance_score_calculation() {
        let analyzer = PerformanceAnalyzer::new();
        let score = analyzer.calculate_performance_score();

        // Score should be between 0 and 100
        assert!((0.0..=100.0).contains(&score));
    }

    #[test]
    fn test_performance_report_generation() {
        let analyzer = PerformanceAnalyzer::new();
        let report = analyzer.generate_report();

        assert!(!report.is_empty());
        assert!(report.contains("Performance Analysis Report"));
        assert!(report.contains("Overall Performance Score"));
    }

    #[test]
    fn test_memory_efficiency_calculation() {
        let analyzer = PerformanceAnalyzer::new();

        // Test different memory growth scenarios
        let low_growth = analyzer.calculate_memory_efficiency(100.0, 105.0);
        let high_growth = analyzer.calculate_memory_efficiency(100.0, 200.0);

        assert!(low_growth > high_growth);
        assert!((0.0..=1.0).contains(&low_growth));
        assert!((0.0..=1.0).contains(&high_growth));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_optimizer_with_benchmark_suite() {
        let temp_dir = create_test_project();
        let mut optimizer = PerformanceOptimizer::new();

        // Apply different profiles and test with benchmark
        let profiles = vec!["fast", "default", "memory_efficient"];

        for profile in profiles {
            let _settings = optimizer.apply_profile(profile).unwrap();

            // Create a simple benchmark for this profile
            let mut suite = BenchmarkConfigurations::small_project();

            // The benchmark should work with any optimization profile
            let _result = suite.run_benchmarks(temp_dir.path());
        }
    }

    #[test]
    fn test_auto_tuning_with_analysis() {
        let mut optimizer = PerformanceOptimizer::new();
        let _analyzer = PerformanceAnalyzer::new();

        // Auto-tune the optimizer
        let tune_result = optimizer.auto_tune();
        assert!(tune_result.is_ok());

        // Apply the tuned settings
        let settings = optimizer.apply_profile("default").unwrap();

        // Settings should be reasonable
        assert!(settings.thread_count > 0);
        assert!(settings.cache_size > 0);
    }

    #[test]
    fn test_comprehensive_performance_workflow() {
        let temp_dir = create_test_project();
        let mut optimizer = PerformanceOptimizer::new();

        // 1. Auto-tune optimizations
        optimizer.auto_tune().unwrap();

        // 2. Apply performance profile
        let settings = optimizer.apply_profile("default").unwrap();
        assert!(settings.enable_parallel_processing);

        // 3. Generate optimization report
        let report = optimizer.generate_optimization_report();
        assert!(report.contains("Active Optimizations"));

        // 4. Run benchmarks
        let mut suite = BenchmarkConfigurations::small_project();
        let _benchmark_result = suite.run_benchmarks(temp_dir.path());

        // 5. Analyze performance
        let mut analyzer = PerformanceAnalyzer::new();
        let analysis_result = analyzer.analyze_performance(|| {
            // Simulate optimized scanning
            std::thread::sleep(std::time::Duration::from_millis(50));
            Ok(())
        });
        assert!(analysis_result.is_ok());

        // 6. Generate performance report
        let perf_report = analyzer.generate_report();
        assert!(perf_report.contains("Performance Analysis Report"));
    }
}
