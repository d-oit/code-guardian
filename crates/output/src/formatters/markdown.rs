use super::Formatter;
use code_guardian_core::Match;

/// Formatter that outputs matches in Markdown table format.
/// Suitable for documentation or GitHub issues.
pub struct MarkdownFormatter;

impl Formatter for MarkdownFormatter {
    fn format(&self, matches: &[Match]) -> String {
        if matches.is_empty() {
            return "No matches found.".to_string();
        }

        let mut output = String::from("| File | Line | Column | Pattern | Message |\n");
        output.push_str("|------|------|--------|---------|---------|\n");

        for m in matches {
            output.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                escape_md(&m.file_path),
                m.line_number,
                m.column,
                escape_md(&m.pattern),
                escape_md(&m.message)
            ));
        }

        output
    }
}

/// Escapes pipe characters in markdown table cells.
fn escape_md(text: &str) -> String {
    text.replace('|', "\\|")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_matches() {
        let formatter = MarkdownFormatter;
        let matches = vec![];
        let output = formatter.format(&matches);
        assert_eq!(output, "No matches found.");
    }

    #[test]
    fn test_single_match() {
        let formatter = MarkdownFormatter;
        let matches = vec![Match {
            file_path: "test.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "TODO: fix this".to_string(),
        }];
        let output = formatter.format(&matches);
        assert!(output.contains("| test.rs | 1 | 1 | TODO | TODO: fix this |"));
        assert!(output.contains("|------|------|--------|---------|---------|"));
    }

    #[test]
    fn test_escape_pipes() {
        let formatter = MarkdownFormatter;
        let matches = vec![Match {
            file_path: "test|file.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "TODO|fix".to_string(),
        }];
        let output = formatter.format(&matches);
        assert!(output.contains("test\\|file.rs"));
        assert!(output.contains("TODO\\|fix"));
    }

    #[test]
    fn test_multiple_matches() {
        let formatter = MarkdownFormatter;
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
