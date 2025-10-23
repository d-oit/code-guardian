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
        let id = scan.id.ok_or_else(|| anyhow::anyhow!("Scan missing ID"))?;
        let timestamp = chrono::DateTime::from_timestamp(scan.timestamp, 0)
            .ok_or_else(|| anyhow::anyhow!("Invalid timestamp: {}", scan.timestamp))?;
        println!(
            "ID: {}, Timestamp: {}, Path: {}",
            id,
            timestamp.format("%Y-%m-%d %H:%M:%S"),
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
    let benchmark_path = match path {
        Some(p) => p,
        None => std::env::current_dir()
            .map_err(|e| anyhow::anyhow!("Failed to get current directory: {}", e))?,
    };

    // Validate that the path exists
    if !benchmark_path.exists() {
        return Err(anyhow::anyhow!(
            "Path does not exist: {}",
            benchmark_path.display()
        ));
    }

    if quick {
        benchmark::quick_performance_test(&benchmark_path)
    } else {
        benchmark::run_benchmark(&benchmark_path)
    }
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
                return Err(anyhow::anyhow!(
                    "❌ Error: {} is not a git repository",
                    path.display()
                ));
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
                return Err(anyhow::anyhow!(
                    "❌ Error: {} is not a git repository",
                    path.display()
                ));
            }

            let repo_root = GitIntegration::get_repo_root(&path)?;
            GitIntegration::uninstall_pre_commit_hook(&repo_root)?;
            Ok(())
        }
        GitAction::Staged { path } => {
            println!("📋 Listing staged files...");

            if !GitIntegration::is_git_repo(&path) {
                return Err(anyhow::anyhow!(
                    "❌ Error: {} is not a git repository",
                    path.display()
                ));
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
