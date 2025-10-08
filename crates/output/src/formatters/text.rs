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
        table
            .enforce_styling()
            .set_header(vec!["File", "Line", "Column", "Pattern", "Message"]);

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
            message: "TODO comment".to_string(),
        }];
        let output = formatter.format(&matches);
        assert!(output.contains("test.rs"));
        assert!(output.contains("TODO"));
    }

    #[test]
    fn test_multiple_matches_snapshot() {
        let formatter = TextFormatter;
        let matches = vec![
            Match {
                file_path: "src/main.rs".to_string(),
                line_number: 5,
                column: 3,
                pattern: "TODO".to_string(),
                message: "TODO: implement feature".to_string(),
            },
            Match {
                file_path: "src/lib.rs".to_string(),
                line_number: 10,
                column: 1,
                pattern: "FIXME".to_string(),
                message: "FIXME: temporary workaround".to_string(),
            },
        ];
        let output = formatter.format(&matches);
        // Check that the output contains the expected data
        assert!(output.contains("src/main.rs"));
        assert!(output.contains("5"));
        assert!(output.contains("3"));
        assert!(output.contains("TODO"));
        assert!(output.contains("TODO: implement feature"));
        assert!(output.contains("src/lib.rs"));
        assert!(output.contains("10"));
        assert!(output.contains("1"));
        assert!(output.contains("FIXME"));
        assert!(output.contains("FIXME: temporary workaround"));
        // Ensure it's a table format
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

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    fn arb_match() -> impl Strategy<Value = Match> {
        (
            "[a-zA-Z0-9_.]+",
            1..10000usize,
            1..10000usize,
            "[A-Z]+",
            ".*",
        )
            .prop_map(|(fp, ln, col, pat, msg)| Match {
                file_path: fp.to_string(),
                line_number: ln,
                column: col,
                pattern: pat.to_string(),
                message: msg.to_string(),
            })
    }

    proptest! {
        #[test]
        fn test_text_formatter_arbitrary_matches(matches in proptest::collection::vec(arb_match(), 0..10)) {
            let formatter = TextFormatter;
            let output = formatter.format(&matches);
            // Just check no panic, and if not empty, contains something
            if !matches.is_empty() {
                prop_assert!(!output.is_empty());
            } else {
                prop_assert_eq!(output, "No matches found.");
            }
        }
    }
}
