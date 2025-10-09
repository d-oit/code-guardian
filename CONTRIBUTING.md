# Contributing to Code-Guardian

Thank you for your interest in contributing to Code-Guardian! This document provides guidelines and information for contributors.

## Code of Conduct

This project follows a code of conduct to ensure a welcoming environment for all contributors. Please be respectful and constructive in all interactions.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Set up the development environment:
   ```bash
   git clone https://github.com/d-oit/code-guardian.git
   cd code-guardian
   cargo build
   cargo test
   ```

## Development Guidelines

### Code Style
- Use `cargo fmt` for formatting (4-space indentation, 100 char lines)
- Follow Rust naming conventions: snake_case for functions/variables, PascalCase for types
- Group imports: std, external crates, then local crates
- Use explicit imports over globs where possible

### Architecture
- Follow the modular architecture with separate crates for different concerns
- Keep modules under 500 lines of code
- Use strong typing and prefer `&str` over `String` for parameters
- Use `Result<T, E>` for fallible operations

### Error Handling
- Use `thiserror` for custom errors, `anyhow` for generic errors
- Prefer the `?` operator for error propagation

### Documentation
- Document all public APIs with `///` comments
- Use `cargo doc` to generate documentation
- Write clear, concise documentation with examples where helpful

### Testing
- Write unit tests for all new code using `#[test]`
- Use `#[cfg(test)]` modules for test-specific code
- Aim for 82%+ test coverage
- Run tests with `cargo test`

### Performance
- Use `rayon` for parallelism where appropriate
- Prefer channels over shared state for concurrency

### Serialization
- Use `serde` with derive macros
- Prefer JSON/YAML over binary formats when possible

## Agent Roles

Code-Guardian uses specialized AI agents for different aspects:

- **Core Agent**: Scanning logic and pattern detection
- **Storage Agent**: Database operations
- **Output Agent**: Formatting and output generation
- **CLI Agent**: User interface and command handling
- **Testing Agent**: Quality assurance and testing
- **CI Agent**: Automation and continuous integration
- **Docs Agent**: Documentation management

## Workflow

1. Create a feature branch from `main`
2. Make your changes following the guidelines above
3. Run the quality checks:
   ```bash
   cargo fmt --check
   cargo clippy
   cargo test
   cargo build
   ```
4. Commit with clear, descriptive messages
5. Push to your fork and create a pull request

### Commit Messages

Use conventional commit format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `refactor:` for code refactoring
- `test:` for test additions/changes
- `chore:` for maintenance tasks

Example: `feat: add support for custom pattern detection`

## Pull Request Process

1. Ensure your PR description clearly describes the changes and their purpose
2. Reference any related issues
3. Ensure all CI checks pass
4. Request review from maintainers
5. Address any feedback and make necessary changes

## Reporting Issues

When reporting bugs or requesting features:

- Use the GitHub issue tracker
- Provide clear, detailed descriptions
- Include steps to reproduce for bugs
- Specify your environment (OS, Rust version, etc.)

## Adding New Patterns

To add new pattern detectors:

1. Implement the `PatternDetector` trait in the `core` crate
2. Add the detector to the scanner in the CLI
3. Update documentation and tests

## Adding Output Formats

To add new output formats:

1. Implement the `Formatter` trait in the `output` crate
2. Add the format option to the CLI
3. Update documentation and tests

## Documentation

- Keep the README up-to-date
- Add examples for new features
- Update API documentation with code changes

## Testing

- Write comprehensive tests for new functionality
- Test edge cases and error conditions
- Ensure integration tests cover cross-crate interactions

## Security

- Be mindful of security implications in code changes
- Report security issues privately to maintainers
- Follow secure coding practices

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

Thank you for contributing to Code-Guardian!