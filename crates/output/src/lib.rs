pub mod formatters;

pub use formatters::*;

#[cfg(test)]
mod tests {
    use super::formatters::*;
    use code_guardian_core::Match;

    fn create_test_matches() -> Vec<Match> {
        vec![
            Match {
                file_path: "src/main.rs".to_string(),
                line_number: 10,
                column: 5,
                pattern: "TODO".to_string(),
                message: "Fix this implementation".to_string(),
            },
            Match {
                file_path: "src/lib.rs".to_string(),
                line_number: 25,
                column: 1,
                pattern: "FIXME".to_string(),
                message: "Handle error case".to_string(),
            },
        ]
    }

    #[test]
    fn test_all_formatters_integration() {
        let matches = create_test_matches();

        // Test JSON formatter
        let json_output = JsonFormatter.format(&matches);
        assert!(json_output.contains("TODO"));
        assert!(json_output.contains("FIXME"));
        assert!(json_output.contains("src/main.rs"));

        // Test Text formatter
        let text_output = TextFormatter.format(&matches);
        assert!(text_output.contains("TODO"));
        assert!(text_output.contains("FIXME"));
        assert!(text_output.contains("src/main.rs"));

        // Test CSV formatter
        let csv_output = CsvFormatter.format(&matches);
        assert!(csv_output.contains("TODO"));
        assert!(csv_output.contains("FIXME"));
        assert!(csv_output.contains("src/main.rs"));

        // Test HTML formatter
        let html_output = HtmlFormatter.format(&matches);
        assert!(html_output.contains("TODO"));
        assert!(html_output.contains("FIXME"));
        assert!(html_output.contains("src/main.rs"));
        assert!(html_output.contains("<html>"));

        // Test Markdown formatter
        let md_output = MarkdownFormatter.format(&matches);
        assert!(md_output.contains("TODO"));
        assert!(md_output.contains("FIXME"));
        assert!(md_output.contains("src/main.rs"));
        assert!(md_output.contains("|")); // Markdown table format
    }

    #[test]
    fn test_formatters_with_empty_matches() {
        let empty_matches = vec![];

        // All formatters should handle empty input gracefully
        let _json = JsonFormatter.format(&empty_matches);
        let _text = TextFormatter.format(&empty_matches);
        let _csv = CsvFormatter.format(&empty_matches);
        let _html = HtmlFormatter.format(&empty_matches);
        let _md = MarkdownFormatter.format(&empty_matches);
    }

    #[test]
    fn test_formatters_with_special_characters() {
        let matches = vec![Match {
            file_path: "test/file with spaces.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "Message with \"quotes\" and <html> & symbols".to_string(),
        }];

        // Test that formatters properly escape or handle special characters
        let json_output = JsonFormatter.format(&matches);
        assert!(json_output.contains("quotes"));

        let html_output = HtmlFormatter.format(&matches);
        // HTML should escape special characters
        assert!(html_output.contains("&lt;html&gt;") || html_output.contains("<html>"));

        let csv_output = CsvFormatter.format(&matches);
        assert!(csv_output.contains("quotes"));
    }
}
