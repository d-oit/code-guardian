use std::sync::Arc;
use anyhow;
use anyhow;
use std::time::{Duration, Instant};
use sysinfo::System;
use tokio::sync::Mutex;
use tokio::time;
use tracing::{error, info, warn};

/// Performance monitor for tracking execution times and resource usage
#[derive(Debug)]
pub struct PerformanceMonitor {
    system: Arc<Mutex<System>>,
    start_time: Instant,
    operation_start: Option<Instant>,
    timeout_duration: Duration,
    memory_threshold_mb: usize,
    cpu_threshold_percent: f64,
}

impl PerformanceMonitor {
    /// Create a new performance monitor with default thresholds
    pub fn new() -> Self {
        Self::with_thresholds(Duration::from_secs(300), 1024, 90.0) // 5 min timeout, 1GB memory, 90% CPU
    }

    /// Create a new performance monitor with custom thresholds
    pub fn with_thresholds(timeout: Duration, memory_threshold_mb: usize, cpu_threshold_percent: f64) -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            system: Arc::new(Mutex::new(system)),
            start_time: Instant::now(),
            operation_start: None,
            timeout_duration: timeout,
            memory_threshold_mb,
            cpu_threshold_percent,
        }
    }

    /// Start monitoring an operation
    pub fn start_operation(&mut self, operation_name: &str) {
        self.operation_start = Some(Instant::now());
        info!("Started operation: {}", operation_name);
    }

    /// End monitoring an operation and log metrics
    pub async fn end_operation(&mut self, operation_name: &str) -> Result<(), anyhow::Error> {
        let duration = match self.operation_start.take() {
            Some(start) => start.elapsed(),
            None => return Err(anyhow::anyhow!("No operation started")),
        };

        let metrics = self.collect_metrics().await?;
        info!(
            "Completed operation: {} in {:?} - CPU: {:.1}%, Memory: {:.1}MB",
            operation_name, duration, metrics.cpu_usage, metrics.memory_usage_mb
        );

        // Check thresholds
        if duration > self.timeout_duration {
            warn!("Operation {} exceeded timeout threshold: {:?} > {:?}", operation_name, duration, self.timeout_duration);
        }
        if metrics.memory_usage_mb > self.memory_threshold_mb as f64 {
            warn!("Operation {} exceeded memory threshold: {:.1}MB > {}MB", operation_name, metrics.memory_usage_mb, self.memory_threshold_mb);
        }
        if metrics.cpu_usage > self.cpu_threshold_percent {
            warn!("Operation {} exceeded CPU threshold: {:.1}% > {:.1}%", operation_name, metrics.cpu_usage, self.cpu_threshold_percent);
        }

        Ok(())
    }

    /// Collect current system metrics
    pub async fn collect_metrics(&self) -> Result<SystemMetrics, anyhow::Error> {
        let mut system = self.system.lock().await;
        system.refresh_all();

        let cpu_usage = system.global_cpu_info().cpu_usage() as f64;
        let memory_usage_mb = system.used_memory() as f64 / 1024.0 / 1024.0;

        Ok(SystemMetrics {
            cpu_usage,
            memory_usage_mb,
            total_memory_mb: system.total_memory() as f64 / 1024.0 / 1024.0,
        })
    }

    /// Start async monitoring task that logs metrics periodically
    pub async fn start_async_monitoring(&self, interval: Duration) {
        let system = Arc::clone(&self.system);
        let timeout_duration = self.timeout_duration;

        tokio::spawn(async move {
            let mut interval = time::interval(interval);
            let start_time = Instant::now();

            loop {
                interval.tick().await;

                let elapsed = start_time.elapsed();
                if elapsed > timeout_duration {
                    error!("Monitoring timeout exceeded: {:?} > {:?}", elapsed, timeout_duration);
                    break;
                }

                let mut sys = system.lock().await;
                sys.refresh_all();

                let cpu = sys.global_cpu_info().cpu_usage();
                let mem_mb = sys.used_memory() as f64 / 1024.0 / 1024.0;

                info!(
                    "Monitoring - Elapsed: {:?}, CPU: {:.1}%, Memory: {:.1}MB",
                    elapsed, cpu, mem_mb
                );
            }
        });
    }

    /// Get total elapsed time since monitor creation
    pub fn total_elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// System metrics snapshot
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage_mb: f64,
    pub total_memory_mb: f64,
}

/// Async operation wrapper with monitoring
pub struct MonitoredOperation<T> {
    monitor: PerformanceMonitor,
    operation_name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> MonitoredOperation<T> {
    pub fn new(operation_name: &str) -> Self {
        Self {
            monitor: PerformanceMonitor::new(),
            operation_name: operation_name.to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn execute<F, Fut>(&mut self, operation: F) -> Result<T, anyhow::Error>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, String>>,
    {
        self.monitor.start_operation(&self.operation_name);

        // Start monitoring
        self.monitor.start_async_monitoring(Duration::from_secs(10)).await;

        // Execute with timeout
        let result = time::timeout(self.monitor.timeout_duration, operation()).await
            .map_err(|_| anyhow::anyhow!("Operation {} timed out after {:?}", self.operation_name, self.monitor.timeout_duration))?
            .map_err(|e| anyhow::anyhow!("Operation {} failed: {}", self.operation_name, e))?;

        self.monitor.end_operation(&self.operation_name).await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new();
        monitor.start_operation("test_op");

        sleep(Duration::from_millis(100)).await;

        let result = monitor.end_operation("test_op").await;
        assert!(result.is_ok());

        let metrics = monitor.collect_metrics().await.unwrap();
        assert!(metrics.cpu_usage >= 0.0);
        assert!(metrics.memory_usage_mb >= 0.0);
    }

    #[tokio::test]
    async fn test_monitored_operation() {
        let mut monitored = MonitoredOperation::<String>::new("test_async_op");

        let result = monitored.execute(|| async {
            sleep(Duration::from_millis(50)).await;
            Ok("success".to_string())
        }).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }
}
