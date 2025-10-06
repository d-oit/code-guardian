use crate::{Match, PatternDetector};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use std::time::Instant;

/// Work unit for distributed processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUnit {
    pub id: String,
    pub files: Vec<PathBuf>,
    pub detector_types: Vec<String>,
    pub priority: u8, // 0-255, higher = more priority
    pub estimated_duration_ms: u64,
}

/// Result from processing a work unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkResult {
    pub unit_id: String,
    pub worker_id: String,
    pub matches: Vec<Match>,
    pub files_processed: usize,
    pub processing_time_ms: u64,
    pub timestamp: u64,
    pub errors: Vec<String>,
}

/// Worker node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    pub worker_id: String,
    pub max_concurrent_units: usize,
    pub supported_detectors: Vec<String>,
    pub cpu_cores: usize,
    pub memory_limit_mb: usize,
    pub endpoint: Option<String>, // For remote workers
}

/// Distributed scan coordinator
pub struct DistributedCoordinator {
    workers: Vec<WorkerConfig>,
    work_queue: Vec<WorkUnit>,
    completed_work: HashMap<String, WorkResult>,
    detectors: HashMap<String, Box<dyn PatternDetector>>,
}

impl DistributedCoordinator {
    pub fn new() -> Self {
        Self {
            workers: Vec::new(),
            work_queue: Vec::new(),
            completed_work: HashMap::new(),
            detectors: HashMap::new(),
        }
    }

    /// Register a worker node
    pub fn register_worker(&mut self, config: WorkerConfig) {
        println!("🤖 Registered worker: {} (cores: {}, memory: {}MB)", 
                 config.worker_id, config.cpu_cores, config.memory_limit_mb);
        self.workers.push(config);
    }

    /// Register pattern detectors
    pub fn register_detector(&mut self, name: String, detector: Box<dyn PatternDetector>) {
        self.detectors.insert(name, detector);
    }

    /// Create work units from file list
    pub fn create_work_units(&mut self, files: Vec<PathBuf>, batch_size: usize) -> Result<()> {
        for (unit_id, chunk) in files.chunks(batch_size).enumerate() {
            let estimated_duration = self.estimate_processing_time(chunk);

            let work_unit = WorkUnit {
                id: format!("unit_{}", unit_id),
                files: chunk.to_vec(),
                detector_types: self.detectors.keys().cloned().collect(),
                priority: self.calculate_priority(chunk),
                estimated_duration_ms: estimated_duration,
            };
            
            self.work_queue.push(work_unit);
        }
        
        // Sort by priority (higher priority first)
        self.work_queue.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        println!("📦 Created {} work units from {} files", 
                 self.work_queue.len(), files.len());
        Ok(())
    }

    /// Distribute and execute work units
    pub fn execute_distributed_scan(&mut self) -> Result<Vec<Match>> {
        let start_time = Instant::now();
        let total_units = self.work_queue.len();
        
        println!("🚀 Starting distributed scan with {} workers and {} work units", 
                 self.workers.len(), total_units);

        if self.workers.is_empty() {
            // Fallback to local processing
            return self.execute_local_fallback();
        }

        // Simulate distributed processing (in real implementation, this would use
        // actual network communication, message queues, etc.)
        self.simulate_distributed_execution()?;

        let total_matches: Vec<Match> = self.completed_work
            .values()
            .flat_map(|result| result.matches.clone())
            .collect();

        let duration = start_time.elapsed();
        self.print_execution_summary(duration, total_matches.len());

        Ok(total_matches)
    }

    /// Get distributed scan statistics
    pub fn get_statistics(&self) -> DistributedStats {
        let total_files: usize = self.completed_work.values()
            .map(|r| r.files_processed)
            .sum();
        
        let total_processing_time: u64 = self.completed_work.values()
            .map(|r| r.processing_time_ms)
            .sum();

        let worker_utilization: HashMap<String, f64> = self.workers.iter()
            .map(|w| {
                let worker_results: Vec<&WorkResult> = self.completed_work.values()
                    .filter(|r| r.worker_id == w.worker_id)
                    .collect();
                
                let utilization = if !worker_results.is_empty() {
                    worker_results.len() as f64 / self.work_queue.len() as f64
                } else {
                    0.0
                };
                
                (w.worker_id.clone(), utilization)
            })
            .collect();

        DistributedStats {
            total_workers: self.workers.len(),
            total_work_units: self.work_queue.len(),
            completed_units: self.completed_work.len(),
            total_files_processed: total_files,
            total_processing_time_ms: total_processing_time,
            worker_utilization,
            average_unit_size: if !self.work_queue.is_empty() {
                total_files as f64 / self.work_queue.len() as f64
            } else {
                0.0
            },
        }
    }

    fn simulate_distributed_execution(&mut self) -> Result<()> {
        use rayon::prelude::*;
        
        // Process work units in parallel (simulating distributed workers)
        let results: Vec<WorkResult> = self.work_queue
            .par_iter()
            .enumerate()
            .map(|(i, unit)| {
                let worker_id = format!("worker_{}", i % self.workers.len());
                self.process_work_unit(unit, &worker_id)
            })
            .collect::<Result<Vec<_>>>()?;

        // Store results
        for result in results {
            self.completed_work.insert(result.unit_id.clone(), result);
        }

        Ok(())
    }

    fn process_work_unit(&self, unit: &WorkUnit, worker_id: &str) -> Result<WorkResult> {
        let start_time = Instant::now();
        let mut all_matches = Vec::new();
        let mut errors = Vec::new();
        let mut files_processed = 0;

        for file_path in &unit.files {
            match std::fs::read_to_string(file_path) {
                Ok(content) => {
                    for detector_name in &unit.detector_types {
                        if let Some(detector) = self.detectors.get(detector_name) {
                            let matches = detector.detect(&content, file_path);
                            all_matches.extend(matches);
                        }
                    }
                    files_processed += 1;
                }
                Err(e) => {
                    errors.push(format!("Failed to read {}: {}", file_path.display(), e));
                }
            }
        }

        let processing_time = start_time.elapsed();

        Ok(WorkResult {
            unit_id: unit.id.clone(),
            worker_id: worker_id.to_string(),
            matches: all_matches,
            files_processed,
            processing_time_ms: processing_time.as_millis() as u64,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            errors,
        })
    }

    fn execute_local_fallback(&mut self) -> Result<Vec<Match>> {
        println!("⚠️  No workers available, falling back to local processing");
        
        let mut all_matches = Vec::new();
        for unit in &self.work_queue {
            let mut result = self.process_work_unit(unit, "local_worker")?;
            let matches = std::mem::take(&mut result.matches);
            self.completed_work.insert(unit.id.clone(), result);
            all_matches.extend(matches);
        }
        
        Ok(all_matches)
    }

    fn estimate_processing_time(&self, files: &[PathBuf]) -> u64 {
        // Simple estimation: 1ms per file + size factor
        let base_time = files.len() as u64;
        let size_factor: u64 = files.iter()
            .filter_map(|f| std::fs::metadata(f).ok())
            .map(|m| (m.len() / 1024).min(100)) // Cap at 100ms per file
            .sum();
        
        base_time + size_factor
    }

    fn calculate_priority(&self, files: &[PathBuf]) -> u8 {
        // Higher priority for smaller batches (process quickly)
        // and files that are likely to have issues
        let size_priority = match files.len() {
            1..=10 => 200,
            11..=50 => 150,
            51..=100 => 100,
            _ => 50,
        };

        // Boost priority for certain file types
        let type_priority = files.iter()
            .filter_map(|f| f.extension())
            .filter_map(|ext| ext.to_str())
            .map(|ext| match ext {
                "rs" => 50,  // Rust files get higher priority
                "py" | "js" | "ts" => 30,
                _ => 10,
            })
            .max()
            .unwrap_or(0);

        (size_priority + type_priority).min(255) as u8
    }

    fn print_execution_summary(&self, duration: std::time::Duration, total_matches: usize) {
        println!("✅ Distributed scan completed!");
        println!("   Duration: {:?}", duration);
        println!("   Total matches: {}", total_matches);
        println!("   Work units processed: {}", self.completed_work.len());
        
        let stats = self.get_statistics();
        println!("   Files processed: {}", stats.total_files_processed);
        println!("   Average unit size: {:.1} files", stats.average_unit_size);
        
        // Show worker utilization
        for (worker_id, utilization) in &stats.worker_utilization {
            println!("   {}: {:.1}% utilization", worker_id, utilization * 100.0);
        }
    }
}

/// Statistics for distributed scanning
#[derive(Debug, Clone)]
pub struct DistributedStats {
    pub total_workers: usize,
    pub total_work_units: usize,
    pub completed_units: usize,
    pub total_files_processed: usize,
    pub total_processing_time_ms: u64,
    pub worker_utilization: HashMap<String, f64>,
    pub average_unit_size: f64,
}

impl Default for DistributedCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detectors::TodoDetector;
    use tempfile::TempDir;

    #[test]
    fn test_distributed_coordinator_creation() {
        let coordinator = DistributedCoordinator::new();
        assert_eq!(coordinator.workers.len(), 0);
        assert_eq!(coordinator.work_queue.len(), 0);
    }

    #[test]
    fn test_worker_registration() {
        let mut coordinator = DistributedCoordinator::new();
        
        let worker_config = WorkerConfig {
            worker_id: "test_worker".to_string(),
            max_concurrent_units: 4,
            supported_detectors: vec!["TODO".to_string()],
            cpu_cores: 8,
            memory_limit_mb: 4096,
            endpoint: None,
        };
        
        coordinator.register_worker(worker_config);
        assert_eq!(coordinator.workers.len(), 1);
    }

    #[test]
    fn test_work_unit_creation() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");
        std::fs::write(&test_file, "// TODO: test").unwrap();
        
        let mut coordinator = DistributedCoordinator::new();
        coordinator.register_detector("TODO".to_string(), Box::new(TodoDetector));
        
        let files = vec![test_file];
        coordinator.create_work_units(files, 10).unwrap();
        
        assert_eq!(coordinator.work_queue.len(), 1);
        assert_eq!(coordinator.work_queue[0].files.len(), 1);
    }
}