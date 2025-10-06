use super::Formatter;
use code_guardian_core::Match;

/// Formatter that outputs matches in CSV format.
/// Includes headers for spreadsheet compatibility.
pub struct CsvFormatter;

impl Formatter for CsvFormatter {
    fn format(&self, matches: &[Match]) -> String {
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record(["file_path", "line_number", "column", "pattern", "message"])
            .unwrap();

        for m in matches {
            wtr.write_record([
                &m.file_path,
                &m.line_number.to_string(),
                &m.column.to_string(),
                &m.pattern,
                &m.message,
            ])
            .unwrap();
        }

        wtr.flush().unwrap();
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_matches() {
        let formatter = CsvFormatter;
        let matches = vec![];
        let output = formatter.format(&matches);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 1); // Only header
        assert!(lines[0].contains("file_path,line_number,column,pattern,message"));
    }

    #[test]
    fn test_single_match() {
        let formatter = CsvFormatter;
        let matches = vec![Match {
            file_path: "test.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "TODO: fix this".to_string(),
        }];
        let output = formatter.format(&matches);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[1].contains("test.rs,1,1,TODO,TODO: fix this"));
    }

    #[test]
    fn test_multiple_matches() {
        let formatter = CsvFormatter;
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
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[1].contains("test.rs"));
        assert!(lines[2].contains("test.js"));
    }

    #[test]
    fn test_csv_escaping() {
        let formatter = CsvFormatter;
        let matches = vec![Match {
            file_path: "test,file.rs".to_string(),
            line_number: 1,
            column: 1,
            pattern: "TODO".to_string(),
            message: "TODO, with comma".to_string(),
        }];
        let output = formatter.format(&matches);
        let lines: Vec<&str> = output.lines().collect();
        assert!(lines[1].contains("\"test,file.rs\""));
        assert!(lines[1].contains("\"TODO, with comma\""));
    }
}
