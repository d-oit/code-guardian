use anyhow::Result;
use code_guardian_core::Match;
use code_guardian_storage::{Scan, ScanRepository, SqliteScanRepository};
use std::path::PathBuf;

use crate::report_handlers::get_formatter;
use crate::utils::get_db_path;

pub fn handle_compare(id1: i64, id2: i64, format: String, db: Option<PathBuf>) -> Result<()> {
    let formatter = get_formatter(&format)?;
    let db_path = get_db_path(db);
    let repo = SqliteScanRepository::new(&db_path)?;
    let scan1 = repo.get_scan(id1)?;
    let scan2 = repo.get_scan(id2)?;
    match (scan1, scan2) {
        (Some(s1), Some(s2)) => {
            let diff = compare_scans(&s1, &s2);
            println!("{}", formatter.format(&diff));
        }
        _ => println!("One or both scans not found."),
    }
    Ok(())
}

pub fn compare_scans(scan1: &Scan, scan2: &Scan) -> Vec<Match> {
    // Simple diff: matches in scan2 not in scan1
    // For simplicity, assume matches are unique by file_path, line_number, pattern
    let set1: std::collections::HashSet<_> = scan1
        .matches
        .iter()
        .map(|m| (m.file_path.clone(), m.line_number, m.pattern.clone()))
        .collect();
    scan2
        .matches
        .iter()
        .filter(|m| !set1.contains(&(m.file_path.clone(), m.line_number, m.pattern.clone())))
        .cloned()
        .collect()
}
