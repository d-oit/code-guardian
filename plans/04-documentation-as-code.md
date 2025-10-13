# Documentation as Code

## 🎯 Objective
Create comprehensive, auto-generated documentation that accelerates adoption, reduces support overhead, and maintains itself through code integration.

## 🔍 Current Documentation State
- Basic README with setup instructions
- Tutorial files in `docs/tutorials/`
- Example configurations in `examples/`
- Missing: Auto-generated API docs, architectural documentation, integration guides

## 📋 Action Plan

### Phase 1: API Documentation Automation (3-4 hours)
1. **Enhanced Rust Documentation**
   ```rust
   /// Scans a directory for code patterns and security issues.
   /// 
   /// # Arguments
   /// 
   /// * `path` - The directory path to scan
   /// * `config` - Scanner configuration options
   /// 
   /// # Returns
   /// 
   /// A `Result` containing scan results or an error
   /// 
   /// # Examples
   /// 
   /// ```
   /// use code_guardian_core::Scanner;
   /// 
   /// let scanner = Scanner::new()?;
   /// let results = scanner.scan_directory("./src", &config)?;
   /// ```
   /// 
   /// # Errors
   /// 
   /// This function will return an error if:
   /// - The directory doesn't exist
   /// - Permission denied to read files
   /// - Invalid configuration provided
   pub fn scan_directory(path: &Path, config: &ScanConfig) -> Result<ScanResults> {
       // Implementation
   }
   ```

2. **Documentation Generation Pipeline**
   ```bash
   # Automated doc generation
   cargo doc --all-features --no-deps --document-private-items
   
   # Include examples and tests
   cargo test --doc
   
   # Generate coverage for docs
   cargo deadlinks
   ```

3. **Documentation Site Setup**
   ```yaml
   # .github/workflows/docs.yml enhancement
   - name: Generate API Documentation
     run: |
       cargo doc --all-features --no-deps
       mdbook build docs/
       cargo run --bin doc-generator
   ```

### Phase 2: Architectural Documentation (4-5 hours)
1. **System Architecture Diagrams**
   ```mermaid
   # docs/architecture/system-overview.md
   graph TD
       A[CLI Interface] --> B[Core Scanner]
       B --> C[Detector Factory]
       B --> D[Configuration Manager]
       C --> E[Built-in Detectors]
       C --> F[Custom Detectors]
       B --> G[Storage Layer]
       B --> H[Output Formatters]
       G --> I[SQLite Database]
       H --> J[JSON/CSV/HTML/Markdown]
   ```

2. **Decision Records (ADRs)**
   ```markdown
   # docs/architecture/decisions/001-modular-crate-structure.md
   
   # ADR-001: Modular Crate Structure
   
   ## Status
   Accepted
   
   ## Context
   Need to organize code for maintainability and independent development.
   
   ## Decision
   Split functionality into focused crates: core, cli, storage, output.
   
   ## Consequences
   + Better separation of concerns
   + Independent versioning possible
   + Easier testing and maintenance
   - Increased complexity in dependency management
   ```

3. **Integration Patterns**
   ```markdown
   # docs/integration/patterns.md
   
   ## Common Integration Patterns
   
   ### CI/CD Integration
   ```yaml
   # .github/workflows/security-scan.yml
   - name: Security Scan
     run: |
       code-guardian scan --format json --output security-report.json
       code-guardian report --threshold high
   ```
   
   ### Git Hook Integration
   ```bash
   #!/bin/bash
   # .git/hooks/pre-commit
   code-guardian scan --changed-only --fail-on-issues
   ```
   ```

### Phase 3: Interactive Documentation (5-6 hours)
1. **Code Examples with Testing**
   ```rust
   // examples/comprehensive_usage.rs
   //! # Code Guardian Comprehensive Usage Examples
   //! 
   //! This example demonstrates all major features of Code Guardian
   //! in realistic scenarios.
   
   use code_guardian_core::*;
   use code_guardian_output::formatters::*;
   
   fn main() -> Result<()> {
       // Example 1: Basic scanning
       basic_scan_example()?;
       
       // Example 2: Custom configuration
       custom_config_example()?;
       
       // Example 3: Multiple output formats
       multi_format_output_example()?;
       
       Ok(())
   }
   
   /// Demonstrates basic directory scanning
   fn basic_scan_example() -> Result<()> {
       let scanner = Scanner::default();
       let results = scanner.scan_directory("./examples/sample_code")?;
       
       println!("Found {} issues", results.issues.len());
       Ok(())
   }
   ```

2. **Interactive Tutorials**
   ```markdown
   # docs/tutorials/interactive/getting-started.md
   
   # Getting Started - Interactive Tutorial
   
   Follow along with this tutorial using the provided examples.
   
   ## Step 1: Basic Scan
   
   Run this command in your terminal:
   ```bash
   cd examples/sample_project
   code-guardian scan --verbose
   ```
   
   **Expected Output:**
   ```
   🔍 Scanning 15 files...
   ⚠️  Found 3 potential issues
   📊 Scan completed in 1.2s
   ```
   
   ## Step 2: Configuration
   
   Create a custom configuration:
   ```toml
   # code-guardian.toml
   [scanning]
   parallel = true
   max_file_size = "10MB"
   
   [detectors]
   security = true
   performance = false
   ```
   ```

3. **Live Documentation Examples**
   ```bash
   # scripts/generate-live-examples.sh
   #!/bin/bash
   
   # Generate examples from actual codebase
   echo "# Live Examples from Code Guardian" > docs/examples/live.md
   
   # Scan our own codebase for examples
   code-guardian scan --format markdown >> docs/examples/live.md
   
   # Generate performance benchmarks
   cargo bench 2>&1 | grep -A 5 "Performance" >> docs/performance/latest.md
   ```

### Phase 4: Documentation Automation (3-4 hours)
1. **Auto-generated CLI Documentation**
   ```rust
   // src/bin/doc-generator.rs
   use clap::CommandFactory;
   use code_guardian_cli::Cli;
   
   fn main() {
       let cmd = Cli::command();
       
       // Generate markdown documentation
       let md = clap_markdown::help_markdown::<Cli>();
       std::fs::write("docs/cli/commands.md", md).unwrap();
       
       // Generate man pages
       let man = clap_mangen::Man::new(cmd);
       let mut buffer = Vec::new();
       man.render(&mut buffer).unwrap();
       std::fs::write("docs/man/code-guardian.1", buffer).unwrap();
   }
   ```

2. **Configuration Documentation**
   ```rust
   // Auto-generate config docs from schema
   use serde_json::schema::RootSchema;
   
   fn generate_config_docs() {
       let schema = schema_for!(ScanConfig);
       let markdown = json_schema_to_markdown(&schema);
       std::fs::write("docs/configuration/schema.md", markdown).unwrap();
   }
   ```

3. **Changelog Automation**
   ```bash
   # scripts/update-changelog.sh
   #!/bin/bash
   
   # Generate changelog from git commits
   git-cliff --config cliff.toml --output CHANGELOG.md
   
   # Update version documentation
   cargo pkgid | cut -d# -f2 > docs/VERSION
   ```

## 📊 Documentation Structure
```
docs/
├── README.md                    # Main documentation entry
├── api/                        # Auto-generated API docs
│   ├── core/
│   ├── cli/
│   ├── storage/
│   └── output/
├── architecture/               # System design
│   ├── overview.md
│   ├── decisions/              # ADRs
│   └── diagrams/
├── tutorials/                  # Step-by-step guides
│   ├── getting-started.md
│   ├── advanced-usage.md
│   ├── custom-detectors.md
│   └── integration/
├── examples/                   # Code examples
│   ├── basic/
│   ├── advanced/
│   └── integration/
├── configuration/              # Config documentation
│   ├── schema.md
│   ├── examples/
│   └── migration-guides/
├── performance/                # Benchmarks and optimization
│   ├── latest.md
│   ├── historical/
│   └── optimization-guide.md
└── contributing/               # Development docs
    ├── setup.md
    ├── workflow.md
    └── release-process.md
```

## 🔧 Documentation Tools
```toml
# Additional dependencies for doc generation
[dev-dependencies]
clap_markdown = "0.1"
clap_mangen = "0.2"
schemars = "0.8"
mdbook = "0.4"
git-cliff = "1.0"
```

## 📈 Success Metrics
- [ ] 100% public API documented with examples
- [ ] <5 minutes from code change to updated docs
- [ ] Self-service adoption rate >80%
- [ ] Support ticket reduction of 50%
- [ ] Developer onboarding time <30 minutes

## 🔄 Automation Workflows
```yaml
# .github/workflows/docs-auto-update.yml
name: Auto-update Documentation

on:
  push:
    paths: ['src/**', 'examples/**', 'docs/**']

jobs:
  update-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Generate API docs
        run: cargo doc --all-features
      - name: Generate CLI docs
        run: cargo run --bin doc-generator
      - name: Build documentation site
        run: mdbook build
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
```

## 🚨 Quality Gates
- **Documentation coverage**: >90% of public APIs
- **Example testing**: All code examples must pass tests
- **Link checking**: No broken internal/external links
- **Freshness**: Auto-generated content updated on every build

## 📈 Expected Impact
- **High**: Faster user onboarding (60% reduction in time-to-first-success)
- **High**: Reduced support overhead (50% fewer basic questions)
- **Medium**: Increased adoption through better discoverability
- **Medium**: Improved code quality through documentation-driven development

## 🔄 Maintenance Strategy
1. **Automated updates** on every code change
2. **Quarterly documentation reviews** for accuracy
3. **User feedback integration** from support tickets
4. **Analytics tracking** for documentation usage

## 📝 Deliverables
- [~] Auto-generated API documentation (70% complete)  
  *Completed items*: `cargo doc` generation via CI/CD, deployment to GitHub Pages, well-documented public APIs in core crates with examples.  
  *Remaining*: mdbook site setup, doctest inclusion, deadlinks checking, full docs/api/ structure.
- [~] Comprehensive tutorial suite (75% complete)  
  *Completed items*: `docs/tutorials/` with 4 guides (getting-started, advanced-usage, custom-detectors, automation).  
  *Remaining*: Interactive tutorials, `docs/tutorials/integration/` subdirectory.
- [ ] Architectural decision records (0% complete)  
  *Remaining*: Create `docs/architecture/decisions/` with ADRs (e.g., modular crate structure).
- [x] Live examples and demos (100% complete)  
  *Completed items*: `examples/` directory with multiple demos, configs, and README.
- [ ] Configuration schema documentation (0% complete)  
  *Remaining*: Generate `docs/configuration/schema.md` from config structs.
- [ ] Performance benchmarking reports (0% complete)  
  *Remaining*: Create `docs/performance/` with latest.md and historical data.
- [x] Contributing guidelines (100% complete)  
  *Completed items*: `CONTRIBUTING.md` in root with setup, workflow, and release guidelines.
- [~] Documentation automation pipeline (40% complete)  
  *Completed items*: `docs.yml` workflow for generation and deployment.  
  *Remaining*: mdbook integration, `doc-generator` binary, changelog automation, config schema generation.

This assessment indicates approximately 48% overall completion of the documentation-as-code plan, with strong foundations in API docs, tutorials, examples, and contributing guidelines, but gaps in architectural docs, configuration schemas, performance reports, and advanced automation. Next steps could prioritize Phase 2 (architectural docs) and Phase 4 (automation enhancements) for the most impact.

## 🤖 LLM-Assisted Documentation

### Documentation Added: llm_detection_demo.md

The `examples/llm_detection_demo.md` provides a comprehensive guide for using LLM-specific detectors, including:

- CLI command examples for LLM security and quality scans
- Detailed descriptions of 18 LLM-specific detection patterns
- Configuration examples for integrating LLM detection into workflows
- CI/CD integration examples
- Severity guidelines and best practices

This documentation covers critical security issues like SQL injection, hardcoded credentials, and XSS, as well as quality issues like async anti-patterns and performance problems commonly found in LLM-generated code.

### Automated Doc Generation

Building on the existing documentation automation pipeline, LLM-assisted documentation leverages:

- **Pattern-based documentation**: Auto-generation of detector descriptions from code patterns
- **Example extraction**: Automated creation of code examples from test cases
- **Consistency checks**: Integration of LLM detectors to verify documentation accuracy

The pipeline includes:
```bash
# Generate LLM detector documentation
cargo run --bin doc-generator -- --llm-detectors

# Validate documentation consistency
code-guardian scan --profile llm-comprehensive docs/
```

### Ensuring Code-Doc Consistency with LLM Detection

LLM detection plays a crucial role in maintaining documentation quality by:

1. **Detecting Hallucinated Documentation**: Identifies references to non-existent APIs or functions in docs
2. **Comment Validation**: Flags AI-generated comments that may be inaccurate or misleading
3. **Example Verification**: Ensures code examples in documentation are syntactically correct and follow best practices
4. **Configuration Consistency**: Validates that documented configurations match actual implementation

By integrating LLM detectors into the documentation pipeline, we ensure that:
- Documentation examples are free from common LLM vulnerabilities
- API references are accurate and up-to-date
- Code samples demonstrate secure and efficient patterns
- Configuration documentation reflects actual behavior

This creates a self-reinforcing cycle where high-quality documentation serves as both learning material and validation for the codebase itself.