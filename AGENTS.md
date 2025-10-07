# AI Development Guidelines for Code-Guardian

## Commands
- **Build**: `cargo build` (workspace), `cargo build -p <crate>` (single crate)
- **Test**: `cargo test` (all), `cargo test -p <crate>` (crate), `cargo test <test_name>` (single test)
- **Lint**: `cargo clippy` (all), `cargo clippy -p <crate>` (crate)
- **Format**: `cargo fmt` (all), `cargo fmt -p <crate>` (crate)
- **Check**: `cargo check` (all), `cargo check -p <crate>` (crate)

## Code Style
- **Formatting**: Use `cargo fmt` (4-space indentation, 100 char lines)
- **Naming**: snake_case for functions/variables, PascalCase for types, SCREAMING_SNAKE_CASE for constants
- **Imports**: Group std, external crates, then local crates; use explicit imports over globs
- **Types**: Use strong typing; prefer `&str` over `String` for parameters; use `Result<T, E>` for fallible operations
- **Error Handling**: Use `thiserror` for custom errors, `anyhow` for generic errors; prefer `?` operator
- **Documentation**: Document public APIs with `///` comments; use `cargo doc` to generate docs
- **Testing**: Write unit tests with `#[test]`; use `#[cfg(test)]` modules; aim for 80%+ coverage
- **Concurrency**: Use `rayon` for parallelism; prefer channels over shared state
- **Serialization**: Use `serde` with derive macros; prefer JSON/YAML over binary formats

## Agent Roles
- **Agent Coordinator**: Orchestrating multi-agent workflows for complex tasks, managing handoffs between agents
- **Atomic Commit Creator**: Analyzing code changes to create atomic commits, suggesting splits and providing commit messages
- **CI Agent**: Handling CI/CD setup, automation, builds, tests, releases, and pipeline health monitoring
- **CLI Agent**: Developing and maintaining command-line interface, building commands, handling user input
- **Clean Code Developer**: Developing or refactoring code with emphasis on clean code principles like readability, maintainability, simplicity
- **Code Review Agent**: Performing automated code reviews on diffs for style, security, and best practices
- **Codebase Consolidator**: Consolidating and cleaning up codebases by removing redundancies, refactoring for better structure
- **Context7 MCP Agent**: Resolving library IDs and fetching up-to-date documentation from external sources via the Context7 MCP
- **Core Agent**: Implementing core scanning logic, pattern detection, scanner implementation, performance optimization
- **Docs Agent**: Managing and creating project documentation, writing READMEs, generating API docs
- **False Positive Validator**: Auditing and validating flagged issues from automated tools to determine if they are genuine problems or false positives
- **General**: Handling general-purpose tasks like researching complex questions, searching for code, and executing multi-step tasks
- **Git Handler**: Performing Git-related operations like committing, branching, merging, resolving conflicts
- **GitHub**: Performing GitHub operations using GitHub CLI, like creating issues, managing PRs, cloning repos
- **GOAP Planner**: Planning and coordinating multi-agent workflows using Goal-Oriented Action Planning
- **Hive Mind Orchestrator**: Coordinating multiple specialized agents for complex tasks using swarm intelligence
- **OpenCode Agent Manager**: Updating existing .md files or creating new ones in the .opencode/agent/ folder or AGENTS.md specifically for OpenCode-related documentation or agent configurations
- **OpenCode Plugin Agent Creator**: Creating new agent configurations based on OpenCode plugins, referencing documentation and generating precise specs
- **Output Agent**: Handling output formatting and serialization, implementing formatters for various formats
- **Package Updater**: Managing dependency updates, checking for newer versions, and verifying changes through build, test, and lint processes
- **Rust Codebase Analyzer**: Analyzing Rust codebase structure, quality, dependencies, performance
- **Rust Codebase Locator**: Locating specific files, functions, modules in Rust codebase
- **Rust Performance Optimizer**: Optimizing Rust code for performance, analyzing loops, allocations, async code
- **Rust Security Auditor**: Auditing Rust code for security vulnerabilities, analyzing unsafe blocks, input validation
- **Storage Agent**: Managing database operations, storage implementation, migrations, data integrity
- **Testing Agent**: Ensuring code quality through testing, writing unit/integration tests, achieving coverage

## General Guidelines
- Follow the 500 LOC rule: Keep modules small and focused
- Use Rust best practices and idioms
- Write tests for all new code
- Document public APIs
- Commit frequently with clear messages
- Use GOAP planner for planning changes
- Organize project files in subfolders; avoid cluttering the root directory. Reserve root for best practices, core configs, and essential files only

## Collaboration
- Agents communicate via issues/PRs
- Use the GOAP planner for complex tasks
- Review code across agents for integration

## Quality Control
- 80%+ test coverage
- Pass CI/CD before merge
- Adhere to modular architecture