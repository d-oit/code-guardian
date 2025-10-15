use code_guardian_cli::git_integration::*;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn create_git_repo() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repo
    let output = Command::new("git")
        .args(["init"])
        .current_dir(repo_path)
        .output();

    if output.is_err() {
        // Git not available, skip git-specific setup
        return temp_dir;
    }

    // Configure git user (required for commits)
    let _ = Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output();

    let _ = Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo_path)
        .output();

    // Create initial files
    std::fs::create_dir_all(repo_path.join("src")).unwrap();
    std::fs::write(
        repo_path.join("src/main.rs"),
        r#"
fn main() {
    println!("Hello, world!");
}
"#,
    )
    .unwrap();

    std::fs::write(
        repo_path.join("README.md"),
        "# Test Project\n\nThis is a test project.\n",
    )
    .unwrap();

    // Add and commit files
    let _ = Command::new("git")
        .args(["add", "."])
        .current_dir(repo_path)
        .output();

    let _ = Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(repo_path)
        .output();

    temp_dir
}

fn create_git_repo_with_changes() -> TempDir {
    let temp_dir = create_git_repo();
    let repo_path = temp_dir.path();

    // Make some changes
    std::fs::write(
        repo_path.join("src/main.rs"),
        r#"
fn main() {
    println!("Hello, world!");
    // TODO: Add more functionality
}

fn new_function() {
    println!("New function added");
}
"#,
    )
    .unwrap();

    // Create a new file
    std::fs::write(
        repo_path.join("src/lib.rs"),
        r#"
pub fn library_function() {
    println!("Library function");
}
"#,
    )
    .unwrap();

    temp_dir
}

#[cfg(test)]
mod git_integration_tests {
    use super::*;

    #[test]
    fn test_is_git_repo_valid() {
        let temp_dir = create_git_repo();
        let path = temp_dir.path();

        // May or may not be a git repo depending on system setup
        let _result = GitIntegration::is_git_repo(path);
    }

    #[test]
    fn test_is_git_repo_invalid_path() {
        let invalid_path = PathBuf::from("nonexistent/path");

        let result = GitIntegration::is_git_repo(&invalid_path);
        assert!(!result);
    }

    #[test]
    fn test_get_staged_files() {
        let temp_dir = create_git_repo_with_changes();
        let path = temp_dir.path();

        let _result = GitIntegration::get_staged_files(path);
        // Should return staged files or empty list (may error if git not available)
    }

    #[test]
    fn test_get_repo_root() {
        let temp_dir = create_git_repo();
        let path = temp_dir.path();

        let _result = GitIntegration::get_repo_root(path);
        // Should return repo root or error if not a git repo
    }

    #[test]
    fn test_install_pre_commit_hook() {
        let temp_dir = create_git_repo();
        let path = temp_dir.path();

        let _result = GitIntegration::install_pre_commit_hook(path);
        // Should install hook or error if git setup issues
    }

    #[test]
    fn test_uninstall_pre_commit_hook() {
        let temp_dir = create_git_repo();
        let path = temp_dir.path();

        let _result = GitIntegration::uninstall_pre_commit_hook(path);
        // Should handle missing hook gracefully
    }

    #[test]
    fn test_get_staged_lines() {
        let temp_dir = create_git_repo_with_changes();
        let path = temp_dir.path();

        let _result = GitIntegration::get_staged_lines(path);
        // Should return staged changes or error
    }
}

#[cfg(test)]
mod git_workflow_tests {
    use super::*;

    #[test]
    fn test_pre_commit_hook_workflow() {
        let temp_dir = create_git_repo();
        let path = temp_dir.path();

        // Test installing and uninstalling pre-commit hook
        let _install_result = GitIntegration::install_pre_commit_hook(path);

        let _uninstall_result = GitIntegration::uninstall_pre_commit_hook(path);
    }

    #[test]
    fn test_staged_files_workflow() {
        let temp_dir = create_git_repo_with_changes();
        let path = temp_dir.path();

        // Test getting staged files
        let _staged_files = GitIntegration::get_staged_files(path);

        // Test getting staged lines
        let _staged_lines = GitIntegration::get_staged_lines(path);
    }

    #[test]
    fn test_repo_detection() {
        let temp_dir = create_git_repo();
        let path = temp_dir.path();

        // Test repo detection
        let _is_repo = GitIntegration::is_git_repo(path);

        // Test getting repo root
        let _repo_root = GitIntegration::get_repo_root(path);
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_git_not_available() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create a directory without git
        std::fs::create_dir_all(path.join("src")).unwrap();
        std::fs::write(
            path.join("src/main.rs"),
            "fn main() { println!(\"Hello\"); }",
        )
        .unwrap();

        // Should handle non-git directories
        let is_repo = GitIntegration::is_git_repo(path);
        assert!(!is_repo);

        let result = GitIntegration::get_repo_root(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_repository() {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        // Initialize empty git repo
        let output = Command::new("git")
            .args(["init"])
            .current_dir(repo_path)
            .output();

        if output.is_ok() {
            // Test operations on empty repo
            let _staged_files = GitIntegration::get_staged_files(repo_path);

            let _staged_lines = GitIntegration::get_staged_lines(repo_path);
        }
    }
}
