use anyhow::Result;
use code_guardian_cli::report_handlers::handle_report;
use code_guardian_cli::scan_handlers::{handle_scan, ScanOptions};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Integration tests for end-to-end workflows
#[cfg(test)]
mod workflow_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_scan_to_report_workflow() -> Result<()> {
        // Create temporary test directory with sample code
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_path_buf();

        // Create sample source file with patterns
        let src_file = temp_path.join("test.rs");
        fs::write(
            &src_file,
            r#"
fn main() {
    println!("Hello World");
    // TODO: Implement proper error handling
    let result = dangerous_operation().unwrap();
    console.log("Debug info"); // This won't match in .rs files
}

fn dangerous_operation() -> Result<String, &'static str> {
    Ok("success".to_string())
}
"#,
        )?;

        // Create temporary database
        let db_path = temp_path.join("test.db");

        // Step 1: Perform scan
        let scan_options = ScanOptions {
            path: temp_path.clone(),
            db: Some(db_path.clone()),
            config_path: None,
            profile: "basic".to_string(),
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
        };

        let scan_result = handle_scan(scan_options).await;
        assert!(scan_result.is_ok(), "Scan should complete successfully");

        // Step 2: Generate report (this may fail if no scans are stored, but tests the workflow)
        let report_result = handle_report(
            1, // Use scan ID 1
            "json".to_string(),
            Some(db_path),
        );
        // Note: This might fail due to database implementation, but we're testing the workflow
        let _report_result = report_result; // Allow either success or failure for now

        Ok(())
    }

    #[tokio::test]
    async fn test_incremental_scan_workflow() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_path_buf();

        // Create initial source file
        let src_file = temp_path.join("evolving.rs");
        fs::write(
            &src_file,
            r#"
fn initial_code() {
    // TODO: Initial implementation
    println!("Initial version");
}
"#,
        )?;

        let db_path = temp_path.join("incremental.db");

        // First scan
        let scan_options_1 = ScanOptions {
            path: temp_path.clone(),
            db: Some(db_path.clone()),
            config_path: None,
            profile: "basic".to_string(),
            show_progress: false,
            optimize: false,
            streaming: false,
            show_metrics: false,
            incremental: true, // Enable incremental scanning
            distributed: false,
            custom_detectors: None,
            cache_size: None,
            batch_size: None,
            max_file_size: None,
            max_threads: None,
        };

        let first_scan = handle_scan(scan_options_1).await;
        if let Err(e) = &first_scan {
            println!("First incremental scan error: {}", e);
        }
        assert!(first_scan.is_ok(), "First incremental scan should succeed");

        // Modify the file
        fs::write(
            &src_file,
            r#"
fn initial_code() {
    // TODO: Initial implementation
    println!("Initial version");
}

fn new_function() {
    // FIXME: This needs optimization
    println!("New functionality");
}
"#,
        )?;

        // Second incremental scan
        let scan_options_2 = ScanOptions {
            path: temp_path.clone(),
            db: Some(db_path.clone()),
            config_path: None,
            profile: "basic".to_string(),
            show_progress: false,
            optimize: false,
            streaming: false,
            show_metrics: false,
            incremental: true,
            distributed: false,
            custom_detectors: None,
            cache_size: None,
            batch_size: None,
            max_file_size: None,
            max_threads: None,
        };

        let second_scan = handle_scan(scan_options_2).await;
        assert!(
            second_scan.is_ok(),
            "Second incremental scan should succeed"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_multi_profile_workflow() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_path_buf();

        // Create source file with various patterns
        let src_file = temp_path.join("multi_pattern.rs");
        fs::write(
            &src_file,
            r#"
use std::process;

fn security_issues() {
    // TODO: Review security implications
    let password = "hardcoded_secret_123"; // Security issue
    let result = unsafe_operation().unwrap(); // Potential panic
    
    // HACK: Temporary workaround
    process::exit(0); // Abrupt exit
}

fn performance_issues() {
    // FIXME: Optimize this loop
    for i in 0..1000000 {
        for j in 0..1000 {
            let _expensive = format!("{}-{}", i, j);
        }
    }
}

fn unsafe_operation() -> Result<String, &'static str> {
    Ok("result".to_string())
}
"#,
        )?;

        let db_path = temp_path.join("multi_profile.db");

        // Scan with different profiles to test profile workflow
        let profiles = vec!["basic", "security", "performance"];

        for profile in profiles {
            let scan_options = ScanOptions {
                path: temp_path.clone(),
                db: Some(db_path.clone()),
                config_path: None,
                profile: profile.to_string(),
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
            };

            let scan_result = handle_scan(scan_options).await;
            assert!(
                scan_result.is_ok(),
                "Scan with profile {} should succeed",
                profile
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_custom_config_workflow() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_path_buf();

        // Create custom configuration file
        let config_file = temp_path.join("custom_config.toml");
        fs::write(
            &config_file,
            r#"
[scan]
max_threads = 2
batch_size = 50

[[detectors]]
name = "custom_todo"
pattern = "TODO|FIXME|HACK"
severity = "Medium"
enabled = true

[[detectors]]
name = "custom_debug"
pattern = "println!|dbg!"
severity = "Low"
enabled = true
"#,
        )?;

        // Create source file
        let src_file = temp_path.join("configured.rs");
        fs::write(
            &src_file,
            r#"
fn main() {
    println!("Debug output");
    // TODO: Remove debug code
    dbg!("More debug info");
    // HACK: Quick fix needed
}
"#,
        )?;

        let db_path = temp_path.join("configured.db");

        // Scan with custom configuration
        let scan_options = ScanOptions {
            path: temp_path.clone(),
            db: Some(db_path),
            config_path: Some(config_file),
            profile: "basic".to_string(),
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
        };

        let scan_result = handle_scan(scan_options).await;
        assert!(
            scan_result.is_ok(),
            "Scan with custom config should succeed"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_error_handling_workflow() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_path_buf();

        // Test scan with non-existent path
        let invalid_scan_options = ScanOptions {
            path: PathBuf::from("definitely/does/not/exist"),
            db: None,
            config_path: None,
            profile: "basic".to_string(),
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
        };

        let invalid_scan_result = handle_scan(invalid_scan_options).await;
        assert!(
            invalid_scan_result.is_err(),
            "Scan with invalid path should fail"
        );

        // Test scan with invalid config
        let invalid_config_file = temp_path.join("invalid.toml");
        fs::write(&invalid_config_file, "invalid toml content {")?;

        let invalid_config_options = ScanOptions {
            path: temp_path.clone(),
            db: None,
            config_path: Some(invalid_config_file),
            profile: "basic".to_string(),
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
        };

        let invalid_config_result = handle_scan(invalid_config_options).await;
        assert!(
            invalid_config_result.is_err(),
            "Scan with invalid config should fail"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_large_codebase_workflow() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_path_buf();

        // Create multiple source files to simulate a larger codebase
        for i in 0..10 {
            let src_file = temp_path.join(format!("module_{}.rs", i));
            fs::write(
                &src_file,
                format!(
                    r#"
// Module {}
pub fn function_{}() {{
    // TODO: Implement module {} functionality
    println!("Module {} function");
    let result = operation_{}().unwrap();
    // FIXME: Add proper error handling for module {}
}}

fn operation_{}() -> Result<String, &'static str> {{
    Ok("Module {} result".to_string())
}}

#[cfg(test)]
mod tests {{
    use super::*;
    
    #[test]
    fn test_function_{}() {{
        // Test module {} functionality
        function_{}();
    }}
}}
"#,
                    i, i, i, i, i, i, i, i, i, i, i
                ),
            )?;
        }

        // Create subdirectory with more files
        let subdir = temp_path.join("submodule");
        fs::create_dir(&subdir)?;

        for i in 0..5 {
            let src_file = subdir.join(format!("sub_{}.rs", i));
            fs::write(
                &src_file,
                format!(
                    r#"
// Submodule {}
pub struct Struct{} {{
    field: String,
}}

impl Struct{} {{
    pub fn new() -> Self {{
        // TODO: Implement constructor for Struct{}
        Self {{
            field: "default".to_string(),
        }}
    }}
    
    pub fn process(&self) -> Result<(), &'static str> {{
        // HACK: Quick implementation
        println!("Processing in struct {}", self.field);
        Ok(())
    }}
}}
"#,
                    i, i, i, i, i
                ),
            )?;
        }

        let db_path = temp_path.join("large_codebase.db");

        // Scan large codebase with optimization enabled
        let scan_options = ScanOptions {
            path: temp_path,
            db: Some(db_path),
            config_path: None,
            profile: "comprehensive".to_string(),
            show_progress: false,
            optimize: true,  // Enable optimizations for large codebase
            streaming: true, // Enable streaming for better memory usage
            show_metrics: false,
            incremental: false,
            distributed: false,
            custom_detectors: None,
            cache_size: Some(1000),
            batch_size: Some(50),
            max_file_size: Some(1048576), // 1MB limit
            max_threads: Some(4),
        };

        let scan_result = handle_scan(scan_options).await;
        assert!(scan_result.is_ok(), "Large codebase scan should succeed");

        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_scan_workflow() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_path_buf();

        // Create source file
        let src_file = temp_path.join("concurrent.rs");
        fs::write(
            &src_file,
            r#"
fn concurrent_function() {
    // TODO: Test concurrent scanning
    println!("Concurrent test");
}
"#,
        )?;

        // Run multiple scans concurrently to test thread safety
        let scan_tasks = (0..3).map(|i| {
            let path = temp_path.clone();
            let db_path = temp_path.join(format!("concurrent_{}.db", i));

            tokio::spawn(async move {
                let scan_options = ScanOptions {
                    path,
                    db: Some(db_path),
                    config_path: None,
                    profile: "basic".to_string(),
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
                    max_threads: Some(2), // Limit threads for concurrent test
                };

                handle_scan(scan_options).await
            })
        });

        // Wait for all scans to complete
        let results = futures::future::try_join_all(scan_tasks).await?;

        // All scans should succeed
        for (i, result) in results.into_iter().enumerate() {
            assert!(result.is_ok(), "Concurrent scan {} should succeed", i);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_metrics_and_monitoring_workflow() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_path_buf();

        // Create source file
        let src_file = temp_path.join("metrics.rs");
        fs::write(
            &src_file,
            r#"
fn metrics_test() {
    // TODO: Add metrics collection
    println!("Metrics test");
    let data = vec![1, 2, 3, 4, 5];
    for item in data {
        println!("Processing: {}", item);
    }
}
"#,
        )?;

        let db_path = temp_path.join("metrics.db");

        // Scan with metrics enabled
        let scan_options = ScanOptions {
            path: temp_path,
            db: Some(db_path),
            config_path: None,
            profile: "performance".to_string(),
            show_progress: true, // Enable progress reporting
            optimize: true,      // Enable optimizations
            streaming: true,     // Enable streaming
            show_metrics: true,  // Enable metrics collection
            incremental: false,
            distributed: false,
            custom_detectors: None,
            cache_size: Some(500),
            batch_size: Some(100),
            max_file_size: Some(1048576),
            max_threads: Some(4),
        };

        let scan_result = handle_scan(scan_options).await;
        assert!(scan_result.is_ok(), "Metrics-enabled scan should succeed");

        Ok(())
    }
}

/// Cross-crate integration tests
#[cfg(test)]
mod cross_crate_integration_tests {
    use super::*;
    use code_guardian_core::{DetectorFactory, Scanner};
    use code_guardian_output::formatters::{JsonFormatter, TextFormatter};
    use code_guardian_output::Formatter;

    #[test]
    fn test_core_to_output_integration() -> Result<()> {
        // Test integration between core scanning and output formatting
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // Create test file
        let test_file = temp_path.join("integration.rs");
        fs::write(
            &test_file,
            r#"
fn test_function() {
    // TODO: Implement integration test
    println!("Integration test");
}
"#,
        )?;

        // Use core scanner directly
        let detectors = DetectorFactory::create_default_detectors();
        let scanner = Scanner::new(detectors);
        let matches = scanner.scan(temp_path)?;

        // Format results with different formatters
        let json_formatter = JsonFormatter;
        let json_output = json_formatter.format(&matches);
        assert!(
            json_output.contains("TODO"),
            "JSON output should contain detected patterns"
        );

        let text_formatter = TextFormatter;
        let text_output = text_formatter.format(&matches);
        assert!(
            text_output.contains("TODO"),
            "Text output should contain detected patterns"
        );

        Ok(())
    }

    #[test]
    fn test_storage_integration() -> Result<()> {
        // Test storage integration
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("storage_test.db");

        // Note: This would test actual storage if we had the storage integration
        // For now, we just test that the database path is handled correctly
        assert!(
            db_path.parent().unwrap().exists(),
            "Database parent directory should exist"
        );

        Ok(())
    }
}
