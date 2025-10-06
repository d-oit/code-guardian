use anyhow::Result;
use clap::{Parser, Subcommand};
use code_guardian_core::{FixmeDetector, Match, PatternDetector, Scanner, TodoDetector};
use code_guardian_output::formatters::{Formatter, TextFormatter};
use code_guardian_storage::{Scan, ScanRepository, SqliteScanRepository};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "code-guardian")]
#[command(about = "A tool to scan codebases for patterns like TODO and FIXME")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a directory for patterns and save results
    Scan {
        /// Path to the directory to scan
        path: PathBuf,
        /// Database file path (optional, defaults to code-guardian.db)
        #[arg(short, long)]
        db: Option<PathBuf>,
    },
    /// List all scan history
    History {
        /// Database file path (optional, defaults to code-guardian.db)
        #[arg(short, long)]
        db: Option<PathBuf>,
    },
    /// Generate a report for a specific scan
    Report {
        /// Scan ID
        id: i64,
        /// Output format: text, json, csv, markdown, html
        #[arg(short, long, default_value = "text")]
        format: String,
        /// Database file path (optional, defaults to code-guardian.db)
        #[arg(short, long)]
        db: Option<PathBuf>,
    },
    /// Compare two scans
    Compare {
        /// First scan ID
        id1: i64,
        /// Second scan ID
        id2: i64,
        /// Output format: text, json, csv, markdown, html
        #[arg(short, long, default_value = "text")]
        format: String,
        /// Database file path (optional, defaults to code-guardian.db)
        #[arg(short, long)]
        db: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, db } => handle_scan(path, db),
        Commands::History { db } => handle_history(db),
        Commands::Report { id, format, db } => handle_report(id, format, db),
        Commands::Compare {
            id1,
            id2,
            format,
            db,
        } => handle_compare(id1, id2, format, db),
    }
}

fn get_db_path(db: Option<PathBuf>) -> PathBuf {
    db.unwrap_or_else(|| PathBuf::from("code-guardian.db"))
}

fn create_scanner() -> Scanner {
    let detectors: Vec<Box<dyn PatternDetector>> =
        vec![Box::new(TodoDetector), Box::new(FixmeDetector)];
    Scanner::new(detectors)
}

fn handle_scan(path: PathBuf, db: Option<PathBuf>) -> Result<()> {
    let db_path = get_db_path(db);
    let mut repo = SqliteScanRepository::new(&db_path)?;
    let scanner = create_scanner();
    let matches = scanner.scan(&path)?;
    let timestamp = chrono::Utc::now().timestamp();
    let scan = Scan {
        id: None,
        timestamp,
        root_path: path.to_string_lossy().to_string(),
        matches,
    };
    let id = repo.save_scan(&scan)?;
    println!("Scan completed and saved with ID: {}", id);
    let formatter = TextFormatter;
    println!("{}", formatter.format(&scan.matches));
    Ok(())
}

fn handle_history(db: Option<PathBuf>) -> Result<()> {
    let db_path = get_db_path(db);
    let repo = SqliteScanRepository::new(&db_path)?;
    let scans = repo.get_all_scans()?;
    if scans.is_empty() {
        println!("No scans found.");
        return Ok(());
    }
    println!("Scan History:");
    for scan in scans {
        println!(
            "ID: {}, Timestamp: {}, Path: {}",
            scan.id.unwrap(),
            chrono::DateTime::from_timestamp(scan.timestamp, 0)
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S"),
            scan.root_path
        );
    }
    Ok(())
}

fn handle_report(id: i64, format: String, db: Option<PathBuf>) -> Result<()> {
    let db_path = get_db_path(db);
    let repo = SqliteScanRepository::new(&db_path)?;
    let scan = repo.get_scan(id)?;
    match scan {
        Some(scan) => {
            let formatter = get_formatter(&format)?;
            println!("{}", formatter.format(&scan.matches));
        }
        None => println!("Scan with ID {} not found.", id),
    }
    Ok(())
}

fn handle_compare(id1: i64, id2: i64, format: String, db: Option<PathBuf>) -> Result<()> {
    let db_path = get_db_path(db);
    let repo = SqliteScanRepository::new(&db_path)?;
    let scan1 = repo.get_scan(id1)?;
    let scan2 = repo.get_scan(id2)?;
    match (scan1, scan2) {
        (Some(s1), Some(s2)) => {
            let diff = compare_scans(&s1, &s2);
            let formatter = get_formatter(&format)?;
            println!("{}", formatter.format(&diff));
        }
        _ => println!("One or both scans not found."),
    }
    Ok(())
}

fn get_formatter(format: &str) -> Result<Box<dyn Formatter>> {
    match format {
        "text" => Ok(Box::new(TextFormatter)),
        "json" => Ok(Box::new(code_guardian_output::formatters::JsonFormatter)),
        "csv" => Ok(Box::new(code_guardian_output::formatters::CsvFormatter)),
        "markdown" => Ok(Box::new(
            code_guardian_output::formatters::MarkdownFormatter,
        )),
        "html" => Ok(Box::new(code_guardian_output::formatters::HtmlFormatter)),
        _ => Err(anyhow::anyhow!("Unsupported format: {}", format)),
    }
}

fn compare_scans(scan1: &Scan, scan2: &Scan) -> Vec<Match> {
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
