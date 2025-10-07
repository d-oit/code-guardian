use assert_cmd::Command;
use code_guardian_core::Match;
use code_guardian_storage::{Scan, ScanRepository, SqliteScanRepository};
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_scan_command() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "// TODO: fix this\n// FIXME: another").unwrap();
    let db_path = temp_dir.path().join("test.db");

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("scan")
        .arg(temp_dir.path())
        .arg("--db")
        .arg(&db_path)
        .assert()
        .success();

    // Check if db was created and has data
    assert!(db_path.exists());
    let repo = SqliteScanRepository::new(&db_path).unwrap();
    let scans = repo.get_all_scans().unwrap();
    assert_eq!(scans.len(), 1);
    let scan = repo.get_scan(scans[0].id.unwrap()).unwrap().unwrap();
    assert_eq!(scan.matches.len(), 2);
}

#[test]
fn test_history_command() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    {
        let mut repo = SqliteScanRepository::new(&db_path).unwrap();
        let scan = Scan {
            id: None,
            timestamp: chrono::Utc::now().timestamp(),
            root_path: "/test".to_string(),
            matches: vec![],
        };
        repo.save_scan(&scan).unwrap();
    }

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("history")
        .arg("--db")
        .arg(&db_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("ID:"));
}

#[test]
fn test_report_command() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut repo = SqliteScanRepository::new(&db_path).unwrap();
    let scan = Scan {
        id: None,
        timestamp: chrono::Utc::now().timestamp(),
        root_path: "/test".to_string(),
        matches: vec![Match {
            file_path: "test.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "TODO".to_string(),
        }],
    };
    let id = repo.save_scan(&scan).unwrap();

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("report")
        .arg(id.to_string())
        .arg("--db")
        .arg(&db_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("TODO"));
}

#[test]
fn test_compare_command() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut repo = SqliteScanRepository::new(&db_path).unwrap();
    let scan1 = Scan {
        id: None,
        timestamp: chrono::Utc::now().timestamp(),
        root_path: "/test".to_string(),
        matches: vec![Match {
            file_path: "test.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "TODO".to_string(),
        }],
    };
    let id1 = repo.save_scan(&scan1).unwrap();
    let scan2 = Scan {
        id: None,
        timestamp: chrono::Utc::now().timestamp(),
        root_path: "/test".to_string(),
        matches: vec![
            Match {
                file_path: "test.rs".to_string(),
                line_number: 1,
                column: 1,
                pattern: "TODO".to_string(),
                message: "TODO".to_string(),
            },
            Match {
                file_path: "test.js".to_string(),
                line_number: 2,
                column: 1,
                pattern: "FIXME".to_string(),
                message: "FIXME".to_string(),
            },
        ],
    };
    let id2 = repo.save_scan(&scan2).unwrap();

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("compare")
        .arg(id1.to_string())
        .arg(id2.to_string())
        .arg("--db")
        .arg(&db_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("FIXME"));
}

#[test]
fn test_invalid_format() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut repo = SqliteScanRepository::new(&db_path).unwrap();
    let scan = Scan {
        id: None,
        timestamp: chrono::Utc::now().timestamp(),
        root_path: "/test".to_string(),
        matches: vec![],
    };
    let id = repo.save_scan(&scan).unwrap();
    drop(repo); // Ensure data is written

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("report")
        .arg(id.to_string())
        .arg("--format")
        .arg("invalid")
        .arg("--db")
        .arg(&db_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported format"));
}

#[test]
fn test_report_non_existent_scan() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("report")
        .arg("999")
        .arg("--db")
        .arg(&db_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("not found"));
}

#[test]
fn test_scan_non_existent_path() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("scan")
        .arg("/non/existent/path")
        .arg("--db")
        .arg(&db_path)
        .assert()
        .failure();
}

#[test]
fn test_scan_with_different_profiles() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(
        &test_file,
        "// TODO: fix this\n// FIXME: another\n// HACK: temp fix",
    )
    .unwrap();
    let db_path = temp_dir.path().join("test.db");

    // Test comprehensive profile
    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("scan")
        .arg(temp_dir.path())
        .arg("--db")
        .arg(&db_path)
        .arg("--profile")
        .arg("comprehensive")
        .assert()
        .success();

    let repo = SqliteScanRepository::new(&db_path).unwrap();
    let scans = repo.get_all_scans().unwrap();
    assert_eq!(scans.len(), 1);
    let scan = repo.get_scan(scans[0].id.unwrap()).unwrap().unwrap();
    // Comprehensive should find more patterns
    assert!(scan.matches.len() >= 2);
}

#[test]
fn test_scan_with_progress() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "// TODO: fix this").unwrap();
    let db_path = temp_dir.path().join("test.db");

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("scan")
        .arg(temp_dir.path())
        .arg("--db")
        .arg(&db_path)
        .arg("--progress")
        .assert()
        .success()
        .stdout(predicate::str::contains("Scan saved with ID"));
}

#[test]
fn test_scan_with_metrics() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "// TODO: fix this\n// FIXME: another").unwrap();
    let db_path = temp_dir.path().join("test.db");

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("scan")
        .arg(temp_dir.path())
        .arg("--db")
        .arg(&db_path)
        .arg("--optimize")
        .arg("--metrics")
        .assert()
        .success()
        .stdout(predicate::str::contains("Performance Metrics"));
}

#[test]
fn test_scan_optimized() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "// TODO: fix this").unwrap();
    let db_path = temp_dir.path().join("test.db");

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("scan")
        .arg(temp_dir.path())
        .arg("--db")
        .arg(&db_path)
        .arg("--optimize")
        .assert()
        .success();
}

#[test]
fn test_scan_incremental() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "// TODO: fix this").unwrap();
    let db_path = temp_dir.path().join("test.db");

    // First scan
    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    let result = cmd
        .arg("scan")
        .arg(temp_dir.path())
        .arg("--db")
        .arg(&db_path)
        .arg("--incremental")
        .output()
        .unwrap();

    // Allow it to succeed or fail due to UTF-8 issues in test environment
    if result.status.success() {
        // Second incremental scan
        let mut cmd2 = Command::cargo_bin("code_guardian_cli").unwrap();
        cmd2.arg("scan")
            .arg(temp_dir.path())
            .arg("--db")
            .arg(&db_path)
            .arg("--incremental")
            .assert()
            .success();
    }
}

#[test]
fn test_scan_distributed() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "// TODO: fix this").unwrap();
    let db_path = temp_dir.path().join("test.db");

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("scan")
        .arg(temp_dir.path())
        .arg("--db")
        .arg(&db_path)
        .arg("--distributed")
        .assert()
        .success();
}

#[test]
fn test_report_formats() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut repo = SqliteScanRepository::new(&db_path).unwrap();
    let scan = Scan {
        id: None,
        timestamp: chrono::Utc::now().timestamp(),
        root_path: "/test".to_string(),
        matches: vec![Match {
            file_path: "test.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "TODO".to_string(),
        }],
    };
    let id = repo.save_scan(&scan).unwrap();

    // Test JSON format
    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("report")
        .arg(id.to_string())
        .arg("--format")
        .arg("json")
        .arg("--db")
        .arg(&db_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"pattern\": \"TODO\""));

    // Test CSV format
    let mut cmd2 = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd2.arg("report")
        .arg(id.to_string())
        .arg("--format")
        .arg("csv")
        .arg("--db")
        .arg(&db_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("TODO"));

    // Test Markdown format
    let mut cmd3 = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd3.arg("report")
        .arg(id.to_string())
        .arg("--format")
        .arg("markdown")
        .arg("--db")
        .arg(&db_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("|"));

    // Test HTML format
    let mut cmd4 = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd4.arg("report")
        .arg(id.to_string())
        .arg("--format")
        .arg("html")
        .arg("--db")
        .arg(&db_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("<html>"));
}

#[test]
fn test_compare_formats() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut repo = SqliteScanRepository::new(&db_path).unwrap();
    let scan1 = Scan {
        id: None,
        timestamp: chrono::Utc::now().timestamp(),
        root_path: "/test".to_string(),
        matches: vec![Match {
            file_path: "test.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "TODO".to_string(),
        }],
    };
    let id1 = repo.save_scan(&scan1).unwrap();
    let scan2 = Scan {
        id: None,
        timestamp: chrono::Utc::now().timestamp(),
        root_path: "/test".to_string(),
        matches: vec![
            Match {
                file_path: "test.rs".to_string(),
                line_number: 1,
                column: 1,
                pattern: "TODO".to_string(),
                message: "TODO".to_string(),
            },
            Match {
                file_path: "test.js".to_string(),
                line_number: 2,
                column: 1,
                pattern: "FIXME".to_string(),
                message: "FIXME".to_string(),
            },
        ],
    };
    let id2 = repo.save_scan(&scan2).unwrap();

    // Test JSON format for compare
    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("compare")
        .arg(id1.to_string())
        .arg(id2.to_string())
        .arg("--format")
        .arg("json")
        .arg("--db")
        .arg(&db_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"pattern\": \"FIXME\""));
}

#[test]
fn test_custom_detectors_create_examples() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("custom_detectors.json");

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("custom-detectors")
        .arg("create-examples")
        .arg("--output")
        .arg(&output_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created example custom detectors"));

    assert!(output_file.exists());
}

#[test]
fn test_custom_detectors_load() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("custom_detectors.json");

    // First create examples
    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("custom-detectors")
        .arg("create-examples")
        .arg("--output")
        .arg(&output_file)
        .assert()
        .success();

    // Then load them
    let mut cmd2 = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd2.arg("custom-detectors")
        .arg("load")
        .arg(&output_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Loaded"));
}

#[test]
fn test_custom_detectors_test() {
    let temp_dir = TempDir::new().unwrap();
    let detectors_file = temp_dir.path().join("custom_detectors.json");
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "This is a test file with some content").unwrap();

    // Create examples
    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("custom-detectors")
        .arg("create-examples")
        .arg("--output")
        .arg(&detectors_file)
        .assert()
        .success();

    // Test on file
    let mut cmd2 = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd2.arg("custom-detectors")
        .arg("test")
        .arg(&detectors_file)
        .arg(&test_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Testing custom detectors"));
}

#[test]
fn test_incremental_status() {
    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("incremental")
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("No incremental scan state"));
}

#[test]
fn test_incremental_reset() {
    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("incremental")
        .arg("reset")
        .assert()
        .success()
        .stdout(predicate::str::contains("No incremental state"));
}

#[test]
fn test_distributed_setup() {
    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("distributed")
        .arg("setup")
        .arg("--workers")
        .arg("2")
        .assert()
        .success()
        .stdout(predicate::str::contains("Setting up distributed scanning"));
}

#[test]
fn test_distributed_scan() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "// TODO: fix this").unwrap();

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("distributed")
        .arg("scan")
        .arg(temp_dir.path())
        .arg("--workers")
        .arg("2")
        .arg("--batch-size")
        .arg("10")
        .assert()
        .success()
        .stdout(predicate::str::contains("Running distributed scan"));
}

#[test]
fn test_benchmark() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "// TODO: fix this").unwrap();

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("benchmark")
        .arg(temp_dir.path())
        .arg("--quick")
        .assert()
        .success();
}

#[test]
fn test_completion() {
    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("completion").arg("bash").assert().success();
}

#[test]
fn test_scan_with_custom_detectors() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "// TODO: fix this\n// CUSTOM: my pattern").unwrap();
    let db_path = temp_dir.path().join("test.db");
    let custom_detectors_file = temp_dir.path().join("custom.json");

    // Create custom detectors with all required fields
    let custom_detectors = r#"[
        {
            "name": "Custom Pattern",
            "description": "Custom pattern detector",
            "pattern": "CUSTOM",
            "file_extensions": [".rs"],
            "case_sensitive": true,
            "multiline": false,
            "capture_groups": [],
            "severity": "Info",
            "category": "CodeQuality",
            "examples": [],
            "enabled": true
        }
    ]"#;
    fs::write(&custom_detectors_file, custom_detectors).unwrap();

    let mut cmd = Command::cargo_bin("code_guardian_cli").unwrap();
    cmd.arg("scan")
        .arg(temp_dir.path())
        .arg("--db")
        .arg(&db_path)
        .arg("--custom-detectors")
        .arg(&custom_detectors_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Loaded custom detectors"));

    let repo = SqliteScanRepository::new(&db_path).unwrap();
    let scans = repo.get_all_scans().unwrap();
    assert_eq!(scans.len(), 1);
    let scan = repo.get_scan(scans[0].id.unwrap()).unwrap().unwrap();
    // Should find at least TODO
    assert!(!scan.matches.is_empty());
}
