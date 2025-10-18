use anyhow::Result;
use code_guardian_cli::production_handlers::*;
use std::path::PathBuf;
use tempfile::TempDir;

#[cfg(test)]
mod production_handler_tests {
    use super::*;

    #[test]
    fn test_handle_production_check_invalid_path() {
        let invalid_path = PathBuf::from("nonexistent/path");
        let result =
            handle_production_check(invalid_path, "json".to_string(), false, false, vec![], None);
        // Function handles invalid paths gracefully (returns empty results)
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_production_check_empty_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_path_buf();

        let result = handle_production_check(path, "json".to_string(), false, false, vec![], None);
        // Should succeed even with empty directory
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_handle_production_check_different_formats() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_path_buf();

        // Test JSON format
        let result =
            handle_production_check(path.clone(), "json".to_string(), false, false, vec![], None);
        assert!(result.is_ok());

        // Test summary format
        let result = handle_production_check(
            path.clone(),
            "summary".to_string(),
            false,
            false,
            vec![],
            None,
        );
        assert!(result.is_ok());

        // Test text format
        let result = handle_production_check(path, "text".to_string(), false, false, vec![], None);
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_handle_ci_gate_invalid_path() {
        let invalid_path = PathBuf::from("nonexistent/path");
        let result = handle_ci_gate(invalid_path, None, None, 0, 0);
        // Function handles invalid paths gracefully (returns empty results)
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_ci_gate_empty_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_path_buf();

        let result = handle_ci_gate(path, None, None, 10, 20);
        // Should succeed with empty directory
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_handle_ci_gate_with_output_file() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_path_buf();
        let output_file = temp_dir.path().join("ci_report.json");

        let result = handle_ci_gate(path, None, Some(output_file.clone()), 5, 10);
        assert!(result.is_ok());

        // Check that output file was created
        assert!(output_file.exists());

        Ok(())
    }

    #[test]
    fn test_handle_pre_commit_invalid_path() {
        let invalid_path = PathBuf::from("nonexistent/path");
        let result = handle_pre_commit(invalid_path, false, false);
        // Function handles invalid paths gracefully (returns empty results)
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_pre_commit_empty_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_path_buf();

        let result = handle_pre_commit(path, false, false);
        // Should succeed with empty directory
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_handle_pre_commit_fast_mode() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_path_buf();

        let result = handle_pre_commit(path, false, true);
        // Should succeed in fast mode
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_handle_lang_scan_invalid_path() {
        let invalid_path = PathBuf::from("nonexistent/path");
        let result = handle_lang_scan(
            vec!["rust".to_string()],
            invalid_path,
            "json".to_string(),
            false,
        );
        // Function handles invalid paths gracefully (returns empty results)
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_lang_scan_empty_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_path_buf();

        let result = handle_lang_scan(
            vec!["rust".to_string(), "javascript".to_string()],
            path,
            "json".to_string(),
            false,
        );
        // Should succeed with empty directory
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_handle_lang_scan_production_mode() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_path_buf();

        let result = handle_lang_scan(
            vec!["python".to_string()],
            path,
            "summary".to_string(),
            true, // production mode
        );
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_handle_watch_basic() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_path_buf();

        let result = handle_watch(
            path,
            vec!["*.rs".to_string()],
            vec!["target/**".to_string()],
            1000,
        );
        // Should succeed (currently returns "coming soon" message)
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_handle_watch_empty_filters() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_path_buf();

        let result = handle_watch(path, vec![], vec![], 500);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_production_check_severity_filter() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_path_buf();

        // Test with specific severity filter
        let result = handle_production_check(
            path,
            "json".to_string(),
            false,
            false,
            vec!["Critical".to_string(), "High".to_string()],
            None,
        );
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_production_check_with_output_file() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_path_buf();
        let output_file = temp_dir.path().join("production_report.json");

        let result = handle_production_check(
            path,
            "json".to_string(),
            false,
            false,
            vec![],
            Some(output_file.clone()),
        );
        assert!(result.is_ok());

        // Check that output file was created
        assert!(output_file.exists());

        Ok(())
    }
}
