//! Standardized error types and recovery mechanisms for Code Guardian

use thiserror::Error;
use std::path::PathBuf;

/// Standard error types across all Code Guardian operations
#[derive(Error, Debug)]
pub enum ScanError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Detector error in {detector}: {message}")]
    Detector { detector: String, message: String },

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("File too large: {path} ({size} bytes exceeds limit of {limit} bytes)")]
    FileTooLarge {
        path: PathBuf,
        size: u64,
        limit: u64,
    },

    #[error("Timeout during {operation} after {timeout_seconds} seconds")]
    Timeout {
        operation: String,
        timeout_seconds: u64,
    },

    #[error("Permission denied accessing {path}")]
    PermissionDenied { path: PathBuf },

    #[error("Invalid file format for {path}: {reason}")]
    InvalidFileFormat { path: PathBuf, reason: String },

    #[error("Memory limit exceeded: {current_mb}MB > {limit_mb}MB")]
    MemoryLimitExceeded { current_mb: u64, limit_mb: u64 },

    #[error("Network error: {message}")]
    Network { message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Git operation failed: {message}")]
    Git { message: String },

    #[error("Custom detector error: {message}")]
    CustomDetector { message: String },

    #[error("Distributed scanning error: {message}")]
    Distributed { message: String },
}

/// Recovery strategies for different error types
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// Retry the operation with exponential backoff
    Retry { max_attempts: u32, base_delay_ms: u64 },
    /// Skip the problematic item and continue
    Skip,
    /// Fall back to alternative implementation
    Fallback,
    /// Fail immediately without recovery
    FailFast,
    /// Continue with degraded functionality
    Degrade,
}

impl ScanError {
    /// Get the appropriate recovery strategy for this error type
    pub fn recovery_strategy(&self) -> RecoveryStrategy {
        match self {
            ScanError::Io(_) => RecoveryStrategy::Retry {
                max_attempts: 3,
                base_delay_ms: 100,
            },
            ScanError::FileTooLarge { .. } => RecoveryStrategy::Skip,
            ScanError::Timeout { .. } => RecoveryStrategy::Retry {
                max_attempts: 2,
                base_delay_ms: 1000,
            },
            ScanError::PermissionDenied { .. } => RecoveryStrategy::Skip,
            ScanError::InvalidFileFormat { .. } => RecoveryStrategy::Skip,
            ScanError::MemoryLimitExceeded { .. } => RecoveryStrategy::Degrade,
            ScanError::Network { .. } => RecoveryStrategy::Retry {
                max_attempts: 5,
                base_delay_ms: 500,
            },
            ScanError::Config { .. } => RecoveryStrategy::FailFast,
            ScanError::Database(_) => RecoveryStrategy::Retry {
                max_attempts: 3,
                base_delay_ms: 200,
            },
            ScanError::Detector { .. } => RecoveryStrategy::Fallback,
            ScanError::Serialization(_) => RecoveryStrategy::FailFast,
            ScanError::Git { .. } => RecoveryStrategy::Fallback,
            ScanError::CustomDetector { .. } => RecoveryStrategy::Skip,
            ScanError::Distributed { .. } => RecoveryStrategy::Retry {
                max_attempts: 2,
                base_delay_ms: 1000,
            },
        }
    }

    /// Get severity level for monitoring and alerting
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            ScanError::Config { .. } | ScanError::Serialization(_) => ErrorSeverity::Critical,
            ScanError::Database(_) | ScanError::MemoryLimitExceeded { .. } => ErrorSeverity::High,
            ScanError::Io(_) | ScanError::Network { .. } | ScanError::Timeout { .. } => ErrorSeverity::Medium,
            ScanError::FileTooLarge { .. } | ScanError::PermissionDenied { .. } | 
            ScanError::InvalidFileFormat { .. } => ErrorSeverity::Low,
            ScanError::Detector { .. } | ScanError::Git { .. } | 
            ScanError::CustomDetector { .. } | ScanError::Distributed { .. } => ErrorSeverity::Medium,
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self.recovery_strategy(),
            RecoveryStrategy::Retry { .. }
        )
    }

    /// Get error context for structured logging
    pub fn context(&self) -> ErrorContext {
        match self {
            ScanError::FileTooLarge { path, size, limit } => ErrorContext {
                operation: "file_scan".to_string(),
                resource: Some(path.display().to_string()),
                details: Some(format!("size={}, limit={}", size, limit)),
            },
            ScanError::PermissionDenied { path } => ErrorContext {
                operation: "file_access".to_string(),
                resource: Some(path.display().to_string()),
                details: None,
            },
            ScanError::Timeout { operation, timeout_seconds } => ErrorContext {
                operation: operation.clone(),
                resource: None,
                details: Some(format!("timeout={}s", timeout_seconds)),
            },
            ScanError::MemoryLimitExceeded { current_mb, limit_mb } => ErrorContext {
                operation: "memory_management".to_string(),
                resource: None,
                details: Some(format!("current={}MB, limit={}MB", current_mb, limit_mb)),
            },
            ScanError::Detector { detector, .. } => ErrorContext {
                operation: "detection".to_string(),
                resource: Some(detector.clone()),
                details: None,
            },
            _ => ErrorContext {
                operation: "unknown".to_string(),
                resource: None,
                details: None,
            },
        }
    }
}

/// Error severity levels for monitoring
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorSeverity {
    Critical,
    High,
    Medium,
    Low,
}

/// Structured error context for logging and monitoring
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub resource: Option<String>,
    pub details: Option<String>,
}

/// Error recovery manager with retry logic and circuit breaker
pub struct ErrorRecoveryManager {
    max_retries: u32,
    circuit_breaker_threshold: u32,
    failure_count: std::sync::atomic::AtomicU32,
}

impl ErrorRecoveryManager {
    pub fn new() -> Self {
        Self {
            max_retries: 3,
            circuit_breaker_threshold: 10,
            failure_count: std::sync::atomic::AtomicU32::new(0),
        }
    }

    /// Execute operation with automatic retry and recovery
    pub async fn execute_with_recovery<T, F, Fut>(
        &self,
        operation: F,
        error_context: &str,
    ) -> Result<T, ScanError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, ScanError>>,
    {
        let mut attempts = 0;
        let mut last_error = None;

        while attempts < self.max_retries {
            match operation().await {
                Ok(result) => {
                    // Reset failure count on success
                    self.failure_count.store(0, std::sync::atomic::Ordering::Relaxed);
                    return Ok(result);
                }
                Err(error) => {
                    attempts += 1;
                    last_error = Some(error.clone());
                    
                    // Check if error is retryable
                    if !error.is_retryable() {
                        tracing::warn!(
                            error = %error,
                            context = error_context,
                            "Non-retryable error encountered"
                        );
                        return Err(error);
                    }

                    // Implement exponential backoff
                    if let RecoveryStrategy::Retry { base_delay_ms, .. } = error.recovery_strategy() {
                        let delay = base_delay_ms * 2_u64.pow(attempts - 1);
                        tracing::warn!(
                            error = %error,
                            attempt = attempts,
                            delay_ms = delay,
                            context = error_context,
                            "Retrying operation after error"
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                    }
                }
            }
        }

        // Increment failure count for circuit breaker
        self.failure_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        let error = last_error.unwrap();
        tracing::error!(
            error = %error,
            attempts = attempts,
            context = error_context,
            "Operation failed after all retry attempts"
        );
        
        Err(error)
    }

    /// Check if circuit breaker should trip
    pub fn should_circuit_break(&self) -> bool {
        self.failure_count.load(std::sync::atomic::Ordering::Relaxed) >= self.circuit_breaker_threshold
    }
}

impl Default for ErrorRecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_recovery_strategies() {
        let io_error = ScanError::Io(std::io::Error::from(std::io::ErrorKind::ConnectionRefused));
        assert!(matches!(io_error.recovery_strategy(), RecoveryStrategy::Retry { .. }));
        assert!(io_error.is_retryable());

        let config_error = ScanError::Config { message: "Invalid config".to_string() };
        assert!(matches!(config_error.recovery_strategy(), RecoveryStrategy::FailFast));
        assert!(!config_error.is_retryable());
    }

    #[test]
    fn test_error_severity() {
        let config_error = ScanError::Config { message: "Invalid".to_string() };
        assert_eq!(config_error.severity(), ErrorSeverity::Critical);

        let file_error = ScanError::FileTooLarge { 
            path: PathBuf::from("/test"), 
            size: 1000, 
            limit: 500 
        };
        assert_eq!(file_error.severity(), ErrorSeverity::Low);
    }

    #[test]
    fn test_error_context() {
        let timeout_error = ScanError::Timeout {
            operation: "scan".to_string(),
            timeout_seconds: 30,
        };
        let context = timeout_error.context();
        assert_eq!(context.operation, "scan");
        assert!(context.details.unwrap().contains("timeout=30s"));
    }

    #[tokio::test]
    async fn test_recovery_manager() {
        let manager = ErrorRecoveryManager::new();
        
        // Test successful operation
        let result = manager.execute_with_recovery(
            || async { Ok::<i32, ScanError>(42) },
            "test_operation"
        ).await;
        assert_eq!(result.unwrap(), 42);
        
        // Test operation that always fails with non-retryable error
        let result = manager.execute_with_recovery(
            || async { 
                Err::<i32, ScanError>(ScanError::Config { 
                    message: "Invalid config".to_string() 
                })
            },
            "test_operation"
        ).await;
        assert!(result.is_err());
    }
}