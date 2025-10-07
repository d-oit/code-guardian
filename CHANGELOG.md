## [0.1.0-alpha] - 2025-10-06
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **🌍 Multi-Language Non-Production Code Detection**: Added 13 new detectors for finding non-production code across 30+ programming languages
  - **Development/Phase Patterns**: `Dev`, `Debug`, `Test`, `Phase`, `Staging` 
  - **Non-Production Code**: `ConsoleLog`, `Print`, `Alert`, `Debugger`, `UnusedVar`, `DeadCode`, `Experimental`
- **🎯 Production-Ready Detector Profile**: New comprehensive profile specifically for production readiness scanning
- **📱 Language-Specific Detection**: Smart filtering (e.g., console.log only detected in JS/TS files, alerts in web files)
- **⚙️ Enhanced Multi-Language Support**: Extended support for 30+ file extensions including TypeScript, Python, C#, Go, PHP, Java, Kotlin, Swift, Dart, Scala, and more
- **📋 Production Configuration Template**: Complete `examples/production_ready_config.toml` with severity levels and custom patterns
- **📖 Comprehensive Documentation**: Detailed `examples/production_ready_scan_demo.md` with usage guides for multi-language scanning
- **✅ Comprehensive Test Coverage**: Integration tests demonstrating multi-language detection across JavaScript, TypeScript, Python, Rust, and Go

### Enhanced
- **🔧 Extended DetectorType Enum**: Added 13 new detector types with appropriate severity levels (Critical, High, Medium, Low)
- **🏭 Enhanced DetectorFactory**: New `create_production_ready_detectors()` method and `ProductionReady` profile
- **📁 Expanded File Support**: Default configuration now includes 30+ file extensions for comprehensive language coverage
- **🎨 Improved Regex Patterns**: Optimized detection patterns for better accuracy across different programming languages

### Technical Improvements
- All new detectors follow Rust best practices with comprehensive error handling
- Language-specific filtering prevents false positives (e.g., console.log not detected in Python files)
- Smart test file exclusion (TestDetector skips actual test directories)
- Backward compatible - no breaking changes to existing API

## [0.1.0-alpha] - 2025-10-06

### Added
- Initial alpha release of Code Guardian, a comprehensive code scanning and analysis tool.
- **CLI Crate**: Command-line interface with handlers for scanning, reporting, benchmarking, and advanced operations.
- **Core Crate**: Core scanning engine featuring:
  - Built-in detectors for common code issues.
  - Support for custom detectors via JSON configuration.
  - Distributed scanning for large codebases.
  - Incremental scanning to optimize performance.
  - Enhanced configuration options.
  - Performance monitoring and optimizations.
- **Output Crate**: Multiple output formatters including CSV, HTML, JSON, Markdown, and plain text.
- **Storage Crate**: Database-backed storage with initial schema migrations for persistent data management.
- Comprehensive documentation including tutorials for getting started, advanced usage, automation, and custom detectors.
- Example configurations and detector files to help users get started.
- CI/CD workflows for continuous integration, documentation generation, and automated releases.
- Agent-based development system for collaborative and automated code management.
- Benchmarks and performance tests to ensure optimal scanning speed.
- Test suites across crates for reliability and code quality.