use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use code_guardian_storage::ScanRepository;
use std::io;
use std::path::PathBuf;



mod benchmark;
mod advanced_handlers;
mod utils;
mod scan_handlers;
mod report_handlers;
mod comparison_handlers;

use advanced_handlers::*;

#[derive(Parser)]
#[command(
    name = "code-guardian",
    about = "A tool to scan codebases for patterns like TODO and FIXME",
    long_about = "Code Guardian is a command-line tool designed to scan your codebase for common patterns such as TODO and FIXME comments. It helps developers track unfinished work and potential issues in their code.\n\nUse subcommands to perform scans, view history, generate reports, and compare scans.",
    version
)]
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
        /// Database file path (optional, defaults to data/code-guardian.db)
        #[arg(short, long)]
        db: Option<PathBuf>,
        /// Config file path (optional)
        #[arg(short, long)]
        config: Option<PathBuf>,
        /// Detector profile: basic, comprehensive, security, performance, rust
        #[arg(long, default_value = "basic")]
        profile: String,
        /// Show progress bar
        #[arg(long)]
        progress: bool,
        /// Use optimized scanner for better performance
        #[arg(long)]
        optimize: bool,
        /// Use streaming scanner for large codebases
        #[arg(long)]
        streaming: bool,
        /// Show performance metrics
        #[arg(long)]
        metrics: bool,
        /// Use incremental scanning (only scan changed files)
        #[arg(long)]
        incremental: bool,
        /// Use distributed scanning across multiple workers
        #[arg(long)]
        distributed: bool,
        /// Path to custom detectors configuration file
        #[arg(long)]
        custom_detectors: Option<PathBuf>,
        /// Cache size for optimized scanning
        #[arg(long)]
        cache_size: Option<usize>,
        /// Batch size for distributed scanning
        #[arg(long)]
        batch_size: Option<usize>,
        /// Maximum file size to scan (in bytes)
        #[arg(long)]
        max_file_size: Option<usize>,
        /// Maximum number of threads
        #[arg(long)]
        max_threads: Option<usize>,
    },
    /// List all scan history from the database
    History {
        /// Database file path (optional, defaults to data/code-guardian.db)
        #[arg(short, long, help = "Specify the database file path. If not provided, uses 'data/code-guardian.db'")]
        db: Option<PathBuf>,
    },
    /// Generate a report for a specific scan in various formats
    Report {
        /// Scan ID to generate report for
        id: i64,
        /// Output format: text, json, csv, markdown, html (default: text)
        #[arg(short, long, default_value = "text", help = "Choose the output format for the report")]
        format: String,
        /// Database file path (optional, defaults to data/code-guardian.db)
        #[arg(short, long, help = "Specify the database file path. If not provided, uses 'data/code-guardian.db'")]
        db: Option<PathBuf>,
    },
    /// Compare two scans and show differences
    Compare {
        /// First scan ID
        id1: i64,
        /// Second scan ID
        id2: i64,
        /// Output format: text, json, csv, markdown, html (default: text)
        #[arg(short, long, default_value = "text", help = "Choose the output format for the comparison")]
        format: String,
        /// Database file path (optional, defaults to data/code-guardian.db)
        #[arg(short, long, help = "Specify the database file path. If not provided, uses 'data/code-guardian.db'")]
        db: Option<PathBuf>,
    },
    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completion for (bash, zsh, fish, etc.)
        shell: Shell,
    },
    /// Run performance benchmark
    Benchmark {
        /// Path to benchmark (optional, defaults to current directory)
        path: Option<PathBuf>,
        /// Run quick test only
        #[arg(long)]
        quick: bool,
    },
    /// Manage custom detectors
    CustomDetectors {
        #[command(subcommand)]
        action: CustomDetectorAction,
    },
    /// Incremental scan management
    Incremental {
        #[command(subcommand)]
        action: IncrementalAction,
    },
    /// Distributed scanning setup
    Distributed {
        #[command(subcommand)]
        action: DistributedAction,
    },
}

#[derive(Subcommand)]
enum CustomDetectorAction {
    /// List all custom detectors
    List,
    /// Create example custom detectors
    CreateExamples {
        /// Output file for examples
        #[arg(short, long, default_value = "custom_detectors.json")]
        output: PathBuf,
    },
    /// Load custom detectors from file
    Load {
        /// Path to custom detectors file
        file: PathBuf,
    },
    /// Test custom detectors on a file
    Test {
        /// Path to detectors file
        detectors: PathBuf,
        /// Path to test file
        test_file: PathBuf,
    },
}

#[derive(Subcommand)]
enum IncrementalAction {
    /// Show incremental scan status
    Status,
    /// Force full rescan on next scan
    Reset,
    /// Show incremental scan statistics
    Stats,
}

#[derive(Subcommand)]
enum DistributedAction {
    /// Setup distributed scanning
    Setup {
        /// Number of worker nodes to simulate
        #[arg(short, long, default_value = "4")]
        workers: usize,
    },
    /// Run distributed scan
    Scan {
        /// Path to scan
        path: PathBuf,
        /// Number of workers
        #[arg(short, long, default_value = "4")]
        workers: usize,
        /// Batch size per worker
        #[arg(short, long, default_value = "50")]
        batch_size: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, db, config, profile, progress, optimize, streaming, metrics, incremental, distributed, custom_detectors, cache_size, batch_size, max_file_size, max_threads } => {
            let options = scan_handlers::ScanOptions {
                path,
                db,
                config_path: config,
                profile,
                show_progress: progress,
                optimize,
                streaming,
                show_metrics: metrics,
                incremental,
                distributed,
                custom_detectors,
                cache_size,
                batch_size,
                max_file_size,
                max_threads,
            };
            scan_handlers::handle_scan(options)
        }
        Commands::History { db } => handle_history(db),
        Commands::Report { id, format, db } => report_handlers::handle_report(id, format, db),
        Commands::Compare {
            id1,
            id2,
            format,
            db,
        } => comparison_handlers::handle_compare(id1, id2, format, db),
        Commands::Completion { shell } => handle_completion(shell),
        Commands::Benchmark { path, quick } => handle_benchmark(path, quick),
        Commands::CustomDetectors { action } => handle_custom_detectors(action),
        Commands::Incremental { action } => handle_incremental(action),
        Commands::Distributed { action } => handle_distributed(action),
    }
}

fn handle_history(db: Option<PathBuf>) -> Result<()> {
    let db_path = utils::get_db_path(db);
    let repo = code_guardian_storage::SqliteScanRepository::new(&db_path)?;
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

fn handle_completion(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    generate(shell, &mut cmd, bin_name, &mut io::stdout());
    Ok(())
}

fn handle_benchmark(path: Option<PathBuf>, quick: bool) -> Result<()> {
    let benchmark_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());

    if quick {
        benchmark::quick_performance_test(&benchmark_path)
    } else {
        benchmark::run_benchmark(&benchmark_path)
    }
}
