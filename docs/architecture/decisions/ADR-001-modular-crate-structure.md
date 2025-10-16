# ADR-001: Modular Crate Structure

## Status
Accepted

## Context
Code-Guardian started as a monolithic security scanner but has grown to include multiple concerns: scanning logic, CLI interface, output formatting, and data storage. As the codebase expanded, we needed to improve maintainability, testability, and reusability.

## Decision
We will organize Code-Guardian into multiple focused crates using Cargo's workspace feature:

- **`core`**: Core scanning logic, detectors, and algorithms
- **`cli`**: Command-line interface and user interaction
- **`output`**: Formatting and reporting functionality  
- **`storage`**: Database and persistence layer

## Rationale

### Benefits
1. **Separation of Concerns**: Each crate has a single, well-defined responsibility
2. **Independent Testing**: Crates can be tested in isolation with focused test suites
3. **Reusability**: Core scanning logic can be used by other interfaces (future web UI, APIs)
4. **Compilation Performance**: Incremental compilation only rebuilds changed crates
5. **Dependency Management**: Each crate declares only the dependencies it needs

### Trade-offs
1. **Initial Complexity**: More complex project structure compared to single crate
2. **Inter-crate Coordination**: Changes affecting multiple crates require coordination
3. **Release Management**: Multiple crates may need synchronized versioning

## Implementation Details

### Crate Dependencies
```
cli -> core, output, storage
output -> core
storage -> core
core -> (external dependencies only)
```

### Module Organization
- **Core crate**: detectors/, scanner/, config/, types/
- **CLI crate**: commands/, handlers/, utils/
- **Output crate**: formatters/ (json, html, text, markdown)
- **Storage crate**: sqlite/, migrations/

### Build Configuration
- Workspace-level `Cargo.toml` manages shared dependencies
- Individual crate `Cargo.toml` files specify crate-specific dependencies
- Shared configuration in workspace root (clippy.toml, rustfmt.toml)

## Consequences

### Positive
- **Faster Development**: Developers can focus on specific concerns
- **Better Testing**: 82%+ coverage target more achievable with focused tests
- **Cleaner APIs**: Well-defined interfaces between components
- **Future Extensibility**: Easy to add new interfaces or swap implementations

### Negative
- **Learning Curve**: New contributors need to understand workspace structure
- **Build Complexity**: More complex CI/CD pipeline with per-crate builds
- **Documentation Overhead**: Need to document inter-crate relationships

## Alternatives Considered

1. **Single Monolithic Crate**: Simpler but poor separation of concerns
2. **Microservices Architecture**: Over-engineered for current scale
3. **Plugin-based Architecture**: Added complexity without clear benefits

## Implementation Status
- ✅ Core crate extracted (85.2% test coverage)
- ✅ CLI crate modularized (52.1% test coverage, improving)
- ✅ Output crate implemented (100% test coverage)
- ✅ Storage crate implemented (99.4% test coverage)
- ✅ Workspace configuration completed
- 🔄 Performance optimization ongoing

## Future Considerations
- Consider extracting detector plugins as separate crates
- Evaluate async interfaces between crates for better performance
- Plan for potential WASM compilation of core crate