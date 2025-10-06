---
description: Start a new feature workflow
agent: goap-planner
---

# Workflow New Feature Command

## Overview
The Workflow New Feature Command initiates development workflows for new features, planning and coordinating agents for implementation, testing, and documentation.

## Purpose
To streamline feature development by creating plans, coordinating agents, and ensuring iterative quality checks.

## Inputs/Outputs
- **Inputs**: Feature description as arguments.
- **Outputs**: Plan summary, agent coordination logs, commit confirmations.

## Dependencies
- Agents: GOAP Planner, Hive Mind Orchestrator, Core Agent, Testing Agent, Docs Agent, Rust Security Auditor, Rust Performance Optimizer, Git Handler.

## Usage Examples
- Basic feature: `/workflow-new-feature "Add user login"`
- Complex feature: `/workflow-new-feature "Implement payment system with security"`

## Changelog
- v1.0: Initial workflow with planning and coordination.
- v1.1: Added iterative checks and reviews.

## Error Scenarios
- **Planning Failures**: If GOAP Planner can't create a plan, seek more details from user.
- **Agent Coordination Issues**: Handoff failures; retry launches or adjust plan.
- **Check Failures**: Build/test errors; iterate fixes before proceeding.
- **Review Conflicts**: Security/performance issues; resolve before commit.
- **Commit Errors**: Git conflicts; resolve manually.
- **Resource Limits**: Too many agents; prioritize subtasks.

## Integration Notes
- **Handoff Protocols**: Start with planner, then orchestrator for coordination. Iterative handoffs for checks/reviews. Confirm each step.
- **Collaboration**: Integrates with Core for code, Testing for validation, Auditors for reviews, Git for commits. Use Orchestrator for complex features.
- **Best Practices**: Run checks iteratively; commit frequently. Provide summaries.
- **Edge Cases**: For urgent features, skip some reviews (with caution); handle dependencies in plan.

Initiate new feature development. Describe the feature: $ARGUMENTS

Steps:
1. Use @goap-planner to create a plan for the feature.
2. @hive-mind-orchestrator to coordinate agents (e.g., @core-agent for implementation, @testing-agent for tests, @docs-agent for docs).
3. Run /cargo-check and /cargo-test iteratively.
4. @rust-security-auditor and @rust-performance-optimizer for reviews.
5. Commit with clear message using @git-handler.

Provide a summary of the plan and next steps.