---
description: Start a new feature workflow
agent: goap-planner
---
Initiate new feature development. Describe the feature: $ARGUMENTS

Steps:
1. Use @goap-planner to create a plan for the feature.
2. @hive-mind-orchestrator to coordinate agents (e.g., @core-agent for implementation, @testing-agent for tests, @docs-agent for docs).
3. Run /cargo-check and /cargo-test iteratively.
4. @rust-security-auditor and @rust-performance-optimizer for reviews.
5. Commit with clear message using @git-handler.

Provide a summary of the plan and next steps.