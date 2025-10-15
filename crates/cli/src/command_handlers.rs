use anyhow::Result;
use clap::CommandFactory;
use clap_complete::generate;
use clap_complete::Shell;
use code_guardian_storage::ScanRepository;
use std::io;
use std::path::PathBuf;

use crate::benchmark;
use crate::cli_definitions::{Cli, GitAction};
use crate::git_integration::GitIntegration;
use crate::utils;

/// Handle history command - show all scan history from database
pub fn handle_history(db: Option<PathBuf>) -> Result<()> {
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

/// Handle shell completion generation
pub fn handle_completion(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    generate(shell, &mut cmd, bin_name, &mut io::stdout());
    Ok(())
}

/// Handle benchmark command
pub fn handle_benchmark(path: Option<PathBuf>, quick: bool) -> Result<()> {
    let benchmark_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());

    if quick {
        benchmark::quick_performance_test(&benchmark_path)
    } else {
        benchmark::run_benchmark(&benchmark_path)
    }
}

/// Handle comprehensive benchmark command with suite selection
#[allow(dead_code)]
pub fn handle_comprehensive_benchmark(path: Option<PathBuf>, suite: String) -> Result<()> {
    let benchmark_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
    benchmark::run_comprehensive_benchmark(&benchmark_path, &suite)
}

/// Handle performance analysis command
#[allow(dead_code)]
pub fn handle_performance_analysis(path: Option<PathBuf>) -> Result<()> {
    let analysis_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
    benchmark::run_performance_analysis(&analysis_path)
}

// These functions are re-exported from advanced_handlers
pub use crate::advanced_handlers::{
    handle_custom_detectors, handle_distributed, handle_incremental,
};

/// Handle git integration commands
pub fn handle_git(action: GitAction) -> Result<()> {
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
