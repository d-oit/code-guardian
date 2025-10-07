use anyhow::Result;
use code_guardian_core::{
    config::load_config, CustomDetectorManager, DistributedCoordinator, IncrementalScanner,
    OptimizedScanner, Scanner, StreamingScanner, WorkerConfig,
};
use code_guardian_output::formatters::Formatter;
use code_guardian_storage::{Scan, ScanRepository, SqliteScanRepository};
use indicatif::ProgressBar;
use std::path::PathBuf;

use crate::utils::get_detectors_from_profile;

#[derive(Debug)]
pub struct ScanOptions {
    pub path: PathBuf,
    pub db: Option<PathBuf>,
    pub config_path: Option<PathBuf>,
    pub profile: String,
    pub show_progress: bool,
    pub optimize: bool,
    pub streaming: bool,
    pub show_metrics: bool,
    pub incremental: bool,
    pub distributed: bool,
    pub custom_detectors: Option<PathBuf>,
    pub cache_size: Option<usize>,
    pub batch_size: Option<usize>,
    pub max_file_size: Option<usize>,
    pub max_threads: Option<usize>,
}

pub fn handle_scan(options: ScanOptions) -> Result<()> {
    if !options.path.exists() {
        return Err(anyhow::anyhow!(
            "Path '{}' does not exist",
            options.path.display()
        ));
    }
    if !options.path.is_dir() {
        return Err(anyhow::anyhow!(
            "Path '{}' is not a directory",
            options.path.display()
        ));
    }
    let mut config = load_config(options.config_path)?;
    // Override config with CLI args if provided
    if let Some(val) = options.cache_size {
        config.cache_size = val;
    }
    if let Some(val) = options.batch_size {
        config.batch_size = val;
    }
    if let Some(val) = options.max_file_size {
        config.max_file_size = val;
    }
    if let Some(val) = options.max_threads {
        config.max_threads = val;
    }
    let db_path = options
        .db
        .unwrap_or_else(|| PathBuf::from(&config.database_path));
    let mut repo = SqliteScanRepository::new(&db_path)?;

    // Load custom detectors if specified
    let mut custom_detector_manager = CustomDetectorManager::new();
    if let Some(custom_path) = options.custom_detectors {
        custom_detector_manager.load_from_file(&custom_path)?;
        println!("📁 Loaded custom detectors from {}", custom_path.display());
    }

    // Create scanner based on profile
    let mut detectors = get_detectors_from_profile(&options.profile);

    // Add custom detectors
    let custom_detectors_vec = custom_detector_manager.get_detectors();
    if !custom_detectors_vec.is_empty() {
        detectors.extend(custom_detectors_vec);
        println!(
            "🔧 Added {} custom detectors",
            detectors.len() - get_detectors_from_profile(&options.profile).len()
        );
    }

    let pb = if options.show_progress {
        let pb = ProgressBar::new_spinner();
        pb.set_message("Scanning directory for patterns...");
        Some(pb)
    } else {
        None
    };

    let (matches, scan_metrics) = if options.incremental {
        // Use incremental scanning
        if let Some(pb) = &pb {
            pb.set_message("Incremental scanning (only changed files)...");
        }

        let state_file = db_path.with_extension("incremental");
        let mut incremental_scanner = IncrementalScanner::new(detectors, state_file)?;
        let (matches, result) = incremental_scanner.scan_incremental(&options.path)?;

        // Convert incremental result to scan metrics
        let metrics = code_guardian_core::ScanMetrics {
            total_files_scanned: result.files_scanned,
            total_lines_processed: 0, // Not tracked in incremental
            total_matches_found: result.total_matches,
            scan_duration_ms: result.scan_duration_ms,
            cache_hits: result.files_skipped,
            cache_misses: result.files_scanned,
        };

        (matches, Some(metrics))
    } else if options.distributed {
        // Use distributed scanning
        if let Some(pb) = &pb {
            pb.set_message("Distributed scanning across multiple workers...");
        }

        let mut coordinator = DistributedCoordinator::new();

        // Register simulated workers
        for i in 0..4 {
            let worker_config = WorkerConfig {
                worker_id: format!("worker_{}", i),
                max_concurrent_units: 2,
                supported_detectors: vec!["TODO".to_string(), "FIXME".to_string()],
                cpu_cores: 2,
                memory_limit_mb: 1024,
                endpoint: None,
            };
            coordinator.register_worker(worker_config);
        }

        // Register detectors with coordinator
        for (i, detector) in detectors.into_iter().enumerate() {
            coordinator.register_detector(format!("detector_{}", i), detector);
        }

        // Collect files
        let files: Vec<PathBuf> = ignore::WalkBuilder::new(&options.path)
            .build()
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    if e.file_type()?.is_file() {
                        Some(e.path().to_path_buf())
                    } else {
                        None
                    }
                })
            })
            .collect();

        coordinator.create_work_units(files, config.batch_size)?;
        let matches = coordinator.execute_distributed_scan()?;

        // Create basic metrics
        let metrics = code_guardian_core::ScanMetrics {
            total_files_scanned: coordinator.get_statistics().total_files_processed,
            total_lines_processed: 0,
            total_matches_found: matches.len(),
            scan_duration_ms: 100, // Placeholder
            cache_hits: 0,
            cache_misses: 0,
        };

        (matches, Some(metrics))
    } else if options.streaming {
        // Use streaming scanner for large codebases
        if let Some(pb) = &pb {
            pb.set_message("Streaming scan of large codebase...");
        }

        let streaming_scanner = StreamingScanner::new(detectors);
        let mut all_matches = Vec::new();

        let metrics = streaming_scanner.scan_streaming(&options.path, |batch_matches| {
            all_matches.extend(batch_matches);
            Ok(())
        })?;

        (all_matches, Some(metrics))
    } else if options.optimize {
        // Use optimized scanner
        if let Some(pb) = &pb {
            pb.set_message("Optimized scanning with caching...");
        }

        let optimized_scanner = OptimizedScanner::new(detectors).with_cache_size(config.cache_size);
        let (matches, metrics) = optimized_scanner.scan_optimized(&options.path)?;
        (matches, Some(metrics))
    } else {
        // Use standard scanner
        if let Some(pb) = &pb {
            pb.set_message("Scanning directory for patterns...");
        }

        let scanner = Scanner::new(detectors);
        let matches = scanner.scan(&options.path)?;
        (matches, None)
    };

    if let Some(pb) = pb {
        pb.finish_with_message("Scan completed.");
    }
    let timestamp = chrono::Utc::now().timestamp();
    let scan = Scan {
        id: None,
        timestamp,
        root_path: options.path.to_string_lossy().to_string(),
        matches: matches.clone(),
    };
    let id = repo.save_scan(&scan)?;
    println!("Scan saved with ID: {}", id);

    // Show performance metrics if requested
    if options.show_metrics {
        if let Some(metrics) = scan_metrics {
            println!("\n📊 Performance Metrics:");
            println!("   Files scanned: {}", metrics.total_files_scanned);
            println!("   Lines processed: {}", metrics.total_lines_processed);
            println!("   Matches found: {}", metrics.total_matches_found);
            println!("   Scan duration: {}ms", metrics.scan_duration_ms);

            if metrics.cache_hits > 0 || metrics.cache_misses > 0 {
                let hit_rate =
                    metrics.cache_hits as f64 / (metrics.cache_hits + metrics.cache_misses) as f64;
                println!("   Cache hit rate: {:.1}%", hit_rate * 100.0);
            }

            let files_per_sec =
                metrics.total_files_scanned as f64 / (metrics.scan_duration_ms as f64 / 1000.0);
            let lines_per_sec =
                metrics.total_lines_processed as f64 / (metrics.scan_duration_ms as f64 / 1000.0);
            println!(
                "   Performance: {:.1} files/sec, {:.1} lines/sec",
                files_per_sec, lines_per_sec
            );
        }
        println!();
    }

    let formatter = code_guardian_output::formatters::TextFormatter;
    println!("{}", formatter.format(&matches));
    Ok(())
}
