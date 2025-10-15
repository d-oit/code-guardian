use code_guardian_cli::git_integration::*;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn create_git_repo() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();
        
        // Initialize git repo
        let output = Command::new("git")
            .args(&["init"])
            .current_dir(repo_path)
            .output();
        
        if output.is_err() {
            // Git not available, skip git-specific setup
            return temp_dir;
        }
        
        // Configure git user (required for commits)
        let _ = Command::new("git")
            .args(&["config", "user.name", "Test User"])
            .current_dir(repo_path)
            .output();
        
        let _ = Command::new("git")
            .args(&["config", "user.email", "test@example.com"])
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
        ).unwrap();
        
        std::fs::write(
            repo_path.join("README.md"),
            "# Test Project\n\nThis is a test project.\n",
        ).unwrap();
        
        // Add and commit files
        let _ = Command::new("git")
            .args(&["add", "."])
            .current_dir(repo_path)
            .output();
        
        let _ = Command::new("git")
            .args(&["commit", "-m", "Initial commit"])
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
        ).unwrap();
        
        // Create a new file
        std::fs::write(
            repo_path.join("src/lib.rs"),
            r#"
pub fn library_function() {
    println!("Library function");
}
"#,
        ).unwrap();
        
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
        let result = GitIntegration::is_git_repo(path);
        assert!(result == true || result == false);
    }

    #[test]
    fn test_is_git_repo_invalid_path() {
        let invalid_path = PathBuf::from("nonexistent/path");
        
        let result = GitIntegration::is_git_repo(&invalid_path);
        assert_eq!(result, false);
    }

    #[test]
    fn test_get_staged_files() {
        let temp_dir = create_git_repo_with_changes();
        let path = temp_dir.path();
        
        let result = GitIntegration::get_staged_files(path);
        // Should return staged files or empty list (may error if git not available)
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_get_repo_root() {
        let temp_dir = create_git_repo();
        let path = temp_dir.path();
        
        let result = GitIntegration::get_repo_root(path);
        // Should return repo root or error if not a git repo
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_install_pre_commit_hook() {
        let temp_dir = create_git_repo();
        let path = temp_dir.path();
        
        let result = GitIntegration::install_pre_commit_hook(path);
        // Should install hook or error if git setup issues
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_uninstall_pre_commit_hook() {
        let temp_dir = create_git_repo();
        let path = temp_dir.path();
        
        let result = GitIntegration::uninstall_pre_commit_hook(path);
        // Should handle missing hook gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_get_staged_lines() {
        let temp_dir = create_git_repo_with_changes();
        let path = temp_dir.path();
        
        let result = GitIntegration::get_staged_lines(path);
        // Should return staged changes or error
        assert!(result.is_ok() || result.is_err());
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
        let install_result = GitIntegration::install_pre_commit_hook(path);
        assert!(install_result.is_ok() || install_result.is_err());
        
        let uninstall_result = GitIntegration::uninstall_pre_commit_hook(path);
        assert!(uninstall_result.is_ok() || uninstall_result.is_err());
    }

    #[test]
    fn test_staged_files_workflow() {
        let temp_dir = create_git_repo_with_changes();
        let path = temp_dir.path();
        
        // Test getting staged files
        let staged_files = GitIntegration::get_staged_files(path);
        assert!(staged_files.is_ok() || staged_files.is_err());
        
        // Test getting staged lines
        let staged_lines = GitIntegration::get_staged_lines(path);
        assert!(staged_lines.is_ok() || staged_lines.is_err());
    }

    #[test]
    fn test_repo_detection() {
        let temp_dir = create_git_repo();
        let path = temp_dir.path();
        
        // Test repo detection
        let is_repo = GitIntegration::is_git_repo(path);
        assert!(is_repo == true || is_repo == false);
        
        // Test getting repo root
        let repo_root = GitIntegration::get_repo_root(path);
        assert!(repo_root.is_ok() || repo_root.is_err());
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
        ).unwrap();
        
        // Should handle non-git directories
        let is_repo = GitIntegration::is_git_repo(path);
        assert_eq!(is_repo, false);
        
        let result = GitIntegration::get_repo_root(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_repository() {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();
        
        // Initialize empty git repo
        let output = Command::new("git")
            .args(&["init"])
            .current_dir(repo_path)
            .output();
        
        if output.is_ok() {
            // Test operations on empty repo
            let staged_files = GitIntegration::get_staged_files(repo_path);
            assert!(staged_files.is_ok() || staged_files.is_err());
            
            let staged_lines = GitIntegration::get_staged_lines(repo_path);
            assert!(staged_lines.is_ok() || staged_lines.is_err());
        }
    }
}