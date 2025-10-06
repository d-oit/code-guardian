use super::Formatter;
use code_guardian_core::Match;
use comfy_table::{Cell, Table};

/// Formatter that outputs matches in a plain text table format.
/// Uses a table for structured display.
pub struct TextFormatter;

impl Formatter for TextFormatter {
    fn format(&self, matches: &[Match]) -> String {
        if matches.is_empty() {
            return "No matches found.".to_string();
        }

        let mut table = Table::new();
        table.set_header(vec!["File", "Line", "Column", "Pattern", "Message"]);

        for m in matches {
            table.add_row(vec![
                Cell::new(&m.file_path),
                Cell::new(m.line_number.to_string()),
                Cell::new(m.column.to_string()),
                Cell::new(&m.pattern),
                Cell::new(&m.message),
            ]);
        }

        table.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_matches() {
        let formatter = TextFormatter;
        let matches = vec![];
        let output = formatter.format(&matches);
        assert_eq!(output, "No matches found.");
    }

    #[test]
    fn test_single_match() {
        let formatter = TextFormatter;
        let matches = vec![Match {
            file_path: "test.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "TODO: fix this".to_string(),
        }];
        let output = formatter.format(&matches);
        assert!(output.contains("test.rs"));
        assert!(output.contains("1"));
        assert!(output.contains("TODO"));
        assert!(output.contains("TODO: fix this"));
    }

    #[test]
    fn test_multiple_matches() {
        let formatter = TextFormatter;
        let matches = vec![
            Match {
                file_path: "test.rs".to_string(),
                line_number: 1,
                column: 1,
                pattern: "TODO".to_string(),
                message: "TODO".to_string(),
            },
            Match {
                file_path: "test.js".to_string(),
                line_number: 2,
                column: 3,
                pattern: "FIXME".to_string(),
                message: "FIXME".to_string(),
            },
        ];
        let output = formatter.format(&matches);
        assert!(output.contains("test.rs"));
        assert!(output.contains("test.js"));
        assert!(output.contains("TODO"));
        assert!(output.contains("FIXME"));
    }
}
