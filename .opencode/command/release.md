---
description: Prepare and execute a release
agent: ci-agent
---
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
6. If needed, @hive-mind-orchestrator for complex releases.

Confirm release completion.