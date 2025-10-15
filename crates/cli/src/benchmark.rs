use anyhow::Result;
use code_guardian_core::{
    BenchmarkConfigurations, BenchmarkSuite, DetectorFactory, DetectorProfile, OptimizedScanner,
    PerformanceAnalyzer, Scanner, StreamingScanner,
};
use std::path::Path;
use std::time::Instant;

/// Run performance benchmarks on different scanner types
pub fn run_benchmark(path: &Path) -> Result<()> {
    println!("🚀 Code-Guardian Performance Benchmark");
    println!("=====================================\n");

    println!("📁 Scanning path: {}", path.display());
    println!("🔍 Testing different scanner configurations...\n");

    // Test basic scanner
    println!("1️⃣ Basic Scanner (TODO + FIXME only)");
    let start = Instant::now();
    let basic_scanner = Scanner::new(DetectorFactory::create_default_detectors());
    let basic_matches = basic_scanner.scan(path)?;
    let basic_duration = start.elapsed();
    println!("   ⏱️  Duration: {:?}", basic_duration);
    println!("   📊 Matches found: {}", basic_matches.len());
    println!();

    // Test comprehensive scanner
    println!("2️⃣ Comprehensive Scanner (All detectors)");
    let start = Instant::now();
    let comprehensive_scanner = Scanner::new(DetectorProfile::Comprehensive.get_detectors());
    let comprehensive_matches = comprehensive_scanner.scan(path)?;
    let comprehensive_duration = start.elapsed();
    println!("   ⏱️  Duration: {:?}", comprehensive_duration);
    println!("   📊 Matches found: {}", comprehensive_matches.len());
    println!();

    // Test optimized scanner
    println!("3️⃣ Optimized Scanner (With caching)");
    let start = Instant::now();
    let optimized_scanner = OptimizedScanner::new(DetectorProfile::Comprehensive.get_detectors())
        .with_cache_size(10000);
    let (optimized_matches, optimized_metrics) = optimized_scanner.scan_optimized(path)?;
    let optimized_duration = start.elapsed();
    println!("   ⏱️  Duration: {:?}", optimized_duration);
    println!("   📊 Matches found: {}", optimized_matches.len());
    println!(
        "   📈 Files scanned: {}",
        optimized_metrics.total_files_scanned
    );
    println!(
        "   📈 Lines processed: {}",
        optimized_metrics.total_lines_processed
    );
    println!("   🎯 Cache hits: {}", optimized_metrics.cache_hits);
    println!("   🎯 Cache misses: {}", optimized_metrics.cache_misses);
    println!();

    // Test streaming scanner
    println!("4️⃣ Streaming Scanner (Memory efficient)");
    let start = Instant::now();
    let streaming_scanner = StreamingScanner::new(DetectorProfile::Comprehensive.get_detectors());
    let mut streaming_matches = Vec::new();
    let streaming_metrics = streaming_scanner.scan_streaming(path, |batch| {
        streaming_matches.extend(batch);
        Ok(())
    })?;
    let streaming_duration = start.elapsed();
    println!("   ⏱️  Duration: {:?}", streaming_duration);
    println!("   📊 Matches found: {}", streaming_matches.len());
    println!(
        "   📈 Files scanned: {}",
        streaming_metrics.total_files_scanned
    );
    println!(
        "   📈 Lines processed: {}",
        streaming_metrics.total_lines_processed
    );
    println!();

    // Performance comparison
    println!("📊 Performance Comparison");
    println!("========================");

    let basic_files_per_sec =
        optimized_metrics.total_files_scanned as f64 / basic_duration.as_secs_f64();
    let comprehensive_files_per_sec =
        optimized_metrics.total_files_scanned as f64 / comprehensive_duration.as_secs_f64();
    let optimized_files_per_sec =
        optimized_metrics.total_files_scanned as f64 / optimized_duration.as_secs_f64();
    let streaming_files_per_sec =
        streaming_metrics.total_files_scanned as f64 / streaming_duration.as_secs_f64();

    println!("📈 Files per second:");
    println!("   Basic:        {:.1}", basic_files_per_sec);
    println!("   Comprehensive: {:.1}", comprehensive_files_per_sec);
    println!("   Optimized:    {:.1}", optimized_files_per_sec);
    println!("   Streaming:    {:.1}", streaming_files_per_sec);
    println!();

    println!("🎯 Speed improvements:");
    let optimized_speedup = optimized_files_per_sec / comprehensive_files_per_sec;
    let streaming_speedup = streaming_files_per_sec / comprehensive_files_per_sec;
    println!("   Optimized vs Comprehensive: {:.2}x", optimized_speedup);
    println!("   Streaming vs Comprehensive: {:.2}x", streaming_speedup);
    println!();

    println!("💡 Recommendations:");
    if optimized_speedup > 1.2 {
        println!("   ✅ Use --optimize flag for better performance");
    }
    if streaming_speedup > 1.1 {
        println!("   ✅ Use --streaming flag for large codebases");
    }
    if optimized_metrics.cache_hits > 0 {
        println!("   ✅ Caching is effective for repeated scans");
    }

    println!();
    println!("🏁 Benchmark completed!");

    Ok(())
}

/// Run comprehensive benchmark suite
#[allow(dead_code)]
pub fn run_comprehensive_benchmark(path: &Path, suite_type: &str) -> Result<()> {
    println!("🚀 Code-Guardian Comprehensive Benchmark");
    println!("=========================================\n");

    let mut suite = match suite_type {
        "small" => BenchmarkConfigurations::small_project(),
        "medium" => BenchmarkConfigurations::medium_project(),
        "large" => BenchmarkConfigurations::large_project(),
        "regression" => BenchmarkConfigurations::regression_detection(),
        "all" => {
            // Run all benchmark suites
            run_all_benchmark_suites(path)?;
            return Ok(());
        }
        _ => {
            println!("❌ Unknown benchmark suite: {}", suite_type);
            println!("Available suites: small, medium, large, regression, all");
            return Ok(());
        }
    };

    suite.run_benchmarks(path)?;

    // Generate detailed report
    generate_benchmark_report(&suite)?;

    Ok(())
}

/// Run all benchmark suites
#[allow(dead_code)]
pub fn run_all_benchmark_suites(path: &Path) -> Result<()> {
    println!("🏃‍♂️ Running All Benchmark Suites");
    println!("==================================\n");

    let suites = vec![
        ("Small Project", BenchmarkConfigurations::small_project()),
        ("Medium Project", BenchmarkConfigurations::medium_project()),
        ("Large Project", BenchmarkConfigurations::large_project()),
        (
            "Regression Detection",
            BenchmarkConfigurations::regression_detection(),
        ),
    ];

    let mut all_results = Vec::new();
    let mut total_passed = 0;
    let mut total_tests = 0;

    for (name, mut suite) in suites {
        println!("🔄 Running {} Suite...", name);
        suite.run_benchmarks(path)?;

        total_passed += suite.summary.passed_tests;
        total_tests += suite.summary.total_tests;
        all_results.push((name, suite));
    }

    // Overall summary
    println!("\n🏁 Overall Benchmark Results");
    println!("=============================");
    println!("Total Suites: {}", all_results.len());
    println!("Total Tests: {}", total_tests);
    println!("Total Passed: ✅ {}", total_passed);
    println!("Total Failed: ❌ {}", total_tests - total_passed);

    let success_rate = (total_passed as f64 / total_tests as f64) * 100.0;
    println!("Success Rate: {:.1}%", success_rate);

    if success_rate >= 90.0 {
        println!("🎉 Excellent performance across all benchmarks!");
    } else if success_rate >= 70.0 {
        println!("✅ Good performance with some areas for improvement");
    } else {
        println!("⚠️ Performance issues detected - optimization recommended");
    }

    Ok(())
}

/// Run performance analysis with detailed metrics
#[allow(dead_code)]
pub fn run_performance_analysis(path: &Path) -> Result<()> {
    println!("🔍 Performance Analysis");
    println!("======================\n");

    let mut analyzer = PerformanceAnalyzer::new();

    analyzer.analyze_performance(|| {
        // Run a comprehensive scan for analysis
        let scanner = OptimizedScanner::new(DetectorProfile::Comprehensive.get_detectors())
            .with_cache_size(10000);
        let (_matches, _metrics) = scanner.scan_optimized(path)?;
        Ok(())
    })?;

    // Generate and display performance report
    let report = analyzer.generate_report();
    println!("{}", report);

    // Save analysis to file
    save_performance_analysis(&analyzer, path)?;

    Ok(())
}

/// Generate comprehensive benchmark report
#[allow(dead_code)]
fn generate_benchmark_report(suite: &BenchmarkSuite) -> Result<()> {
    println!("\n📊 Detailed Benchmark Report");
    println!("============================");

    println!("Suite: {}", suite.name);
    println!(
        "Performance Score: {:.1}/100",
        suite.summary.performance_score
    );

    if !suite.results.is_empty() {
        println!("\n📈 Individual Test Results:");
        for result in &suite.results {
            println!("\n🔍 {}", result.name);
            println!("   Duration: {:?}", result.duration);
            println!("   Files/sec: {:.1}", result.throughput.files_per_second);
            println!("   Lines/sec: {:.0}", result.throughput.lines_per_second);
            println!("   Memory: {:.1} MB", result.resource_usage.memory_mb);
            println!("   Score: {:.1}/100", result.performance_score);
        }
    }

    if !suite.summary.improvement_areas.is_empty() {
        println!("\n💡 Recommendations:");
        for area in &suite.summary.improvement_areas {
            println!("   • {}", area);
        }
    }

    // Save report to file
    save_benchmark_report(suite)?;

    Ok(())
}

/// Save benchmark report to file
#[allow(dead_code)]
fn save_benchmark_report(suite: &BenchmarkSuite) -> Result<()> {
    use std::fs;
    use std::path::PathBuf;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!(
        "benchmark_report_{}_{}.json",
        suite.name.replace(" ", "_").to_lowercase(),
        timestamp
    );

    let reports_dir = PathBuf::from("reports");
    fs::create_dir_all(&reports_dir)?;

    let report_path = reports_dir.join(filename);
    let json_report = serde_json::to_string_pretty(suite)?;
    fs::write(&report_path, json_report)?;

    println!("\n📄 Report saved to: {}", report_path.display());
    Ok(())
}

/// Save performance analysis to file
#[allow(dead_code)]
fn save_performance_analysis(analyzer: &PerformanceAnalyzer, _path: &Path) -> Result<()> {
    use std::fs;
    use std::path::PathBuf;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("performance_analysis_{}.json", timestamp);

    let reports_dir = PathBuf::from("reports");
    fs::create_dir_all(&reports_dir)?;

    let report_path = reports_dir.join(filename);
    let json_analysis = serde_json::to_string_pretty(analyzer)?;
    fs::write(&report_path, json_analysis)?;

    println!("\n📄 Analysis saved to: {}", report_path.display());
    Ok(())
}

/// Quick performance test
pub fn quick_performance_test(path: &Path) -> Result<()> {
    println!("⚡ Quick Performance Test");
    println!("========================\n");

    let start = Instant::now();
    let scanner = OptimizedScanner::new(DetectorProfile::Basic.get_detectors());
    let (matches, metrics) = scanner.scan_optimized(path)?;
    let duration = start.elapsed();

    println!("📊 Results:");
    println!("   Duration: {:?}", duration);
    println!("   Files scanned: {}", metrics.total_files_scanned);
    println!("   Lines processed: {}", metrics.total_lines_processed);
    println!("   Matches found: {}", matches.len());
    println!(
        "   Files/sec: {:.1}",
        metrics.total_files_scanned as f64 / duration.as_secs_f64()
    );
    println!(
        "   Lines/sec: {:.1}",
        metrics.total_lines_processed as f64 / duration.as_secs_f64()
    );

    if metrics.cache_hits > 0 {
        let hit_rate =
            metrics.cache_hits as f64 / (metrics.cache_hits + metrics.cache_misses) as f64;
        println!("   Cache hit rate: {:.1}%", hit_rate * 100.0);
    }

    Ok(())
}
