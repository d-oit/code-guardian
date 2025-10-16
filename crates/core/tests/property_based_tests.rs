use code_guardian_core::detectors::TodoDetector;
use code_guardian_core::{PatternDetector, Scanner};
use proptest::prelude::*;
use tempfile::TempDir;

/// Property-based tests for complex logic validation
#[cfg(test)]
mod property_tests {
    use super::*;

    fn create_test_scanner() -> Scanner {
        let detectors: Vec<Box<dyn PatternDetector>> = vec![Box::new(TodoDetector)];
        Scanner::new(detectors)
    }

    prop_compose! {
        fn arbitrary_code_content()(
            lines in prop::collection::vec(
                prop::string::string_regex(r"[a-zA-Z0-9\s\-_./\\]*").unwrap(),
                1..20
            )
        ) -> String {
            lines.join("\n")
        }
    }

    prop_compose! {
        fn arbitrary_file_path()(
            name in prop::string::string_regex(r"[a-zA-Z0-9_]+").unwrap(),
            ext in prop::option::of(prop::string::string_regex(r"[a-z]{1,4}").unwrap())
        ) -> String {
            match ext {
                Some(ext) => format!("{}.{}", name, ext),
                None => name,
            }
        }
    }

    proptest! {
        #[test]
        fn test_scanner_deterministic_results(
            content in arbitrary_code_content(),
            file_path in arbitrary_file_path()
        ) {
            // Property: Scanner should always produce the same results for the same input
            let temp_dir = TempDir::new().unwrap();
            let test_file = temp_dir.path().join(&file_path);

            if let Some(parent) = test_file.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::write(&test_file, &content).unwrap();

            let scanner = create_test_scanner();
            let matches1 = scanner.scan(temp_dir.path()).unwrap_or_default();
            let matches2 = scanner.scan(temp_dir.path()).unwrap_or_default();

            prop_assert_eq!(matches1.len(), matches2.len());
            prop_assert_eq!(matches1, matches2);
        }

        #[test]
        fn test_match_line_numbers_valid(
            content in arbitrary_code_content(),
            file_path in arbitrary_file_path()
        ) {
            // Property: All match line numbers should be valid (within file bounds)
            let temp_dir = TempDir::new().unwrap();
            let test_file = temp_dir.path().join(&file_path);

            if let Some(parent) = test_file.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::write(&test_file, &content).unwrap();

            let scanner = create_test_scanner();
            let matches = scanner.scan(temp_dir.path()).unwrap_or_default();

            let line_count = content.lines().count().max(1);

            for m in matches {
                prop_assert!(m.line_number >= 1, "Line number should be 1-based");
                prop_assert!(m.line_number <= line_count, "Line number should not exceed file length");
                prop_assert!(m.column >= 1, "Column should be 1-based");
            }
        }

        #[test]
        fn test_file_path_consistency(
            content in arbitrary_code_content(),
            file_path in arbitrary_file_path()
        ) {
            // Property: All matches should reference the correct file path
            let temp_dir = TempDir::new().unwrap();
            let test_file = temp_dir.path().join(&file_path);

            if let Some(parent) = test_file.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::write(&test_file, &content).unwrap();

            let scanner = create_test_scanner();
            let matches = scanner.scan(temp_dir.path()).unwrap_or_default();

            // All matches should reference files within the temp directory
            for m in matches {
                prop_assert!(m.file_path.contains(&file_path),
                    "Match file path should contain the test file name");
            }
        }
    }
}
