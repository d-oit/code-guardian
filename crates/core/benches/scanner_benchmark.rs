use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use code_guardian_core::{DetectorFactory, DetectorProfile, Scanner, PatternDetector};
use std::path::PathBuf;
use tempfile::TempDir;

fn create_test_files(dir: &TempDir, num_files: usize, lines_per_file: usize) -> Vec<PathBuf> {
    let mut files = Vec::new();
    
    for i in 0..num_files {
        let file_path = dir.path().join(format!("test_{}.rs", i));
        let mut content = String::new();
        
        for j in 0..lines_per_file {
            match j % 5 {
                0 => content.push_str("// TODO: implement this function\n"),
                1 => content.push_str("// FIXME: this needs optimization\n"),
                2 => content.push_str("let value = some_option.unwrap();\n"),
                3 => content.push_str("let data = vec.clone();\n"),
                4 => content.push_str("fn normal_function() {}\n"),
                _ => unreachable!(),
            }
        }
        
        std::fs::write(&file_path, content).unwrap();
        files.push(file_path);
    }
    
    files
}

fn bench_scanner_basic(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    
    let mut group = c.benchmark_group("scanner_basic");
    
    for &num_files in &[10, 50, 100] {
        for &lines_per_file in &[100, 500, 1000] {
            let files = create_test_files(&temp_dir, num_files, lines_per_file);
            let scanner = Scanner::new(DetectorFactory::create_default_detectors());
            
            group.bench_with_input(
                BenchmarkId::new("files_lines", format!("{}_{}", num_files, lines_per_file)),
                &(num_files, lines_per_file),
                |b, _| {
                    b.iter(|| {
                        let matches = scanner.scan(black_box(temp_dir.path())).unwrap();
                        black_box(matches);
                    });
                },
            );
        }
    }
    
    group.finish();
}

fn bench_scanner_profiles(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let _files = create_test_files(&temp_dir, 50, 500);
    
    let mut group = c.benchmark_group("scanner_profiles");
    
    let profiles = vec![
        ("basic", DetectorProfile::Basic),
        ("comprehensive", DetectorProfile::Comprehensive),
        ("security", DetectorProfile::Security),
        ("performance", DetectorProfile::Performance),
        ("rust", DetectorProfile::Rust),
    ];
    
    for (name, profile) in profiles {
        let scanner = Scanner::new(profile.get_detectors());
        
        group.bench_function(name, |b| {
            b.iter(|| {
                let matches = scanner.scan(black_box(temp_dir.path())).unwrap();
                black_box(matches);
            });
        });
    }
    
    group.finish();
}

fn bench_large_files(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    
    let mut group = c.benchmark_group("large_files");
    
    for &file_size_kb in &[10, 100, 500, 1000] {
        let lines = file_size_kb * 10; // Rough approximation
        let file_path = temp_dir.path().join(format!("large_{}.rs", file_size_kb));
        
        let mut content = String::new();
        for i in 0..lines {
            if i % 20 == 0 {
                content.push_str("// TODO: optimize this section\n");
            } else if i % 30 == 0 {
                content.push_str("// FIXME: handle error case\n");
            } else {
                content.push_str("fn regular_code_line() { let x = 42; }\n");
            }
        }
        
        std::fs::write(&file_path, content).unwrap();
        
        let scanner = Scanner::new(DetectorFactory::create_default_detectors());
        
        group.bench_function(format!("{}kb", file_size_kb), |b| {
            b.iter(|| {
                let matches = scanner.scan(black_box(temp_dir.path())).unwrap();
                black_box(matches);
            });
        });
    }
    
    group.finish();
}

fn bench_regex_performance(c: &mut Criterion) {
    use code_guardian_core::detectors::*;
    use std::path::Path;

    let content = "// TODO: implement\n// FIXME: bug here\nlet x = val.unwrap();\nlet y = data.clone();\n".repeat(1000);
    let path = Path::new("test.rs");

    let mut group = c.benchmark_group("regex_performance");

    let detectors: Vec<(&str, Box<dyn code_guardian_core::PatternDetector>)> = vec![
        ("todo", Box::new(TodoDetector)),
        ("fixme", Box::new(FixmeDetector)),
        ("unwrap", Box::new(UnwrapDetector)),
        ("clone", Box::new(CloneDetector)),
    ];

    for (name, detector) in detectors {
        group.bench_function(name, |b| {
            b.iter(|| {
                let matches = detector.detect(black_box(&content), black_box(path));
                black_box(matches);
            });
        });
    }

    group.finish();
}

fn bench_custom_detectors(c: &mut Criterion) {
    use code_guardian_core::custom_detectors::*;
    use std::path::Path;

    let mut group = c.benchmark_group("custom_detectors");

    // Create smaller test content for faster benchmarks
    let content = "// TODO: implement feature\nlet password = \"secret123\";\nfn some_function() {\n    let x = vec.clone();\n    let y = option.unwrap();\n}\nclass MyClass extends Base {\n    constructor() {}\n}\n".repeat(10);
    let path = Path::new("test.rs");

    // Simple custom detector
    let simple_config = CustomDetectorConfig {
        name: "SIMPLE_TODO".to_string(),
        description: "Simple TODO detector".to_string(),
        pattern: r"TODO".to_string(),
        file_extensions: vec![],
        case_sensitive: false,
        multiline: false,
        capture_groups: vec![],
        severity: code_guardian_core::Severity::Low,
        category: DetectorCategory::Testing,
        examples: vec![],
        enabled: true,
    };
    let simple_detector = CustomDetector::new(simple_config).unwrap();

    // Complex custom detector with alternation
    let complex_config = CustomDetectorConfig {
        name: "COMPLEX_CLASS".to_string(),
        description: "Complex class detector".to_string(),
        pattern: r"\bclass\s+\w+\s+extends\s+\w+\s*\{".to_string(),
        file_extensions: vec![],
        case_sensitive: true,
        multiline: false,
        capture_groups: vec![],
        severity: code_guardian_core::Severity::Medium,
        category: DetectorCategory::CodeQuality,
        examples: vec![],
        enabled: true,
    };
    let complex_detector = CustomDetector::new(complex_config).unwrap();

    // Custom detector with capture groups
    let capture_config = CustomDetectorConfig {
        name: "CAPTURE_PASSWORD".to_string(),
        description: "Capture password patterns".to_string(),
        pattern: r"let\s+(\w+)\s*=\s*(\w+);".to_string(),
        file_extensions: vec![],
        case_sensitive: true,
        multiline: false,
        capture_groups: vec!["var".to_string(), "value".to_string()],
        severity: code_guardian_core::Severity::High,
        category: DetectorCategory::Security,
        examples: vec![],
        enabled: true,
    };
    let capture_detector = CustomDetector::new(capture_config).unwrap();

    let detectors = vec![
        ("simple", simple_detector),
        ("complex", complex_detector),
        ("capture", capture_detector),
    ];

    for (name, detector) in detectors {
        group.bench_function(name, |b| {
            b.iter(|| {
                let matches = detector.detect(black_box(&content), black_box(path));
                black_box(matches);
            });
        });
    }

    group.finish();
}

fn bench_custom_detectors_large_files(c: &mut Criterion) {
    use code_guardian_core::custom_detectors::*;
    use std::path::Path;

    let mut group = c.benchmark_group("custom_detectors_large");

    for &size_kb in &[10, 50, 100] {
        let lines = size_kb * 10; // Approximate
        let mut content = String::new();
        for i in 0..lines {
            if i % 5 == 0 {
                content.push_str("// TODO: optimize this large file\n");
            } else if i % 10 == 0 {
                content.push_str("let hardcoded_password = \"secret123456789\";\n");
            } else {
                content.push_str("fn regular_function() { let x = 42; println!(\"{}\", x); }\n");
            }
        }
        let path = Path::new("large_test.rs");

        // Complex regex for large files
        let config = CustomDetectorConfig {
            name: "LARGE_COMPLEX".to_string(),
            description: "Complex pattern on large file".to_string(),
            pattern: r"(?i)password\s*[=:]\s*\w{8,}".to_string(),
            file_extensions: vec![],
            case_sensitive: false,
            multiline: false,
            capture_groups: vec![],
            severity: code_guardian_core::Severity::High,
            category: DetectorCategory::Security,
            examples: vec![],
            enabled: true,
        };
        let detector = CustomDetector::new(config).unwrap();

        group.bench_function(format!("{}kb", size_kb), |b| {
            b.iter(|| {
                let matches = detector.detect(black_box(&content), black_box(path));
                black_box(matches);
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_scanner_basic,
    bench_scanner_profiles,
    bench_large_files,
    bench_regex_performance,
    bench_custom_detectors,
    bench_custom_detectors_large_files
);
criterion_main!(benches);