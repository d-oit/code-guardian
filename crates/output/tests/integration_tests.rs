use code_guardian_core::Match;
use code_guardian_output::*;

#[test]
fn test_formatter_trait_object() {
    let matches = vec![Match {
        file_path: "test.rs".to_string(),
        line_number: 1,
        column: 1,
        pattern: "TODO".to_string(),
        message: "Test message".to_string(),
    }];

    let formatters: Vec<Box<dyn Formatter>> = vec![
        Box::new(JsonFormatter),
        Box::new(TextFormatter),
        Box::new(CsvFormatter),
        Box::new(HtmlFormatter),
        Box::new(MarkdownFormatter),
    ];

    for formatter in formatters {
        let output = formatter.format(&matches);
        assert!(!output.is_empty());
        assert!(output.contains("test.rs"));
    }
}

#[test]
fn test_formatters_with_unicode_content() {
    let matches = vec![Match {
        file_path: "测试.rs".to_string(),
        line_number: 1,
        column: 1,
        pattern: "TODO".to_string(),
        message: "Message with émojis 🚀 and unicode: αβγ".to_string(),
    }];

    // Test all formatters handle unicode correctly
    let json_output = JsonFormatter.format(&matches);
    assert!(json_output.contains("测试.rs"));
    assert!(json_output.contains("🚀"));

    let text_output = TextFormatter.format(&matches);
    assert!(text_output.contains("测试.rs"));
    assert!(text_output.contains("🚀"));

    let csv_output = CsvFormatter.format(&matches);
    assert!(csv_output.contains("测试.rs"));
    assert!(csv_output.contains("🚀"));

    let html_output = HtmlFormatter.format(&matches);
    assert!(html_output.contains("测试.rs"));
    assert!(html_output.contains("🚀"));

    let md_output = MarkdownFormatter.format(&matches);
    assert!(md_output.contains("测试.rs"));
    assert!(md_output.contains("🚀"));
}

#[test]
fn test_formatters_with_very_long_content() {
    let long_message = "x".repeat(10000);
    let long_path = format!("very/long/path/{}/file.rs", "dir/".repeat(100));

    let matches = vec![Match {
        file_path: long_path.clone(),
        line_number: 999999,
        column: 999999,
        pattern: "TODO".to_string(),
        message: long_message.clone(),
    }];

    // Test all formatters handle very long content
    let json_output = JsonFormatter.format(&matches);
    assert!(json_output.contains(&long_path));
    assert!(json_output.contains(&long_message));

    let text_output = TextFormatter.format(&matches);
    assert!(text_output.contains(&long_path));
    assert!(text_output.contains(&long_message));

    let csv_output = CsvFormatter.format(&matches);
    assert!(csv_output.contains(&long_path));
    assert!(csv_output.contains(&long_message));

    let html_output = HtmlFormatter.format(&matches);
    assert!(html_output.contains(&long_path));
    assert!(html_output.contains(&long_message));

    let md_output = MarkdownFormatter.format(&matches);
    assert!(md_output.contains(&long_path));
    assert!(md_output.contains(&long_message));
}

#[test]
fn test_formatters_with_edge_case_numbers() {
    let matches = vec![
        Match {
            file_path: "test.rs".to_string(),
            line_number: 0,
            column: 0,
            pattern: "TODO".to_string(),
            message: "Zero values".to_string(),
        },
        Match {
            file_path: "test2.rs".to_string(),
            line_number: usize::MAX,
            column: usize::MAX,
            pattern: "FIXME".to_string(),
            message: "Max values".to_string(),
        },
    ];

    // Test all formatters handle edge case numbers
    let json_output = JsonFormatter.format(&matches);
    assert!(json_output.contains("0"));
    assert!(json_output.contains(&usize::MAX.to_string()));

    let text_output = TextFormatter.format(&matches);
    assert!(text_output.contains("0"));
    assert!(text_output.contains(&usize::MAX.to_string()));

    let csv_output = CsvFormatter.format(&matches);
    assert!(csv_output.contains("0"));
    assert!(csv_output.contains(&usize::MAX.to_string()));

    let html_output = HtmlFormatter.format(&matches);
    assert!(html_output.contains("0"));
    assert!(html_output.contains(&usize::MAX.to_string()));

    let md_output = MarkdownFormatter.format(&matches);
    assert!(md_output.contains("0"));
    assert!(md_output.contains(&usize::MAX.to_string()));
}

#[test]
fn test_formatters_comprehensive_special_chars() {
    let matches = vec![Match {
        file_path: "test\n\r\t\"'\\&<>/file.rs".to_string(),
        line_number: 1,
        column: 1,
        pattern: "TODO\n\r\t".to_string(),
        message: "Message\nwith\rnewlines\tand\ttabs\"quotes'apostrophes\\backslashes&ampersands<less>greater/slashes".to_string(),
    }];

    // Test JSON handles all special characters
    let json_output = JsonFormatter.format(&matches);
    assert!(serde_json::from_str::<Vec<Match>>(&json_output).is_ok());

    // Test CSV handles all special characters
    let csv_output = CsvFormatter.format(&matches);
    let mut rdr = ::csv::ReaderBuilder::new().from_reader(csv_output.as_bytes());
    let records: Result<Vec<_>, _> = rdr.records().collect();
    assert!(records.is_ok());

    // Test HTML escapes dangerous characters
    let html_output = HtmlFormatter.format(&matches);
    assert!(html_output.contains("&lt;"));
    assert!(html_output.contains("&gt;"));
    assert!(html_output.contains("&amp;"));
    assert!(html_output.contains("&quot;"));

    // Test Markdown escapes pipes
    let md_output = MarkdownFormatter.format(&matches);
    assert!(!md_output.contains("||")); // Should not have unescaped adjacent pipes
}
