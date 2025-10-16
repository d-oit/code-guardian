use code_guardian_core::Match;
use code_guardian_output::*;
use std::time::Instant;

#[test]
fn test_formatter_performance_with_large_datasets() {
    let large_matches: Vec<Match> = (0..1000)
        .map(|i| Match {
            file_path: format!("file_{}.rs", i),
            line_number: i,
            column: i % 100,
            pattern: format!("PATTERN_{}", i % 10),
            message: format!(
                "Message number {} with some additional text to make it longer",
                i
            ),
        })
        .collect();

    let formatters: Vec<(&str, Box<dyn Formatter>)> = vec![
        ("JSON", Box::new(JsonFormatter)),
        ("Text", Box::new(TextFormatter)),
        ("CSV", Box::new(CsvFormatter)),
        ("HTML", Box::new(HtmlFormatter)),
        ("Markdown", Box::new(MarkdownFormatter)),
    ];

    for (name, formatter) in formatters {
        let start = Instant::now();
        let output = formatter.format(&large_matches);
        let duration = start.elapsed();

        // Performance should be reasonable (less than 1 second for 1000 matches)
        assert!(
            duration.as_secs() < 1,
            "Formatter {} took too long: {:?}",
            name,
            duration
        );

        // Output should not be empty
        assert!(
            !output.is_empty(),
            "Formatter {} produced empty output",
            name
        );

        // Should contain data from first and last match
        assert!(
            output.contains("file_0.rs"),
            "Formatter {} missing first file",
            name
        );
        assert!(
            output.contains("file_999.rs"),
            "Formatter {} missing last file",
            name
        );
    }
}

#[test]
fn test_memory_efficiency() {
    // Test that formatters don't use excessive memory with many matches
    let matches: Vec<Match> = (0..10000)
        .map(|i| Match {
            file_path: format!("memory_test_{}.rs", i),
            line_number: i,
            column: i,
            pattern: "TODO".to_string(),
            message: "x".repeat(100), // 100 character message
        })
        .collect();

    // Each formatter should handle this without panicking
    let _json = JsonFormatter.format(&matches);
    let _text = TextFormatter.format(&matches);
    let _csv = CsvFormatter.format(&matches);
    let _html = HtmlFormatter.format(&matches);
    let _markdown = MarkdownFormatter.format(&matches);
}

#[test]
fn test_concurrent_usage() {
    use std::sync::Arc;
    use std::thread;

    let matches = Arc::new(vec![Match {
        file_path: "concurrent_test.rs".to_string(),
        line_number: 1,
        column: 1,
        pattern: "TODO".to_string(),
        message: "Concurrent access test".to_string(),
    }]);

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let matches = matches.clone();
            thread::spawn(move || {
                let formatters: Vec<Box<dyn Formatter>> = vec![
                    Box::new(JsonFormatter),
                    Box::new(TextFormatter),
                    Box::new(CsvFormatter),
                    Box::new(HtmlFormatter),
                    Box::new(MarkdownFormatter),
                ];

                for formatter in formatters {
                    let output = formatter.format(&matches);
                    assert!(output.contains("concurrent_test.rs"));
                    assert!(output.contains("TODO"));
                }
                i
            })
        })
        .collect();

    // All threads should complete successfully
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
}

#[test]
fn test_formatter_consistency_across_runs() {
    let matches = vec![Match {
        file_path: "consistency_test.rs".to_string(),
        line_number: 42,
        column: 10,
        pattern: "TODO".to_string(),
        message: "Consistency test message".to_string(),
    }];

    let formatters: Vec<Box<dyn Formatter>> = vec![
        Box::new(JsonFormatter),
        Box::new(TextFormatter),
        Box::new(CsvFormatter),
        Box::new(HtmlFormatter),
        Box::new(MarkdownFormatter),
    ];

    for formatter in formatters {
        // Run the same formatter multiple times - should produce identical output
        let output1 = formatter.format(&matches);
        let output2 = formatter.format(&matches);
        let output3 = formatter.format(&matches);

        assert_eq!(output1, output2, "Formatter should be deterministic");
        assert_eq!(output2, output3, "Formatter should be deterministic");
    }
}
