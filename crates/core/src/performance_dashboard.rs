use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Performance metrics for the dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub timestamp: u64,
    pub scan_duration_ms: u64,
    pub files_processed: usize,
    pub lines_processed: usize,
    pub matches_found: usize,
    pub memory_usage_mb: u64,
    pub cpu_usage_percent: f64,
    pub throughput_files_per_second: f64,
    pub throughput_lines_per_second: f64,
}

/// Historical performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceHistory {
    pub metrics: Vec<PerformanceMetrics>,
    pub max_entries: usize,
}

impl PerformanceHistory {
    pub fn new(max_entries: usize) -> Self {
        Self {
            metrics: Vec::new(),
            max_entries,
        }
    }

    pub fn add_metrics(&mut self, metrics: PerformanceMetrics) {
        self.metrics.push(metrics);

        // Keep only the most recent entries
        if self.metrics.len() > self.max_entries {
            self.metrics.remove(0);
        }
    }

    pub fn get_latest(&self) -> Option<&PerformanceMetrics> {
        self.metrics.last()
    }

    pub fn get_average_duration(&self) -> Option<f64> {
        if self.metrics.is_empty() {
            return None;
        }

        let sum: u64 = self.metrics.iter().map(|m| m.scan_duration_ms).sum();
        Some(sum as f64 / self.metrics.len() as f64)
    }

    pub fn get_average_throughput(&self) -> Option<f64> {
        if self.metrics.is_empty() {
            return None;
        }

        let sum: f64 = self
            .metrics
            .iter()
            .map(|m| m.throughput_files_per_second)
            .sum();
        Some(sum / self.metrics.len() as f64)
    }
}

/// Performance dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub enabled: bool,
    pub update_interval_seconds: u64,
    pub history_size: usize,
    pub metrics_to_track: Vec<String>,
    pub alert_thresholds: HashMap<String, f64>,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        let mut alert_thresholds = HashMap::new();
        alert_thresholds.insert("max_scan_duration_ms".to_string(), 30000.0); // 30 seconds
        alert_thresholds.insert("min_throughput_files_per_second".to_string(), 10.0);
        alert_thresholds.insert("max_memory_usage_mb".to_string(), 1024.0); // 1GB
        alert_thresholds.insert("max_cpu_usage_percent".to_string(), 80.0);

        Self {
            enabled: true,
            update_interval_seconds: 60,
            history_size: 100,
            metrics_to_track: vec![
                "scan_duration".to_string(),
                "throughput".to_string(),
                "memory_usage".to_string(),
                "cpu_usage".to_string(),
            ],
            alert_thresholds,
        }
    }
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub metric_name: String,
    pub current_value: f64,
    pub threshold_value: f64,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// Main performance dashboard
pub struct PerformanceDashboard {
    config: DashboardConfig,
    history: PerformanceHistory,
    active_alerts: Vec<PerformanceAlert>,
    start_time: SystemTime,
}

impl PerformanceDashboard {
    pub fn new(config: DashboardConfig) -> Self {
        let history = PerformanceHistory::new(config.history_size);

        Self {
            config,
            history,
            active_alerts: Vec::new(),
            start_time: SystemTime::now(),
        }
    }

    pub fn new_with_defaults() -> Self {
        Self::new(DashboardConfig::default())
    }

    /// Record new performance metrics
    pub fn record_scan_metrics(
        &mut self,
        scan_duration: Duration,
        files_processed: usize,
        lines_processed: usize,
        matches_found: usize,
    ) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let scan_duration_ms = scan_duration.as_millis() as u64;

        // Calculate throughput
        let duration_seconds = scan_duration.as_secs_f64();
        let throughput_files_per_second = if duration_seconds > 0.0 {
            files_processed as f64 / duration_seconds
        } else {
            0.0
        };

        let throughput_lines_per_second = if duration_seconds > 0.0 {
            lines_processed as f64 / duration_seconds
        } else {
            0.0
        };

        // Get system metrics
        let memory_usage_mb = self.get_memory_usage();
        let cpu_usage_percent = self.get_cpu_usage();

        let metrics = PerformanceMetrics {
            timestamp,
            scan_duration_ms,
            files_processed,
            lines_processed,
            matches_found,
            memory_usage_mb,
            cpu_usage_percent,
            throughput_files_per_second,
            throughput_lines_per_second,
        };

        // Check for alerts
        self.check_alerts(&metrics);

        // Store metrics
        self.history.add_metrics(metrics);

        Ok(())
    }

    /// Generate dashboard report
    pub fn generate_report(&self) -> DashboardReport {
        let current_metrics = self.history.get_latest().cloned();
        let average_duration = self.history.get_average_duration();
        let average_throughput = self.history.get_average_throughput();

        let uptime_seconds = self.start_time.elapsed().unwrap_or_default().as_secs();

        let total_scans = self.history.metrics.len();
        let total_files_processed: usize =
            self.history.metrics.iter().map(|m| m.files_processed).sum();

        let total_matches_found: usize = self.history.metrics.iter().map(|m| m.matches_found).sum();

        DashboardReport {
            current_metrics,
            average_duration_ms: average_duration,
            average_throughput_files_per_second: average_throughput,
            total_scans,
            total_files_processed,
            total_matches_found,
            uptime_seconds,
            active_alerts: self.active_alerts.clone(),
            history_size: self.history.metrics.len(),
        }
    }

    /// Generate HTML dashboard
    pub fn generate_html_dashboard(&self) -> String {
        let report = self.generate_report();

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Code Guardian Performance Dashboard</title>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; background-color: #f5f5f5; }}
        .dashboard {{ max-width: 1200px; margin: 0 auto; }}
        .header {{ background-color: #2c3e50; color: white; padding: 20px; border-radius: 8px; margin-bottom: 20px; }}
        .metrics-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin-bottom: 20px; }}
        .metric-card {{ background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .metric-value {{ font-size: 2em; font-weight: bold; color: #27ae60; }}
        .metric-label {{ color: #7f8c8d; margin-top: 5px; }}
        .alerts {{ margin-bottom: 20px; }}
        .alert {{ padding: 15px; margin-bottom: 10px; border-radius: 5px; }}
        .alert-warning {{ background-color: #f39c12; color: white; }}
        .alert-critical {{ background-color: #e74c3c; color: white; }}
        .alert-info {{ background-color: #3498db; color: white; }}
        .history-chart {{ background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .timestamp {{ color: #95a5a6; font-size: 0.9em; }}
    </style>
</head>
<body>
    <div class="dashboard">
        <div class="header">
            <h1>🛡️ Code Guardian Performance Dashboard</h1>
            <p class="timestamp">Last updated: {}</p>
            <p>Uptime: {} minutes</p>
        </div>

        {}

        <div class="metrics-grid">
            <div class="metric-card">
                <div class="metric-value">{}</div>
                <div class="metric-label">Total Scans</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">{}</div>
                <div class="metric-label">Files Processed</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">{}</div>
                <div class="metric-label">Matches Found</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">{:.1} ms</div>
                <div class="metric-label">Average Scan Duration</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">{:.1} files/sec</div>
                <div class="metric-label">Average Throughput</div>
            </div>
            {}
        </div>

        <div class="history-chart">
            <h3>📊 Performance History</h3>
            <p>Tracking {} recent scans</p>
            <p>Configure monitoring with: <code>cargo run -- scan . --metrics</code></p>
        </div>
    </div>
</body>
</html>"#,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            report.uptime_seconds / 60,
            self.generate_alerts_html(&report.active_alerts),
            report.total_scans,
            report.total_files_processed,
            report.total_matches_found,
            report.average_duration_ms.unwrap_or(0.0),
            report.average_throughput_files_per_second.unwrap_or(0.0),
            self.generate_current_metrics_html(&report.current_metrics),
            report.history_size,
        )
    }

    fn generate_alerts_html(&self, alerts: &[PerformanceAlert]) -> String {
        if alerts.is_empty() {
            return String::new();
        }

        let mut html = String::from(r#"<div class="alerts"><h3>⚠️ Active Alerts</h3>"#);

        for alert in alerts {
            let class = match alert.severity {
                AlertSeverity::Info => "alert-info",
                AlertSeverity::Warning => "alert-warning",
                AlertSeverity::Critical => "alert-critical",
            };

            html.push_str(&format!(
                r#"<div class="alert {}"><strong>{}:</strong> {}</div>"#,
                class, alert.metric_name, alert.message
            ));
        }

        html.push_str("</div>");
        html
    }

    fn generate_current_metrics_html(&self, metrics: &Option<PerformanceMetrics>) -> String {
        if let Some(m) = metrics {
            format!(
                r#"<div class="metric-card">
                    <div class="metric-value">{:.1} MB</div>
                    <div class="metric-label">Current Memory Usage</div>
                </div>
                <div class="metric-card">
                    <div class="metric-value">{:.1}%</div>
                    <div class="metric-label">Current CPU Usage</div>
                </div>"#,
                m.memory_usage_mb, m.cpu_usage_percent
            )
        } else {
            String::from(
                r#"<div class="metric-card">
                    <div class="metric-value">-</div>
                    <div class="metric-label">No Recent Data</div>
                </div>"#,
            )
        }
    }

    /// Check for performance alerts
    fn check_alerts(&mut self, metrics: &PerformanceMetrics) {
        self.active_alerts.clear(); // Clear previous alerts

        let timestamp = metrics.timestamp;

        // Check scan duration
        if let Some(&threshold) = self.config.alert_thresholds.get("max_scan_duration_ms") {
            if metrics.scan_duration_ms as f64 > threshold {
                self.active_alerts.push(PerformanceAlert {
                    metric_name: "Scan Duration".to_string(),
                    current_value: metrics.scan_duration_ms as f64,
                    threshold_value: threshold,
                    severity: AlertSeverity::Warning,
                    message: format!(
                        "Scan took {}ms (threshold: {}ms)",
                        metrics.scan_duration_ms, threshold as u64
                    ),
                    timestamp,
                });
            }
        }

        // Check throughput
        if let Some(&threshold) = self
            .config
            .alert_thresholds
            .get("min_throughput_files_per_second")
        {
            if metrics.throughput_files_per_second < threshold {
                self.active_alerts.push(PerformanceAlert {
                    metric_name: "Throughput".to_string(),
                    current_value: metrics.throughput_files_per_second,
                    threshold_value: threshold,
                    severity: AlertSeverity::Warning,
                    message: format!(
                        "Low throughput: {:.1} files/sec (threshold: {:.1})",
                        metrics.throughput_files_per_second, threshold
                    ),
                    timestamp,
                });
            }
        }

        // Check memory usage
        if let Some(&threshold) = self.config.alert_thresholds.get("max_memory_usage_mb") {
            if metrics.memory_usage_mb as f64 > threshold {
                self.active_alerts.push(PerformanceAlert {
                    metric_name: "Memory Usage".to_string(),
                    current_value: metrics.memory_usage_mb as f64,
                    threshold_value: threshold,
                    severity: AlertSeverity::Critical,
                    message: format!(
                        "High memory usage: {}MB (threshold: {}MB)",
                        metrics.memory_usage_mb, threshold as u64
                    ),
                    timestamp,
                });
            }
        }

        // Check CPU usage
        if let Some(&threshold) = self.config.alert_thresholds.get("max_cpu_usage_percent") {
            if metrics.cpu_usage_percent > threshold {
                self.active_alerts.push(PerformanceAlert {
                    metric_name: "CPU Usage".to_string(),
                    current_value: metrics.cpu_usage_percent,
                    threshold_value: threshold,
                    severity: AlertSeverity::Warning,
                    message: format!(
                        "High CPU usage: {:.1}% (threshold: {:.1}%)",
                        metrics.cpu_usage_percent, threshold
                    ),
                    timestamp,
                });
            }
        }
    }

    /// Get current memory usage (simplified)
    fn get_memory_usage(&self) -> u64 {
        // In a real implementation, this would use system calls
        // For now, return a placeholder value
        64 // MB
    }

    /// Get current CPU usage (simplified)
    fn get_cpu_usage(&self) -> f64 {
        // In a real implementation, this would calculate actual CPU usage
        // For now, return a placeholder value
        15.0 // percent
    }

    /// Export metrics to JSON
    pub fn export_metrics_json(&self) -> Result<String> {
        let report = self.generate_report();
        serde_json::to_string_pretty(&report)
            .map_err(|e| anyhow!("Failed to serialize metrics: {}", e))
    }

    /// Save dashboard to file
    pub fn save_dashboard(&self, path: &std::path::Path) -> Result<()> {
        let html = self.generate_html_dashboard();
        std::fs::write(path, html)?;
        Ok(())
    }
}

/// Dashboard report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardReport {
    pub current_metrics: Option<PerformanceMetrics>,
    pub average_duration_ms: Option<f64>,
    pub average_throughput_files_per_second: Option<f64>,
    pub total_scans: usize,
    pub total_files_processed: usize,
    pub total_matches_found: usize,
    pub uptime_seconds: u64,
    pub active_alerts: Vec<PerformanceAlert>,
    pub history_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = PerformanceDashboard::new_with_defaults();
        assert!(dashboard.config.enabled);
        assert!(!dashboard.config.alert_thresholds.is_empty());
    }

    #[test]
    fn test_metrics_recording() {
        let mut dashboard = PerformanceDashboard::new_with_defaults();

        let result = dashboard.record_scan_metrics(Duration::from_millis(1000), 100, 5000, 25);

        assert!(result.is_ok());
        assert_eq!(dashboard.history.metrics.len(), 1);

        let metrics = dashboard.history.get_latest().unwrap();
        assert_eq!(metrics.files_processed, 100);
        assert_eq!(metrics.lines_processed, 5000);
        assert_eq!(metrics.matches_found, 25);
    }

    #[test]
    fn test_performance_history() {
        let mut history = PerformanceHistory::new(3);

        for i in 0..5 {
            let metrics = PerformanceMetrics {
                timestamp: i,
                scan_duration_ms: 1000 + i * 100,
                files_processed: 100,
                lines_processed: 1000,
                matches_found: 10,
                memory_usage_mb: 64,
                cpu_usage_percent: 15.0,
                throughput_files_per_second: 100.0,
                throughput_lines_per_second: 1000.0,
            };
            history.add_metrics(metrics);
        }

        // Should only keep the last 3 entries
        assert_eq!(history.metrics.len(), 3);
        assert_eq!(history.metrics[0].timestamp, 2);
        assert_eq!(history.metrics[2].timestamp, 4);
    }

    #[test]
    fn test_alert_generation() {
        let mut config = DashboardConfig::default();
        config
            .alert_thresholds
            .insert("max_scan_duration_ms".to_string(), 500.0);

        let mut dashboard = PerformanceDashboard::new(config);

        // Record metrics that should trigger an alert
        let result = dashboard.record_scan_metrics(
            Duration::from_millis(1000), // Exceeds 500ms threshold
            100,
            5000,
            25,
        );

        assert!(result.is_ok());
        assert!(!dashboard.active_alerts.is_empty());
        assert_eq!(dashboard.active_alerts[0].metric_name, "Scan Duration");
    }

    #[test]
    fn test_dashboard_report() {
        let mut dashboard = PerformanceDashboard::new_with_defaults();

        // Add some metrics
        dashboard
            .record_scan_metrics(Duration::from_millis(1000), 100, 5000, 25)
            .unwrap();
        dashboard
            .record_scan_metrics(Duration::from_millis(1500), 150, 7500, 30)
            .unwrap();

        let report = dashboard.generate_report();
        assert!(report.current_metrics.is_some());
        assert_eq!(report.total_scans, 2);
        assert_eq!(report.total_files_processed, 250);
        assert_eq!(report.total_matches_found, 55);
        assert!(report.average_duration_ms.is_some());
    }

    #[test]
    fn test_html_generation() {
        let dashboard = PerformanceDashboard::new_with_defaults();
        let html = dashboard.generate_html_dashboard();

        assert!(html.contains("Code Guardian Performance Dashboard"));
        assert!(html.contains("Total Scans"));
        assert!(html.contains("Files Processed"));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn test_json_export() {
        let dashboard = PerformanceDashboard::new_with_defaults();
        let result = dashboard.export_metrics_json();

        assert!(result.is_ok());
        let json = result.unwrap();
        assert!(json.contains("total_scans"));
        assert!(json.contains("uptime_seconds"));
    }
}
