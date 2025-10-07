use anyhow::Result;
use code_guardian_core::Match;
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::Path;

refinery::embed_migrations!("migrations");

/// Represents a scan session with its metadata and results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scan {
    /// Unique identifier for the scan. None if not yet saved.
    pub id: Option<i64>,
    /// Timestamp when the scan was performed (Unix timestamp).
    pub timestamp: i64,
    /// Root path of the scanned directory.
    pub root_path: String,
    /// List of matches found during the scan.
    pub matches: Vec<Match>,
}

/// Repository trait for scan data access.
pub trait ScanRepository {
    /// Saves a new scan and returns its ID.
    fn save_scan(&mut self, scan: &Scan) -> Result<i64>;
    /// Retrieves a scan by ID, including its matches.
    fn get_scan(&self, id: i64) -> Result<Option<Scan>>;
    /// Retrieves all scans, without matches for performance.
    fn get_all_scans(&self) -> Result<Vec<Scan>>;
    /// Deletes a scan and its matches.
    fn delete_scan(&mut self, id: i64) -> Result<()>;
}

/// SQLite implementation of the scan repository.
pub struct SqliteScanRepository {
    conn: Connection,
}

impl SqliteScanRepository {
    /// Creates a new repository with an in-memory database for testing.
    pub fn new_in_memory() -> Result<Self> {
        let mut conn = Connection::open_in_memory()?;
        Self::init_db(&mut conn)?;
        Ok(Self { conn })
    }

    /// Creates a new repository with a file-based database.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut conn = Connection::open(path)?;
        Self::init_db(&mut conn)?;
        Ok(Self { conn })
    }

    /// Initializes the database schema using migrations.
    fn init_db(conn: &mut Connection) -> Result<()> {
        migrations::runner().run(conn)?;
        Ok(())
    }
}

impl ScanRepository for SqliteScanRepository {
    fn save_scan(&mut self, scan: &Scan) -> Result<i64> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT INTO scans (timestamp, root_path) VALUES (?1, ?2)",
            (scan.timestamp, &scan.root_path),
        )?;
        let scan_id = tx.last_insert_rowid();
        for m in &scan.matches {
            tx.execute(
                "INSERT INTO matches (scan_id, file_path, line_number, column, pattern, message) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (scan_id, &m.file_path, m.line_number as i64, m.column as i64, &m.pattern, &m.message),
            )?;
        }
        tx.commit()?;
        Ok(scan_id)
    }

    fn get_scan(&self, id: i64) -> Result<Option<Scan>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, timestamp, root_path FROM scans WHERE id = ?1")?;
        let scan_opt = stmt
            .query_row([id], |row| {
                Ok(Scan {
                    id: Some(row.get(0)?),
                    timestamp: row.get(1)?,
                    root_path: row.get(2)?,
                    matches: Vec::new(),
                })
            })
            .optional()?;
        if let Some(mut scan) = scan_opt {
            let mut stmt = self.conn.prepare(
                "SELECT file_path, line_number, column, pattern, message FROM matches WHERE scan_id = ?1",
            )?;
            let matches_iter = stmt.query_map([id], |row| {
                Ok(Match {
                    file_path: row.get(0)?,
                    line_number: row.get(1)?,
                    column: row.get(2)?,
                    pattern: row.get(3)?,
                    message: row.get(4)?,
                })
            })?;
            for m in matches_iter {
                scan.matches.push(m?);
            }
            Ok(Some(scan))
        } else {
            Ok(None)
        }
    }

    fn get_all_scans(&self) -> Result<Vec<Scan>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, timestamp, root_path FROM scans ORDER BY timestamp DESC")?;
        let scans_iter = stmt.query_map([], |row| {
            Ok(Scan {
                id: Some(row.get(0)?),
                timestamp: row.get(1)?,
                root_path: row.get(2)?,
                matches: Vec::new(), // Not loaded for performance
            })
        })?;
        let mut scans = Vec::new();
        for scan in scans_iter {
            scans.push(scan?);
        }
        Ok(scans)
    }

    fn delete_scan(&mut self, id: i64) -> Result<()> {
        let tx = self.conn.transaction()?;
        tx.execute("DELETE FROM matches WHERE scan_id = ?1", [id])?;
        tx.execute("DELETE FROM scans WHERE id = ?1", [id])?;
        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::TempDir;

    #[test]
    fn test_save_and_get_scan() {
        let mut repo = SqliteScanRepository::new_in_memory().unwrap();
        let now = Utc::now().timestamp();
        let scan = Scan {
            id: None,
            timestamp: now,
            root_path: "/test/path".to_string(),
            matches: vec![Match {
                file_path: "file.rs".to_string(),
                line_number: 1,
                column: 1,
                pattern: "TODO".to_string(),
                message: "TODO".to_string(),
            }],
        };
        let id = repo.save_scan(&scan).unwrap();
        let retrieved = repo.get_scan(id).unwrap().unwrap();
        assert_eq!(retrieved.id, Some(id));
        assert_eq!(retrieved.timestamp, now);
        assert_eq!(retrieved.root_path, scan.root_path);
        assert_eq!(retrieved.matches.len(), 1);
        assert_eq!(retrieved.matches[0], scan.matches[0]);
    }

    #[test]
    fn test_get_all_scans() {
        let mut repo = SqliteScanRepository::new_in_memory().unwrap();
        let now1 = Utc::now().timestamp();
        let scan1 = Scan {
            id: None,
            timestamp: now1,
            root_path: "/path1".to_string(),
            matches: vec![],
        };
        let now2 = Utc::now().timestamp();
        let scan2 = Scan {
            id: None,
            timestamp: now2,
            root_path: "/path2".to_string(),
            matches: vec![],
        };
        repo.save_scan(&scan1).unwrap();
        repo.save_scan(&scan2).unwrap();
        let all = repo.get_all_scans().unwrap();
        assert_eq!(all.len(), 2);
        // Ordered by timestamp desc
        assert_eq!(all[0].timestamp, now2);
        assert_eq!(all[1].timestamp, now1);
    }

    #[test]
    fn test_delete_scan() {
        let mut repo = SqliteScanRepository::new_in_memory().unwrap();
        let scan = Scan {
            id: None,
            timestamp: Utc::now().timestamp(),
            root_path: "/test".to_string(),
            matches: vec![Match {
                file_path: "f.rs".to_string(),
                line_number: 1,
                column: 1,
                pattern: "FIXME".to_string(),
                message: "FIXME".to_string(),
            }],
        };
        let id = repo.save_scan(&scan).unwrap();
        repo.delete_scan(id).unwrap();
        assert!(repo.get_scan(id).unwrap().is_none());
    }

    #[test]
    fn test_file_based_repo() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        {
            let mut repo = SqliteScanRepository::new(&db_path).unwrap();
            let scan = Scan {
                id: None,
                timestamp: Utc::now().timestamp(),
                root_path: "/file/test".to_string(),
                matches: vec![],
            };
            repo.save_scan(&scan).unwrap();
        }
        {
            let repo = SqliteScanRepository::new(&db_path).unwrap();
            let all = repo.get_all_scans().unwrap();
            assert_eq!(all.len(), 1);
        }
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use chrono::Utc;
    use proptest::prelude::*;

    fn arb_match() -> impl Strategy<Value = Match> {
        (
            "[a-zA-Z0-9_.]+",
            1..10000usize,
            1..10000usize,
            "[A-Z]+",
            ".*",
        )
            .prop_map(|(fp, ln, col, pat, msg)| Match {
                file_path: fp.to_string(),
                line_number: ln,
                column: col,
                pattern: pat.to_string(),
                message: msg.to_string(),
            })
    }

    proptest! {
        #[test]
        fn test_save_get_arbitrary_scan(matches in proptest::collection::vec(arb_match(), 0..10)) {
            let mut repo = SqliteScanRepository::new_in_memory().unwrap();
            let scan = Scan {
                id: None,
                timestamp: Utc::now().timestamp(),
                root_path: "test_path".to_string(),
                matches: matches.clone(),
            };
            let id = repo.save_scan(&scan).unwrap();
            let retrieved = repo.get_scan(id).unwrap().unwrap();
            assert_eq!(retrieved.matches.len(), scan.matches.len());
            // Since order might not be preserved, check sets
            use std::collections::HashSet;
            let set1: HashSet<_> = scan.matches.into_iter().collect();
            let set2: HashSet<_> = retrieved.matches.into_iter().collect();
            prop_assert_eq!(set1, set2);
        }
    }
}
