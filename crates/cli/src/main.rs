use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use code_guardian_storage::ScanRepository;
use std::io;
use std::path::PathBuf;

mod advanced_handlers;
mod benchmark;
mod comparison_handlers;
mod git_integration;
mod production_handlers;
mod report_handlers;
mod scan_handlers;
mod utils;

use advanced_handlers::*;
use git_integration::GitIntegration;
use production_handlers::*;

#[derive(Parser)]
#[command(
    name = "code-guardian",
    about = "Multi-language code analysis tool for production readiness and code quality",
    long_about = "Code Guardian is a comprehensive code analysis tool that scans codebases across 30+ programming languages for production readiness issues, code quality problems, and development artifacts.\n\nFeatures:\n• Production readiness scanning (console.log, debugger, dev/staging code)\n• Multi-language support (JavaScript, TypeScript, Python, Rust, Go, Java, C#, PHP, etc.)\n• Technology stack presets (web, backend, fullstack, mobile, systems)\n• CI/CD integration with proper exit codes\n• Pre-commit hooks for code quality gates\n• Real-time file watching (coming soon)\n\nUse subcommands for different scanning modes and integrations.",
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

#[derive(Subcommand)]
enum StackPreset {
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
enum GitAction {
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            path,
            db,
            config,
            profile,
            progress,
            optimize,
            streaming,
            metrics,
            incremental,
            distributed,
            custom_detectors,
            cache_size,
            batch_size,
            max_file_size,
            max_threads,
        } => {
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
        Commands::ProductionCheck {
            path,
            format,
            fail_on_critical,
            fail_on_high,
            severity,
            output,
        } => handle_production_check(
            path,
            format,
            fail_on_critical,
            fail_on_high,
            severity,
            output,
        ),
        Commands::PreCommit {
            path,
            staged_only,
            fast,
        } => handle_pre_commit(path, staged_only, fast),
        Commands::CiGate {
            path,
            config,
            output,
            max_critical,
            max_high,
        } => handle_ci_gate(path, config, output, max_critical, max_high),
        Commands::Lang {
            languages,
            path,
            format,
            production,
        } => handle_lang_scan(languages, path, format, production),
        Commands::Stack { preset } => handle_stack_preset_main(preset),
        Commands::Watch {
            path,
            include,
            exclude,
            delay,
        } => handle_watch(path, include, exclude, delay),
        Commands::Git { action } => handle_git(action),
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

fn handle_stack_preset_main(preset: StackPreset) -> Result<()> {
    match preset {
        StackPreset::Web { path, production } => {
            let languages = vec![
                "js".to_string(),
                "ts".to_string(),
                "jsx".to_string(),
                "tsx".to_string(),
                "vue".to_string(),
                "svelte".to_string(),
            ];
            handle_lang_scan(languages, path, "text".to_string(), production)
        }
        StackPreset::Backend { path, production } => {
            let languages = vec![
                "py".to_string(),
                "java".to_string(),
                "go".to_string(),
                "cs".to_string(),
                "php".to_string(),
                "rb".to_string(),
            ];
            handle_lang_scan(languages, path, "text".to_string(), production)
        }
        StackPreset::Fullstack { path, production } => {
            let languages = vec![
                "js".to_string(),
                "ts".to_string(),
                "py".to_string(),
                "java".to_string(),
                "go".to_string(),
                "rs".to_string(),
            ];
            handle_lang_scan(languages, path, "text".to_string(), production)
        }
        StackPreset::Mobile { path, production } => {
            let languages = vec![
                "js".to_string(),
                "ts".to_string(),
                "swift".to_string(),
                "kt".to_string(),
                "dart".to_string(),
            ];
            handle_lang_scan(languages, path, "text".to_string(), production)
        }
        StackPreset::Systems { path, production } => {
            let languages = vec![
                "rs".to_string(),
                "cpp".to_string(),
                "c".to_string(),
                "go".to_string(),
            ];
            handle_lang_scan(languages, path, "text".to_string(), production)
        }
    }
}

fn handle_git(action: GitAction) -> Result<()> {
    match action {
        GitAction::InstallHook { path } => {
            println!("🔧 Installing Code-Guardian pre-commit hook...");

            if !GitIntegration::is_git_repo(&path) {
                eprintln!("❌ Error: {} is not a git repository", path.display());
                std::process::exit(1);
            }

            let repo_root = GitIntegration::get_repo_root(&path)?;
            GitIntegration::install_pre_commit_hook(&repo_root)?;

            println!("💡 Usage: The hook will automatically run on 'git commit'");
            println!("💡 Manual run: code-guardian pre-commit --staged-only --fast");
            Ok(())
        }
        GitAction::UninstallHook { path } => {
            println!("🗑️  Uninstalling Code-Guardian pre-commit hook...");

            if !GitIntegration::is_git_repo(&path) {
                eprintln!("❌ Error: {} is not a git repository", path.display());
                std::process::exit(1);
            }

            let repo_root = GitIntegration::get_repo_root(&path)?;
            GitIntegration::uninstall_pre_commit_hook(&repo_root)?;
            Ok(())
        }
        GitAction::Staged { path } => {
            println!("📋 Listing staged files...");

            if !GitIntegration::is_git_repo(&path) {
                eprintln!("❌ Error: {} is not a git repository", path.display());
                std::process::exit(1);
            }

            let repo_root = GitIntegration::get_repo_root(&path)?;
            let staged_files = GitIntegration::get_staged_files(&repo_root)?;

            if staged_files.is_empty() {
                println!("ℹ️  No staged files found.");
            } else {
                println!("🔍 Found {} staged file(s):", staged_files.len());
                for (i, file) in staged_files.iter().enumerate() {
                    println!("  {}. {}", i + 1, file.display());
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_help_generation() {
        let mut cli = Cli::command();
        let help_output = cli.render_help();

        // Verify the CLI help contains our new commands
        assert!(help_output.to_string().contains("production-check"));
        assert!(help_output.to_string().contains("pre-commit"));
        assert!(help_output.to_string().contains("ci-gate"));
        assert!(help_output.to_string().contains("lang"));
        assert!(help_output.to_string().contains("stack"));
        assert!(help_output.to_string().contains("watch"));
    }

    #[test]
    fn test_cli_description() {
        let cli = Cli::command();
        let about = cli.get_about().unwrap().to_string();

        // Verify the CLI description mentions key features
        assert!(about.contains("Multi-language"));
        assert!(about.contains("production readiness"));
        assert!(about.contains("code quality"));
        // The about text is: "Multi-language code analysis tool for production readiness and code quality"
    }

    #[test]
    fn test_stack_preset_variants() {
        // Test that all stack presets are properly defined
        let web_preset = StackPreset::Web {
            path: PathBuf::from("."),
            production: false,
        };
        let backend_preset = StackPreset::Backend {
            path: PathBuf::from("."),
            production: false,
        };
        let fullstack_preset = StackPreset::Fullstack {
            path: PathBuf::from("."),
            production: false,
        };
        let mobile_preset = StackPreset::Mobile {
            path: PathBuf::from("."),
            production: false,
        };
        let systems_preset = StackPreset::Systems {
            path: PathBuf::from("."),
            production: false,
        };

        // Basic validation that variants exist
        match web_preset {
            StackPreset::Web { .. } => (),
            _ => panic!("Web preset should match"),
        }
        match backend_preset {
            StackPreset::Backend { .. } => (),
            _ => panic!("Backend preset should match"),
        }
        match fullstack_preset {
            StackPreset::Fullstack { .. } => (),
            _ => panic!("Fullstack preset should match"),
        }
        match mobile_preset {
            StackPreset::Mobile { .. } => (),
            _ => panic!("Mobile preset should match"),
        }
        match systems_preset {
            StackPreset::Systems { .. } => (),
            _ => panic!("Systems preset should match"),
        }
    }
}
