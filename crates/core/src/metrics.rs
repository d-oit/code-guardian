use lazy_static::lazy_static;
use prometheus::{
    Encoder, Gauge, Histogram, HistogramOpts, IntCounter, IntGauge, Registry, TextEncoder,
};
use std::sync::Once;
use std::time::Instant;

static INIT: Once = Once::new();

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    // Scan metrics
    pub static ref SCANS_TOTAL: IntCounter = IntCounter::new(
        "code_guardian_scans_total",
        "Total number of scans performed"
    ).expect("metric can be created");

    pub static ref FILES_SCANNED_TOTAL: IntCounter = IntCounter::new(
        "code_guardian_files_scanned_total",
        "Total number of files scanned"
    ).expect("metric can be created");

    pub static ref ISSUES_FOUND_TOTAL: IntCounter = IntCounter::new(
        "code_guardian_issues_found_total",
        "Total number of issues found"
    ).expect("metric can be created");

    // Performance metrics
    pub static ref SCAN_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new(
            "code_guardian_scan_duration_seconds",
            "Time spent scanning in seconds"
        ).buckets(vec![0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0])
    ).expect("metric can be created");

    pub static ref FILE_SCAN_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new(
            "code_guardian_file_scan_duration_seconds",
            "Time spent scanning individual files in seconds"
        ).buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0])
    ).expect("metric can be created");

    // Resource metrics
    pub static ref MEMORY_USAGE: Gauge = Gauge::new(
        "code_guardian_memory_usage_bytes",
        "Current memory usage in bytes"
    ).expect("metric can be created");

    pub static ref CPU_USAGE: Gauge = Gauge::new(
        "code_guardian_cpu_usage_percent",
        "Current CPU usage percentage"
    ).expect("metric can be created");

    // Detector metrics
    pub static ref DETECTOR_EXECUTIONS: IntCounter = IntCounter::new(
        "code_guardian_detector_executions_total",
        "Total number of detector executions"
    ).expect("metric can be created");

    pub static ref LLM_DETECTIONS: IntCounter = IntCounter::new(
        "code_guardian_llm_detections_total",
        "Total number of LLM-specific detections"
    ).expect("metric can be created");

    // Cache metrics
    pub static ref CACHE_HITS: IntCounter = IntCounter::new(
        "code_guardian_cache_hits_total",
        "Total number of cache hits"
    ).expect("metric can be created");

    pub static ref CACHE_MISSES: IntCounter = IntCounter::new(
        "code_guardian_cache_misses_total",
        "Total number of cache misses"
    ).expect("metric can be created");

    // Error metrics
    pub static ref ERRORS_TOTAL: IntCounter = IntCounter::new(
        "code_guardian_errors_total",
        "Total number of errors encountered"
    ).expect("metric can be created");

    // Current state metrics
    pub static ref ACTIVE_SCANS: IntGauge = IntGauge::new(
        "code_guardian_active_scans",
        "Number of currently active scans"
    ).expect("metric can be created");
}

pub fn init_metrics() -> Result<(), prometheus::Error> {
    INIT.call_once(|| {
        let _ = REGISTRY.register(Box::new(SCANS_TOTAL.clone()));
        let _ = REGISTRY.register(Box::new(FILES_SCANNED_TOTAL.clone()));
        let _ = REGISTRY.register(Box::new(ISSUES_FOUND_TOTAL.clone()));
        let _ = REGISTRY.register(Box::new(SCAN_DURATION.clone()));
        let _ = REGISTRY.register(Box::new(FILE_SCAN_DURATION.clone()));
        let _ = REGISTRY.register(Box::new(MEMORY_USAGE.clone()));
        let _ = REGISTRY.register(Box::new(CPU_USAGE.clone()));
        let _ = REGISTRY.register(Box::new(DETECTOR_EXECUTIONS.clone()));
        let _ = REGISTRY.register(Box::new(LLM_DETECTIONS.clone()));
        let _ = REGISTRY.register(Box::new(CACHE_HITS.clone()));
        let _ = REGISTRY.register(Box::new(CACHE_MISSES.clone()));
        let _ = REGISTRY.register(Box::new(ERRORS_TOTAL.clone()));
        let _ = REGISTRY.register(Box::new(ACTIVE_SCANS.clone()));
    });

    Ok(())
}

pub struct MetricsCollector {
    start_time: Instant,
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCollector {
    pub fn new() -> Self {
        SCANS_TOTAL.inc();
        ACTIVE_SCANS.inc();

        Self {
            start_time: Instant::now(),
        }
    }

    pub fn record_file_scanned(&self) {
        FILES_SCANNED_TOTAL.inc();
    }

    pub fn record_issue_found(&self) {
        ISSUES_FOUND_TOTAL.inc();
    }

    pub fn record_detector_execution(&self) {
        DETECTOR_EXECUTIONS.inc();
    }

    pub fn record_llm_detection(&self) {
        LLM_DETECTIONS.inc();
    }

    pub fn record_cache_hit(&self) {
        CACHE_HITS.inc();
    }

    pub fn record_cache_miss(&self) {
        CACHE_MISSES.inc();
    }

    pub fn record_error(&self) {
        ERRORS_TOTAL.inc();
    }

    pub fn record_file_scan_duration(&self, duration: std::time::Duration) {
        FILE_SCAN_DURATION.observe(duration.as_secs_f64());
    }

    pub fn update_resource_usage(&self) {
        use sysinfo::System;

        let mut sys = System::new();
        sys.refresh_all();

        // Memory usage
        let _total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        MEMORY_USAGE.set(used_memory as f64);

        // CPU usage (approximate)
        if let Some(process) = sys.processes().values().next() {
            CPU_USAGE.set(process.cpu_usage() as f64);
        }
    }
}

impl Drop for MetricsCollector {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        SCAN_DURATION.observe(duration.as_secs_f64());
        ACTIVE_SCANS.dec();
    }
}

pub fn get_metrics() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer)?;
    Ok(String::from_utf8(buffer)?)
}

// HTTP endpoint for Prometheus scraping
pub async fn metrics_handler() -> Result<String, axum::http::StatusCode> {
    get_metrics().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_initialization() {
        let result = init_metrics();
        assert!(result.is_ok());
    }

    #[test]
    fn test_metrics_collector() {
        init_metrics().unwrap();
        let collector = MetricsCollector::new();

        // Test recording various metrics
        collector.record_file_scanned();
        collector.record_issue_found();
        collector.record_detector_execution();
        collector.record_llm_detection();
        collector.record_cache_hit();
        collector.record_cache_miss();
        collector.record_error();

        let duration = std::time::Duration::from_millis(100);
        collector.record_file_scan_duration(duration);

        collector.update_resource_usage();

        // Verify metrics can be collected
        let metrics_output = get_metrics();
        assert!(metrics_output.is_ok());

        let output = metrics_output.unwrap();
        assert!(output.contains("code_guardian_scans_total"));
        assert!(output.contains("code_guardian_files_scanned_total"));
        assert!(output.contains("code_guardian_issues_found_total"));
    }

    #[test]
    fn test_metrics_handler() {
        init_metrics().unwrap();
        tokio_test::block_on(async {
            let result = metrics_handler().await;
            assert!(result.is_ok());

            let metrics = result.unwrap();
            assert!(metrics.contains("code_guardian"));
        });
    }
}
