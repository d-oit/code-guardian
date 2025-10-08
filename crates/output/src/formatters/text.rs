use super::Formatter;
use code_guardian_core::Match;

/// Formatter that outputs matches in a simple text format.
/// Each match is displayed as "file:line:column: pattern - message".
pub struct TextFormatter;

impl Formatter for TextFormatter {
    fn format(&self, matches: &[Match]) -> String {
        if matches.is_empty() {
            return "No matches found.".to_string();
        }

        let mut output = String::new();
        for m in matches {
            output.push_str(&format!(
                "{}:{}:{}: {} - {}\n",
                m.file_path, m.line_number, m.column, m.pattern, m.message
            ));
        }
        output.trim_end().to_string()
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
        let expected = "test.rs:1:1: TODO - TODO comment";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_multiple_matches_snapshot() {
        let formatter = TextFormatter;
        let matches = vec![
            Match {
                file_path: "src/main.rs".to_string(),
                line_number: 10,
                column: 5,
                pattern: "TODO".to_string(),
                message: "Found a TODO".to_string(),
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
        let expected = "src/main.rs:10:5: TODO - Found a TODO\nsrc/lib.rs:10:1: FIXME - FIXME: temporary workaround";
        assert_eq!(output, expected);
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
        let expected = "test.rs:1:1: TODO - TODO\ntest.js:2:3: FIXME - FIXME";
        assert_eq!(output, expected);
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
