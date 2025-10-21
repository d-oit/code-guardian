use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Git integration utilities for Code-Guardian
pub struct GitIntegration;

impl GitIntegration {
    /// Get list of staged files (files in git index)
    pub fn get_staged_files(repo_path: &Path) -> Result<Vec<PathBuf>> {
        let output = Command::new("git")
            .args(["diff", "--cached", "--name-only", "--diff-filter=ACMR"])
            .current_dir(repo_path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Git command failed: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let files: Vec<PathBuf> = stdout
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| repo_path.join(line.trim()))
            .filter(|path| path.exists()) // Only include files that still exist
            .collect();

        Ok(files)
    }

    /// Get the root directory of the git repository
    pub fn get_repo_root(start_path: &Path) -> Result<PathBuf> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(start_path)
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Not in a git repository or git command failed"));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let repo_root = stdout.trim();
        Ok(PathBuf::from(repo_root))
    }

    /// Check if the current directory is in a git repository
    pub fn is_git_repo(path: &Path) -> bool {
        Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .current_dir(path)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Get modified lines for staged files (useful for line-specific scanning)
    #[allow(dead_code)]
    pub fn get_staged_lines(repo_path: &Path) -> Result<Vec<StagedChange>> {
        let output = Command::new("git")
            .args(["diff", "--cached", "--unified=0"])
            .current_dir(repo_path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Git diff command failed: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(parse_git_diff(&stdout, repo_path))
    }

    /// Install pre-commit hook for Code-Guardian
    pub fn install_pre_commit_hook(repo_path: &Path) -> Result<()> {
        let hooks_dir = repo_path.join(".git").join("hooks");
        let hook_path = hooks_dir.join("pre-commit");

        // Create hooks directory if it doesn't exist
        std::fs::create_dir_all(&hooks_dir)?;

        // Pre-commit hook script
        let hook_script = r#"#!/bin/sh
# Code-Guardian pre-commit hook
# This hook runs Code-Guardian on staged files before commit

# Check if code-guardian is available
if ! command -v code-guardian >/dev/null 2>&1; then
    echo "Error: code-guardian not found in PATH"
    echo "Please install code-guardian or add it to your PATH"
    exit 1
fi

# Run Code-Guardian pre-commit check
exec code-guardian pre-commit --staged-only --fast
"#;

        std::fs::write(&hook_path, hook_script)?;

        // Make the hook executable (Unix-like systems)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&hook_path)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&hook_path, perms)?;
        }

        println!("✅ Pre-commit hook installed at: {}", hook_path.display());
        println!("🔧 The hook will run 'code-guardian pre-commit --staged-only --fast' before each commit");

        Ok(())
    }

    /// Uninstall pre-commit hook
    pub fn uninstall_pre_commit_hook(repo_path: &Path) -> Result<()> {
        let hook_path = repo_path.join(".git").join("hooks").join("pre-commit");

        if hook_path.exists() {
            // Check if it's our hook before removing
            let content = std::fs::read_to_string(&hook_path)?;
            if content.contains("Code-Guardian pre-commit hook") {
                std::fs::remove_file(&hook_path)?;
                println!("✅ Code-Guardian pre-commit hook removed");
            } else {
                println!(
                    "⚠️  Pre-commit hook exists but doesn't appear to be Code-Guardian's hook"
                );
                println!("   Manual removal required: {}", hook_path.display());
            }
        } else {
            println!("ℹ️  No pre-commit hook found");
        }

        Ok(())
    }
}

/// Represents a staged change in git
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StagedChange {
    pub file_path: PathBuf,
    pub added_lines: Vec<LineRange>,
    pub removed_lines: Vec<LineRange>,
}

/// Represents a range of lines
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LineRange {
    pub start: usize,
    pub count: usize,
}

/// Parse git diff output to extract staged changes
#[allow(dead_code)]
fn parse_git_diff(diff_output: &str, repo_path: &Path) -> Vec<StagedChange> {
    let mut changes = Vec::new();
    let mut current_file: Option<PathBuf> = None;
    let mut added_lines = Vec::new();
    let mut removed_lines = Vec::new();

    for line in diff_output.lines() {
        if line.starts_with("diff --git") {
            // Save previous file's changes
            if let Some(file_path) = current_file.take() {
                changes.push(StagedChange {
                    file_path,
                    added_lines: std::mem::take(&mut added_lines),
                    removed_lines: std::mem::take(&mut removed_lines),
                });
            }
        } else if line.starts_with("+++") {
            // Extract new file path
            if let Some(path_part) = line.strip_prefix("+++ b/") {
                current_file = Some(repo_path.join(path_part));
            }
        } else if line.starts_with("@@") {
            // Parse hunk header: @@ -old_start,old_count +new_start,new_count @@
            if let Some(hunk_info) = line.strip_prefix("@@").and_then(|s| s.strip_suffix("@@")) {
                let parts: Vec<&str> = hunk_info.split_whitespace().collect();
                if parts.len() >= 2 {
                    // Parse removed lines (-old_start,old_count)
                    if let Some(removed_part) = parts[0].strip_prefix('-') {
                        if let Some((start_str, count_str)) = removed_part.split_once(',') {
                            if let (Ok(start), Ok(count)) =
                                (start_str.parse::<usize>(), count_str.parse::<usize>())
                            {
                                if count > 0 {
                                    removed_lines.push(LineRange { start, count });
                                }
                            }
                        }
                    }

                    // Parse added lines (+new_start,new_count)
                    if let Some(added_part) = parts[1].strip_prefix('+') {
                        if let Some((start_str, count_str)) = added_part.split_once(',') {
                            if let (Ok(start), Ok(count)) =
                                (start_str.parse::<usize>(), count_str.parse::<usize>())
                            {
                                if count > 0 {
                                    added_lines.push(LineRange { start, count });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Don't forget the last file
    if let Some(file_path) = current_file {
        changes.push(StagedChange {
            file_path,
            added_lines,
            removed_lines,
        });
    }

    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use proptest::prelude::*;
    use tempfile::TempDir;

    #[test]
    fn test_git_integration_basic() {
        // Test that basic structures work
        let range = LineRange { start: 5, count: 3 };
        assert_eq!(range.start, 5);
        assert_eq!(range.count, 3);

        // Test that we can create staged change
        let temp_dir = TempDir::new().unwrap();
        let change = StagedChange {
            file_path: temp_dir.path().join("test.rs"),
            added_lines: vec![range],
            removed_lines: vec![],
        };

        assert_eq!(change.added_lines.len(), 1);
        assert_eq!(change.removed_lines.len(), 0);
    }

    #[test]
    fn test_is_git_repo() {
        let temp_dir = TempDir::new().unwrap();
        assert!(!GitIntegration::is_git_repo(temp_dir.path()));
    }

    #[test]
    fn test_line_range() {
        let range = LineRange { start: 5, count: 3 };
        assert_eq!(range.start, 5);
        assert_eq!(range.count, 3);
    }

    // Property-based tests using proptest

    /* TODO: Enable proptest when ready
    proptest! {
        fn test_parse_git_diff_does_not_panic_on_arbitrary_input(input in any::<String>()) {
            let temp_dir = TempDir::new().unwrap();
            let repo_path = temp_dir.path();
            // This should not panic regardless of input
            let _result = parse_git_diff(&input, repo_path);
        }

        fn test_parse_git_diff_returns_valid_structures(input in any::<String>()) {
            let temp_dir = TempDir::new().unwrap();
            let repo_path = temp_dir.path();
            let changes = parse_git_diff(&input, repo_path);

            // All returned changes should have valid file paths and line ranges
            for change in &changes {
                // File path should be absolute and within repo
                prop_assert!(change.file_path.is_absolute());
                prop_assert!(change.file_path.starts_with(repo_path));

                // Line ranges should have valid start and count
                for range in &change.added_lines {
                    prop_assert!(range.start > 0); // Git line numbers start from 1
                    prop_assert!(range.count > 0);
                }
                for range in &change.removed_lines {
                    prop_assert!(range.start > 0);
                    prop_assert!(range.count > 0);
                }
            }
        }

        fn test_parse_git_diff_empty_input() {
            let temp_dir = TempDir::new().unwrap();
            let repo_path = temp_dir.path();
            let changes = parse_git_diff("", repo_path);
            prop_assert!(changes.is_empty());
        }

        fn test_parse_git_diff_malformed_hunk_headers(input in any::<String>()) {
            let temp_dir = TempDir::new().unwrap();
            let repo_path = temp_dir.path();
            // Should handle malformed hunk headers gracefully
            let changes = parse_git_diff(&input, repo_path);
            // Might return empty or partial results, but shouldn't panic
            prop_assert!(changes.len() >= 0);
        }

        #[test]
        fn test_parse_git_diff_valid_diff_format(
            file_name in "[a-zA-Z0-9_.-]+",
            old_start in 1..1000u32,
            old_count in 0..100u32,
            new_start in 1..1000u32,
            new_count in 0..100u32
        ) {
            let temp_dir = TempDir::new().unwrap();
            let repo_path = temp_dir.path();

            let diff = format!(
                "diff --git a/{} b/{}\n\
                 --- a/{}\n\
                 +++ b/{}\n\
                 @@ -{},{} +{},{} @@\n\
                 +new line\n",
                file_name, file_name, file_name, file_name,
                old_start, old_count, new_start, new_count
            );

            let changes = parse_git_diff(&diff, repo_path);

            if new_count > 0 {
                prop_assert_eq!(changes.len(), 1);
                let change = &changes[0];
                prop_assert!(change.file_path.ends_with(&file_name));
                prop_assert!(!change.added_lines.is_empty() || !change.removed_lines.is_empty());
            }
        }

        #[test]
        fn test_parse_git_diff_multiple_files(
            file1 in "[a-zA-Z0-9_.-]+",
            file2 in "[a-zA-Z0-9_.-]+"
        ) {
            let temp_dir = TempDir::new().unwrap();
            let repo_path = temp_dir.path();

            let diff = format!(
                "diff --git a/{} b/{}\n\
                 --- a/{}\n\
                 +++ b/{}\n\
                 @@ -1,1 +1,1 @@\n\
                 +change1\n\
                 diff --git a/{} b/{}\n\
                 --- a/{}\n\
                 +++ b/{}\n\
                 @@ -2,2 +2,2 @@\n\
                 +change2\n",
                file1, file1, file1, file1,
                file2, file2, file2, file2
            );

            let changes = parse_git_diff(&diff, repo_path);
            prop_assert!(changes.len() <= 2); // Should parse up to 2 files
        }
    }
    */
}
