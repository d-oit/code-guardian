use super::Formatter;
use code_guardian_core::Match;

/// Formatter that outputs matches in JSON format.
/// Uses pretty-printed JSON for readability.
pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(&self, matches: &[Match]) -> String {
        serde_json::to_string_pretty(matches).unwrap_or_else(|_| "[]".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use code_guardian_core::Match;

    #[test]
    fn test_empty_matches() {
        let formatter = JsonFormatter;
        let matches = vec![];
        let output = formatter.format(&matches);
        assert_eq!(output, "[]");
    }

    #[test]
    fn test_single_match() {
        let formatter = JsonFormatter;
        let matches = vec![Match {
            file_path: "test.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "TODO: fix this".to_string(),
        }];
        let output = formatter.format(&matches);
        let expected = r#"[
  {
    "file_path": "test.rs",
    "line_number": 1,
    "column": 1,
    "pattern": "TODO",
    "message": "TODO: fix this"
  }
]"#;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_multiple_matches() {
        let formatter = JsonFormatter;
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
        // Check that it's valid JSON and contains the data
        let parsed: Vec<Match> = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed, matches);
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    fn arb_match() -> impl Strategy<Value = Match> {
        ("[a-zA-Z0-9_.]+", 1..10000usize, 1..10000usize, "[A-Z]+", ".*").prop_map(|(fp, ln, col, pat, msg)| Match {
            file_path: fp.to_string(),
            line_number: ln,
            column: col,
            pattern: pat.to_string(),
            message: msg.to_string(),
        })
    }

    proptest! {
        #[test]
        fn test_json_formatter_arbitrary_matches(matches in proptest::collection::vec(arb_match(), 0..10)) {
            let formatter = JsonFormatter;
            let output = formatter.format(&matches);
            // Check that it's valid JSON
            let parsed: Vec<Match> = serde_json::from_str(&output).unwrap();
            prop_assert_eq!(parsed, matches);
        }
    }
}
