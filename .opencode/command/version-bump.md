---
description: Determine next version based on git commits
agent: core-agent
---
Analyze recent git commits to determine the next semantic version bump (major, minor, patch). If $ARGUMENTS includes "alpha", make it a pre-release alpha version.

Git log analysis: !`git log --oneline --since="last tag" | head -20`

Rules:
- If commits include "BREAKING CHANGE" or "feat!:", bump major.
- If commits include "feat:", bump minor.
- If commits include "fix:" or "perf:", bump patch.
- Otherwise, no bump or patch.

Current version from Cargo.toml: !`grep '^version' Cargo.toml`

Suggest new version. If "alpha" specified, append "-alpha" to the version.