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
- **Core Agent**: Scanning logic
- **Storage Agent**: Database operations
- **Output Agent**: Formatting
- **CLI Agent**: User interface
- **Testing Agent**: Quality assurance
- **CI Agent**: Automation
- **Docs Agent**: Documentation

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