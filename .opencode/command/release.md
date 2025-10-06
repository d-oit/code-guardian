---
description: Prepare and execute a release
agent: ci-agent
---

# Release Command

## Overview
The Release Command automates the preparation and execution of software releases, including quality checks, versioning, and deployment.

## Purpose
To ensure releases are built, tested, and deployed reliably, coordinating multiple agents for a seamless process.

## Inputs/Outputs
- **Inputs**: Optional arguments like "alpha" for pre-releases.
- **Outputs**: Release confirmation, version updates, or error reports.

## Dependencies
- Cargo toolchain.
- Agents: CI Agent, Storage Agent, Docs Agent, Git Handler, GitHub, Hive Mind Orchestrator.

## Usage Examples
- Standard release: `/release`
- Pre-release: `/release alpha`
- Patch release: `/release patch`

## Changelog
- v1.0: Basic release with checks.
- v1.1: Added agent coordination.

## Error Scenarios
- **Build Failures**: Stop and report errors; suggest fixes before retrying.
- **Test Failures**: Halt release; run diagnostics on failing tests.
- **Lint Issues**: Report warnings/errors; require resolution for release.
- **Format Errors**: Stop and suggest running `cargo fmt`.
- **Version Conflicts**: If version bump fails, check existing tags.
- **Agent Handoff Failures**: If an agent fails (e.g., CI pipeline), retry or escalate.
- **Network/Permission Issues**: For pushing tags, ensure auth and connectivity.

## Integration Notes
- **Handoff Protocols**: Sequential handoffs: checks -> version bump -> CI -> storage update -> docs -> git -> github (PR creation and merge). Confirm each step before proceeding.
- **Collaboration**: Uses CI Agent for pipelines, Storage for versions, Docs for updates, Git for commits/tags, GitHub for PR creation and merging. For complex releases, hand off to Hive Mind Orchestrator.
- **Best Practices**: Always run checks first; log all actions. Confirm completion with user.
- **Edge Cases**: Handle pre-releases differently; for hotfixes, skip some checks if urgent (with caution).

Prepare for release. Optional: $ARGUMENTS (e.g., "alpha" for pre-release)

First, run quality checks. Stop on any errors or warnings.

Build check: !`cargo build`
If build fails, stop and report errors.

Test check: !`cargo test`
If tests fail, stop and report failures.

Lint check: !`cargo clippy`
If clippy finds issues, stop and report warnings/errors.

Format check: !`cargo fmt --check`
If formatting issues, stop and report.

If all checks pass:
1. Determine new version: /version-bump $ARGUMENTS
2. @ci-agent to handle CI/CD pipeline.
3. Update version in Cargo.toml using @storage-agent.
4. @docs-agent to update changelog/docs.
5. @git-handler for tagging and pushing.
6. @github agent to create a pull request from the current branch (e.g., develop) to main.
7. @github agent to merge the pull request.
8. Switch to develop branch and merge main back into it to sync changes.
9. If needed, @hive-mind-orchestrator for complex releases.

10. @github agent to monitor the GitHub Actions workflows (especially the release workflow) to ensure they complete successfully.

Confirm release completion.