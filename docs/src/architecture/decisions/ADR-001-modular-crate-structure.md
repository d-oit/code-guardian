# ADR-001: Modular Crate Structure

## Status
Accepted

## Context
Code Guardian needed to be organized for maintainability, independent development, and clear separation of concerns. The monolithic structure was becoming difficult to manage as the codebase grew.

## Decision
Split functionality into focused crates:
- **core**: Scanner engine, detectors, and business logic
- **cli**: Command-line interface and user interactions  
- **storage**: Database operations and persistence
- **output**: Formatting and reporting functionality

## Consequences

### Positive
- Better separation of concerns
- Independent versioning possible
- Easier testing and maintenance
- Clear dependency boundaries
- Parallel development by different team members

### Negative
- Increased complexity in dependency management
- More complex build process
- Additional coordination required between crates

## Implementation Details
```
code-guardian/
├── crates/
│   ├── core/          # Scanner engine and detectors
│   ├── cli/           # Command-line interface
│   ├── storage/       # Database and persistence
│   └── output/        # Formatters and reports
└── Cargo.toml         # Workspace configuration
```

## Date
2024-10-16

## Reviewers
- Code Guardian Team