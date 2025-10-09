# Code Guardian Development Workflow Makefile
# This Makefile provides convenient shortcuts for common development tasks

.PHONY: help build test lint fmt check clean install dev-setup quality-fix coverage docs release

# Default target
help: ## Show this help message
	@echo "🚀 Code Guardian Development Workflow"
	@echo "===================================="
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Development setup
dev-setup: ## Install development dependencies and tools
	@echo "🔧 Setting up development environment..."
	cargo install cargo-watch cargo-audit cargo-tarpaulin cargo-llvm-cov
	@echo "✅ Development tools installed"

# Build commands
build: ## Build all crates in debug mode
	@echo "🔨 Building workspace..."
	cargo build

build-release: ## Build all crates in release mode
	@echo "🔨 Building workspace (release)..."
	cargo build --release

build-cli: ## Build only the CLI crate
	@echo "🔨 Building CLI crate..."
	cargo build -p cli

# Testing commands
test: ## Run all tests
	@echo "🧪 Running tests..."
	cargo test

test-watch: ## Run tests in watch mode
	@echo "👀 Watching tests..."
	cargo watch -x test

test-cli: ## Run CLI crate tests only
	@echo "🧪 Running CLI tests..."
	cargo test -p cli

test-core: ## Run core crate tests only
	@echo "🧪 Running core tests..."
	cargo test -p core

# Code quality commands
fmt: ## Format code with rustfmt
	@echo "🎨 Formatting code..."
	cargo fmt --all

fmt-check: ## Check code formatting without modifying files
	@echo "🔍 Checking code formatting..."
	cargo fmt --all -- --check

lint: ## Run clippy linter
	@echo "📎 Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

lint-fix: ## Run clippy with auto-fix
	@echo "🔧 Running clippy with auto-fix..."
	cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

# Quality control
check: ## Run all checks (format, lint, build, test)
	@echo "🔍 Running all checks..."
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo build
	cargo test

quality-fix: ## Auto-fix code quality issues (format + clippy)
	@echo "🔧 Auto-fixing code quality issues..."
	./scripts/fix-code-quality.sh

# Coverage
coverage: ## Generate test coverage report
	@echo "📊 Generating coverage report..."
	cargo llvm-cov --all-features --workspace --html --open

coverage-ci: ## Generate coverage report for CI (no browser)
	@echo "📊 Generating coverage report..."
	cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Security
audit: ## Run security audit on dependencies
	@echo "🔒 Running security audit..."
	cargo audit

# Documentation
docs: ## Generate and open documentation
	@echo "📚 Generating documentation..."
	cargo doc --open --no-deps

docs-watch: ## Generate docs in watch mode
	@echo "👀 Watching documentation..."
	cargo watch -x "doc --no-deps"

# Development workflow
dev: ## Start development mode (build and test on changes)
	@echo "🚀 Starting development mode..."
	cargo watch -x check -x test

# Release workflow
release: ## Create a new release (requires version argument)
	@echo "📦 Creating release..."
	@if [ -z "$(version)" ]; then \
		echo "❌ Error: Please specify version with 'make release version=X.Y.Z'"; \
		exit 1; \
	fi
	git tag -a v$(version) -m "Release v$(version)"
	git push origin v$(version)
	@echo "✅ Release v$(version) created"

# Cleaning
clean: ## Clean build artifacts
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

clean-all: ## Clean everything including caches
	@echo "🧹 Deep cleaning..."
	cargo clean
	rm -rf ~/.cargo/registry/cache ~/.cargo/git/checkouts

# Workspace management
update-deps: ## Update all dependencies
	@echo "📦 Updating dependencies..."
	cargo update

outdated: ## Check for outdated dependencies
	@echo "📦 Checking for outdated dependencies..."
	cargo outdated

# Quick development cycle
quick-check: fmt-check lint build test ## Run quick quality check (format, lint, build, test)

# CI simulation
ci-simulate: ## Simulate CI pipeline locally
	@echo "🔄 Simulating CI pipeline..."
	make fmt-check
	make lint
	make build
	make test
	make coverage-ci
	make audit

# Performance profiling
bench: ## Run benchmarks
	@echo "⚡ Running benchmarks..."
	cargo bench

bench-core: ## Run core crate benchmarks
	@echo "⚡ Running core benchmarks..."
	cargo bench -p core

# Agent development helpers
agents-update: ## Update agent documentation
	@echo "🤖 Updating agent documentation..."
	# This would trigger the opencode-agent-manager agent
	@echo "Run: opencode agent-manager update"

agents-validate: ## Validate agent configurations
	@echo "🔍 Validating agent configurations..."
	# This would trigger validation of all agent configs
	@echo "Run: opencode agent-manager validate"

# Docker helpers (if using Docker for development)
docker-build: ## Build Docker image
	@echo "🐳 Building Docker image..."
	docker build -t code-guardian .

docker-run: ## Run Docker container
	@echo "🐳 Running Docker container..."
	docker run --rm -it code-guardian

# Pre-commit hook setup
install-hooks: ## Install pre-commit hooks
	@echo "🔗 Installing pre-commit hooks..."
	@echo "#!/bin/bash" > .git/hooks/pre-commit
	@echo "make quick-check" >> .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "✅ Pre-commit hooks installed"

# Workspace info
info: ## Show workspace information
	@echo "📊 Workspace Information"
	@echo "======================="
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "Workspace members:"
	@grep -A 10 "\[workspace\]" Cargo.toml | grep "members" -A 10 | tail -n +2 | head -n 10
	@echo ""
	@echo "Available commands: make help"