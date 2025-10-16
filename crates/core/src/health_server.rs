//! HTTP health check server for production monitoring

use crate::observability::{HealthChecker, HealthStatus};
use serde_json;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct HealthServer {
    health_checker: Arc<HealthChecker>,
    addr: SocketAddr,
}

impl HealthServer {
    pub fn new(health_checker: HealthChecker, addr: SocketAddr) -> Self {
        Self {
            health_checker: Arc::new(health_checker),
            addr,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(self.addr).await?;
        println!("Health server listening on {}", self.addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let health_checker = Arc::clone(&self.health_checker);
            
            tokio::spawn(async move {
                if let Err(e) = handle_request(stream, health_checker).await {
                    eprintln!("Error handling request: {}", e);
                }
            });
        }
    }
}

async fn handle_request(
    mut stream: TcpStream,
    health_checker: Arc<HealthChecker>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    
    let request = String::from_utf8_lossy(&buffer[..n]);
    
    if request.starts_with("GET /health") {
        let health_status = health_checker.check_health().await;
        let response = create_health_response(health_status).await;
        stream.write_all(response.as_bytes()).await?;
    } else if request.starts_with("GET /ready") {
        let health_status = health_checker.check_health().await;
        let response = create_readiness_response(health_status).await;
        stream.write_all(response.as_bytes()).await?;
    } else {
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

async fn create_health_response(health_status: HealthStatus) -> String {
    let status_code = match health_status.status {
        crate::observability::HealthState::Healthy => "200 OK",
        crate::observability::HealthState::Degraded => "200 OK",
        crate::observability::HealthState::Unhealthy => "503 SERVICE UNAVAILABLE",
    };

    let body = serde_json::to_string_pretty(&health_status).unwrap_or_else(|_| "{}".to_string());
    
    format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        body.len(),
        body
    )
}

async fn create_readiness_response(health_status: HealthStatus) -> String {
    let status_code = match health_status.status {
        crate::observability::HealthState::Healthy => "200 OK",
        _ => "503 SERVICE UNAVAILABLE",
    };

    let body = serde_json::json!({
        "ready": health_status.status == crate::observability::HealthState::Healthy,
        "timestamp": health_status.timestamp
    });
    
    let body_str = body.to_string();
    
    format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        body_str.len(),
        body_str
    )
}