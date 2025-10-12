use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "code-guardian",
    about = "Multi-language code analysis tool for production readiness and code quality",
    long_about = "Code Guardian is a comprehensive code analysis tool that scans codebases across 30+ programming languages for production readiness issues, code quality problems, and development artifacts.\n\nFeatures:\n• Production readiness scanning (console.log, debugger, dev/staging code)\n• Multi-language support (JavaScript, TypeScript, Python, Rust, Go, Java, C#, PHP, etc.)\n• Technology stack presets (web, backend, fullstack, mobile, systems)\n• CI/CD integration with proper exit codes",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
        #[arg(
            short,
            long,
            help = "Specify the database file path. If not provided, uses 'data/code-guardian.db'"
        )]
        db: Option<PathBuf>,
    },
    /// Generate a report for a specific scan in various formats
    Report {
        /// Scan ID to generate report for
        id: i64,
        /// Output format: text, json, csv, markdown, html (default: text)
        #[arg(
            short,
            long,
            default_value = "text",
            help = "Choose the output format for the report"
        )]
        format: String,
        /// Database file path (optional, defaults to data/code-guardian.db)
        #[arg(
            short,
            long,
            help = "Specify the database file path. If not provided, uses 'data/code-guardian.db'"
        )]
        db: Option<PathBuf>,
    },
    /// Compare two scans and show differences
    Compare {
        /// First scan ID
        id1: i64,
        /// Second scan ID
        id2: i64,
        /// Output format: text, json, csv, markdown, html (default: text)
        #[arg(
            short,
            long,
            default_value = "text",
            help = "Choose the output format for the comparison"
        )]
        format: String,
        /// Database file path (optional, defaults to data/code-guardian.db)
        #[arg(
            short,
            long,
            help = "Specify the database file path. If not provided, uses 'data/code-guardian.db'"
        )]
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
    /// Quick production readiness check
    ProductionCheck {
        /// Path to the directory to scan
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Output format: text, json, summary (default: text)
        #[arg(short, long, default_value = "text")]
        format: String,
        /// Exit with non-zero code if critical issues found
        #[arg(long)]
        fail_on_critical: bool,
        /// Exit with non-zero code if high severity issues found
        #[arg(long)]
        fail_on_high: bool,
        /// Only show issues with specified severity levels
        #[arg(long, value_delimiter = ',')]
        severity: Vec<String>,
        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Pre-commit hook for checking code quality
    PreCommit {
        /// Path to the directory to scan
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Check only staged files
        #[arg(long)]
        staged_only: bool,
        /// Fast mode - only critical and high severity issues
        #[arg(long)]
        fast: bool,
    },
    /// CI/CD gate with proper exit codes
    CiGate {
        /// Path to the directory to scan
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Configuration file for CI settings
        #[arg(short, long)]
        config: Option<PathBuf>,
        /// Output JSON report to file
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Maximum allowed critical issues (default: 0)
        #[arg(long, default_value = "0")]
        max_critical: u32,
        /// Maximum allowed high severity issues (default: 5)
        #[arg(long, default_value = "5")]
        max_high: u32,
    },
    /// Language-specific scanning presets
    Lang {
        /// Target languages (comma-separated): js,ts,py,rs,go,java,cs,php,etc.
        #[arg(value_delimiter = ',', required = true)]
        languages: Vec<String>,
        /// Path to the directory to scan
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        /// Output format: text, json, summary (default: text)
        #[arg(short, long, default_value = "text")]
        format: String,
        /// Include production readiness checks
        #[arg(long)]
        production: bool,
    },
    /// Technology stack presets
    Stack {
        #[command(subcommand)]
        preset: StackPreset,
    },
    /// Live scanning with file watching
    Watch {
        /// Path to the directory to watch
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Only scan files matching these patterns
        #[arg(long, value_delimiter = ',')]
        include: Vec<String>,
        /// Exclude files matching these patterns  
        #[arg(long, value_delimiter = ',')]
        exclude: Vec<String>,
        /// Debounce delay in milliseconds
        #[arg(long, default_value = "500")]
        delay: u64,
    },
    /// Git integration and hook management
    Git {
        #[command(subcommand)]
        action: GitAction,
    },
}

#[derive(Subcommand)]
pub enum CustomDetectorAction {
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
pub enum IncrementalAction {
    /// Show incremental scan status
    Status,
    /// Force full rescan on next scan
    Reset,
    /// Show incremental scan statistics
    Stats,
}

#[derive(Subcommand)]
pub enum DistributedAction {
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

#[derive(Subcommand)]
pub enum StackPreset {
    /// Web frontend (JavaScript, TypeScript, React, Vue, etc.)
    Web {
        /// Path to scan
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Include production readiness checks
        #[arg(long)]
        production: bool,
    },
    /// Backend services (Python, Java, Go, C#, etc.)
    Backend {
        /// Path to scan
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Include production readiness checks
        #[arg(long)]
        production: bool,
    },
    /// Full-stack monorepo
    Fullstack {
        /// Path to scan
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Include production readiness checks
        #[arg(long)]
        production: bool,
    },
    /// Mobile development (React Native, Flutter, Swift, Kotlin)
    Mobile {
        /// Path to scan
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Include production readiness checks
        #[arg(long)]
        production: bool,
    },
    /// Systems programming (Rust, C++, C)
    Systems {
        /// Path to scan
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Include production readiness checks
        #[arg(long)]
        production: bool,
    },
}

#[derive(Subcommand)]
pub enum GitAction {
    /// Install pre-commit hook
    InstallHook {
        /// Path to git repository (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Uninstall pre-commit hook
    UninstallHook {
        /// Path to git repository (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// List staged files that would be scanned
    Staged {
        /// Path to git repository (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}
