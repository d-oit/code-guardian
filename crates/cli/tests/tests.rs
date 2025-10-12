extern crate code_guardian_cli;

use clap::Parser;
use code_guardian_cli::cli_definitions::{Cli, StackPreset};
use std::path::PathBuf;

#[cfg(test)]
#[test]
fn test_cli_help_generation() {
    use clap::CommandFactory;
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

#[cfg(test)]
#[test]
fn test_cli_description() {
    use clap::CommandFactory;
    let cli = Cli::command();
    let about = cli.get_about().unwrap().to_string();

    // Verify the CLI description mentions key features
    assert!(about.contains("Multi-language"));
    assert!(about.contains("production readiness"));
    assert!(about.contains("code quality"));
    // The about text is: "Multi-language code analysis tool for production readiness and code quality"
}

#[cfg(test)]
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

#[cfg(test)]
#[test]
fn test_cli_parse_invalid_subcommand() {
    let args = vec!["code-guardian", "invalid"];
    let result = Cli::try_parse_from(args);
    assert!(result.is_err());
}

#[cfg(test)]
#[test]
fn test_cli_parse_scan_missing_path() {
    let args = vec!["code-guardian", "scan"];
    let result = Cli::try_parse_from(args);
    assert!(result.is_err());
}

#[cfg(test)]
#[test]
fn test_cli_parse_report_missing_id() {
    let args = vec!["code-guardian", "report"];
    let result = Cli::try_parse_from(args);
    assert!(result.is_err());
}

#[cfg(test)]
#[test]
fn test_handle_history_invalid_db() {
    use code_guardian_cli::command_handlers::handle_history;
    let invalid_db = PathBuf::from("/invalid/path/db.db");
    let result = handle_history(Some(invalid_db));
    assert!(result.is_err());
}

#[cfg(test)]
#[test]
fn test_handle_benchmark_invalid_path() {
    use code_guardian_cli::command_handlers::handle_benchmark;
    let invalid_path = PathBuf::from("/invalid/path");
    let result = handle_benchmark(Some(invalid_path), false);
    assert!(result.is_err());
}
