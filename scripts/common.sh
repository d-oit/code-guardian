#!/bin/bash
# Common utilities for Code Guardian scripts
# Provides shared functions for error handling, logging, and tool verification

set -euo pipefail

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly PURPLE='\033[0;35m'
readonly CYAN='\033[0;36m'
readonly NC='\033[0m' # No Color

# Log levels
readonly LOG_ERROR=0
readonly LOG_WARN=1
readonly LOG_INFO=2
readonly LOG_DEBUG=3

# Default log level
LOG_LEVEL=${LOG_LEVEL:-2}

# Global variables
DRY_RUN=false
VERBOSE=false
LOG_FILE=""

# Function to log messages with different levels
log() {
    local level=$1
    local message=$2
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local level_str=""
    local color=""
    
    case $level in
        $LOG_ERROR)
            level_str="ERROR"
            color="$RED"
            ;;
        $LOG_WARN)
            level_str="WARN"
            color="$YELLOW"
            ;;
        $LOG_INFO)
            level_str="INFO"
            color="$GREEN"
            ;;
        $LOG_DEBUG)
            level_str="DEBUG"
            color="$CYAN"
            ;;
        *)
            level_str="UNKNOWN"
            color="$NC"
            ;;
    esac
    
    # Only log if level is <= current LOG_LEVEL
    if [ $level -le $LOG_LEVEL ]; then
        echo -e "${color}[$timestamp] [$level_str]${NC} $message"
        
        # Also log to file if LOG_FILE is set
        if [ -n "$LOG_FILE" ]; then
            echo "[$timestamp] [$level_str] $message" >> "$LOG_FILE"
        fi
    fi
}

# Function to set log level
set_log_level() {
    case "${1:-info}" in
        "error") LOG_LEVEL=$LOG_ERROR ;;
        "warn") LOG_LEVEL=$LOG_WARN ;;
        "info") LOG_LEVEL=$LOG_INFO ;;
        "debug") LOG_LEVEL=$LOG_DEBUG ;;
        *) log $LOG_ERROR "Invalid log level: $1" ; exit 1 ;;
    esac
    log $LOG_INFO "Log level set to: $1"
}

# Function to enable/disable dry run mode
set_dry_run() {
    if [ "$1" = "true" ]; then
        DRY_RUN=true
        log $LOG_WARN "DRY RUN MODE ENABLED - No actual changes will be made"
    else
        DRY_RUN=false
    fi
}

# Function to enable/disable verbose mode
set_verbose() {
    if [ "$1" = "true" ]; then
        VERBOSE=true
        set_log_level "debug"
    else
        VERBOSE=false
        set_log_level "info"
    fi
}

# Function to set log file
set_log_file() {
    LOG_FILE="$1"
    mkdir -p "$(dirname "$LOG_FILE")"
    log $LOG_INFO "Log file set to: $LOG_FILE"
}

# Function to check if command exists
command_exists() {
    local cmd="$1"
    if command -v "$cmd" >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Function to verify required tools are installed
verify_tools() {
    local tools=("$@")
    local missing_tools=()
    
    for tool in "${tools[@]}"; do
        if ! command_exists "$tool"; then
            missing_tools+=("$tool")
        fi
    done
    
    if [ ${#missing_tools[@]} -ne 0 ]; then
        log $LOG_ERROR "Missing required tools: ${missing_tools[*]}"
        log $LOG_INFO "Please install the missing tools and try again"
        exit 1
    fi
    
    log $LOG_INFO "All required tools are available"
}

# Function to check if running in a git repository
is_git_repo() {
    if git rev-parse --git-dir > /dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Function to check if there are uncommitted changes
has_uncommitted_changes() {
    if ! git diff --quiet || ! git diff --staged --quiet; then
        return 0
    else
        return 1
    fi
}

# Function to prompt for confirmation
confirm() {
    local message="$1"
    local default="${2:-y}"
    
    if [ "$DRY_RUN" = "true" ]; then
        log $LOG_INFO "DRY RUN: Would prompt: $message"
        return 0
    fi
    
    local prompt="$message"
    if [ "$default" = "y" ]; then
        prompt="$prompt [Y/n] "
    else
        prompt="$prompt [y/N] "
    fi
    
    read -p "$prompt" -n 1 -r response
    echo
    
    case "$response" in
        [yY]|'') return 0 ;;
        [nN]) return 1 ;;
        *) confirm "$message" "$default" ;;
    esac
}

# Function to execute command with proper error handling
execute() {
    local cmd="$1"
    local description="${2:-$cmd}"
    
    log $LOG_INFO "Executing: $description"
    
    if [ "$VERBOSE" = "true" ]; then
        log $LOG_DEBUG "Command: $cmd"
    fi
    
    if [ "$DRY_RUN" = "true" ]; then
        log $LOG_WARN "DRY RUN: Would execute: $cmd"
        return 0
    fi
    
    if eval "$cmd"; then
        log $LOG_INFO "Successfully executed: $description"
        return 0
    else
        log $LOG_ERROR "Failed to execute: $description"
        return 1
    fi
}

# Function to execute command and capture output
execute_capture() {
    local cmd="$1"
    local description="${2:-$cmd}"
    
    log $LOG_INFO "Executing (capture): $description"
    
    if [ "$VERBOSE" = "true" ]; then
        log $LOG_DEBUG "Command: $cmd"
    fi
    
    if [ "$DRY_RUN" = "true" ]; then
        log $LOG_WARN "DRY RUN: Would execute: $cmd"
        return 0
    fi
    
    local output
    if output=$(eval "$cmd" 2>&1); then
        log $LOG_INFO "Successfully executed: $description"
        echo "$output"
        return 0
    else
        log $LOG_ERROR "Failed to execute: $description"
        echo "$output"
        return 1
    fi
}

# Function to measure execution time
measure_time() {
    local cmd="$1"
    local description="${2:-$cmd}"
    
    local start_time
    start_time=$(date +%s.%N)
    
    log $LOG_INFO "Measuring: $description"
    
    if execute "$cmd" "$description"; then
        local end_time
        end_time=$(date +%s.%N)
        local duration
        duration=$(echo "$end_time - $start_time" | bc -l)
        log $LOG_INFO "Completed in ${duration}s: $description"
        echo "$duration"
        return 0
    else
        log $LOG_ERROR "Failed measurement: $description"
        return 1
    fi
}

# Function to validate file exists
file_exists() {
    local file="$1"
    if [ -f "$file" ]; then
        return 0
    else
        log $LOG_ERROR "File not found: $file"
        return 1
    fi
}

# Function to validate directory exists
dir_exists() {
    local dir="$1"
    if [ -d "$dir" ]; then
        return 0
    else
        log $LOG_ERROR "Directory not found: $dir"
        return 1
    fi
}

# Function to create directory with parents
create_dir() {
    local dir="$1"
    
    if [ "$DRY_RUN" = "true" ]; then
        log $LOG_WARN "DRY RUN: Would create directory: $dir"
        return 0
    fi
    
    if mkdir -p "$dir"; then
        log $LOG_INFO "Created directory: $dir"
        return 0
    else
        log $LOG_ERROR "Failed to create directory: $dir"
        return 1
    fi
}

# Function to cleanup temporary files
cleanup() {
    local files=("$@")
    
    for file in "${files[@]}"; do
        if [ -e "$file" ]; then
            if [ "$DRY_RUN" = "true" ]; then
                log $LOG_WARN "DRY RUN: Would remove: $file"
            else
                rm -rf "$file"
                log $LOG_INFO "Removed: $file"
            fi
        fi
    done
}

# Function to show usage/help
show_usage() {
    local script_name="$(basename "$0")"
    local usage_text="$1"
    
    echo "Usage: $script_name [OPTIONS] [COMMAND]"
    echo ""
    echo "Options:"
    echo "  --dry-run, -d      Enable dry run mode (no actual changes)"
    echo "  --verbose, -v     Enable verbose output"
    echo "  --log-level LEVEL Set log level (error, warn, info, debug)"
    echo "  --log-file FILE   Log to specified file"
    echo "  --help, -h        Show this help message"
    echo ""
    echo -e "$usage_text"
}

# Global variable for remaining arguments
REMAINING_ARGS=()

# Function to parse common arguments
parse_common_args() {
    REMAINING_ARGS=()
    while [[ $# -gt 0 ]]; do
        case $1 in
            --dry-run|-d)
                set_dry_run "true"
                shift
                ;;
            --verbose|-v)
                set_verbose "true"
                shift
                ;;
            --log-level)
                set_log_level "$2"
                shift 2
                ;;
            --log-file)
                set_log_file "$2"
                shift 2
                ;;
            --help|-h)
                # Let the main script handle help
                REMAINING_ARGS+=("$1")
                shift
                ;;
            --*)
                log $LOG_ERROR "Unknown option: $1"
                show_usage ""
                exit 1
                ;;
            *)
                REMAINING_ARGS+=("$1")
                shift
                ;;
        esac
    done
}

# Function to validate Rust project structure
validate_rust_project() {
    log $LOG_INFO "Validating Rust project structure..."
    
    if ! file_exists "Cargo.toml"; then
        log $LOG_ERROR "Not a Rust project (Cargo.toml not found)"
        return 1
    fi
    
    local required_dirs=("src" "tests")
    for dir in "${required_dirs[@]}"; do
        if ! dir_exists "$dir"; then
            log $LOG_WARN "Missing directory: $dir"
        fi
    done
    
    log $LOG_INFO "Rust project structure validated"
    return 0
}

# Function to get Rust version information
get_rust_info() {
    local rust_version
    rust_version=$(rustc --version 2>/dev/null | cut -d' ' -f2 || echo "unknown")
    local cargo_version
    cargo_version=$(cargo --version 2>/dev/null | cut -d' ' -f2 || echo "unknown")
    
    echo "Rust: $rust_version, Cargo: $cargo_version"
}

# Function to check if we're in the project root
is_project_root() {
    if file_exists "Cargo.toml" && file_exists "scripts/common.sh"; then
        return 0
    else
        return 1
    fi
}

# Function to ensure we're in the project root
ensure_project_root() {
    if ! is_project_root; then
        log $LOG_ERROR "Must be run from the project root directory"
        exit 1
    fi
}

# Export functions for use in other scripts
export -f log set_log_level set_dry_run set_verbose set_log_file

export -f command_exists verify_tools is_git_repo has_uncommitted_changes confirm
export -f execute execute_capture measure_time file_exists dir_exists create_dir cleanup
export -f show_usage parse_common_args validate_rust_project get_rust_info is_project_root ensure_project_root

log $LOG_INFO "Common utilities loaded successfully"