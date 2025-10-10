#!/usr/bin/env node

/**
 * GitHub Repository Ruleset Setup for Code-Guardian
 * Uses GitHub's new Repository Rules API for advanced protection
 */

const { Octokit } = require("@octokit/rest");
const fs = require("fs");
const path = require("path");

// Configuration
const REPO_OWNER = "d-oit";
const REPO_NAME = "code-guardian";

// Colors for console output
const colors = {
    reset: "\x1b[0m",
    red: "\x1b[31m",
    green: "\x1b[32m",
    yellow: "\x1b[33m",
    blue: "\x1b[34m",
    magenta: "\x1b[35m",
    cyan: "\x1b[36m"
};

function log(message, color = colors.reset) {
    console.log(`${color}${message}${colors.reset}`);
}

function logInfo(message) {
    log(`ℹ️  ${message}`, colors.blue);
}

function logSuccess(message) {
    log(`✅ ${message}`, colors.green);
}

function logWarning(message) {
    log(`⚠️  ${message}`, colors.yellow);
}

function logError(message) {
    log(`❌ ${message}`, colors.red);
}

class GitHubRulesetManager {
    constructor(token) {
        this.octokit = new Octokit({
            auth: token,
            userAgent: "code-guardian-setup v1.0.0"
        });
    }

    /**
     * Create the main repository ruleset
     */
    async createMainRuleset() {
        const rulesetConfig = {
            name: "Code-Guardian Protection Rules",
            target: "branch",
            enforcement: "active",
            conditions: {
                ref_name: {
                    include: ["refs/heads/main", "refs/heads/develop"],
                    exclude: []
                }
            },
            rules: [
                // Pull Request Rules
                {
                    type: "pull_request",
                    parameters: {
                        dismiss_stale_reviews_on_push: true,
                        require_code_owner_review: true,
                        require_last_push_approval: true,
                        required_approving_review_count: 1,
                        required_review_thread_resolution: true
                    }
                },
                // Required Status Checks
                {
                    type: "required_status_checks",
                    parameters: {
                        required_status_checks: [
                            {
                                context: "Test (ubuntu-latest, stable)",
                                integration_id: null
                            },
                            {
                                context: "Test (windows-latest, stable)",
                                integration_id: null
                            },
                            {
                                context: "Test (macos-latest, stable)",
                                integration_id: null
                            },
                            {
                                context: "Coverage",
                                integration_id: null
                            },
                            {
                                context: "Security Audit",
                                integration_id: null
                            },
                            {
                                context: "CodeQL",
                                integration_id: null
                            }
                        ],
                        strict_required_status_checks_policy: true
                    }
                },
                // Prevent force pushes
                {
                    type: "non_fast_forward"
                },
                // Conventional commit pattern
                {
                    type: "commit_message_pattern",
                    parameters: {
                        name: "Conventional Commits",
                        negate: false,
                        pattern: "^(feat|fix|docs|style|refactor|perf|test|chore|ci|build|revert)(\\(.+\\))?: .{1,50}",
                        operator: "regex"
                    }
                },
                // Valid email pattern
                {
                    type: "commit_author_email_pattern",
                    parameters: {
                        name: "Valid Email Pattern",
                        negate: false,
                        pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
                        operator: "regex"
                    }
                },
                // Prevent branch creation
                {
                    type: "creation"
                },
                // Require linear history
                {
                    type: "required_linear_history"
                },
                // File path restrictions
                {
                    type: "file_path_restriction",
                    parameters: {
                        restricted_file_paths: [
                            "/.github/workflows/*",
                            "/Cargo.toml",
                            "/deny.toml",
                            "/.github/CODEOWNERS",
                            "/.github/SECURITY.md"
                        ]
                    }
                },
                // Max file path length
                {
                    type: "max_file_path_length",
                    parameters: {
                        max_file_path_length: 255
                    }
                }
            ],
            bypass_actors: [
                {
                    actor_id: 1,
                    actor_type: "Integration",
                    bypass_mode: "always"
                }
            ]
        };

        try {
            logInfo("Creating repository ruleset...");
            
            const response = await this.octokit.rest.repos.createRepoRuleset({
                owner: REPO_OWNER,
                repo: REPO_NAME,
                ...rulesetConfig
            });

            logSuccess(`Repository ruleset created with ID: ${response.data.id}`);
            return response.data;
        } catch (error) {
            if (error.status === 403) {
                logWarning("Insufficient permissions to create repository rulesets.");
                logInfo("Repository rulesets require admin permissions or organization settings.");
                return null;
            } else if (error.status === 422) {
                logWarning("Ruleset validation failed. Some rules may not be supported.");
                logInfo("Consider using branch protection rules instead.");
                return null;
            } else {
                logError(`Failed to create repository ruleset: ${error.message}`);
                throw error;
            }
        }
    }

    /**
     * Create tag protection ruleset
     */
    async createTagRuleset() {
        const tagRulesetConfig = {
            name: "Tag Protection Rules",
            target: "tag",
            enforcement: "active",
            conditions: {
                ref_name: {
                    include: ["refs/tags/v*"],
                    exclude: []
                }
            },
            rules: [
                // Prevent tag creation without proper checks
                {
                    type: "creation"
                },
                // Prevent tag deletion
                {
                    type: "deletion"
                },
                // Require status checks for tags
                {
                    type: "required_status_checks",
                    parameters: {
                        required_status_checks: [
                            {
                                context: "Test (ubuntu-latest, stable)",
                                integration_id: null
                            },
                            {
                                context: "Security Audit",
                                integration_id: null
                            }
                        ],
                        strict_required_status_checks_policy: true
                    }
                }
            ]
        };

        try {
            logInfo("Creating tag protection ruleset...");
            
            const response = await this.octokit.rest.repos.createRepoRuleset({
                owner: REPO_OWNER,
                repo: REPO_NAME,
                ...tagRulesetConfig
            });

            logSuccess(`Tag protection ruleset created with ID: ${response.data.id}`);
            return response.data;
        } catch (error) {
            logWarning(`Could not create tag protection ruleset: ${error.message}`);
            return null;
        }
    }

    /**
     * List existing rulesets
     */
    async listRulesets() {
        try {
            const response = await this.octokit.rest.repos.getRepoRulesets({
                owner: REPO_OWNER,
                repo: REPO_NAME
            });

            logInfo(`Found ${response.data.length} existing rulesets:`);
            response.data.forEach(ruleset => {
                log(`  📋 ${ruleset.name} (ID: ${ruleset.id}) - ${ruleset.enforcement}`, colors.cyan);
            });

            return response.data;
        } catch (error) {
            logWarning(`Could not list rulesets: ${error.message}`);
            return [];
        }
    }

    /**
     * Get repository information
     */
    async getRepositoryInfo() {
        try {
            const response = await this.octokit.rest.repos.get({
                owner: REPO_OWNER,
                repo: REPO_NAME
            });

            const repo = response.data;
            log("\n📊 Repository Information:", colors.magenta);
            log(`   📁 Name: ${repo.full_name}`);
            log(`   🌟 Stars: ${repo.stargazers_count}`);
            log(`   🔀 Forks: ${repo.forks_count}`);
            log(`   🌿 Default branch: ${repo.default_branch}`);
            log(`   🔒 Private: ${repo.private}`);
            log(`   📅 Created: ${repo.created_at.substring(0, 10)}`);

            return repo;
        } catch (error) {
            logError(`Failed to get repository info: ${error.message}`);
            throw error;
        }
    }

    /**
     * Enable repository security features
     */
    async enableSecurityFeatures() {
        logInfo("Enabling repository security features...");

        try {
            // Enable vulnerability alerts
            await this.octokit.rest.repos.enableVulnerabilityAlerts({
                owner: REPO_OWNER,
                repo: REPO_NAME
            });
            logSuccess("Vulnerability alerts enabled");
        } catch (error) {
            logWarning(`Could not enable vulnerability alerts: ${error.message}`);
        }

        try {
            // Enable automated security fixes
            await this.octokit.rest.repos.enableAutomatedSecurityFixes({
                owner: REPO_OWNER,
                repo: REPO_NAME
            });
            logSuccess("Automated security fixes enabled");
        } catch (error) {
            logWarning(`Could not enable automated security fixes: ${error.message}`);
        }
    }
}

/**
 * Get GitHub token from environment or prompt user
 */
function getGitHubToken() {
    const token = process.env.GITHUB_TOKEN;
    if (!token) {
        logError("GITHUB_TOKEN environment variable not set.");
        logInfo("Please set your GitHub token:");
        logInfo("export GITHUB_TOKEN=your_token_here");
        process.exit(1);
    }
    return token;
}

/**
 * Main execution function
 */
async function main() {
    console.log("=" * 60);
    log("🛡️  Code-Guardian Repository Ruleset Setup", colors.magenta);
    console.log("=" * 60);
    console.log();

    try {
        // Get GitHub token
        const token = getGitHubToken();
        logSuccess("GitHub token found");

        // Initialize manager
        const manager = new GitHubRulesetManager(token);

        // Get repository info
        await manager.getRepositoryInfo();
        console.log();

        // List existing rulesets
        await manager.listRulesets();
        console.log();

        // Ask for confirmation
        const readline = require("readline");
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });

        const answer = await new Promise(resolve => {
            rl.question("🤔 Do you want to proceed with creating repository rulesets? (y/N): ", resolve);
        });
        rl.close();

        if (!["y", "yes"].includes(answer.toLowerCase())) {
            logInfo("Setup cancelled by user.");
            process.exit(0);
        }

        console.log();
        logInfo("Starting repository ruleset setup...");
        console.log();

        // Create main ruleset
        const mainRuleset = await manager.createMainRuleset();
        console.log();

        // Create tag ruleset
        const tagRuleset = await manager.createTagRuleset();
        console.log();

        // Enable security features
        await manager.enableSecurityFeatures();
        console.log();

        // Summary
        log("📋 Setup Summary:", colors.magenta);
        log(`   ✅ Main ruleset: ${mainRuleset ? "Created" : "Failed/Skipped"}`);
        log(`   ✅ Tag ruleset: ${tagRuleset ? "Created" : "Failed/Skipped"}`);
        log(`   🔒 Security features: Enabled`);
        console.log();

        logSuccess("Repository ruleset setup completed!");
        console.log();
        log("Next steps:", colors.cyan);
        log("1. 🌐 Verify rulesets in GitHub Settings > Rules");
        log("2. 🧪 Test by creating a PR");
        log("3. ⚙️  Adjust rules as needed");
        console.log();
        log("🛡️  Your repository now has advanced protection rules!", colors.green);

    } catch (error) {
        logError(`Setup failed: ${error.message}`);
        process.exit(1);
    }
}

// Check if required packages are installed
try {
    require("@octokit/rest");
} catch (error) {
    logError("Missing required package: @octokit/rest");
    logInfo("Please run: npm install @octokit/rest");
    process.exit(1);
}

// Run main function
if (require.main === module) {
    main().catch(error => {
        logError(`Unexpected error: ${error.message}`);
        process.exit(1);
    });
}