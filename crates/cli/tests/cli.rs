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

    let mut cmd = Command::cargo_bin("code-guardian-cli").unwrap();
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

    let mut cmd = Command::cargo_bin("code-guardian-cli").unwrap();
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

    let mut cmd = Command::cargo_bin("code-guardian-cli").unwrap();
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

    let mut cmd = Command::cargo_bin("code-guardian-cli").unwrap();
    cmd.arg("compare")
        .arg(id1.to_string())
        .arg(id2.to_string())
        .arg("--db")
        .arg(&db_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("FIXME"));
}
