#!/bin/bash
# Release Management Script for Code Guardian
# This script helps manage releases with enhanced error handling and modularity

set -euo pipefail

# Load common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Exit codes
readonly EXIT_SUCCESS=0
readonly EXIT_GENERAL_ERROR=1
readonly EXIT_MISSING_TOOLS=2
readonly EXIT_INVALID_ARGS=3
readonly EXIT_AUTH_ERROR=4
readonly EXIT_GIT_ERROR=5

# Function to display help
show_help() {
    cat << EOF
Release Management Script for Code Guardian

Usage: $0 [OPTIONS] [COMMAND] [ARGS...]

Commands:
    help                    Show this help message
    list                    List all releases
    status                  Show release status and next version
    enhance <TAG>           Enhance a specific release description
    sync-changelog          Sync changelog with all releases
    test-workflows          Test release workflows
    create-manual <VERSION> Create a manual release
    validate <TAG>          Validate release format

Options:
    --dry-run, -d           Enable dry run mode (no actual changes)
    --verbose, -v           Enable verbose output
    --log-level <LEVEL>     Set log level (error, warn, info, debug)
    --log-file <FILE>       Log to specified file
    --help, -h              Show this help message

Examples:
    $0 list
    $0 --dry-run enhance v0.1.5
    $0 --verbose sync-changelog
    $0 create-manual v0.1.7
    $0 validate v0.1.6

EOF
}

# Enhanced pre-flight checks for required tools
check_prerequisites() {
    log $LOG_DEBUG "Performing pre-flight checks..."

    # Verify required tools
    verify_tools "gh" "jq" "git" || {
        log $LOG_ERROR "Required tools are missing. Please install them and try again."
        exit $EXIT_MISSING_TOOLS
    }

    # Check GitHub CLI authentication
    if ! gh auth status &> /dev/null; then
        log $LOG_ERROR "GitHub CLI is not authenticated. Please run 'gh auth login' first."
        exit $EXIT_AUTH_ERROR
    fi

    # Check GitHub CLI version (require minimum version)
    local gh_version
    gh_version=$(gh --version | head -1 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' || echo "0.0.0")
    log $LOG_DEBUG "GitHub CLI version: $gh_version"

    # Ensure we're in a git repository
    if ! is_git_repo; then
        log $LOG_ERROR "Not in a git repository. Please run from the project root."
        exit $EXIT_GIT_ERROR
    fi

    # Ensure we're in the project root
    ensure_project_root

    log $LOG_INFO "All pre-flight checks passed."
}

# List all releases
list_releases() {
    log $LOG_INFO "Listing all releases..."
    if ! execute "gh release list --limit 20" "List releases"; then
        log $LOG_ERROR "Failed to list releases. Check your GitHub CLI authentication and repository access."
        return $EXIT_GENERAL_ERROR
    fi
}

# Get latest release tag
get_latest_release() {
    local latest_release
    if ! latest_release=$(execute_capture "gh release list --limit 1 --json tagName --jq '.[0].tagName'" "Get latest release"); then
        log $LOG_ERROR "Failed to get latest release information."
        return $EXIT_GENERAL_ERROR
    fi
    echo "$latest_release"
}

# Get current version from manifest
get_current_version() {
    local current_version=""
    if file_exists ".github/.release-please-manifest.json"; then
        if ! current_version=$(execute_capture "jq -r '.\"\"' .github/.release-please-manifest.json" "Get current version from manifest"); then
            log $LOG_WARN "Failed to parse release manifest."
            return 1
        fi
        log $LOG_INFO "Current version in manifest: v$current_version"
    else
        log $LOG_WARN "Release manifest not found at .github/.release-please-manifest.json"
    fi
    echo "$current_version"
}

# Check for unreleased conventional commits
check_unreleased_commits() {
    local latest_release="$1"
    log $LOG_DEBUG "Checking for unreleased commits since $latest_release..."

    local unreleased_commits
    if ! unreleased_commits=$(execute_capture "git log --oneline \"${latest_release}..HEAD\" 2>/dev/null | grep -E '^(feat|fix|perf|docs|style|refactor|test|chore)' || true" "Check for conventional commits"); then
        log $LOG_WARN "Failed to check for unreleased commits."
        return 1
    fi

    if [[ -n "$unreleased_commits" ]]; then
        log $LOG_WARN "There are unreleased conventional commits since $latest_release"
        log $LOG_INFO "Recent unreleased commits:"
        echo "$unreleased_commits" | head -5
    else
        log $LOG_INFO "No unreleased conventional commits found since last release"
    fi
}

# Show release status
show_status() {
    log $LOG_INFO "Current release status..."

    local latest_release
    if ! latest_release=$(get_latest_release); then
        return $EXIT_GENERAL_ERROR
    fi
    log $LOG_INFO "Latest release: $latest_release"

    local current_version
    current_version=$(get_current_version)

    check_unreleased_commits "$latest_release"
}

# Validate tag format
validate_tag() {
    local tag="$1"
    if [[ ! "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$ ]]; then
        log $LOG_ERROR "Invalid tag format: $tag. Expected format: v1.0.0 or v1.0.0-alpha"
        return 1
    fi
    return 0
}

# Check if release exists
release_exists() {
    local tag="$1"
    if ! execute "gh release view \"$tag\" --json id > /dev/null 2>&1" "Check if release $tag exists"; then
        return 1
    fi
    return 0
}

# Enhance a specific release
enhance_release() {
    local tag="$1"
    if [[ -z "$tag" ]]; then
        log $LOG_ERROR "Please specify a tag to enhance (e.g., v0.1.5)"
        return $EXIT_INVALID_ARGS
    fi

    log $LOG_INFO "Enhancing release description for $tag..."

    # Validate tag format
    if ! validate_tag "$tag"; then
        return $EXIT_INVALID_ARGS
    fi

    # Check if release exists
    if ! release_exists "$tag"; then
        log $LOG_ERROR "Release $tag does not exist or is not accessible"
        return $EXIT_GENERAL_ERROR
    fi

    # Trigger enhanced release workflow
    log $LOG_INFO "Triggering enhanced release workflow..."
    if ! execute "gh workflow run enhanced-release.yml -f tag=\"$tag\"" "Trigger enhanced release workflow"; then
        log $LOG_ERROR "Failed to trigger enhanced release workflow"
        return $EXIT_GENERAL_ERROR
    fi

    log $LOG_INFO "Enhanced release workflow triggered for $tag"
    log $LOG_INFO "You can monitor the workflow at: https://github.com/d-oit/code-guardian/actions"
}

# Sync changelog with releases
sync_changelog() {
    log $LOG_INFO "Syncing changelog with all releases..."

    # Trigger changelog sync workflow
    if ! execute "gh workflow run changelog-sync.yml -f sync_all=true" "Trigger changelog sync workflow"; then
        log $LOG_ERROR "Failed to trigger changelog sync workflow"
        return $EXIT_GENERAL_ERROR
    fi

    log $LOG_INFO "Changelog sync workflow triggered"
    log $LOG_INFO "You can monitor the workflow at: https://github.com/d-oit/code-guardian/actions"
}

# Check if required files exist
check_required_files() {
    local -r files=("$@")
    local missing_files=()

    for file in "${files[@]}"; do
        if file_exists "$file"; then
            log $LOG_INFO "✓ $file exists"
        else
            log $LOG_ERROR "✗ $file is missing"
            missing_files+=("$file")
        fi
    done

    if [[ ${#missing_files[@]} -gt 0 ]]; then
        log $LOG_ERROR "Missing required files: ${missing_files[*]}"
        return 1
    fi
    return 0
}

# Validate JSON file
validate_json_file() {
    local file="$1"
    local description="$2"

    if ! file_exists "$file"; then
        log $LOG_WARN "Skipping validation: $file does not exist"
        return 1
    fi

    if execute "jq empty \"$file\"" "Validate $description"; then
        log $LOG_INFO "✓ $description is valid JSON"
        return 0
    else
        log $LOG_ERROR "✗ $description is invalid JSON"
        return 1
    fi
}

# Test workflows and configurations
test_workflows() {
    log $LOG_INFO "Testing release workflows and configurations..."

    # Define required workflows
    local -r workflows=(
        ".github/workflows/enhanced-release.yml"
        ".github/workflows/changelog-sync.yml"
        ".github/workflows/release-please.yml"
    )

    # Define required config files
    local -r configs=(
        ".github/release-please-config.json"
        ".github/.release-please-manifest.json"
        ".github/RELEASE_TEMPLATE.md"
    )

    # Check workflows
    if ! check_required_files "${workflows[@]}"; then
        log $LOG_WARN "Some workflow files are missing"
    fi

    # Check configs
    if ! check_required_files "${configs[@]}"; then
        log $LOG_WARN "Some configuration files are missing"
    fi

    # Validate JSON files
    local json_validation_failed=0
    if ! validate_json_file ".github/release-please-config.json" "release-please-config.json"; then
        ((json_validation_failed++))
    fi

    if ! validate_json_file ".github/.release-please-manifest.json" ".release-please-manifest.json"; then
        ((json_validation_failed++))
    fi

    if [[ $json_validation_failed -gt 0 ]]; then
        log $LOG_ERROR "JSON validation failed for $json_validation_failed file(s)"
        return $EXIT_GENERAL_ERROR
    fi

    log $LOG_INFO "Workflow and configuration test completed successfully"
}

# Normalize version string
normalize_version() {
    local version="$1"
    # Remove 'v' prefix if present
    local clean_version="${version#v}"

    # Validate version format
    if [[ ! "$clean_version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$ ]]; then
        log $LOG_ERROR "Invalid version format: $version. Use semantic versioning (e.g., 1.0.0 or 1.0.0-alpha)"
        return 1
    fi

    # Add 'v' prefix if not present
    if [[ "$version" != v* ]]; then
        version="v$version"
    fi

    echo "$version"
}

# Check if tag already exists
tag_exists() {
    local tag="$1"
    if execute_capture "git tag -l | grep -q \"^$tag$\"" "Check if tag $tag exists" >/dev/null 2>&1; then
        return 0
    fi
    return 1
}

# Create manual release
create_manual_release() {
    local version="$1"
    if [[ -z "$version" ]]; then
        log $LOG_ERROR "Please specify a version (e.g., v0.1.7)"
        return $EXIT_INVALID_ARGS
    fi

    log $LOG_INFO "Creating manual release $version..."

    # Normalize and validate version
    version=$(normalize_version "$version") || return $EXIT_INVALID_ARGS

    # Check if tag already exists
    if tag_exists "$version"; then
        log $LOG_ERROR "Tag $version already exists"
        return $EXIT_GENERAL_ERROR
    fi

     # Check for uncommitted changes
     if has_uncommitted_changes; then
         log $LOG_WARN "There are uncommitted changes in the working directory"
         if ! confirm "Continue with manual release creation?"; then
             log $LOG_INFO "Manual release creation cancelled"
             return $EXIT_SUCCESS
         fi
     fi

     # Ensure commits are atomic (conventional) before release
     log $LOG_INFO "Checking for atomic commits (conventional commits) since last release..."
     local latest_release
     if latest_release=$(get_latest_release 2>/dev/null); then
         local unreleased_commits
         unreleased_commits=$(git log --oneline "${latest_release}..HEAD" 2>/dev/null | grep -E '^(feat|fix|perf|docs|style|refactor|test|chore)' || true)
         if [[ -z "$unreleased_commits" ]]; then
             log $LOG_WARN "No conventional commits found since last release $latest_release."
             log $LOG_WARN "To ensure atomic commits, consider using the atomic-commit-creator agent to restructure commits."
             if ! confirm "Continue with manual release creation?"; then
                 log $LOG_INFO "Manual release creation cancelled"
                 return $EXIT_SUCCESS
             fi
         else
             log $LOG_INFO "Found conventional commits since last release"
         fi
     else
         log $LOG_WARN "Could not determine latest release for atomic commit check"
     fi

    # Create tag
    log $LOG_INFO "Creating tag $version..."
    if ! execute "git tag -a \"$version\" -m \"Release $version\"" "Create git tag"; then
        log $LOG_ERROR "Failed to create git tag"
        return $EXIT_GIT_ERROR
    fi

    if ! execute "git push origin \"$version\"" "Push tag to remote"; then
        log $LOG_ERROR "Failed to push tag to remote. You may need to push manually."
        return $EXIT_GIT_ERROR
    fi

    log $LOG_INFO "Tag $version created and pushed successfully"
    log $LOG_INFO "The enhanced release workflow should be triggered automatically"
    log $LOG_INFO "Monitor at: https://github.com/d-oit/code-guardian/actions"
}

# Get release body
get_release_body() {
    local tag="$1"
    local release_body
    if ! release_body=$(execute_capture "gh release view \"$tag\" --json body --jq .body" "Get release body for $tag"); then
        log $LOG_ERROR "Failed to get release body for $tag"
        return 1
    fi
    echo "$release_body"
}

# Check for required sections in release body
check_release_sections() {
    local release_body="$1"
    local tag="$2"

    # Define required sections using associative array for better readability
    local -A required_sections=(
        ["assets"]="### .*Assets"
        ["installation"]="### .*Installation"
        ["links"]="### .*Links"
    )

    local missing_sections=()
    for section_name in "${!required_sections[@]}"; do
        local pattern="${required_sections[$section_name]}"
        if ! echo "$release_body" | grep -E "$pattern" >/dev/null; then
            missing_sections+=("$section_name")
        fi
    done

    if [[ ${#missing_sections[@]} -eq 0 ]]; then
        log $LOG_INFO "✓ Release $tag has all required sections"
        return 0
    else
        log $LOG_WARN "✗ Release $tag is missing sections: ${missing_sections[*]}"
        log $LOG_INFO "Consider running: $0 enhance $tag"
        return 1
    fi
}

# Check for professional title formatting
check_release_title() {
    local release_body="$1"
    local tag="$2"

    if echo "$release_body" | grep -E "Code Guardian v[0-9]+" >/dev/null; then
        log $LOG_INFO "✓ Release $tag has professional title format"
        return 0
    else
        log $LOG_WARN "✗ Release $tag may need title formatting improvement"
        return 1
    fi
}

# Validate release format
validate_release() {
    local tag="$1"
    if [[ -z "$tag" ]]; then
        log $LOG_ERROR "Please specify a tag to validate (e.g., v0.1.5)"
        return $EXIT_INVALID_ARGS
    fi

    log $LOG_INFO "Validating release format for $tag..."

    # Validate tag format
    if ! validate_tag "$tag"; then
        return $EXIT_INVALID_ARGS
    fi

    # Check if release exists
    if ! release_exists "$tag"; then
        log $LOG_ERROR "Release $tag does not exist or is not accessible"
        return $EXIT_GENERAL_ERROR
    fi

    # Get release body
    local release_body
    if ! release_body=$(get_release_body "$tag"); then
        return $EXIT_GENERAL_ERROR
    fi

    # Perform validations
    local validation_failed=0
    if ! check_release_sections "$release_body" "$tag"; then
        ((validation_failed++))
    fi

    if ! check_release_title "$release_body" "$tag"; then
        ((validation_failed++))
    fi

    if [[ $validation_failed -gt 0 ]]; then
        log $LOG_WARN "Release $tag failed $validation_failed validation(s)"
        return $EXIT_GENERAL_ERROR
    fi

    log $LOG_INFO "Release $tag validation completed successfully"
}

# Main script logic
main() {
    # Parse common arguments first
    parse_common_args "$@"

    # Perform pre-flight checks
    check_prerequisites

    # Get command and arguments from remaining args
    local command="${REMAINING_ARGS[0]:-help}"
    local arg1="${REMAINING_ARGS[1]:-}"
    local arg2="${REMAINING_ARGS[2]:-}"

    case "$command" in
        "help"|"-h"|"--help"|"")
            show_help
            ;;
        "list")
            list_releases || exit $?
            ;;
        "status")
            show_status || exit $?
            ;;
        "enhance")
            enhance_release "$arg1" || exit $?
            ;;
        "sync-changelog")
            sync_changelog || exit $?
            ;;
        "test-workflows")
            test_workflows || exit $?
            ;;
        "create-manual")
            create_manual_release "$arg1" || exit $?
            ;;
        "validate")
            validate_release "$arg1" || exit $?
            ;;
        *)
            log $LOG_ERROR "Unknown command: $command"
            show_help
            exit $EXIT_INVALID_ARGS
            ;;
    esac
}

# Run main function
main "$@"