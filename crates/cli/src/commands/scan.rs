//! Scan command implementations

use anyhow::Result;
use std::path::PathBuf;
use code_guardian_core::{Scanner, ScanOptions as CoreScanOptions};
use crate::utils::{get_db_path, get_detectors_from_profile};

#[derive(Debug, Clone)]
pub struct ScanCommand {
    pub path: PathBuf,
    pub db: Option<PathBuf>,
    pub config_path: Option<PathBuf>,
    pub profile: Option<String>,
    pub show_progress: bool,
    pub optimize: bool,
    pub streaming: bool,
    pub show_metrics: bool,
    pub incremental: bool,
    pub distributed: bool,
    pub custom_detectors: Option<PathBuf>,
    pub cache_size: Option<usize>,
    pub batch_size: Option<usize>,
    pub max_file_size: Option<u64>,
    pub max_threads: Option<usize>,
}

impl ScanCommand {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            db: None,
            config_path: None,
            profile: None,
            show_progress: false,
            optimize: false,
            streaming: false,
            show_metrics: false,
            incremental: false,
            distributed: false,
            custom_detectors: None,
            cache_size: None,
            batch_size: None,
            max_file_size: None,
            max_threads: None,
        }
    }

    pub async fn execute(&self) -> Result<()> {
        let db_path = get_db_path(self.db.clone());
        let detectors = if let Some(profile) = &self.profile {
            get_detectors_from_profile(profile)
        } else {
            get_detectors_from_profile("comprehensive")
        };

        let scan_options = CoreScanOptions {
            detectors,
            custom_detectors: self.custom_detectors.clone(),
            cache_size: self.cache_size.unwrap_or(1000),
            batch_size: self.batch_size.unwrap_or(100),
            max_file_size: self.max_file_size.unwrap_or(10 * 1024 * 1024), // 10MB default
            max_threads: self.max_threads,
            show_progress: self.show_progress,
            streaming: self.streaming,
            incremental: self.incremental,
            distributed: self.distributed,
        };

        let mut scanner = Scanner::new(scan_options);
        
        if self.optimize {
            scanner.enable_optimizations();
        }

        let results = scanner.scan_path(&self.path).await?;

        if self.show_metrics {
            println!("📊 Scan Metrics:");
            println!("  Files scanned: {}", results.metrics.files_scanned);
            println!("  Issues found: {}", results.issues.len());
            println!("  Scan duration: {:?}", results.metrics.scan_duration);
        }

        // Store results in database
        let storage = code_guardian_storage::SqliteStorage::new(&db_path)?;
        storage.store_scan_results(&results).await?;

        println!("✅ Scan completed successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_scan_command_creation() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();
        
        let cmd = ScanCommand::new(path.clone());
        assert_eq!(cmd.path, path);
        assert!(!cmd.show_progress);
        assert!(!cmd.optimize);
    }

    #[test]
    fn test_scan_command_builder_pattern() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();
        
        let mut cmd = ScanCommand::new(path);
        cmd.show_progress = true;
        cmd.optimize = true;
        cmd.profile = Some("security".to_string());
        
        assert!(cmd.show_progress);
        assert!(cmd.optimize);
        assert_eq!(cmd.profile, Some("security".to_string()));
    }
}