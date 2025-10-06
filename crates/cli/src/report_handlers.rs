use anyhow::Result;
use code_guardian_output::formatters::{Formatter, TextFormatter, JsonFormatter, CsvFormatter, MarkdownFormatter, HtmlFormatter};
use code_guardian_storage::{ScanRepository, SqliteScanRepository};
use std::path::PathBuf;

use crate::utils::get_db_path;

pub fn handle_report(id: i64, format: String, db: Option<PathBuf>) -> Result<()> {
    let formatter = get_formatter(&format)?;
    let db_path = get_db_path(db);
    let repo = SqliteScanRepository::new(&db_path)?;
    let scan = repo.get_scan(id)?;
    match scan {
        Some(scan) => {
            println!("{}", formatter.format(&scan.matches));
        }
        None => println!("Scan with ID {} not found.", id),
    }
    Ok(())
}

pub fn get_formatter(format: &str) -> Result<Box<dyn Formatter>> {
    match format {
        "text" => Ok(Box::new(TextFormatter)),
        "json" => Ok(Box::new(JsonFormatter)),
        "csv" => Ok(Box::new(CsvFormatter)),
        "markdown" => Ok(Box::new(MarkdownFormatter)),
        "html" => Ok(Box::new(HtmlFormatter)),
        _ => Err(anyhow::anyhow!("Unsupported format: {}", format)),
    }
}