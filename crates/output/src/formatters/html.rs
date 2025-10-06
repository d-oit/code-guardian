use super::Formatter;
use code_guardian_core::Match;

/// Formatter that outputs matches in HTML table format.
/// Includes basic HTML structure for standalone display.
pub struct HtmlFormatter;

impl Formatter for HtmlFormatter {
    fn format(&self, matches: &[Match]) -> String {
        let mut output = String::from(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Code Guardian Matches</title>
    <style>
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
        tr:nth-child(even) { background-color: #f9f9f9; }
    </style>
</head>
<body>
    <h1>Code Guardian Scan Results</h1>
    <table>
        <thead>
            <tr>
                <th>File</th>
                <th>Line</th>
                <th>Column</th>
                <th>Pattern</th>
                <th>Message</th>
            </tr>
        </thead>
        <tbody>
"#,
        );

        if matches.is_empty() {
            output.push_str("        <tr><td colspan=\"5\">No matches found.</td></tr>\n");
        } else {
            for m in matches {
                output.push_str(&format!(
                    "        <tr>\n            <td>{}</td>\n            <td>{}</td>\n            <td>{}</td>\n            <td>{}</td>\n            <td>{}</td>\n        </tr>\n",
                    html_escape(&m.file_path),
                    m.line_number,
                    m.column,
                    html_escape(&m.pattern),
                    html_escape(&m.message)
                ));
            }
        }

        output.push_str(
            r#"        </tbody>
    </table>
</body>
</html>
"#,
        );

        output
    }
}

/// Escapes HTML special characters.
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_matches() {
        let formatter = HtmlFormatter;
        let matches = vec![];
        let output = formatter.format(&matches);
        assert!(output.contains("<table>"));
        assert!(output.contains("No matches found."));
        assert!(output.contains("</html>"));
    }

    #[test]
    fn test_single_match() {
        let formatter = HtmlFormatter;
        let matches = vec![Match {
            file_path: "test.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "TODO: fix this".to_string(),
        }];
        let output = formatter.format(&matches);
        assert!(output.contains("<table>"));
        assert!(output.contains("<td>test.rs</td>"));
        assert!(output.contains("<td>1</td>"));
        assert!(output.contains("<td>TODO</td>"));
        assert!(output.contains("<td>TODO: fix this</td>"));
        assert!(output.contains("</html>"));
    }

    #[test]
    fn test_html_escape() {
        let formatter = HtmlFormatter;
        let matches = vec![Match {
            file_path: "test&<>\"'.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "TODO&<>\"'".to_string(),
        }];
        let output = formatter.format(&matches);
        assert!(output.contains("test&amp;&lt;&gt;&quot;&#x27;.rs"));
        assert!(output.contains("TODO&amp;&lt;&gt;&quot;&#x27;"));
    }

    #[test]
    fn test_multiple_matches() {
        let formatter = HtmlFormatter;
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
