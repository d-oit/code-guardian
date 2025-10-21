# Performance Best Practices

## Overview

This document outlines performance best practices for Code Guardian development and deployment, covering compilation optimization, runtime performance, and scalability considerations.

## Build Performance

### Compilation Optimization

#### 1. Incremental Compilation
```toml
# .cargo/config.toml
[build]
incremental = true
codegen-units = 256
```

#### 2. Parallel Compilation
```bash
# Use all available cores
export CARGO_BUILD_JOBS=$(nproc)

# Or set a specific number
export CARGO_BUILD_JOBS=8
```

#### 3. Dependency Optimization
```toml
# Cargo.toml
[profile.dev]
incremental = true
debug = 1  # Faster compilation, basic debugging

[profile.dev.package."*"]
opt-level = 1  # Optimize dependencies in dev mode

[profile.release]
lto = "thin"
codegen-units = 1
panic = "abort"
```

#### 4. Feature Flags
```toml
# Use feature flags to conditionally compile expensive features
[features]
default = ["git-integration"]
git-integration = ["git2"]
llm-detection = ["regex", "serde_json"]
```

### Build Caching

#### 1. sccache Integration
```bash
# Install sccache
cargo install sccache

# Configure environment
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR=/tmp/sccache

# Monitor cache usage
sccache --show-stats
```

#### 2. CI/CD Caching
```yaml
# GitHub Actions
- uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target/
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

## Runtime Performance

### Scanning Optimization

#### 1. Parallel Processing
```rust
use rayon::prelude::*;

// Process files in parallel
let results: Vec<ScanResult> = files
    .par_iter()
    .map(|file| scan_file(file))
    .collect();
```

#### 2. Memory-Efficient File Reading
```rust
use std::io::{BufRead, BufReader};

// Stream large files instead of loading entirely
fn scan_large_file(path: &Path) -> Result<ScanResult> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        // Process line by line
    }
}
```

#### 3. Smart Caching
```rust
use std::collections::HashMap;
use std::time::SystemTime;

pub struct FileCache {
    cache: HashMap<PathBuf, (SystemTime, ScanResult)>,
}

impl FileCache {
    pub fn get_or_scan(&mut self, path: &Path) -> Result<ScanResult> {
        let metadata = fs::metadata(path)?;
        let modified = metadata.modified()?;
        
        // Check cache validity
        if let Some((cached_time, result)) = self.cache.get(path) {
            if *cached_time >= modified {
                return Ok(result.clone());
            }
        }
        
        // Scan and cache
        let result = scan_file(path)?;
        self.cache.insert(path.to_owned(), (modified, result.clone()));
        Ok(result)
    }
}
```

### Detector Performance

#### 1. Regex Optimization
```rust
use regex::Regex;
use once_cell::sync::Lazy;

// Compile regex patterns once
static SQL_INJECTION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)select\s+.*\s+from\s+.*\s+where\s+.*=.*\+").unwrap()
});

// Use compiled regex
fn detect_sql_injection(content: &str) -> bool {
    SQL_INJECTION_REGEX.is_match(content)
}
```

#### 2. Early Exit Strategies
```rust
// Skip expensive checks for small files
if file_size < 1024 {
    return Ok(ScanResult::empty());
}

// Skip binary files
if is_binary_file(path) {
    return Ok(ScanResult::empty());
}
```

#### 3. Pattern Matching Optimization
```rust
// Use string matching for simple patterns before regex
if !content.contains("SELECT") {
    return false; // Skip regex check
}

// Then apply more expensive regex
SQL_INJECTION_REGEX.is_match(content)
```

## Memory Management

### 1. String Interning
```rust
use string_cache::DefaultAtom;

// Intern frequently used strings
let pattern_name = DefaultAtom::from("sql_injection");
```

### 2. Zero-Copy Operations
```rust
use std::borrow::Cow;

fn process_content(content: &str) -> Cow<str> {
    if needs_processing(content) {
        Cow::Owned(expensive_processing(content))
    } else {
        Cow::Borrowed(content)
    }
}
```

### 3. Memory Pools
```rust
use object_pool::Pool;

// Reuse expensive objects
static BUFFER_POOL: Lazy<Pool<Vec<u8>>> = Lazy::new(|| {
    Pool::new(32, || Vec::with_capacity(8192))
});
```

## Profiling and Monitoring

### 1. Build Time Analysis
```bash
# Analyze compilation times
cargo build --timings

# Profile dependencies
cargo tree --duplicates
cargo bloat --release --crates
```

### 2. Runtime Profiling
```bash
# CPU profiling with flamegraph
cargo install flamegraph
cargo flamegraph --bin code-guardian -- scan ./test_dir

# Memory profiling
valgrind --tool=massif target/release/code-guardian scan ./test_dir
```

### 3. Benchmark Suite
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_scanner(c: &mut Criterion) {
    let test_file = "test_data/large_file.rs";
    
    c.bench_function("scan_large_file", |b| {
        b.iter(|| scan_file(black_box(test_file)))
    });
    
    c.bench_function("parallel_scan", |b| {
        b.iter(|| scan_directory_parallel(black_box("test_data/")))
    });
}

criterion_group!(benches, benchmark_scanner);
criterion_main!(benches);
```

## Performance Targets

### Build Performance
- **Full workspace build**: < 2 minutes
- **Incremental build**: < 30 seconds
- **CI/CD pipeline**: < 5 minutes

### Runtime Performance
- **Small files (< 1KB)**: < 1ms per file
- **Medium files (1-100KB)**: < 10ms per file
- **Large files (> 1MB)**: < 100ms per file
- **Memory usage**: < 100MB for typical scans

### LLM Detection Performance
- **Security detectors**: < 5ms additional per file
- **Quality detectors**: < 3ms additional per file
- **Comprehensive scan**: < 10% total overhead

## Optimization Checklist

### Development
- [ ] Use incremental compilation
- [ ] Enable parallel builds
- [ ] Configure sccache
- [ ] Use feature flags for optional dependencies
- [ ] Profile build times regularly

### Runtime
- [ ] Implement parallel processing
- [ ] Add smart caching
- [ ] Optimize regex patterns
- [ ] Use early exit strategies
- [ ] Profile memory usage

### Production
- [ ] Enable LTO for release builds
- [ ] Configure appropriate worker counts
- [ ] Monitor performance metrics
- [ ] Set resource limits
- [ ] Implement graceful degradation

## Monitoring and Alerting

### Key Metrics
```rust
pub struct PerformanceMetrics {
    pub scan_duration_ms: u64,
    pub files_processed: u64,
    pub bytes_processed: u64,
    pub memory_usage_mb: u64,
    pub cache_hit_rate: f64,
}
```

### Performance Regression Detection
```bash
# Run benchmarks with baseline comparison
cargo bench -- --save-baseline main
cargo bench -- --baseline main

# Fail CI if performance regresses > 10%
cargo bench -- --baseline main --threshold 10
```

## Troubleshooting

### Common Performance Issues

1. **Slow compilation**
   - Check for large monolithic modules
   - Profile with `cargo build --timings`
   - Consider splitting large crates

2. **High memory usage**
   - Profile with valgrind or heaptrack
   - Check for memory leaks in caches
   - Implement memory limits

3. **Slow scanning**
   - Profile with flamegraph
   - Check regex pattern efficiency
   - Verify parallel processing is working

### Performance Analysis Tools

```bash
# Install profiling tools
cargo install cargo-bloat
cargo install cargo-udeps
cargo install flamegraph
cargo install criterion

# Generate performance reports
cargo bloat --release --crates > performance/binary-size.txt
flamegraph target/release/code-guardian scan test_data/ > performance/flamegraph.svg
```

## Future Optimizations

### Planned Improvements
- [ ] SIMD pattern matching for hot paths
- [ ] Async I/O for concurrent file processing
- [ ] Custom allocator for high-frequency allocations
- [ ] GPU acceleration for large-scale pattern matching

### Research Areas
- Machine learning for pattern optimization
- Just-in-time regex compilation
- Distributed scanning across multiple nodes
- Advanced caching strategies (LRU, LFU)

---

*Last updated: 2024-12-19*
*See also: [Architecture Overview](../architecture/overview.md), [Benchmark Results](latest.md)*