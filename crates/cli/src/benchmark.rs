use anyhow::Result;
use code_guardian_core::{
    performance_optimized_scanner::PerformanceOptimizedScanner, DetectorFactory, DetectorProfile,
    OptimizedScanner, Scanner, StreamingScanner,
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

    // Test performance optimized scanner
    println!("5️⃣ Performance Optimized Scanner (SIMD + Advanced Caching)");
    let start = Instant::now();
    let perf_scanner =
        PerformanceOptimizedScanner::new(DetectorProfile::Comprehensive.get_detectors());
    let (perf_matches, perf_metrics) = perf_scanner.scan_ultra_fast(path)?;
    let perf_duration = start.elapsed();
    println!("   ⏱️  Duration: {:?}", perf_duration);
    println!("   📊 Matches found: {}", perf_matches.len());
    println!("   📈 Files scanned: {}", perf_metrics.total_files_scanned);
    println!(
        "   📈 Lines processed: {}",
        perf_metrics.total_lines_processed
    );
    println!("   🎯 Cache hits: {}", perf_metrics.cache_hits);
    println!("   ⚡ SIMD matches: {}", perf_metrics.simd_matches);
    println!("   🔍 Regex matches: {}", perf_metrics.regex_matches);
    println!("   📖 File read time: {}ms", perf_metrics.file_read_time_ms);
    println!(
        "   🔎 Pattern search time: {}ms",
        perf_metrics.pattern_search_time_ms
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
    let perf_files_per_sec = perf_metrics.total_files_scanned as f64 / perf_duration.as_secs_f64();

    println!("📈 Files per second:");
    println!("   Basic:          {:.1}", basic_files_per_sec);
    println!("   Comprehensive:  {:.1}", comprehensive_files_per_sec);
    println!("   Optimized:      {:.1}", optimized_files_per_sec);
    println!("   Streaming:      {:.1}", streaming_files_per_sec);
    println!("   Performance:    {:.1}", perf_files_per_sec);
    println!();

    println!("🎯 Speed improvements:");
    let optimized_speedup = optimized_files_per_sec / comprehensive_files_per_sec;
    let streaming_speedup = streaming_files_per_sec / comprehensive_files_per_sec;
    let perf_speedup = perf_files_per_sec / comprehensive_files_per_sec;
    println!(
        "   Optimized vs Comprehensive:    {:.2}x",
        optimized_speedup
    );
    println!(
        "   Streaming vs Comprehensive:    {:.2}x",
        streaming_speedup
    );
    println!("   Performance vs Comprehensive:  {:.2}x", perf_speedup);
    println!(
        "   Performance vs Optimized:      {:.2}x",
        perf_files_per_sec / optimized_files_per_sec
    );
    println!();

    println!("💡 Recommendations:");
    if perf_speedup > 2.0 {
        println!(
            "   🚀 Use --ultra-fast flag for maximum performance ({}x faster)",
            perf_speedup
        );
    } else if optimized_speedup > 1.2 {
        println!("   ✅ Use --optimize flag for better performance");
    }
    if streaming_speedup > 1.1 {
        println!("   ✅ Use --streaming flag for large codebases");
    }
    if perf_metrics.cache_hits > 0 {
        println!(
            "   ✅ Caching is highly effective ({}% hit rate)",
            perf_metrics.cache_hits * 100 / (perf_metrics.cache_hits + perf_metrics.cache_misses)
        );
    }
    if perf_metrics.simd_matches > 0 {
        println!(
            "   ⚡ SIMD optimization found {} patterns ultra-fast",
            perf_metrics.simd_matches
        );
    }

    println!();
    println!("🏁 Benchmark completed!");

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
