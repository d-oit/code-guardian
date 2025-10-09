# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2025-01-09

### Fixed
- GitHub Actions release workflow binary name mismatch (code-guardian-cli vs code_guardian_cli)
- Duplicate anyhow import in monitoring module
- Syntax error in CLI main.rs tracing initialization

### Added
- Automatic changelog integration in release workflow
- Enhanced release descriptions from CHANGELOG.md
- Comprehensive workflow issue analysis and documentation

### Changed
- Release workflow now extracts and uses proper changelog content
- Improved error handling in release process
- Updated existing v0.1.1 release with proper description

## [0.1.1] - 2025-10-09

### Added
- Git CLI commands and GitIntegration module for repository operations
- Dev container configuration

### Fixed
- Text formatter for cross-platform compatibility
- Test updates for match data checking
- Removed enforce_styling from text formatter

### Changed
- CI agent tools updates and lib.rs cleanups
- Documentation updates including atomic-commit command and git integration demo

## [0.1.1-alpha] - 2025-10-07

### Added
- Add best practice GitHub Ruleset JSON
- Update various components, add production handlers, examples, and remove plans file

### Fixed
- Fix YAML indentation in docs workflow
- Fix docs deployment: add index.html for workspace docs
- Remove target from cache to fix stale docs builds
- Add Codecov token to resolve rate limit issue
- Address clippy warnings for len_zero and unused_variables

### Changed
- Update workflow to use GitHub artifacts for coverage instead of external services
- Enhance release command documentation with branch sync, dry-run, and best practices
- Update project files and dependencies

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