#!/bin/bash
# Automated Version Management Script for Code Guardian
# Provides local version management capabilities that integrate with GitHub workflows

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CRATES=("core" "cli" "output" "storage")

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Display usage information
show_usage() {
    cat << EOF
Code Guardian Version Manager

USAGE:
    $0 <command> [options]

COMMANDS:
    status                    Show current version status across all crates
    sync <version>           Synchronize all crates to specified version
    bump <type>              Bump version (patch|minor|major)
    check                    Check for version inconsistencies
    validate                 Validate workspace after version changes
    prepare-release <version> Prepare workspace for release
    help                     Show this help message

OPTIONS:
    --dry-run               Show what would be changed without making changes
    --force                 Force operation even if validations fail
    --quiet                 Suppress non-essential output
    --changelog             Update CHANGELOG.md (default for sync/bump)
    --no-changelog          Skip CHANGELOG.md update

EXAMPLES:
    $0 status                           # Show current versions
    $0 sync 0.3.0                      # Sync all crates to v0.3.0
    $0 bump patch                      # Bump patch version for all crates
    $0 prepare-release 0.3.0 --dry-run # Preview release preparation
    $0 check                           # Check version consistency

INTEGRATION:
    This script works with the GitHub Actions workflow (.github/workflows/version-sync.yml)
    and the existing release-please configuration.

EOF
}

# Get current version from a crate
get_crate_version() {
    local crate_name="$1"
    local cargo_toml="$WORKSPACE_ROOT/crates/$crate_name/Cargo.toml"
    
    if [[ -f "$cargo_toml" ]]; then
        grep '^version = ' "$cargo_toml" | cut -d'"' -f2
    else
        log_error "Cargo.toml not found for crate: $crate_name"
        return 1
    fi
}

# Get release-please manifest version
get_manifest_version() {
    local manifest_file="$WORKSPACE_ROOT/.github/.release-please-manifest.json"
    
    if [[ -f "$manifest_file" ]]; then
        jq -r '."."' "$manifest_file" 2>/dev/null || echo "unknown"
    else
        echo "unknown"
    fi
}

# Set version for a crate
set_crate_version() {
    local crate_name="$1"
    local new_version="$2"
    local cargo_toml="$WORKSPACE_ROOT/crates/$crate_name/Cargo.toml"
    
    if [[ -f "$cargo_toml" ]]; then
        sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" "$cargo_toml"
        rm "$cargo_toml.bak" 2>/dev/null || true
        log_success "Updated $crate_name: $new_version"
    else
        log_error "Cargo.toml not found for crate: $crate_name"
        return 1
    fi
}

# Set release-please manifest version
set_manifest_version() {
    local new_version="$1"
    local manifest_file="$WORKSPACE_ROOT/.github/.release-please-manifest.json"
    
    if [[ -f "$manifest_file" ]]; then
        jq --arg version "$new_version" '."." = $version' "$manifest_file" > "$manifest_file.tmp"
        mv "$manifest_file.tmp" "$manifest_file"
        log_success "Updated release-please manifest: $new_version"
    else
        log_error "Release-please manifest not found"
        return 1
    fi
}

# Validate semantic version format
validate_semver() {
    local version="$1"
    
    if [[ $version =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9\.-]+)?(\+[a-zA-Z0-9\.-]+)?$ ]]; then
        return 0
    else
        log_error "Invalid semantic version format: $version"
        log_info "Expected format: MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]"
        return 1
    fi
}

# Bump version according to semver rules
bump_version() {
    local current_version="$1"
    local bump_type="$2"
    
    # Extract version components
    local version_core
    version_core=$(echo "$current_version" | cut -d'-' -f1 | cut -d'+' -f1)
    
    local major minor patch
    IFS='.' read -r major minor patch <<< "$version_core"
    
    case "$bump_type" in
        "major")
            echo "$((major + 1)).0.0"
            ;;
        "minor")
            echo "$major.$((minor + 1)).0"
            ;;
        "patch")
            echo "$major.$minor.$((patch + 1))"
            ;;
        *)
            log_error "Invalid bump type: $bump_type (use: major, minor, patch)"
            return 1
            ;;
    esac
}

# Show current version status
show_status() {
    log_info "Code Guardian Version Status"
    echo
    
    printf "%-15s %-10s\n" "Component" "Version"
    printf "%-15s %-10s\n" "---------" "-------"
    
    local versions=()
    for crate in "${CRATES[@]}"; do
        local version
        version=$(get_crate_version "$crate")
        printf "%-15s %-10s\n" "$crate" "$version"
        versions+=("$version")
    done
    
    local manifest_version
    manifest_version=$(get_manifest_version)
    printf "%-15s %-10s\n" "release-please" "$manifest_version"
    
    echo
    
    # Check consistency
    local unique_versions
    unique_versions=$(printf '%s\n' "${versions[@]}" | sort -u | wc -l)
    
    if [[ $unique_versions -eq 1 ]]; then
        log_success "All crates are synchronized to version ${versions[0]}"
        if [[ "${versions[0]}" == "$manifest_version" ]]; then
            log_success "Release-please manifest is also synchronized"
        else
            log_warning "Release-please manifest version differs: $manifest_version"
        fi
    else
        log_warning "Crates have inconsistent versions"
        log_info "Run '$0 sync <version>' to synchronize all crates"
    fi
}

# Check for version inconsistencies
check_versions() {
    local inconsistencies=0
    local versions=()
    
    for crate in "${CRATES[@]}"; do
        local version
        version=$(get_crate_version "$crate")
        versions+=("$version")
    done
    
    local manifest_version
    manifest_version=$(get_manifest_version)
    
    # Check if all crate versions are the same
    local unique_crate_versions
    unique_crate_versions=$(printf '%s\n' "${versions[@]}" | sort -u | wc -l)
    
    if [[ $unique_crate_versions -gt 1 ]]; then
        log_error "Crates have inconsistent versions:"
        for i in "${!CRATES[@]}"; do
            echo "  ${CRATES[$i]}: ${versions[$i]}"
        done
        inconsistencies=$((inconsistencies + 1))
    fi
    
    # Check if manifest matches crate versions
    if [[ "${versions[0]}" != "$manifest_version" ]]; then
        log_error "Release-please manifest version ($manifest_version) doesn't match crate versions (${versions[0]})"
        inconsistencies=$((inconsistencies + 1))
    fi
    
    if [[ $inconsistencies -eq 0 ]]; then
        log_success "All versions are consistent"
        return 0
    else
        log_error "Found $inconsistencies version inconsistencies"
        return 1
    fi
}

# Synchronize all crates to a specific version
sync_versions() {
    local target_version="$1"
    local dry_run="${2:-false}"
    local update_changelog="${3:-true}"
    
    if ! validate_semver "$target_version"; then
        return 1
    fi
    
    if [[ "$dry_run" == "true" ]]; then
        log_info "DRY RUN: Would synchronize all crates to version $target_version"
    else
        log_info "Synchronizing all crates to version $target_version"
    fi
    
    # Update all crates
    for crate in "${CRATES[@]}"; do
        local current_version
        current_version=$(get_crate_version "$crate")
        
        if [[ "$current_version" != "$target_version" ]]; then
            if [[ "$dry_run" == "true" ]]; then
                log_info "Would update $crate: $current_version → $target_version"
            else
                set_crate_version "$crate" "$target_version"
            fi
        else
            log_info "$crate already at version $target_version"
        fi
    done
    
    # Update release-please manifest
    local current_manifest
    current_manifest=$(get_manifest_version)
    
    if [[ "$current_manifest" != "$target_version" ]]; then
        if [[ "$dry_run" == "true" ]]; then
            log_info "Would update release-please manifest: $current_manifest → $target_version"
        else
            set_manifest_version "$target_version"
        fi
    else
        log_info "Release-please manifest already at version $target_version"
    fi
    
    # Update CHANGELOG if requested
    if [[ "$update_changelog" == "true" && "$dry_run" == "false" ]]; then
        update_changelog_for_version "$target_version"
    fi
    
    if [[ "$dry_run" == "false" ]]; then
        log_success "Version synchronization completed"
    fi
}

# Bump all versions
bump_all_versions() {
    local bump_type="$1"
    local dry_run="${2:-false}"
    local update_changelog="${3:-true}"
    
    # Get current version (use first crate as reference)
    local current_version
    current_version=$(get_crate_version "${CRATES[0]}")
    
    local new_version
    new_version=$(bump_version "$current_version" "$bump_type")
    
    if [[ $? -ne 0 ]]; then
        return 1
    fi
    
    log_info "Bumping $bump_type version: $current_version → $new_version"
    
    sync_versions "$new_version" "$dry_run" "$update_changelog"
}

# Update CHANGELOG for a version
update_changelog_for_version() {
    local version="$1"
    local changelog_file="$WORKSPACE_ROOT/CHANGELOG.md"
    
    if [[ ! -f "$changelog_file" ]]; then
        log_warning "CHANGELOG.md not found, skipping update"
        return 0
    fi
    
    # Check if version already exists in changelog
    if grep -q "## \[$version\]" "$changelog_file"; then
        log_info "CHANGELOG.md already contains entry for version $version"
        return 0
    fi
    
    local temp_file
    temp_file=$(mktemp)
    
    # Extract header
    sed -n '1,/^## \[/p' "$changelog_file" | head -n -1 > "$temp_file"
    
    # Add new version entry
    cat >> "$temp_file" << EOF

## [$version] - $(date +%Y-%m-%d)

### 🔄 Version Management

- Synchronized all workspace crates to version $version
- Updated release-please manifest configuration
- Applied automated version management

EOF
    
    # Append rest of changelog
    sed -n '/^## \[/,$p' "$changelog_file" >> "$temp_file"
    
    mv "$temp_file" "$changelog_file"
    log_success "Updated CHANGELOG.md with entry for version $version"
}

# Validate workspace after version changes
validate_workspace() {
    log_info "Validating workspace after version changes..."
    
    # Check if workspace builds
    if ! cargo check --workspace --quiet; then
        log_error "Workspace build validation failed"
        return 1
    fi
    
    log_success "Workspace builds successfully"
    
    # Check version consistency
    if ! check_versions; then
        return 1
    fi
    
    log_success "Workspace validation completed successfully"
}

# Prepare workspace for release
prepare_release() {
    local version="$1"
    local dry_run="${2:-false}"
    
    log_info "Preparing workspace for release $version"
    
    # Sync versions
    sync_versions "$version" "$dry_run" true
    
    if [[ "$dry_run" == "false" ]]; then
        # Validate workspace
        if ! validate_workspace; then
            log_error "Workspace validation failed during release preparation"
            return 1
        fi
        
        # Run quality checks
        log_info "Running quality checks..."
        if command -v cargo &> /dev/null; then
            cargo fmt --all --check || {
                log_warning "Formatting issues found. Run 'cargo fmt --all' to fix."
            }
            
            cargo clippy --workspace --all-targets -- -D warnings || {
                log_warning "Clippy warnings found. Address before release."
            }
        fi
        
        log_success "Release preparation completed for version $version"
        log_info "Next steps:"
        log_info "1. Review and commit changes"
        log_info "2. Push to trigger release workflow"
        log_info "3. Monitor CI/CD pipeline"
    fi
}

# Parse command line arguments
parse_args() {
    local command="$1"
    shift
    
    local dry_run=false
    local force=false
    local quiet=false
    local update_changelog=true
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --dry-run)
                dry_run=true
                shift
                ;;
            --force)
                force=true
                shift
                ;;
            --quiet)
                quiet=true
                shift
                ;;
            --no-changelog)
                update_changelog=false
                shift
                ;;
            --changelog)
                update_changelog=true
                shift
                ;;
            *)
                break
                ;;
        esac
    done
    
    case "$command" in
        "status")
            show_status
            ;;
        "sync")
            if [[ $# -lt 1 ]]; then
                log_error "sync command requires a version argument"
                show_usage
                exit 1
            fi
            sync_versions "$1" "$dry_run" "$update_changelog"
            ;;
        "bump")
            if [[ $# -lt 1 ]]; then
                log_error "bump command requires a type argument (patch|minor|major)"
                show_usage
                exit 1
            fi
            bump_all_versions "$1" "$dry_run" "$update_changelog"
            ;;
        "check")
            check_versions
            ;;
        "validate")
            validate_workspace
            ;;
        "prepare-release")
            if [[ $# -lt 1 ]]; then
                log_error "prepare-release command requires a version argument"
                show_usage
                exit 1
            fi
            prepare_release "$1" "$dry_run"
            ;;
        "help"|"--help"|"-h")
            show_usage
            ;;
        *)
            log_error "Unknown command: $command"
            show_usage
            exit 1
            ;;
    esac
}

# Main execution
main() {
    cd "$WORKSPACE_ROOT"
    
    if [[ $# -eq 0 ]]; then
        show_usage
        exit 1
    fi
    
    parse_args "$@"
}

# Check if script is being run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi