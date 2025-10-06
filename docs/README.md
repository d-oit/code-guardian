# Code-Guardian Documentation

Welcome to the Code-Guardian documentation! This section contains tutorials, guides, and reference materials.

## Tutorials

### [Getting Started](tutorials/getting-started.md)
Learn the basics of scanning your codebase and generating reports.

### [Advanced Usage](tutorials/advanced-usage.md)
Explore advanced features like custom databases, scan comparisons, and programmatic processing.

### [Custom Detectors Guide](tutorials/custom-detectors.md)
Learn how to create and use custom pattern detectors for security, code quality, and project-specific rules.

### [Automation](tutorials/automation.md)
Set up automated scanning with CI/CD, cron jobs, and monitoring.

## API Documentation

The full API documentation is available at: [GitHub Pages](https://d-oit.github.io/code-guardian/)

To generate docs locally:

```bash
cargo doc --open
```

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines.

## Examples

Check out the [examples](../examples/) directory for sample usage and output formats.

## Architecture

Code-Guardian follows a modular architecture:

- **core**: Scanning logic and pattern detection
- **storage**: SQLite database operations
- **output**: Multiple output format support
- **cli**: Command-line interface

## Support

- [GitHub Issues](https://github.com/d-oit/code-guardian/issues) for bug reports
- [Discussions](https://github.com/d-oit/code-guardian/discussions) for questions

## License

Code-Guardian is licensed under [LICENSE](../LICENSE).