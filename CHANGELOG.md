# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).






## [1.0.1] - 2025-10-28

### 🔄 Version Management

- Synchronized all workspace crates to version 1.0.1
- Updated release-please manifest configuration
- Applied automated version management

## [1.0.0] - 2025-10-28

### 🔄 Version Management

- Synchronized all workspace crates to version 1.0.0
- Updated release-please manifest configuration
- Applied automated version management

## [0.3.0] - 2025-10-28

### 🔄 Version Management

- Synchronized all workspace crates to version 0.3.0
- Updated release-please manifest configuration
- Applied automated version management

## [0.2.4] - 2025-10-28

### 🔄 Version Management

- Synchronized all workspace crates to version 0.2.4
- Updated release-please manifest configuration
- Applied automated version management

## [0.2.3] - 2025-10-28

### 🔄 Version Management

- Synchronized all workspace crates to version 0.2.3
- Updated release-please manifest configuration
- Applied automated version management

## [0.2.2] - 2025-10-27

### 🚀 Features

- Complete release workflow improvements and GitHub Actions fixes
- Enhanced CI/CD pipeline with comprehensive testing
- Improved security scanning and vulnerability detection
- Synchronized all crate versions to v0.2.2 for consistency

### 🐛 Bug Fixes

- Resolve GitHub Actions YAML syntax errors
- Fix release workflow to extract only current version's changelog section
- Correct release notes to include only current version's changes
- Fixed license compliance issues in code-guardian-output crate
- Resolved cargo-deny configuration issues
- Fixed performance benchmark execution to run actual benchmarks

### ⚙️ Miscellaneous Tasks

- Update release-please manifest configurations
- Improve documentation and changelog management
- Enhanced workflow automation and reliability
- Synchronized workspace crate versions (core: 0.1.10→0.2.2, cli: 0.1.10→0.2.2)
- Updated release-please manifest from 0.1.9 to 0.2.2

## [0.2.1] - 2025-10-24

### 🚀 Features

- Integrated release changes with comprehensive fixes
- Enhanced workflow improvements and dependency updates
- Production-ready CI/CD enhancements

### 🐛 Bug Fixes

- Resolve final GitHub Actions workflow issues
- Complete major YAML syntax fixes
- Improve conditional logic in enhanced-ci.yml

### ⚙️ Miscellaneous Tasks

- Merge release/v0.2.1 into main with all fixes applied
- Update various components and configurations
- Comprehensive testing and validation improvements

## [0.1.10] - 2025-10-23

### 🚀 Features

- Add real-world performance verification results
- Add comprehensive performance documentation

### ⚙️ Miscellaneous Tasks

- 3c2281e Bump version to 0.2.1
- 07d10a3 Update .gitignore to ignore generated documentation, coverage reports, performance logs, and macOS files
- a179d66 Merge pull request #32 from d-oit/update-release-workflow
- a232064 Update release workflow to extract only current version's changelog section and add commit summary
- b4aebcd Merge pull request #31 from d-oit/fix-release-notes
- 885158c Ensure release notes include only current version's changes
- 012bf2d Merge pull request #30 from d-oit/changelog-update
- 0fa52a1 docs: update CHANGELOG.md to remove v0.2.0 entry and correct v0.1.9 summary
- 990fe5c chore(release): prepare for v0.2.0
- e95ac9a chore: update release-please manifest to v0.1.9
- 5671f96 feat: complete v0.1.10 release preparation with version updates and changelog

## [0.1.9] - 2025-10-21

### ⚙️ Miscellaneous Tasks

• 82947f0 chore(release): prepare for v0.1.8
• f537214 fix: resolve TruffleHog BASE/HEAD same commit issue (#24)
• 2b92f55 Update Perplexity Agents (#22)
• 95f65c3 fix: add gitleaks configuration to handle test data (#21)
• 974c83d fix: final security workflow syntax and artifact issues
• fd1ad5a fix: final security workflow fixes
• 739d606 fix: resolve security workflow issues
• 3335bae fix: resolve GitHub Actions failures

### 🚀 Features

- Add production-ready CI/CD, documentation, testing, and monitoring features

## [0.1.8] - 2025-10-21

### 🐛 Bug Fixes

- Resolve GitHub Actions failures
- Resolve security workflow issues
- Final security workflow fixes
- Final security workflow syntax and artifact issues
- Add gitleaks configuration to handle test data (#21)
- Resolve TruffleHog BASE/HEAD same commit issue (#24)

### 🚀 Features

- Consolidate workflows following GitHub Actions best practices

## [0.1.7] - 2025-10-18

### 🐛 Bug Fixes

- Resolve CI issues for v0.1.7 release - update metrics expect to unwrap, add gitleaks config, fix workflow permissions and syntax

### 💼 Other

- Add Prometheus metrics support
  - Add comprehensive metrics collection for scans, performance, and resources
  - Implement HTTP endpoint for Prometheus scraping
  - Add dependencies: prometheus, axum, async-trait, aho-corasick
  - Bump version to 0.1.6 across all crates

- Enhance CI/CD pipelines with sccache, nextest, and incremental builds

- Adjust clippy settings to treat warnings as warnings instead of errors for 0.1.7 release

### 🚀 Features

- Add Perplexity AI provider support

- Enhance Perplexity AI agents with detailed descriptions and improved functionality

- Create enhanced CI/CD workflow combining best features from existing workflows

- Add health monitoring server with HTTP endpoints

- Enhance detector factory and LLM detection capabilities

### 🚜 Refactor

- Update perplexity agents to use frontmatter config with temperature

## [0.1.6] - 2025-10-16

## [0.1.4] - 2025-10-16

### 🐛 Bug Fixes

- Update changelog for v0.1.3 and fix release workflow YAML formatting

### 💼 Other

- Remove temporary GOAP coordination files

### 📚 Documentation

- Update agent documentation with GOAP coordination learnings

### 🚀 Features

- GOAP Phase 1-2 Quality Check Optimization

- GOAP Phase 3 Complete - Long-term CI/CD Optimizations

- Enhance release-please configuration for automatic changelog generation

- Complete Phase 1 & 2 implementation - Quality checks and comprehensive test coverage

- Add Phase 3 optimization files and documentation

## [0.1.3] - 2025-10-12

### 🚀 Features

- Add monitoring workflow to track recent workflow failures

- Complete GitHub workflows and branch protection

## [0.1.3-test] - 2025-10-10

### 🐛 Bug Fixes

- Apply cargo fmt formatting

- Apply formatting and improve CI workflow

- Make CI workflow cross-platform compatible

- Apply cargo fmt formatting to fix CI issues

- Remove invalid --fail-under option from cargo llvm-cov

- Update GitHub Actions workflows and code fixes

- Correct YAML indentation in ci.yml

- Correct indentation in coverage job

### 📚 Documentation

- Optimize ci-agent.md with orchestration workflow and agent handoffs

### 🚀 Features

- Add comprehensive code quality automation

- Optimize development workflow with comprehensive tooling

- Enhance Makefile with comprehensive development targets

## [0.1.2] - 2025-10-09

## [0.1.1] - 2025-10-09

### 🐛 Bug Fixes

- Resolve CI and release workflow issues for v0.1.1-alpha

- Remove insta snapshot test and add context7 mcp agent

- Remove border assertions from text formatter test for cross-platform compatibility

- Remove enforce_styling from text formatter

- Update test to check match data instead of headers

- Change text formatter to simple text output for cross-platform compatibility

### 📚 Documentation

- Add git integration demo example

- Add atomic-commit command documentation

### 🚀 Features

- Enhance text formatter test with header assertions and add dev container configuration

- Add GitIntegration module for repository operations

- Add Git CLI commands and refactor stack preset handler

- Enhance CLI with advanced handlers and scan updates

- Add core modules for caching and monitoring

## [0.1.1-alpha] - 2025-10-07

### 🐛 Bug Fixes

- Address clippy warnings for len_zero and unused_variables

- Add Codecov token to resolve rate limit issue

### 💼 Other

- Update workflow to use GitHub artifacts for coverage instead of external services

### 📚 Documentation

- Enhance release command documentation with branch sync, dry-run, and best practices

## [0.1.0] - 2025-10-06

### 🐛 Bug Fixes

- Format code with cargo fmt

### 🚀 Features

- Add best practice GitHub Ruleset JSON

- Add monitoring of GitHub Actions to release command

- Update various components, add production handlers, examples, and remove plans file