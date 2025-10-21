# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-10-21
### ⚙️ Miscellaneous Tasks

- Update release-please manifest to v0.1.9


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
### ⚙️ Miscellaneous Tasks

- Bump version to 0.1.5 for patch release

- Prepare for v0.1.5 release

- Bump version to 0.1.5 for patch release


### 🐛 Bug Fixes

- Resolve CI issues for v0.1.7 release - update metrics expect to unwrap, add gitleaks config, fix workflow permissions and syntax


### 💼 Other

- Add Prometheus metrics support\n\n- Add comprehensive metrics collection for scans, performance, and resources\n- Implement HTTP endpoint for Prometheus scraping\n- Add dependencies: prometheus, axum, async-trait, aho-corasick\n- Bump version to 0.1.6 across all crates

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
### ⚙️ Miscellaneous Tasks

- Bump version to 0.1.6 for patch release


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
### 💼 Other

- Bump version to 0.1.2


## [0.1.1] - 2025-10-09
### 🐛 Bug Fixes

- Resolve CI and release workflow issues for v0.1.1-alpha

- Remove insta snapshot test and add context7 mcp agent

- Remove border assertions from text formatter test for cross-platform compatibility

- Remove enforce_styling from text formatter

- Update test to check match data instead of headers

- Change text formatter to simple text output for cross-platform compatibility


### 💼 Other

- Update ci-agent tools and add lib.rs cleanups

- Bump version to 0.2.0

- Bump version to 0.1.1

- Bump version to 0.1.1

- Bump version to 0.1.1


### 📚 Documentation

- Add git integration demo example

- Update CHANGELOG.md

- Add atomic-commit command documentation


### 🚀 Features

- Enhance text formatter test with header assertions and add dev container configuration

- Add GitIntegration module for repository operations

- Add Git CLI commands and refactor stack preset handler

- Enhance CLI with advanced handlers and scan updates

- Add core modules for caching and monitoring


## [0.1.1-alpha] - 2025-10-07
### ⚙️ Miscellaneous Tasks

- Update changelog for v0.1.0

- Prepare for release v0.1.1-alpha


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


