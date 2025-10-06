---
description: Benchmark performance of the Rust codebase
agent: rust-performance-optimizer
---
Execute performance benchmarks to measure and analyze the speed and efficiency of the Rust codebase.

Steps:
1. Run benchmarks using `cargo bench` to execute all benchmark tests in the project.
2. Review benchmark results for execution times, throughput, and any regressions.
3. Profile the code if needed using tools like `cargo flamegraph` or `perf` to identify hotspots.
4. Analyze loops, allocations, and async code for optimization opportunities.
5. Suggest and implement performance improvements, such as reducing allocations or optimizing algorithms.
6. Re-run benchmarks to verify improvements and ensure no degradation.

Benchmark output: !`cargo bench`