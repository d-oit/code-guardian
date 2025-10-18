---
description: Prepare and execute a release
agent: ci-agent
---

# Release Command

## Overview
The Release Command automates the preparation and execution of software releases, incorporating best practices for GitHub releases, including branch synchronization, complete changelog inclusion, conventional commits for changelog generation, semantic versioning, protected branches, and automated release notes.

## Purpose
To ensure releases are built, tested, and deployed reliably, coordinating multiple agents for a seamless process while addressing issues like develop branch not being up to date and incomplete changelogs in releases.

## Inputs/Outputs
- **Inputs**: Optional arguments like "alpha" for pre-releases, "patch" for patch releases, "dry-run" for dry runs.
- **Outputs**: Release confirmation, version updates, changelog updates, or error reports.

## Dependencies
- Cargo toolchain.
- Agents: CI Agent, Storage Agent, Docs Agent, Git Handler, GitHub, Hive Mind Orchestrator.
- Tools: git-cliff for changelog generation from conventional commits.

## Usage Examples
- Standard release: `/release`
- Pre-release: `/release alpha`
- Patch release: `/release patch`
- Dry-run: `/release dry-run`

## Error Scenarios
- **Build Failures**: Stop and report errors; suggest fixes before retrying.
- **Test Failures**: Halt release; run diagnostics on failing tests.
- **Lint Issues**: Report warnings/errors; require resolution for release.
- **Format Errors**: Stop and suggest running `cargo fmt`.
- **Version Conflicts**: If version bump fails, check existing tags.
- **Agent Handoff Failures**: If an agent fails (e.g., CI pipeline), retry or escalate.
- **Network/Permission Issues**: For pushing tags, ensure auth and connectivity.
- **Branch Sync Issues**: If develop is not up to date, merge main to develop before proceeding.
- **Changelog Incomplete**: Ensure all commits follow conventional commit format for accurate changelog generation.

## Integration Notes
- **Handoff Protocols**: Sequential handoffs: branch sync check -> checks -> version bump -> CI -> storage update -> docs -> git -> github (PR creation and merge). Confirm each step before proceeding.
- **Collaboration**: Uses CI Agent for pipelines, Storage for versions, Docs for updates, Git for commits/tags, GitHub for PR creation and merging. For complex releases, hand off to Hive Mind Orchestrator.
- **Best Practices**: Always run checks first; use semantic versioning; ensure protected branches (main protected); log all actions. Confirm completion with user. Use conventional commits for automated changelog.
- **Edge Cases**: Handle pre-releases differently; for hotfixes, skip some checks if urgent (with caution). Address develop branch not up to date by merging main to develop.

Prepare for release. Optional: $ARGUMENTS (e.g., "alpha" for pre-release, "patch" for patch release, "dry-run" for dry run)

First, ensure branch synchronization:
- Check if develop branch is up to date with main. If not, merge main into develop to sync changes.
- Switch to develop branch if not already on it.

Then, use the atomic-commit-creator agent to ensure all commits are atomic (each representing a single, complete change). This is necessary to prevent issues with non-atomic commits during releases, such as partial changes that could introduce bugs or inconsistencies. Atomic commits make rollbacks easier and ensure releases are reliable.

After that, run quality checks. Stop on any errors or warnings.

Build check: !`cargo build`
If build fails, report errors. If not dry-run, stop.

Test check: !`cargo test`
If tests fail, report failures. If not dry-run, stop.

Lint check: !`cargo clippy`
If clippy finds issues, report warnings/errors. If not dry-run, stop.

Format check: !`cargo fmt --check`
If formatting issues, report. If not dry-run, stop.

Doc tests: !`cargo test --doc`
If doc tests fail, report failures. If not dry-run, stop.

Coverage generation: !`cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info`
If coverage generation fails, report errors. If not dry-run, stop.

Security audit: !`cargo audit`
If security vulnerabilities are found, report. If not dry-run, stop.

If "dry-run" is specified, report the summary of all check results and stop.

Else if all checks pass:
1. Determine new version using semantic versioning: /version-bump $ARGUMENTS
2. @ci-agent to handle CI/CD pipeline.
3. Update version in Cargo.toml using @storage-agent.
4. @docs-agent to update changelog/docs using git-cliff for conventional commits.
5. @git-handler for committing changes and creating a release branch if needed.
6. @github agent to create a pull request from develop to main.
7. @github agent to merge the pull request (ensure main is protected and requires reviews).
8. @git-handler to tag the release on main with semantic version (e.g., v1.2.3).
9. @git-handler to push the tag, triggering GitHub Actions release workflow.
10. Switch to develop branch and merge main back into it to sync changes.
11. If needed, @hive-mind-orchestrator for complex releases.

12. @github agent to monitor the GitHub Actions workflows (CI, Deploy Docs, Release) to ensure they complete successfully. The release workflow will automatically generate changelog using git-cliff and include it in release notes.

Confirm release completion. Ensure changelog is complete by verifying conventional commit format in commits.