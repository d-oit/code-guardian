//! Report command implementations

use anyhow::Result;
use std::path::PathBuf;
use code_guardian_storage::SqliteStorage;
use code_guardian_output::formatters::{JsonFormatter, TextFormatter, HtmlFormatter, MarkdownFormatter};
use crate::utils::get_db_path;

pub struct ReportCommand {
    pub id: Option<i64>,
    pub format: String,
    pub db: Option<PathBuf>,
    pub output: Option<PathBuf>,
}

impl ReportCommand {
    pub fn new(format: String) -> Self {
        Self {
            id: None,
            format,
            db: None,
            output: None,
        }
    }

    pub fn with_id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_output(mut self, output: PathBuf) -> Self {
        self.output = Some(output);
        self
    }

    pub async fn execute(&self) -> Result<()> {
        let db_path = get_db_path(self.db.clone());
        let storage = SqliteStorage::new(&db_path)?;

        let results = if let Some(id) = self.id {
            vec![storage.get_scan_results(id).await?]
        } else {
            // Get latest scan results
            storage.get_latest_scan_results(1).await?
        };

        if results.is_empty() {
            println!("No scan results found");
            return Ok(());
        }

        let formatted_output = match self.format.as_str() {
            "json" => {
                let formatter = JsonFormatter::new();
                formatter.format(&results[0])?
            }
            "html" => {
                let formatter = HtmlFormatter::new();
                formatter.format(&results[0])?
            }
            "markdown" => {
                let formatter = MarkdownFormatter::new();
                formatter.format(&results[0])?
            }
            _ => {
                let formatter = TextFormatter::new();
                formatter.format(&results[0])?
            }
        };

        if let Some(output_path) = &self.output {
            std::fs::write(output_path, formatted_output)?;
            println!("📄 Report written to {}", output_path.display());
        } else {
            println!("{}", formatted_output);
        }

        Ok(())
    }
}

pub struct HistoryCommand {
    pub db: Option<PathBuf>,
    pub limit: Option<usize>,
}

impl HistoryCommand {
    pub fn new() -> Self {
        Self {
            db: None,
            limit: Some(10),
        }
    }

    pub async fn execute(&self) -> Result<()> {
        let db_path = get_db_path(self.db.clone());
        let storage = SqliteStorage::new(&db_path)?;

        let history = storage.get_scan_history(self.limit.unwrap_or(10)).await?;

        if history.is_empty() {
            println!("No scan history found");
            return Ok(());
        }

        println!("📊 Scan History:");
        println!("{:<5} {:<20} {:<15} {:<10}", "ID", "Timestamp", "Path", "Issues");
        println!("{}", "-".repeat(60));

        for entry in history {
            println!(
                "{:<5} {:<20} {:<15} {:<10}",
                entry.id,
                entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                entry.path.display(),
                entry.issue_count
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_command_creation() {
        let cmd = ReportCommand::new("json".to_string());
        assert_eq!(cmd.format, "json");
        assert!(cmd.id.is_none());
    }

    #[test]
    fn test_report_command_builder() {
        let cmd = ReportCommand::new("html".to_string())
            .with_id(123)
            .with_output(PathBuf::from("output.html"));
        
        assert_eq!(cmd.format, "html");
        assert_eq!(cmd.id, Some(123));
        assert_eq!(cmd.output, Some(PathBuf::from("output.html")));
    }

    #[test]
    fn test_history_command_creation() {
        let cmd = HistoryCommand::new();
        assert_eq!(cmd.limit, Some(10));
        assert!(cmd.db.is_none());
    }
}