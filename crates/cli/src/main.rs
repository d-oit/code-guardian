use anyhow::Result;
use clap::Parser;

// Module declarations
mod advanced_handlers;
mod benchmark;
mod cli_definitions;
mod command_handlers;
mod comparison_handlers;
mod git_integration;
mod production_handlers;
mod report_handlers;
mod scan_handlers;
mod stack_presets;
#[cfg(test)]
mod tests;
mod utils;

// Import the CLI definitions and command handlers
use cli_definitions::{Cli, Commands};
use command_handlers::*;
use comparison_handlers::*;
use production_handlers::*;
use report_handlers::*;
use scan_handlers::*;
use stack_presets::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

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
            let options = ScanOptions {
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
            handle_scan(options).await
        }
        Commands::History { db } => handle_history(db),
        Commands::Report { id, format, db } => handle_report(id, format, db),
        Commands::Compare {
            id1,
            id2,
            format,
            db,
        } => handle_compare(id1, id2, format, db),
        Commands::Completion { shell } => handle_completion(shell),
        Commands::Benchmark { path, quick } => handle_benchmark(path, quick),
        Commands::CustomDetectors { action } => handle_custom_detectors(action),
        Commands::Incremental { action } => handle_incremental(action),
        Commands::Distributed { action } => handle_distributed(action).await,
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
        Commands::Stack { preset } => handle_stack_preset(preset),
        Commands::Watch {
            path,
            include,
            exclude,
            delay,
        } => handle_watch(path, include, exclude, delay),
        Commands::Git { action } => handle_git(action),
    }
}
