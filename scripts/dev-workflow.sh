#!/bin/bash
# Development Workflow Script for Code Guardian
# This script provides a streamlined development experience

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Default log file
DEFAULT_LOG_FILE="logs/dev-workflow.log"

# Function to print colored output
print_status() {
    local status=$1
    local message=$2
    case $status in
        "success")
            echo -e "${GREEN}✅ $message${NC}"
            ;;
        "warning")
            echo -e "${YELLOW}⚠️  $message${NC}"
            ;;
        "error")
            echo -e "${RED}❌ $message${NC}"
            ;;
        "info")
            echo -e "${BLUE}ℹ️  $message${NC}"
            ;;
        "action")
            echo -e "${PURPLE}🚀 $message${NC}"
            ;;
        "dev")
            echo -e "${CYAN}🔧 $message${NC}"
            ;;
        *)
            echo "$message"
            ;;
    esac
}

# Function to show usage
show_usage() {
    echo "🚀 Code Guardian Development Workflow"
    echo "===================================="
    echo ""
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  setup          Set up development environment"
    echo "  check          Run all quality checks (format, lint, build, test)"
    echo "  fix            Auto-fix code quality issues"
    echo "  test           Run tests with options"
    echo "  build          Build project with options"
    echo "  watch          Start watch mode for development"
    echo "  coverage       Generate coverage report"
    echo "  release        Prepare for release"
    echo "  clean          Clean build artifacts"
    echo "  info           Show project information"
    echo ""
    echo "Test Options:"
    echo "  --unit         Run only unit tests"
    echo "  --integration  Run only integration tests"
    echo "  --bench        Run benchmarks"
    echo "  --watch        Run tests in watch mode"
    echo ""
    echo "Build Options:"
    echo "  --release      Build in release mode"
    echo "  --cli          Build only CLI crate"
    echo "  --core         Build only core crate"
    echo ""
    echo "Examples:"
    echo "  $0 setup                    # Set up development environment"
    echo "  $0 check                    # Run all quality checks"
    echo "  $0 test --watch             # Run tests in watch mode"
    echo "  $0 build --release          # Build in release mode"
    echo "  $0 watch                    # Start development watch mode"
}

# Function to setup development environment
setup_dev() {
    print_status action "Setting up development environment..."

    # Check if Rust is installed
    if ! command -v rustc &> /dev/null; then
        print_status error "Rust is not installed. Please install Rust first:"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi

    print_status info "Installing development tools..."
    cargo install cargo-watch cargo-audit cargo-tarpaulin cargo-llvm-cov cargo-outdated

    # Install pre-commit hook
    if [ ! -f .git/hooks/pre-commit ]; then
        print_status info "Installing pre-commit hook..."
        cp scripts/pre-commit.sh .git/hooks/pre-commit
        chmod +x .git/hooks/pre-commit
        print_status success "Pre-commit hook installed"
    fi

    print_status success "Development environment setup complete!"
    echo ""
    echo "Next steps:"
    echo "  1. Run '$0 check' to verify everything works"
    echo "  2. Run '$0 watch' to start development mode"
    echo "  3. Use 'make help' for more commands"
}

# Function to run quality checks
run_checks() {
    print_status action "Running quality checks..."

    # Format check
    print_status info "Checking code formatting..."
    if cargo fmt --all -- --check > /dev/null 2>&1; then
        print_status success "Code formatting is correct"
    else
        print_status error "Code formatting issues found. Run '$0 fix' to auto-fix."
        exit 1
    fi

    # Clippy
    print_status info "Running clippy..."
    if cargo clippy --all-targets --all-features -- -D warnings > /dev/null 2>&1; then
        print_status success "Clippy checks passed"
    else
        print_status error "Clippy issues found. Run '$0 fix' to auto-fix."
        exit 1
    fi

    # Build
    print_status info "Building project..."
    if cargo build > /dev/null 2>&1; then
        print_status success "Build successful"
    else
        print_status error "Build failed"
        exit 1
    fi

    # Tests
    print_status info "Running tests..."
    if cargo test > /dev/null 2>&1; then
        print_status success "Tests passed"
    else
        print_status error "Tests failed"
        exit 1
    fi

    print_status success "All quality checks passed! 🎉"
}

# Function to auto-fix issues
auto_fix() {
    print_status action "Auto-fixing code quality issues..."
    ./scripts/fix-code-quality.sh
}

# Function to run tests
run_tests() {
    local test_type=""
    local watch_mode=false

    while [[ $# -gt 0 ]]; do
        case $1 in
            --unit)
                test_type="--lib"
                shift
                ;;
            --integration)
                test_type="--test '*' --lib"
                shift
                ;;
            --bench)
                print_status action "Running benchmarks..."
                cargo bench
                return
                ;;
            --watch)
                watch_mode=true
                shift
                ;;
            *)
                print_status error "Unknown test option: $1"
                exit 1
                ;;
        esac
    done

    if [ "$watch_mode" = true ]; then
        print_status action "Running tests in watch mode..."
        cargo watch -x "test $test_type"
    else
        print_status action "Running tests..."
        cargo test $test_type
    fi
}

# Function to build project
build_project() {
    local build_mode=""
    local crate_target=""

    while [[ $# -gt 0 ]]; do
        case $1 in
            --release)
                build_mode="--release"
                shift
                ;;
            --cli)
                crate_target="-p cli"
                shift
                ;;
            --core)
                crate_target="-p core"
                shift
                ;;
            *)
                print_status error "Unknown build option: $1"
                exit 1
                ;;
        esac
    done

    print_status action "Building project$build_mode..."
    cargo build $build_mode $crate_target
    print_status success "Build complete"
}

# Function to start watch mode
start_watch() {
    print_status action "Starting development watch mode..."
    print_status info "Press Ctrl+C to stop watching"
    echo ""
    cargo watch -x check -x test
}

# Function to generate coverage
generate_coverage() {
    print_status action "Generating test coverage report..."
    cargo llvm-cov --all-features --workspace --html --open
}

# Function to prepare release
prepare_release() {
    print_status action "Preparing for release..."

    # Run all checks
    run_checks

    # Generate coverage report
    print_status info "Generating coverage report..."
    cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

    # Run security audit
    print_status info "Running security audit..."
    if command -v cargo-audit &> /dev/null; then
        cargo audit
    else
        print_status warning "cargo-audit not installed, skipping security audit"
    fi

    # Build release
    print_status info "Building release version..."
    cargo build --release

    print_status success "Release preparation complete!"
    echo ""
    echo "Release artifacts:"
    echo "  - Binary: target/release/cli"
    echo "  - Coverage: lcov.info"
    echo ""
    echo "To create a git tag: git tag -a v1.0.0 -m 'Release v1.0.0'"
}

# Function to clean project
clean_project() {
    print_status action "Cleaning build artifacts..."
    cargo clean
    print_status success "Clean complete"
}

# Function to show project info
show_info() {
    echo "🚀 Code Guardian - Project Information"
    echo "====================================="
    echo ""
    echo "Rust Version: $(rustc --version)"
    echo "Cargo Version: $(cargo --version)"
    echo ""
    echo "Workspace Members:"
    grep -A 10 "\[workspace\]" Cargo.toml | grep "members" -A 10 | tail -n +2 | head -n 10 | sed 's/.*"\(.*\)".*/  - \1/'
    echo ""
    echo "Available Scripts:"
    ls scripts/ | sed 's/^/  - /'
    echo ""
    echo "Available Make Commands:"
    make help | grep -E "^  [a-zA-Z_-]+.*" | head -10
}

# Main script logic
mkdir -p logs
exec > "$DEFAULT_LOG_FILE" 2>&1
case "${1:-help}" in
    "setup")
        setup_dev
        ;;
    "check")
        run_checks
        ;;
    "fix")
        auto_fix
        ;;
    "test")
        shift
        run_tests "$@"
        ;;
    "build")
        shift
        build_project "$@"
        ;;
    "watch")
        start_watch
        ;;
    "coverage")
        generate_coverage
        ;;
    "release")
        prepare_release
        ;;
    "clean")
        clean_project
        ;;
    "info")
        show_info
        ;;
    "help"|"-h"|"--help")
        show_usage
        ;;
    *)
        print_status error "Unknown command: $1"
        echo ""
        show_usage
        exit 1
        ;;
esac