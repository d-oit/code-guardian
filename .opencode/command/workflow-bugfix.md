---
escription: Start a bugfix workflow
agent: goap-planner
---
Fix a bug. Describe the bug: $ARGUMENTS

Steps:
1. Use @rust-codebase-locator to find related code.
2. @goap-planner to plan the fix.
3. @hive-mind-orchestrator to coordinate (e.g., @core-agent for fix, @testing-agent for regression tests).
4. Run /cargo-test to ensure no new failures.
5. @rust-security-auditor if security-related.
6. Commit fix with @git-handler.

Provide reproduction steps and fix summary.