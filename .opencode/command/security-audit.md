---
description: This command initiates a security audit on the Rust codebase to identify potential vulnerabilities.
agent: rust-security-auditor
---

# Security Audit Command

## Purpose
To ensure the codebase is secure by analyzing for common security issues such as unsafe code usage, input validation flaws, and other risks.

## Agent Assignment
rust-security-auditor

## Steps
1. Scan the codebase for unsafe blocks and review their necessity.
2. Analyze input validation and sanitization in user-facing functions.
3. Check for potential vulnerabilities like buffer overflows, injection attacks, or race conditions.
4. Generate a detailed report of findings with recommendations for fixes.
5. Optionally, integrate with CI/CD for automated audits.

## Dependencies
- Access to the Rust codebase
- Rust Security Auditor agent

## Usage Examples
Run the command via OpenCode: `opencode run security-audit`

## Changelog
- v1.0: Initial creation for basic security auditing.