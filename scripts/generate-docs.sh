#!/bin/bash
# Code-Guardian Documentation Generation Script
# Generates all documentation artifacts automatically with enhanced error handling

set -euo pipefail

# Load common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

log $LOG_INFO "📚 Generating Code-Guardian Documentation..."

# Check if we're in the right directory
ensure_project_root

# Create docs directory structure if it doesn't exist
create_dir "docs/api"
create_dir "docs/architecture/decisions"
create_dir "docs/configuration"
create_dir "docs/performance"
create_dir "docs/tutorials"
create_dir "docs/examples"
create_dir "docs/integrations"

# 1. Generate API Documentation from Code
log $LOG_INFO "Generating API documentation from Rust code..."
execute "cargo doc --no-deps --document-private-items --release" "Generate API documentation"

# Copy generated docs to docs/api/generated/
if dir_exists "target/doc"; then
    cleanup "docs/api/generated"
    execute "cp -r target/doc docs/api/generated" "Copy API documentation"
    log $LOG_INFO "API docs generated and copied to docs/api/generated/"
else
    log $LOG_WARN "API docs generation failed"
fi

# 2. Generate Configuration Schema from Code
log $LOG_INFO "Generating configuration schema documentation..."
if execute "cargo run --bin config-schema-generator > docs/configuration/schema-auto.md" "Generate config schema"; then
    log $LOG_INFO "Configuration schema generated"
else
    log $LOG_WARN "Config schema generator not available, using existing schema.md"
fi

# 3. Generate CLI Documentation
log $LOG_INFO "Generating CLI documentation..."
if execute "cargo run --bin cli-docs-generator > docs/api/cli-commands.md" "Generate CLI documentation"; then
    log $LOG_INFO "CLI documentation generated"
else
    log $LOG_WARN "CLI docs generator not available, using existing CLI docs"
fi

# 4. Run Performance Benchmarks
log $LOG_INFO "Running performance benchmarks..."
if execute "cargo bench --bench scanner_benchmark > docs/performance/benchmark-results.txt" "Run benchmarks"; then
    log $LOG_INFO "Benchmarks completed"
else
    log $LOG_WARN "Benchmarks failed or not available"
fi

# Generate performance report
if file_exists "docs/performance/benchmark-results.txt"; then
    log $LOG_INFO "Generating performance report..."
    execute "./scripts/generate-performance-report.sh" "Generate performance report"
fi

# 5. Update mdBook if available
if command_exists "mdbook"; then
    log $LOG_INFO "Building mdBook documentation..."
    execute "mdbook build docs/" "Build mdBook"
    log $LOG_INFO "mdBook built successfully"
else
    log $LOG_WARN "mdbook not installed, skipping mdBook build"
fi

# 6. Generate Changelog (if not using release-please)
if ! file_exists ".github/release-please-config.json"; then
    log $LOG_INFO "Generating changelog..."
    if command_exists "git-cliff"; then
        execute "git cliff --latest --all --prepend docs/CHANGELOG.md" "Generate changelog"
    else
        log $LOG_WARN "git-cliff not available or failed"
    fi
fi

# 7. Validate Documentation
log $LOG_INFO "Validating documentation..."

# Check for broken links in markdown files
if command_exists "markdown-link-check"; then
    log $LOG_INFO "Checking for broken links..."
    if execute "find docs/ -name \"*.md\" -exec markdown-link-check {} \;" "Check broken links"; then
        log $LOG_INFO "Link validation completed"
    else
        log $LOG_WARN "Some links may be broken"
    fi
else
    log $LOG_WARN "markdown-link-check not installed, skipping link validation"
fi

# Check for TODO/FIXME in docs
TODO_COUNT=$(grep -r "TODO\|FIXME" docs/ | wc -l)
if [ "$TODO_COUNT" -gt 0 ]; then
    log $LOG_WARN "Found $TODO_COUNT TODO/FIXME items in documentation"
fi

log $LOG_INFO "Documentation generation completed!"
echo ""
echo "Generated documentation:"
echo "  📖 API Docs: docs/api/generated/"
echo "  ⚙️  Config Schema: docs/configuration/schema-auto.md"
echo "  🖥️  CLI Docs: docs/api/cli-commands.md"
echo "  📊 Performance: docs/performance/"
echo "  📚 mdBook: docs/book/ (if mdbook installed)"

if dir_exists "docs/book"; then
    log $LOG_INFO "🌐 Open mdBook: docs/book/index.html"
fi