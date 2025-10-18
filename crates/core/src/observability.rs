//! Observability infrastructure for Code Guardian
//!
//! Provides structured logging, metrics collection, and health monitoring.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Correlation ID for tracing requests across components
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CorrelationId(String);

impl CorrelationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Structured logging with correlation IDs and JSON output
pub struct StructuredLogger {
    correlation_id: CorrelationId,
    service: String,
    version: String,
}

impl StructuredLogger {
    pub fn new(service: &str, version: &str) -> Self {
        Self {
            correlation_id: CorrelationId::new(),
            service: service.to_string(),
            version: version.to_string(),
        }
    }

    pub fn with_correlation_id(mut self, correlation_id: CorrelationId) -> Self {
        self.correlation_id = correlation_id;
        self
    }

    pub fn log_info(&self, message: &str, fields: Option<HashMap<String, serde_json::Value>>) {
        self.log("INFO", message, fields);
    }

    pub fn log_warn(&self, message: &str, fields: Option<HashMap<String, serde_json::Value>>) {
        self.log("WARN", message, fields);
    }

    pub fn log_error(&self, message: &str, fields: Option<HashMap<String, serde_json::Value>>) {
        self.log("ERROR", message, fields);
    }

    fn log(&self, level: &str, message: &str, fields: Option<HashMap<String, serde_json::Value>>) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut log_entry = serde_json::json!({
            "timestamp": timestamp,
            "level": level,
            "message": message,
            "correlation_id": self.correlation_id.as_str(),
            "service": self.service,
            "version": self.version,
        });

        if let Some(fields) = fields {
            if let Some(obj) = log_entry.as_object_mut() {
                for (key, value) in fields {
                    obj.insert(key, value);
                }
            }
        }

        println!("{}", log_entry);
    }
}

/// Atomic metrics collection for performance monitoring
#[derive(Debug)]
pub struct ScanMetrics {
    // Counters
    pub files_scanned: AtomicU64,
    pub issues_found: AtomicU64,
    pub files_skipped: AtomicU64,
    pub errors_encountered: AtomicU64,

    // Gauges
    pub current_memory_mb: AtomicU64,
    pub active_threads: AtomicU64,

    // Timers
    pub total_scan_duration: std::sync::Mutex<Duration>,
    pub average_file_scan_time: std::sync::Mutex<Duration>,

    // Metadata
    pub scan_start_time: Instant,
    pub correlation_id: CorrelationId,
}

impl ScanMetrics {
    pub fn new() -> Self {
        Self {
            files_scanned: AtomicU64::new(0),
            issues_found: AtomicU64::new(0),
            files_skipped: AtomicU64::new(0),
            errors_encountered: AtomicU64::new(0),
            current_memory_mb: AtomicU64::new(0),
            active_threads: AtomicU64::new(0),
            total_scan_duration: std::sync::Mutex::new(Duration::ZERO),
            average_file_scan_time: std::sync::Mutex::new(Duration::ZERO),
            scan_start_time: Instant::now(),
            correlation_id: CorrelationId::new(),
        }
    }

    pub fn increment_files_scanned(&self) {
        self.files_scanned.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_issues_found(&self, count: u64) {
        self.issues_found.fetch_add(count, Ordering::Relaxed);
    }

    pub fn increment_files_skipped(&self) {
        self.files_skipped.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_errors(&self) {
        self.errors_encountered.fetch_add(1, Ordering::Relaxed);
    }

    pub fn set_memory_usage(&self, mb: u64) {
        self.current_memory_mb.store(mb, Ordering::Relaxed);
    }

    pub fn set_active_threads(&self, count: u64) {
        self.active_threads.store(count, Ordering::Relaxed);
    }

    pub fn finish_scan(&self) {
        let duration = self.scan_start_time.elapsed();
        *self.total_scan_duration.lock().unwrap() = duration;

        let files_scanned = self.files_scanned.load(Ordering::Relaxed);
        if files_scanned > 0 {
            let avg_time = duration / files_scanned as u32;
            *self.average_file_scan_time.lock().unwrap() = avg_time;
        }
    }

    /// Export metrics in Prometheus format
    pub fn to_prometheus(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "# HELP code_guardian_files_scanned_total Total number of files scanned\n\
             # TYPE code_guardian_files_scanned_total counter\n\
             code_guardian_files_scanned_total{{correlation_id=\"{}\"}} {}\n\n",
            self.correlation_id,
            self.files_scanned.load(Ordering::Relaxed)
        ));

        output.push_str(&format!(
            "# HELP code_guardian_issues_found_total Total number of issues found\n\
             # TYPE code_guardian_issues_found_total counter\n\
             code_guardian_issues_found_total{{correlation_id=\"{}\"}} {}\n\n",
            self.correlation_id,
            self.issues_found.load(Ordering::Relaxed)
        ));

        output.push_str(&format!(
            "# HELP code_guardian_memory_usage_mb Current memory usage in MB\n\
             # TYPE code_guardian_memory_usage_mb gauge\n\
             code_guardian_memory_usage_mb{{correlation_id=\"{}\"}} {}\n\n",
            self.correlation_id,
            self.current_memory_mb.load(Ordering::Relaxed)
        ));

        output.push_str(&format!(
            "# HELP code_guardian_scan_duration_seconds Total scan duration in seconds\n\
             # TYPE code_guardian_scan_duration_seconds gauge\n\
             code_guardian_scan_duration_seconds{{correlation_id=\"{}\"}} {:.3}\n\n",
            self.correlation_id,
            self.total_scan_duration.lock().unwrap().as_secs_f64()
        ));

        output
    }

    /// Export metrics as JSON
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "correlation_id": self.correlation_id.as_str(),
            "counters": {
                "files_scanned": self.files_scanned.load(Ordering::Relaxed),
                "issues_found": self.issues_found.load(Ordering::Relaxed),
                "files_skipped": self.files_skipped.load(Ordering::Relaxed),
                "errors_encountered": self.errors_encountered.load(Ordering::Relaxed),
            },
            "gauges": {
                "current_memory_mb": self.current_memory_mb.load(Ordering::Relaxed),
                "active_threads": self.active_threads.load(Ordering::Relaxed),
            },
            "timers": {
                "total_scan_duration_ms": self.total_scan_duration.lock().unwrap().as_millis(),
                "average_file_scan_time_ms": self.average_file_scan_time.lock().unwrap().as_millis(),
            },
            "timestamps": {
                "scan_start": self.scan_start_time.elapsed().as_secs(),
            }
        })
    }
}

impl Default for ScanMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Health check system for monitoring service health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: HealthState,
    pub timestamp: u64,
    pub version: String,
    pub checks: HashMap<String, ComponentHealth>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthState {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub status: HealthState,
    pub message: Option<String>,
    pub last_check: u64,
    pub response_time_ms: Option<u64>,
}

pub struct HealthChecker {
    version: String,
    checks: HashMap<String, Box<dyn HealthCheck + Send + Sync>>,
}

#[async_trait::async_trait]
pub trait HealthCheck {
    async fn check(&self) -> ComponentHealth;
    fn name(&self) -> &str;
}

impl HealthChecker {
    pub fn new(version: &str) -> Self {
        Self {
            version: version.to_string(),
            checks: HashMap::new(),
        }
    }

    pub fn add_check(&mut self, check: Box<dyn HealthCheck + Send + Sync>) {
        self.checks.insert(check.name().to_string(), check);
    }

    pub async fn check_health(&self) -> HealthStatus {
        let mut checks = HashMap::new();
        let mut overall_status = HealthState::Healthy;

        for (name, check) in &self.checks {
            let start = Instant::now();
            let component_health = check.check().await;
            let duration = start.elapsed();

            let mut health = component_health;
            health.response_time_ms = Some(duration.as_millis() as u64);

            match health.status {
                HealthState::Unhealthy => overall_status = HealthState::Unhealthy,
                HealthState::Degraded if overall_status == HealthState::Healthy => {
                    overall_status = HealthState::Degraded;
                }
                _ => {}
            }

            checks.insert(name.clone(), health);
        }

        HealthStatus {
            status: overall_status,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            version: self.version.clone(),
            checks,
        }
    }
}

/// Database health check implementation
pub struct DatabaseHealthCheck {
    db_path: std::path::PathBuf,
}

impl DatabaseHealthCheck {
    pub fn new(db_path: std::path::PathBuf) -> Self {
        Self { db_path }
    }
}

#[async_trait::async_trait]
impl HealthCheck for DatabaseHealthCheck {
    async fn check(&self) -> ComponentHealth {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if !self.db_path.exists() {
            return ComponentHealth {
                status: HealthState::Unhealthy,
                message: Some("Database file does not exist".to_string()),
                last_check: start_time,
                response_time_ms: None,
            };
        }

        // Try to open database connection
        // Simplified database check - check if path exists
        match std::fs::metadata(&self.db_path) {
            Ok(_metadata) => {
                // Database file exists and is accessible
                ComponentHealth {
                    status: HealthState::Healthy,
                    message: Some("Database file accessible".to_string()),
                    last_check: start_time,
                    response_time_ms: None,
                }
            }
            Err(e) => ComponentHealth {
                status: HealthState::Degraded,
                message: Some(format!("Database file not accessible: {}", e)),
                last_check: start_time,
                response_time_ms: None,
            },
        }
    }

    fn name(&self) -> &str {
        "database"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlation_id() {
        let id1 = CorrelationId::new();
        let id2 = CorrelationId::new();
        assert_ne!(id1, id2);

        let id3 = CorrelationId::from_string("test-id".to_string());
        assert_eq!(id3.as_str(), "test-id");
    }

    #[test]
    fn test_scan_metrics() {
        let metrics = ScanMetrics::new();

        metrics.increment_files_scanned();
        metrics.increment_issues_found(5);
        metrics.set_memory_usage(100);

        assert_eq!(metrics.files_scanned.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.issues_found.load(Ordering::Relaxed), 5);
        assert_eq!(metrics.current_memory_mb.load(Ordering::Relaxed), 100);
    }

    #[test]
    fn test_metrics_prometheus_export() {
        let metrics = ScanMetrics::new();
        metrics.increment_files_scanned();

        let prometheus_output = metrics.to_prometheus();
        assert!(prometheus_output.contains("code_guardian_files_scanned_total"));
        assert!(prometheus_output.contains("1"));
    }

    #[test]
    fn test_metrics_json_export() {
        let metrics = ScanMetrics::new();
        metrics.increment_files_scanned();

        let json_output = metrics.to_json();
        assert_eq!(json_output["counters"]["files_scanned"], 1);
    }

    #[test]
    fn test_structured_logger() {
        let logger = StructuredLogger::new("code-guardian", "0.1.0");

        // Test that logging doesn't panic
        logger.log_info("Test message", None);

        let mut fields = HashMap::new();
        fields.insert(
            "key".to_string(),
            serde_json::Value::String("value".to_string()),
        );
        logger.log_warn("Warning message", Some(fields));
    }

    #[tokio::test]
    async fn test_health_checker() {
        let mut checker = HealthChecker::new("0.1.0");

        // Add a mock health check
        struct MockHealthCheck;

        #[async_trait::async_trait]
        impl HealthCheck for MockHealthCheck {
            async fn check(&self) -> ComponentHealth {
                ComponentHealth {
                    status: HealthState::Healthy,
                    message: Some("Mock check passed".to_string()),
                    last_check: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    response_time_ms: None,
                }
            }

            fn name(&self) -> &str {
                "mock"
            }
        }

        checker.add_check(Box::new(MockHealthCheck));

        let health = checker.check_health().await;
        assert_eq!(health.status, HealthState::Healthy);
        assert!(health.checks.contains_key("mock"));
    }
}
