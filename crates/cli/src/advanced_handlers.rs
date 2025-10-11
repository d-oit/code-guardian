use anyhow::Result;
use code_guardian_core::{CustomDetectorManager, DistributedCoordinator, WorkerConfig};
use std::path::PathBuf;

use crate::{CustomDetectorAction, DistributedAction, IncrementalAction};

pub fn handle_custom_detectors(action: CustomDetectorAction) -> Result<()> {
    match action {
        CustomDetectorAction::List => {
            let manager = CustomDetectorManager::new();
            let detectors = manager.list_detectors();

            if detectors.is_empty() {
                println!("No custom detectors found. Use 'create-examples' to generate some.");
                return Ok(());
            }

            println!("📋 Custom Detectors:");
            for detector in detectors {
                println!("  🔍 {} ({})", detector.name, detector.description);
                println!("     Pattern: {}", detector.pattern);
                println!("     Severity: {:?}", detector.severity);
                println!("     Enabled: {}", detector.enabled);
                if !detector.file_extensions.is_empty() {
                    println!("     Extensions: {}", detector.file_extensions.join(", "));
                }
                println!();
            }
        }

        CustomDetectorAction::CreateExamples { output } => {
            let mut manager = CustomDetectorManager::new();
            manager.create_examples()?;
            manager.save_to_file(&output)?;
            println!(
                "✅ Created example custom detectors in {}",
                output.display()
            );
        }

        CustomDetectorAction::Load { file } => {
            let mut manager = CustomDetectorManager::new();
            manager.load_from_file(&file)?;

            let detectors = manager.list_detectors();
            println!(
                "✅ Loaded {} custom detectors from {}",
                detectors.len(),
                file.display()
            );

            for detector in detectors {
                println!(
                    "  - {} ({})",
                    detector.name,
                    if detector.enabled {
                        "enabled"
                    } else {
                        "disabled"
                    }
                );
            }
        }

        CustomDetectorAction::Test {
            detectors,
            test_file,
        } => {
            let mut manager = CustomDetectorManager::new();
            manager.load_from_file(&detectors)?;

            let content = std::fs::read_to_string(&test_file)?;
            let detector_instances = manager.get_detectors();

            println!("🧪 Testing custom detectors on {}", test_file.display());

            let mut total_matches = 0;
            for detector in detector_instances {
                let matches = detector.detect(&content, &test_file);
                if !matches.is_empty() {
                    println!("  Found {} matches:", matches.len());
                    for mat in &matches {
                        println!("    {}:{} - {}", mat.line_number, mat.column, mat.message);
                    }
                    total_matches += matches.len();
                }
            }

            if total_matches == 0 {
                println!("  ✅ No matches found");
            } else {
                println!("  📊 Total matches: {}", total_matches);
            }
        }
    }

    Ok(())
}

pub fn handle_incremental(action: IncrementalAction) -> Result<()> {
    let state_file = PathBuf::from("code-guardian.incremental");

    match action {
        IncrementalAction::Status => {
            if !state_file.exists() {
                println!("❌ No incremental scan state found.");
                println!("   Run a scan with --incremental to create state.");
                return Ok(());
            }

            // Load state and show status
            println!("📊 Incremental Scan Status:");
            println!("   State file: {}", state_file.display());
            println!(
                "   State file size: {} bytes",
                std::fs::metadata(&state_file)?.len()
            );

            // Try to load and show basic stats
            if let Ok(content) = std::fs::read_to_string(&state_file) {
                if let Ok(state) =
                    serde_json::from_str::<code_guardian_core::IncrementalState>(&content)
                {
                    println!("   Tracked files: {}", state.file_metadata.len());
                    println!("   Scan history: {} entries", state.scan_history.len());

                    if let Some(last_scan) = state.scan_history.last() {
                        println!("   Last scan:");
                        println!("     Files scanned: {}", last_scan.files_scanned);
                        println!("     Files skipped: {}", last_scan.files_skipped);
                        println!("     Duration: {}ms", last_scan.scan_duration_ms);
                    }
                }
            }
        }

        IncrementalAction::Reset => {
            if state_file.exists() {
                std::fs::remove_file(&state_file)?;
                println!("✅ Incremental scan state reset.");
                println!("   Next scan will be a full scan.");
            } else {
                println!("❌ No incremental state to reset.");
            }
        }

        IncrementalAction::Stats => {
            if !state_file.exists() {
                println!("❌ No incremental scan state found.");
                return Ok(());
            }

            let content = std::fs::read_to_string(&state_file)?;
            let state: code_guardian_core::IncrementalState = serde_json::from_str(&content)?;

            println!("📈 Incremental Scan Statistics:");
            println!("   Total tracked files: {}", state.file_metadata.len());
            println!("   Scan history entries: {}", state.scan_history.len());

            if !state.scan_history.is_empty() {
                let recent_scans = state.scan_history.iter().rev().take(5);
                println!("   Recent scans:");

                for (i, scan) in recent_scans.enumerate() {
                    let timestamp = chrono::DateTime::from_timestamp(scan.timestamp as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "Unknown".to_string());

                    println!(
                        "     {}. {} - {} files scanned, {} skipped ({}ms)",
                        i + 1,
                        timestamp,
                        scan.files_scanned,
                        scan.files_skipped,
                        scan.scan_duration_ms
                    );
                }

                // Calculate average speedup
                let total_scanned: usize = state.scan_history.iter().map(|s| s.files_scanned).sum();
                let total_skipped: usize = state.scan_history.iter().map(|s| s.files_skipped).sum();
                let total_files = total_scanned + total_skipped;

                if total_files > 0 {
                    let average_speedup = total_files as f64 / total_scanned.max(1) as f64;
                    println!("   Average speedup: {:.2}x", average_speedup);
                    println!(
                        "   Cache hit rate: {:.1}%",
                        (total_skipped as f64 / total_files as f64) * 100.0
                    );
                }
            }
        }
    }

    Ok(())
}

pub async fn handle_distributed(action: DistributedAction) -> Result<()> {
    match action {
        DistributedAction::Setup { workers } => {
            println!(
                "🚀 Setting up distributed scanning with {} workers",
                workers
            );

            let mut coordinator = DistributedCoordinator::new();

            for i in 0..workers {
                let worker_config = WorkerConfig {
                    worker_id: format!("worker_{}", i),
                    max_concurrent_units: 4,
                    supported_detectors: vec![
                        "TODO".to_string(),
                        "FIXME".to_string(),
                        "HACK".to_string(),
                        "BUG".to_string(),
                    ],
                    cpu_cores: 4,
                    memory_limit_mb: 2048,
                    endpoint: Some(format!("worker-{}.local:8080", i)),
                };

                coordinator.register_worker(worker_config);
            }

            println!("✅ Distributed setup complete!");
            println!("   Workers: {}", workers);
            println!(
                "   Total capacity: {} cores, {}MB memory",
                workers * 4,
                workers * 2048
            );

            println!("\n💡 To run a distributed scan:");
            println!(
                "   code-guardian distributed scan <path> --workers {}",
                workers
            );
        }

        DistributedAction::Scan {
            path,
            workers,
            batch_size,
        } => {
            println!("🌐 Running distributed scan on {}", path.display());
            println!("   Workers: {}, Batch size: {}", workers, batch_size);

            let mut coordinator = DistributedCoordinator::new();

            // Register workers
            for i in 0..workers {
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

            // Register basic detectors
            coordinator.register_detector(
                "TODO".to_string(),
                Box::new(code_guardian_core::TodoDetector),
            );
            coordinator.register_detector(
                "FIXME".to_string(),
                Box::new(code_guardian_core::FixmeDetector),
            );

            // Collect files
            let files: Vec<PathBuf> = ignore::WalkBuilder::new(&path)
                .build()
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        if e.file_type().is_some_and(|ft| ft.is_file()) {
                            Some(e.path().to_path_buf())
                        } else {
                            None
                        }
                    })
                })
                .collect();

            coordinator.create_work_units(files, batch_size)?;
            let matches = coordinator.execute_distributed_scan().await?;

            let stats = coordinator.get_statistics();

            println!("✅ Distributed scan complete!");
            println!("   Total matches: {}", matches.len());
            println!("   Files processed: {}", stats.total_files_processed);
            println!("   Work units: {}", stats.total_work_units);
            println!("   Processing time: {}ms", stats.total_processing_time_ms);

            // Show top matches
            if !matches.is_empty() {
                println!("\n🔍 Sample matches:");
                for (i, mat) in matches.iter().take(5).enumerate() {
                    println!(
                        "   {}. {}:{} - {}",
                        i + 1,
                        mat.line_number,
                        mat.column,
                        mat.message
                    );
                }

                if matches.len() > 5 {
                    println!("   ... and {} more", matches.len() - 5);
                }
            }
        }
    }

    Ok(())
}
