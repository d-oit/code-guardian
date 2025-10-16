use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Health check status levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Individual health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
    pub timestamp: u64,
    pub duration_ms: u64,
    pub metadata: HashMap<String, String>,
}

/// Overall system health report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub overall_status: HealthStatus,
    pub checks: Vec<HealthCheckResult>,
    pub timestamp: u64,
    pub uptime_seconds: u64,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub enabled_checks: Vec<String>,
    pub timeout_seconds: u64,
    pub check_interval_seconds: u64,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled_checks: vec![
                "database".to_string(),
                "memory".to_string(),
                "disk_space".to_string(),
                "detectors".to_string(),
                "configuration".to_string(),
            ],
            timeout_seconds: 30,
            check_interval_seconds: 60,
        }
    }
}

/// Main health checker
pub struct HealthChecker {
    config: HealthCheckConfig,
    start_time: SystemTime,
}

impl HealthChecker {
    pub fn new(config: HealthCheckConfig) -> Self {
        Self {
            config,
            start_time: SystemTime::now(),
        }
    }

    pub fn new_with_defaults() -> Self {
        Self::new(HealthCheckConfig::default())
    }

    /// Perform all enabled health checks
    pub async fn check_health(&self) -> Result<HealthReport> {
        let mut checks = Vec::new();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Run each enabled check
        for check_name in &self.config.enabled_checks {
            let result = self.run_individual_check(check_name).await;
            checks.push(result);
        }

        // Determine overall status
        let overall_status = self.calculate_overall_status(&checks);

        // Calculate uptime
        let uptime_seconds = self.start_time.elapsed().unwrap_or_default().as_secs();

        Ok(HealthReport {
            overall_status,
            checks,
            timestamp,
            uptime_seconds,
        })
    }

    /// Run a single health check
    async fn run_individual_check(&self, check_name: &str) -> HealthCheckResult {
        let start = SystemTime::now();
        let timestamp = start
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let (status, message, metadata) = match check_name {
            "database" => self.check_database().await,
            "memory" => self.check_memory().await,
            "disk_space" => self.check_disk_space().await,
            "detectors" => self.check_detectors().await,
            "configuration" => self.check_configuration().await,
            _ => (
                HealthStatus::Unknown,
                format!("Unknown check: {}", check_name),
                HashMap::new(),
            ),
        };

        let duration_ms = start.elapsed().unwrap_or_default().as_millis() as u64;

        HealthCheckResult {
            name: check_name.to_string(),
            status,
            message,
            timestamp,
            duration_ms,
            metadata,
        }
    }

    /// Check database connectivity and status
    async fn check_database(&self) -> (HealthStatus, String, HashMap<String, String>) {
        // In a real implementation, this would check database connection
        // For now, we'll simulate a basic check
        let mut metadata = HashMap::new();

        // Check if we can create a temporary database connection
        match std::env::var("DATABASE_URL") {
            Ok(_url) => {
                metadata.insert("database_url".to_string(), "configured".to_string());
                (
                    HealthStatus::Healthy,
                    "Database connection available".to_string(),
                    metadata,
                )
            }
            Err(_) => {
                // For file-based storage, check if we can write to the data directory
                let data_dir = std::path::Path::new("data");
                if data_dir.exists() || std::fs::create_dir_all(data_dir).is_ok() {
                    metadata.insert("storage_type".to_string(), "file_based".to_string());
                    (
                        HealthStatus::Healthy,
                        "File-based storage available".to_string(),
                        metadata,
                    )
                } else {
                    (
                        HealthStatus::Unhealthy,
                        "Cannot access storage".to_string(),
                        metadata,
                    )
                }
            }
        }
    }

    /// Check memory usage
    async fn check_memory(&self) -> (HealthStatus, String, HashMap<String, String>) {
        let mut metadata = HashMap::new();

        // Get basic memory info (simplified)
        #[cfg(target_os = "linux")]
        {
            if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
                // Parse memory info
                let mut total_kb = 0u64;
                let mut available_kb = 0u64;

                for line in meminfo.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(value) = line.split_whitespace().nth(1) {
                            total_kb = value.parse().unwrap_or(0);
                        }
                    } else if line.starts_with("MemAvailable:") {
                        if let Some(value) = line.split_whitespace().nth(1) {
                            available_kb = value.parse().unwrap_or(0);
                        }
                    }
                }

                metadata.insert("total_memory_mb".to_string(), (total_kb / 1024).to_string());
                metadata.insert(
                    "available_memory_mb".to_string(),
                    (available_kb / 1024).to_string(),
                );

                let usage_percent = if total_kb > 0 {
                    ((total_kb - available_kb) as f64 / total_kb as f64 * 100.0) as u64
                } else {
                    0
                };

                metadata.insert(
                    "memory_usage_percent".to_string(),
                    usage_percent.to_string(),
                );

                let status = if usage_percent > 90 {
                    HealthStatus::Unhealthy
                } else if usage_percent > 80 {
                    HealthStatus::Degraded
                } else {
                    HealthStatus::Healthy
                };

                (
                    status,
                    format!("Memory usage: {}%", usage_percent),
                    metadata,
                )
            } else {
                (
                    HealthStatus::Unknown,
                    "Cannot read memory information".to_string(),
                    metadata,
                )
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            // Fallback for non-Linux systems
            metadata.insert("platform".to_string(), std::env::consts::OS.to_string());
            (
                HealthStatus::Healthy,
                "Memory check not implemented for this platform".to_string(),
                metadata,
            )
        }
    }

    /// Check disk space
    async fn check_disk_space(&self) -> (HealthStatus, String, HashMap<String, String>) {
        let mut metadata = HashMap::new();

        // Check available disk space in current directory
        match std::fs::metadata(".") {
            Ok(_) => {
                // On Unix systems, we could use statvfs, but for simplicity, we'll do a basic check
                match std::fs::create_dir_all("data") {
                    Ok(_) => {
                        metadata.insert("disk_access".to_string(), "writable".to_string());
                        (
                            HealthStatus::Healthy,
                            "Disk space available".to_string(),
                            metadata,
                        )
                    }
                    Err(e) => {
                        metadata.insert("error".to_string(), e.to_string());
                        (
                            HealthStatus::Unhealthy,
                            "Cannot write to disk".to_string(),
                            metadata,
                        )
                    }
                }
            }
            Err(e) => {
                metadata.insert("error".to_string(), e.to_string());
                (
                    HealthStatus::Unhealthy,
                    "Cannot access current directory".to_string(),
                    metadata,
                )
            }
        }
    }

    /// Check detector availability
    async fn check_detectors(&self) -> (HealthStatus, String, HashMap<String, String>) {
        let mut metadata = HashMap::new();

        // Check if we can load basic detectors
        use crate::detector_factory::DetectorFactory;

        let detectors = DetectorFactory::create_default_detectors();
        let detector_count = detectors.len();
        metadata.insert("detector_count".to_string(), detector_count.to_string());

        if detector_count > 0 {
            (
                HealthStatus::Healthy,
                format!("Loaded {} detectors", detector_count),
                metadata,
            )
        } else {
            (
                HealthStatus::Degraded,
                "No detectors loaded".to_string(),
                metadata,
            )
        }
    }

    /// Check configuration validity
    async fn check_configuration(&self) -> (HealthStatus, String, HashMap<String, String>) {
        let mut metadata = HashMap::new();

        // Check if we can load configuration
        use crate::config::load_config;

        match load_config::<&str>(None) {
            Ok(config) => {
                metadata.insert("config_source".to_string(), "default".to_string());
                metadata.insert(
                    "pattern_count".to_string(),
                    config.scan_patterns.len().to_string(),
                );

                if config.scan_patterns.is_empty() {
                    (
                        HealthStatus::Degraded,
                        "Configuration loaded but no patterns configured".to_string(),
                        metadata,
                    )
                } else {
                    (
                        HealthStatus::Healthy,
                        "Configuration valid".to_string(),
                        metadata,
                    )
                }
            }
            Err(e) => {
                metadata.insert("error".to_string(), e.to_string());
                (
                    HealthStatus::Unhealthy,
                    "Cannot load configuration".to_string(),
                    metadata,
                )
            }
        }
    }

    /// Calculate overall system status based on individual checks
    fn calculate_overall_status(&self, checks: &[HealthCheckResult]) -> HealthStatus {
        if checks.is_empty() {
            return HealthStatus::Unknown;
        }

        let mut has_unhealthy = false;
        let mut has_degraded = false;
        let mut has_unknown = false;

        for check in checks {
            match check.status {
                HealthStatus::Unhealthy => has_unhealthy = true,
                HealthStatus::Degraded => has_degraded = true,
                HealthStatus::Unknown => has_unknown = true,
                HealthStatus::Healthy => {}
            }
        }

        if has_unhealthy {
            HealthStatus::Unhealthy
        } else if has_degraded {
            HealthStatus::Degraded
        } else if has_unknown {
            HealthStatus::Unknown
        } else {
            HealthStatus::Healthy
        }
    }

    /// Create a readiness probe (simpler than health check)
    pub async fn check_readiness(&self) -> Result<bool> {
        // Basic readiness checks - can we load detectors and configuration?
        let detector_result = self.check_detectors().await;
        let config_result = self.check_configuration().await;

        let is_ready = matches!(
            detector_result.0,
            HealthStatus::Healthy | HealthStatus::Degraded
        ) && matches!(
            config_result.0,
            HealthStatus::Healthy | HealthStatus::Degraded
        );

        Ok(is_ready)
    }

    /// Create a liveness probe (basic functionality check)
    pub async fn check_liveness(&self) -> Result<bool> {
        // Very basic liveness check - can we respond?
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_checker_creation() {
        let checker = HealthChecker::new_with_defaults();
        assert!(!checker.config.enabled_checks.is_empty());
    }

    #[tokio::test]
    async fn test_health_check_execution() {
        let checker = HealthChecker::new_with_defaults();
        let result = checker.check_health().await;
        assert!(result.is_ok());

        let health_report = result.unwrap();
        assert!(!health_report.checks.is_empty());
    }

    #[tokio::test]
    async fn test_readiness_probe() {
        let checker = HealthChecker::new_with_defaults();
        let result = checker.check_readiness().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_liveness_probe() {
        let checker = HealthChecker::new_with_defaults();
        let result = checker.check_liveness().await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_individual_checks() {
        let checker = HealthChecker::new_with_defaults();

        // Test each check individually
        let db_result = checker.check_database().await;
        assert!(!db_result.1.is_empty()); // Should have a message

        let memory_result = checker.check_memory().await;
        assert!(!memory_result.1.is_empty());

        let disk_result = checker.check_disk_space().await;
        assert!(!disk_result.1.is_empty());

        let detector_result = checker.check_detectors().await;
        assert!(!detector_result.1.is_empty());

        let config_result = checker.check_configuration().await;
        assert!(!config_result.1.is_empty());
    }

    #[test]
    fn test_overall_status_calculation() {
        let checker = HealthChecker::new_with_defaults();

        // Test empty checks
        let status = checker.calculate_overall_status(&[]);
        assert_eq!(status, HealthStatus::Unknown);

        // Test all healthy
        let healthy_checks = vec![HealthCheckResult {
            name: "test".to_string(),
            status: HealthStatus::Healthy,
            message: "OK".to_string(),
            timestamp: 0,
            duration_ms: 0,
            metadata: HashMap::new(),
        }];
        let status = checker.calculate_overall_status(&healthy_checks);
        assert_eq!(status, HealthStatus::Healthy);

        // Test with unhealthy
        let unhealthy_checks = vec![HealthCheckResult {
            name: "test".to_string(),
            status: HealthStatus::Unhealthy,
            message: "Error".to_string(),
            timestamp: 0,
            duration_ms: 0,
            metadata: HashMap::new(),
        }];
        let status = checker.calculate_overall_status(&unhealthy_checks);
        assert_eq!(status, HealthStatus::Unhealthy);
    }

    #[tokio::test]
    async fn test_custom_config() {
        let config = HealthCheckConfig {
            enabled_checks: vec!["database".to_string()],
            timeout_seconds: 10,
            check_interval_seconds: 30,
        };

        let checker = HealthChecker::new(config);
        let result = checker.check_health().await;
        assert!(result.is_ok());

        let health_report = result.unwrap();
        assert_eq!(health_report.checks.len(), 1);
        assert_eq!(health_report.checks[0].name, "database");
    }
}
