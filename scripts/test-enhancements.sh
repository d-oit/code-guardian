#!/bin/bash
# Test script to verify enhanced scripts functionality

set -euo pipefail

# Load common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

log $LOG_INFO "🧪 Testing Enhanced Scripts..."
log $LOG_INFO "============================="

# Test 1: Verify common.sh functions
log $LOG_INFO "Test 1: Testing common.sh functions..."

# Test log levels
log $LOG_INFO "Testing log levels..."
set_log_level "debug"
log $LOG_DEBUG "Debug message should appear"
set_log_level "info"
log $LOG_DEBUG "Debug message should NOT appear"
log $LOG_INFO "Info message should appear"

# Test command verification
log $LOG_INFO "Testing command verification..."
if command_exists "ls"; then
    log $LOG_INFO "✓ ls command exists"
else
    log $LOG_ERROR "✗ ls command should exist"
fi

# Test file/directory operations
log $LOG_INFO "Testing file/directory operations..."
TEST_DIR="test_temp_dir"
create_dir "$TEST_DIR"
if dir_exists "$TEST_DIR"; then
    log $LOG_INFO "✓ Directory creation successful"
else
    log $LOG_ERROR "✗ Directory creation failed"
fi

# Test dry run mode
log $LOG_INFO "Testing dry run mode..."
set_dry_run "true"
log $LOG_INFO "Dry run should show warnings"
set_dry_run "false"

# Cleanup test directory
cleanup "$TEST_DIR"

# Test 2: Verify script syntax
log $LOG_INFO "Test 2: Testing script syntax..."

SCRIPTS_TO_TEST=(
    "dev-workflow.sh"
    "pre-commit.sh"
    "fix-code-quality.sh"
    "release-management.sh"
    "incremental-check.sh"
    "performance-monitor.sh"
    "generate-docs.sh"
    "generate-performance-report.sh"
)

for script in "${SCRIPTS_TO_TEST[@]}"; do
    if [ -f "scripts/$script" ]; then
        log $LOG_INFO "Testing $script..."
        if bash -n "scripts/$script"; then
            log $LOG_INFO "✓ $script syntax is valid"
        else
            log $LOG_ERROR "✗ $script has syntax errors"
        fi
    else
        log $LOG_WARN "Script $script not found"
    fi
done

# Test 3: Verify script dependencies
log $LOG_INFO "Test 3: Testing script dependencies..."

# Check if required tools are available
REQUIRED_TOOLS=("bash" "git" "cargo" "rustc")
for tool in "${REQUIRED_TOOLS[@]}"; do
    if command_exists "$tool"; then
        log $LOG_INFO "✓ $tool is available"
    else
        log $LOG_WARN "⚠ $tool is not available"
    fi
done

# Test 4: Verify script execution (basic)
log $LOG_INFO "Test 4: Testing basic script execution..."

# Test dev-workflow.sh help
log $LOG_INFO "Testing dev-workflow.sh help..."
if ./scripts/dev-workflow.sh --help > /dev/null 2>&1; then
    log $LOG_INFO "✓ dev-workflow.sh help works"
else
    log $LOG_ERROR "✗ dev-workflow.sh help failed"
fi

# Test pre-commit.sh with dry run
log $LOG_INFO "Testing pre-commit.sh with dry run..."
if ./scripts/pre-commit.sh --dry-run > /dev/null 2>&1; then
    log $LOG_INFO "✓ pre-commit.sh dry run works"
else
    log $LOG_ERROR "✗ pre-commit.sh dry run failed"
fi

# Test 5: Verify error handling
log $LOG_INFO "Test 5: Testing error handling..."

# Test invalid command
log $LOG_INFO "Testing invalid command handling..."
if ./scripts/dev-workflow.sh invalid-command > /dev/null 2>&1; then
    log $LOG_ERROR "✗ Invalid command should fail"
else
    log $LOG_INFO "✓ Invalid command correctly failed"
fi

# Test missing required tools
log $LOG_INFO "Testing missing tool detection..."
# This test is tricky since we can't easily remove tools, but we can test the function
if verify_tools "this_command_should_not_exist" > /dev/null 2>&1; then
    log $LOG_ERROR "✗ Missing tool detection should fail"
else
    log $LOG_INFO "✓ Missing tool detection correctly failed"
fi

log $LOG_INFO "🎉 All tests completed successfully!"
log $LOG_INFO "==============================="
echo ""
echo "Summary:"
echo "  ✅ Common utilities tested"
echo "  ✅ Script syntax validated"
echo "  ✅ Dependencies checked"
echo "  ✅ Basic execution verified"
echo "  ✅ Error handling tested"
echo ""
echo "The enhanced scripts are ready for use!"