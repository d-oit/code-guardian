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
- **Testing**: Write unit tests with `#[test]`; use `#[cfg(test)]` modules; aim for 82%+ coverage
- **Concurrency**: Use `rayon` for parallelism; prefer channels over shared state
- **Serialization**: Use `serde` with derive macros; prefer JSON/YAML over binary formats

## General Guidelines
- Follow the 500 LOC rule: Keep modules small and focused
- Use Rust best practices and idioms
- Write tests for all new code
- Document public APIs
- Commit frequently with clear messages
- Use GOAP planner for planning changes, no dublicate code, always anaylse first before create a new file
- Organize project files in subfolders; avoid cluttering the root directory

## Quality Control
- 82%+ test coverage
- Pass CI/CD before merge
- Adhere to modular architecture

## Development Workflow

### Quick Start
```bash
# Set up development environment
./scripts/dev-workflow.sh setup

# Run all quality checks
./scripts/dev-workflow.sh check

# Start development watch mode
./scripts/dev-workflow.sh watch

# Auto-fix code quality issues
./scripts/dev-workflow.sh fix
```

### Available Commands
- `make help` - Show all available make commands
- `make dev` - Start development mode with auto-rebuild
- `make quick-check` - Run format, lint, build, and test
- `make quality-fix` - Auto-fix formatting and clippy issues
- `make coverage` - Generate test coverage report
- `./scripts/dev-workflow.sh` - Interactive development workflow script

### Pre-commit Hooks
Pre-commit hooks are automatically installed during setup and run:
- Code formatting checks
- Clippy linting
- Test execution (for core changes)
- Security vulnerability scanning

### CI/CD Integration
- Auto-fix workflow applies formatting and clippy fixes automatically
- Comprehensive testing across multiple platforms and Rust versions
- Security auditing and coverage reporting
- Release automation with proper tagging