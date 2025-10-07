use anyhow::Result;
use code_guardian_core::{AlertDetector, ConsoleLogDetector, DebuggerDetector};
use code_guardian_core::{DetectorFactory, Match, PatternDetector, Scanner};
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process;

/// Handle production readiness check command
pub fn handle_production_check(
    path: PathBuf,
    format: String,
    fail_on_critical: bool,
    fail_on_high: bool,
    severity_filter: Vec<String>,
    output: Option<PathBuf>,
) -> Result<()> {
    println!(
        "🔍 {} Production Readiness Check",
        "Code-Guardian".bold().blue()
    );
    println!("📁 Scanning: {}", path.display());

    // Create production-ready detectors
    let detectors = DetectorFactory::create_production_ready_detectors();
    let scanner = Scanner::new(detectors);

    // Perform the scan
    let matches = scanner.scan(&path)?;

    // Filter by severity if specified
    let filtered_matches = if severity_filter.is_empty() {
        matches
    } else {
        filter_by_severity(matches, &severity_filter)
    };

    // Count issues by severity
    let severity_counts = count_by_severity(&filtered_matches);

    // Generate output based on format
    let output_content = match format.as_str() {
        "json" => generate_json_output(&filtered_matches, &severity_counts)?,
        "summary" => generate_summary_output(&filtered_matches, &severity_counts),
        _ => generate_production_text_output(&filtered_matches, &severity_counts),
    };

    // Write to file if specified, otherwise print to stdout
    if let Some(output_path) = output {
        fs::write(&output_path, &output_content)?;
        println!("📄 Report saved to: {}", output_path.display());
    } else {
        println!("{}", output_content);
    }

    // Exit with appropriate code for CI/CD integration
    let critical_count = severity_counts.get("Critical").unwrap_or(&0);
    let high_count = severity_counts.get("High").unwrap_or(&0);

    if fail_on_critical && *critical_count > 0 {
        eprintln!(
            "❌ Production check FAILED: {} critical issues found",
            critical_count
        );
        process::exit(1);
    }

    if fail_on_high && *high_count > 0 {
        eprintln!(
            "⚠️  Production check FAILED: {} high severity issues found",
            high_count
        );
        process::exit(1);
    }

    if *critical_count > 0 || *high_count > 0 {
        println!(
            "⚠️  Production readiness: {} - Address critical and high severity issues",
            "NEEDS ATTENTION".yellow()
        );
    } else {
        println!("✅ Production readiness: {}", "PASSED".green());
    }

    Ok(())
}

/// Handle pre-commit hook command
pub fn handle_pre_commit(path: PathBuf, staged_only: bool, fast: bool) -> Result<()> {
    println!("🔧 {} Pre-commit Check", "Code-Guardian".bold().cyan());

    let detectors: Vec<Box<dyn PatternDetector>> = if fast {
        // Fast mode: only critical issues
        vec![
            Box::new(DebuggerDetector),
            Box::new(ConsoleLogDetector),
            Box::new(AlertDetector),
        ]
    } else {
        DetectorFactory::create_production_ready_detectors()
    };

    let scanner = Scanner::new(detectors);
    let matches = if staged_only {
        // TODO: Implement git diff --cached integration
        scanner.scan(&path)?
    } else {
        scanner.scan(&path)?
    };

    let severity_counts = count_by_severity(&matches);
    let critical_count = severity_counts.get("Critical").unwrap_or(&0);
    let high_count = severity_counts.get("High").unwrap_or(&0);

    if *critical_count > 0 {
        eprintln!(
            "❌ Pre-commit check FAILED: {} critical issues",
            critical_count
        );
        for m in matches.iter().filter(|m| is_critical_severity(&m.pattern)) {
            eprintln!("  {} [{}] {}", m.file_path, m.pattern.red(), m.message);
        }
        process::exit(1);
    }

    if *high_count > 0 {
        println!(
            "⚠️  {} high severity issues found (warnings only)",
            high_count
        );
        for m in matches.iter().filter(|m| is_high_severity(&m.pattern)) {
            println!("  {} [{}] {}", m.file_path, m.pattern.yellow(), m.message);
        }
    }

    println!("✅ Pre-commit check passed");
    Ok(())
}

/// Handle CI/CD gate command
pub fn handle_ci_gate(
    path: PathBuf,
    _config: Option<PathBuf>,
    output: Option<PathBuf>,
    max_critical: u32,
    max_high: u32,
) -> Result<()> {
    println!("🚦 {} CI/CD Gate", "Code-Guardian".bold().green());

    let detectors = DetectorFactory::create_production_ready_detectors();
    let scanner = Scanner::new(detectors);
    let matches = scanner.scan(&path)?;

    let severity_counts = count_by_severity(&matches);
    let critical_count = *severity_counts.get("Critical").unwrap_or(&0) as u32;
    let high_count = *severity_counts.get("High").unwrap_or(&0) as u32;

    // Generate JSON report for CI/CD systems
    let report = serde_json::json!({
        "status": if critical_count <= max_critical && high_count <= max_high { "PASS" } else { "FAIL" },
        "summary": {
            "critical": critical_count,
            "high": high_count,
            "total": matches.len()
        },
        "thresholds": {
            "max_critical": max_critical,
            "max_high": max_high
        },
        "matches": matches.iter().map(|m| serde_json::json!({
            "file": m.file_path,
            "line": m.line_number,
            "column": m.column,
            "pattern": m.pattern,
            "message": m.message,
            "severity": get_severity_for_pattern(&m.pattern)
        })).collect::<Vec<_>>()
    });

    let json_output = serde_json::to_string_pretty(&report)?;

    if let Some(output_path) = output {
        fs::write(&output_path, &json_output)?;
        println!("📄 CI report saved to: {}", output_path.display());
    }

    // Print summary
    println!("📊 Results:");
    println!("  Critical: {}/{}", critical_count, max_critical);
    println!("  High: {}/{}", high_count, max_high);

    if critical_count > max_critical {
        eprintln!(
            "❌ CI Gate FAILED: Too many critical issues ({} > {})",
            critical_count, max_critical
        );
        process::exit(1);
    }

    if high_count > max_high {
        eprintln!(
            "❌ CI Gate FAILED: Too many high severity issues ({} > {})",
            high_count, max_high
        );
        process::exit(1);
    }

    println!("✅ CI Gate PASSED");
    Ok(())
}

/// Handle language-specific scanning
pub fn handle_lang_scan(
    languages: Vec<String>,
    path: PathBuf,
    format: String,
    production: bool,
) -> Result<()> {
    println!(
        "🌍 {} Language-Specific Scan",
        "Code-Guardian".bold().magenta()
    );
    println!("🎯 Languages: {}", languages.join(", "));

    let extensions = map_languages_to_extensions(&languages);
    println!("📁 File extensions: {}", extensions.join(", "));

    let detectors = if production {
        DetectorFactory::create_production_ready_detectors()
    } else {
        DetectorFactory::create_comprehensive_detectors()
    };

    let scanner = Scanner::new(detectors);
    let all_matches = scanner.scan(&path)?;

    // Filter matches to only include specified language extensions
    let filtered_matches: Vec<Match> = all_matches
        .into_iter()
        .filter(|m| {
            extensions
                .iter()
                .any(|ext| m.file_path.ends_with(&format!(".{}", ext)))
        })
        .collect();

    let severity_counts = count_by_severity(&filtered_matches);

    match format.as_str() {
        "json" => {
            let json_output = generate_json_output(&filtered_matches, &severity_counts)?;
            println!("{}", json_output);
        }
        "summary" => {
            let summary = generate_summary_output(&filtered_matches, &severity_counts);
            println!("{}", summary);
        }
        _ => {
            let text_output = generate_production_text_output(&filtered_matches, &severity_counts);
            println!("{}", text_output);
        }
    }

    Ok(())
}

/// Handle technology stack presets
pub fn handle_stack_preset(preset: crate::StackPreset) -> Result<()> {
    use crate::StackPreset;

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

/// Handle file watching command
pub fn handle_watch(
    _path: PathBuf,
    _include: Vec<String>,
    _exclude: Vec<String>,
    _delay: u64,
) -> Result<()> {
    println!("👁️  {} File Watching", "Code-Guardian".bold().cyan());
    println!("⚠️  File watching feature coming soon!");
    println!("📖 This will enable real-time scanning as you edit files");
    Ok(())
}

// Helper functions

fn filter_by_severity(matches: Vec<Match>, severity_filter: &[String]) -> Vec<Match> {
    matches
        .into_iter()
        .filter(|m| {
            let severity = get_severity_for_pattern(&m.pattern);
            severity_filter.contains(&severity)
        })
        .collect()
}

fn count_by_severity(matches: &[Match]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for m in matches {
        let severity = get_severity_for_pattern(&m.pattern);
        *counts.entry(severity).or_insert(0) += 1;
    }
    counts
}

fn get_severity_for_pattern(pattern: &str) -> String {
    match pattern {
        "DEBUGGER" => "Critical",
        "DEV" | "STAGING" | "CONSOLE_LOG" | "ALERT" => "High",
        "DEBUG" | "TEST" | "PHASE" | "PRINT" | "DEAD_CODE" | "EXPERIMENTAL" | "FIXME" | "PANIC"
        | "UNWRAP" => "Medium",
        _ => "Low",
    }
    .to_string()
}

fn is_critical_severity(pattern: &str) -> bool {
    matches!(pattern, "DEBUGGER")
}

fn is_high_severity(pattern: &str) -> bool {
    matches!(pattern, "DEV" | "STAGING" | "CONSOLE_LOG" | "ALERT")
}

fn map_languages_to_extensions(languages: &[String]) -> Vec<String> {
    let mut extensions = Vec::new();
    for lang in languages {
        match lang.as_str() {
            "js" | "javascript" => extensions.extend_from_slice(&["js", "jsx"]),
            "ts" | "typescript" => extensions.extend_from_slice(&["ts", "tsx"]),
            "py" | "python" => extensions.push("py"),
            "rs" | "rust" => extensions.push("rs"),
            "go" => extensions.push("go"),
            "java" => extensions.push("java"),
            "cs" | "csharp" => extensions.push("cs"),
            "php" => extensions.push("php"),
            "rb" | "ruby" => extensions.push("rb"),
            "kt" | "kotlin" => extensions.push("kt"),
            "swift" => extensions.push("swift"),
            "dart" => extensions.push("dart"),
            "cpp" | "c++" => extensions.extend_from_slice(&["cpp", "cxx", "cc"]),
            "c" => extensions.extend_from_slice(&["c", "h"]),
            "vue" => extensions.push("vue"),
            "svelte" => extensions.push("svelte"),
            _ => extensions.push(lang), // Pass through unknown extensions
        }
    }
    extensions.into_iter().map(String::from).collect()
}

fn generate_json_output(
    matches: &[Match],
    severity_counts: &HashMap<String, usize>,
) -> Result<String> {
    let output = serde_json::json!({
        "summary": severity_counts,
        "total": matches.len(),
        "matches": matches
    });
    Ok(serde_json::to_string_pretty(&output)?)
}

fn generate_summary_output(matches: &[Match], severity_counts: &HashMap<String, usize>) -> String {
    let mut output = String::new();
    output.push_str(&format!("📊 {} Summary\n", "Code-Guardian".bold()));
    output.push_str(&format!("Total Issues: {}\n", matches.len()));

    for (severity, count) in severity_counts {
        let icon = match severity.as_str() {
            "Critical" => "🔴",
            "High" => "🟠",
            "Medium" => "🟡",
            "Low" => "🟢",
            _ => "⚪",
        };
        output.push_str(&format!("{} {}: {}\n", icon, severity, count));
    }
    output
}

fn generate_production_text_output(
    matches: &[Match],
    severity_counts: &HashMap<String, usize>,
) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "🔍 {} Production Readiness Report\n\n",
        "Code-Guardian".bold().blue()
    ));

    // Group matches by severity
    let mut critical_issues = Vec::new();
    let mut high_issues = Vec::new();
    let mut medium_issues = Vec::new();
    let mut low_issues = Vec::new();

    for m in matches {
        match get_severity_for_pattern(&m.pattern).as_str() {
            "Critical" => critical_issues.push(m),
            "High" => high_issues.push(m),
            "Medium" => medium_issues.push(m),
            "Low" => low_issues.push(m),
            _ => {}
        }
    }

    // Display issues by severity
    if !critical_issues.is_empty() {
        output.push_str(&format!(
            "🔴 {} ({}):\n",
            "Critical Issues".red().bold(),
            critical_issues.len()
        ));
        for issue in critical_issues {
            output.push_str(&format!(
                "├── {}:{} [{}] {}\n",
                issue.file_path,
                issue.line_number,
                issue.pattern.red(),
                issue.message
            ));
        }
        output.push('\n');
    }

    if !high_issues.is_empty() {
        output.push_str(&format!(
            "🟠 {} ({}):\n",
            "High Severity".yellow().bold(),
            high_issues.len()
        ));
        for issue in high_issues {
            output.push_str(&format!(
                "├── {}:{} [{}] {}\n",
                issue.file_path,
                issue.line_number,
                issue.pattern.yellow(),
                issue.message
            ));
        }
        output.push('\n');
    }

    if !medium_issues.is_empty() {
        output.push_str(&format!(
            "🟡 {} ({}):\n",
            "Medium Severity".cyan().bold(),
            medium_issues.len()
        ));
        for issue in medium_issues {
            output.push_str(&format!(
                "├── {}:{} [{}] {}\n",
                issue.file_path,
                issue.line_number,
                issue.pattern.cyan(),
                issue.message
            ));
        }
        output.push('\n');
    }

    if !low_issues.is_empty() {
        output.push_str(&format!(
            "🟢 {} ({}):\n",
            "Low Severity".green().bold(),
            low_issues.len()
        ));
        for issue in low_issues.iter().take(5) {
            // Limit low severity to first 5
            output.push_str(&format!(
                "├── {}:{} [{}] {}\n",
                issue.file_path,
                issue.line_number,
                issue.pattern.green(),
                issue.message
            ));
        }
        if low_issues.len() > 5 {
            output.push_str(&format!(
                "└── ... and {} more low severity issues\n",
                low_issues.len() - 5
            ));
        }
        output.push('\n');
    }

    // Summary
    output.push_str("📊 Summary:\n");
    output.push_str(&format!("• Total Issues: {}\n", matches.len()));
    output.push_str(&format!(
        "• Critical: {}\n",
        severity_counts.get("Critical").unwrap_or(&0)
    ));
    output.push_str(&format!(
        "• High: {}\n",
        severity_counts.get("High").unwrap_or(&0)
    ));
    output.push_str(&format!(
        "• Medium: {}\n",
        severity_counts.get("Medium").unwrap_or(&0)
    ));
    output.push_str(&format!(
        "• Low: {}\n",
        severity_counts.get("Low").unwrap_or(&0)
    ));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_languages_to_extensions() {
        let languages = vec!["js".to_string(), "py".to_string(), "rs".to_string()];
        let extensions = map_languages_to_extensions(&languages);

        assert!(extensions.contains(&"js".to_string()));
        assert!(extensions.contains(&"jsx".to_string()));
        assert!(extensions.contains(&"py".to_string()));
        assert!(extensions.contains(&"rs".to_string()));
        assert_eq!(extensions.len(), 4); // js, jsx, py, rs
    }

    #[test]
    fn test_get_severity_for_pattern() {
        assert_eq!(get_severity_for_pattern("DEBUGGER"), "Critical");
        assert_eq!(get_severity_for_pattern("CONSOLE_LOG"), "High");
        assert_eq!(get_severity_for_pattern("PRINT"), "Medium");
        assert_eq!(get_severity_for_pattern("TODO"), "Low");
        assert_eq!(get_severity_for_pattern("UNKNOWN"), "Low");
    }

    #[test]
    fn test_is_critical_severity() {
        assert!(is_critical_severity("DEBUGGER"));
        assert!(!is_critical_severity("CONSOLE_LOG"));
        assert!(!is_critical_severity("PRINT"));
        assert!(!is_critical_severity("TODO"));
    }

    #[test]
    fn test_is_high_severity() {
        assert!(is_high_severity("DEV"));
        assert!(is_high_severity("STAGING"));
        assert!(is_high_severity("CONSOLE_LOG"));
        assert!(is_high_severity("ALERT"));
        assert!(!is_high_severity("DEBUGGER"));
        assert!(!is_high_severity("PRINT"));
    }

    #[test]
    fn test_count_by_severity_empty() {
        let matches = vec![];
        let counts = count_by_severity(&matches);
        assert!(counts.is_empty());
    }

    #[test]
    fn test_language_mapping_comprehensive() {
        let test_cases = vec![
            ("javascript", vec!["js", "jsx"]),
            ("typescript", vec!["ts", "tsx"]),
            ("python", vec!["py"]),
            ("rust", vec!["rs"]),
            ("go", vec!["go"]),
        ];

        for (lang, expected_exts) in test_cases {
            let languages = vec![lang.to_string()];
            let extensions = map_languages_to_extensions(&languages);

            for expected_ext in expected_exts {
                assert!(
                    extensions.contains(&expected_ext.to_string()),
                    "Language '{}' should map to extension '{}'",
                    lang,
                    expected_ext
                );
            }
        }
    }
}
