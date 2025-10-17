//! Production-ready health check server with axum
//!
//! Provides Kubernetes-compatible health and readiness endpoints
//! along with Prometheus metrics for comprehensive monitoring.

use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};
// Prometheus imports handled in metrics module
use crate::metrics::{get_metrics, init_metrics};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub timestamp: String,
    pub checks: HealthChecks,
    pub uptime_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthChecks {
    pub database: String,
    pub scanner: String,
    pub memory: String,
    pub disk: String,
}

#[derive(Clone)]
pub struct HealthState {
    pub version: String,
    pub start_time: std::time::Instant,
}

impl Default for HealthState {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            start_time: std::time::Instant::now(),
        }
    }
}

pub async fn health_handler(
    State(state): State<Arc<HealthState>>,
) -> Result<Json<HealthStatus>, StatusCode> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    let uptime = state.start_time.elapsed().as_secs();

    // Perform health checks
    let database_status = check_database_health().await;
    let scanner_status = check_scanner_health().await;
    let memory_status = check_memory_health().await;
    let disk_status = check_disk_health().await;

    let overall_status = if database_status == "healthy"
        && scanner_status == "healthy"
        && memory_status == "healthy"
        && disk_status == "healthy"
    {
        "healthy"
    } else if database_status == "unhealthy" || scanner_status == "unhealthy" {
        "unhealthy"
    } else {
        "degraded"
    };

    let health_status = HealthStatus {
        status: overall_status.to_string(),
        version: state.version.clone(),
        timestamp,
        uptime_seconds: uptime,
        checks: HealthChecks {
            database: database_status,
            scanner: scanner_status,
            memory: memory_status,
            disk: disk_status,
        },
    };

    if overall_status == "healthy" {
        Ok(Json(health_status))
    } else {
        error!("Health check failed: {}", overall_status);
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

pub async fn readiness_handler(
    State(state): State<Arc<HealthState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Readiness check - service is ready to accept traffic
    match check_readiness().await {
        Ok(_) => {
            info!("Readiness check passed");
            Ok(Json(serde_json::json!({
                "status": "ready",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "version": state.version,
                "uptime_seconds": state.start_time.elapsed().as_secs()
            })))
        }
        Err(e) => {
            error!("Readiness check failed: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

pub async fn liveness_handler(State(state): State<Arc<HealthState>>) -> Json<serde_json::Value> {
    // Basic liveness check - service is running
    info!("Liveness check requested");
    Json(serde_json::json!({
        "status": "alive",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": state.version,
        "uptime_seconds": state.start_time.elapsed().as_secs()
    }))
}

pub async fn metrics_handler() -> Result<String, StatusCode> {
    match get_metrics() {
        Ok(metrics) => {
            info!("Metrics endpoint accessed");
            Ok(metrics)
        }
        Err(e) => {
            error!("Failed to get metrics: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn check_database_health() -> String {
    // Check database connectivity
    // For now, assume healthy - in a real implementation, this would ping the database
    match std::env::var("DATABASE_URL") {
        Ok(_) => "healthy".to_string(),
        Err(_) => "degraded".to_string(), // No database configured, but not critical
    }
}

async fn check_scanner_health() -> String {
    // Check if scanner components are functional
    use crate::detector_factory::DetectorFactory;
    let detectors = DetectorFactory::create_default_detectors();
    if detectors.is_empty() {
        "unhealthy".to_string()
    } else {
        "healthy".to_string()
    }
}

async fn check_memory_health() -> String {
    // Check memory usage
    use sysinfo::System;
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;

    if memory_usage_percent > 90.0 {
        "unhealthy".to_string()
    } else if memory_usage_percent > 80.0 {
        "degraded".to_string()
    } else {
        "healthy".to_string()
    }
}

async fn check_disk_health() -> String {
    // Check disk space - simplified implementation
    use std::fs;

    // Check available space on current directory
    if let Ok(metadata) = fs::metadata(".") {
        // In a real implementation, you'd check actual disk usage
        // For now, assume healthy if we can read filesystem metadata
        if metadata.is_dir() {
            "healthy".to_string()
        } else {
            "degraded".to_string()
        }
    } else {
        "unhealthy".to_string()
    }
}

async fn check_readiness() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Check that all dependencies are available and ready

    // Check scanner initialization
    use crate::detector_factory::DetectorFactory;
    let detectors = DetectorFactory::create_default_detectors();
    if detectors.is_empty() {
        return Err("Scanner initialization failed".into());
    }

    // Check metrics system
    if init_metrics().is_err() {
        return Err("Metrics system not initialized".into());
    }

    Ok(())
}

pub async fn start_health_server(
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state = Arc::new(HealthState::default());

    // Initialize metrics
    init_metrics().map_err(|e| format!("Failed to initialize metrics: {}", e))?;

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/ready", get(readiness_handler))
        .route("/live", get(liveness_handler))
        .route("/metrics", get(metrics_handler))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    info!("Health check server starting on {}", addr);
    info!("Endpoints available:");
    info!("  GET /health  - Comprehensive health check");
    info!("  GET /ready   - Readiness probe (Kubernetes)");
    info!("  GET /live    - Liveness probe (Kubernetes)");
    info!("  GET /metrics - Prometheus metrics");

    axum::serve(listener, app).await?;

    Ok(())
}

// Graceful shutdown handler
pub async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down gracefully");
        },
        _ = terminate => {
            info!("Received SIGTERM, shutting down gracefully");
        },
    }
}
