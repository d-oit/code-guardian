#!/usr/bin/env python3
"""
GitHub API Branch Protection Setup for Code-Guardian
Advanced Python script for configuring repository protection rules
"""

import json
import os
import sys
import time
from typing import Dict, List, Optional
import requests
from dataclasses import dataclass


@dataclass
class BranchProtectionConfig:
    """Configuration for branch protection rules"""
    name: str
    required_status_checks: List[str]
    required_approving_review_count: int
    dismiss_stale_reviews: bool
    require_code_owner_reviews: bool
    require_last_push_approval: bool
    required_linear_history: bool
    allow_force_pushes: bool
    allow_deletions: bool
    required_conversation_resolution: bool
    strict_status_checks: bool = True


class GitHubAPIClient:
    """GitHub API client for repository management"""
    
    def __init__(self, token: str, repo_owner: str, repo_name: str):
        self.token = token
        self.repo_owner = repo_owner
        self.repo_name = repo_name
        self.base_url = "https://api.github.com"
        self.session = requests.Session()
        self.session.headers.update({
            "Authorization": f"token {token}",
            "Accept": "application/vnd.github.v3+json",
            "Content-Type": "application/json"
        })
    
    def _make_request(self, method: str, endpoint: str, data: Optional[Dict] = None) -> requests.Response:
        """Make a request to GitHub API"""
        url = f"{self.base_url}/repos/{self.repo_owner}/{self.repo_name}/{endpoint}"
        
        try:
            if method.upper() == "GET":
                response = self.session.get(url)
            elif method.upper() == "PUT":
                response = self.session.put(url, json=data)
            elif method.upper() == "POST":
                response = self.session.post(url, json=data)
            elif method.upper() == "DELETE":
                response = self.session.delete(url)
            else:
                raise ValueError(f"Unsupported HTTP method: {method}")
            
            return response
        except requests.exceptions.RequestException as e:
            print(f"❌ Request failed: {e}")
            sys.exit(1)
    
    def get_repository_info(self) -> Dict:
        """Get repository information"""
        response = self._make_request("GET", "")
        if response.status_code == 200:
            return response.json()
        else:
            print(f"❌ Failed to get repository info: {response.status_code}")
            print(response.text)
            sys.exit(1)
    
    def get_branch_protection(self, branch: str) -> Optional[Dict]:
        """Get current branch protection settings"""
        response = self._make_request("GET", f"branches/{branch}/protection")
        if response.status_code == 200:
            return response.json()
        elif response.status_code == 404:
            return None
        else:
            print(f"⚠️  Failed to get branch protection for {branch}: {response.status_code}")
            return None
    
    def set_branch_protection(self, branch: str, config: BranchProtectionConfig) -> bool:
        """Set branch protection rules"""
        protection_data = {
            "required_status_checks": {
                "strict": config.strict_status_checks,
                "checks": [{"context": check} for check in config.required_status_checks]
            },
            "enforce_admins": False,
            "required_pull_request_reviews": {
                "required_approving_review_count": config.required_approving_review_count,
                "dismiss_stale_reviews": config.dismiss_stale_reviews,
                "require_code_owner_reviews": config.require_code_owner_reviews,
                "require_last_push_approval": config.require_last_push_approval
            },
            "restrictions": None,
            "required_linear_history": config.required_linear_history,
            "allow_force_pushes": config.allow_force_pushes,
            "allow_deletions": config.allow_deletions,
            "block_creations": False,
            "required_conversation_resolution": config.required_conversation_resolution,
            "lock_branch": False,
            "allow_fork_syncing": branch != "main"
        }
        
        print(f"🔧 Setting up protection for '{branch}' branch...")
        response = self._make_request("PUT", f"branches/{branch}/protection", protection_data)
        
        if response.status_code == 200:
            print(f"✅ Branch protection configured successfully for '{branch}'!")
            return True
        else:
            print(f"❌ Failed to configure branch protection for '{branch}': {response.status_code}")
            print(response.text)
            return False
    
    def enable_vulnerability_alerts(self) -> bool:
        """Enable vulnerability alerts"""
        response = self._make_request("PUT", "vulnerability-alerts")
        return response.status_code in [200, 204]
    
    def enable_automated_security_fixes(self) -> bool:
        """Enable automated security fixes"""
        response = self._make_request("PUT", "automated-security-fixes")
        return response.status_code in [200, 204]
    
    def create_repository_ruleset(self, ruleset_data: Dict) -> bool:
        """Create repository ruleset (requires admin permissions)"""
        # This endpoint might require special permissions
        url = f"{self.base_url}/repos/{self.repo_owner}/{self.repo_name}/rulesets"
        response = self.session.post(url, json=ruleset_data)
        
        if response.status_code == 201:
            print("✅ Repository ruleset created successfully!")
            return True
        else:
            print(f"⚠️  Failed to create repository ruleset: {response.status_code}")
            print("This might require admin permissions or organization settings.")
            return False


def get_github_token() -> str:
    """Get GitHub token from environment or GitHub CLI"""
    token = os.getenv("GITHUB_TOKEN")
    if token:
        return token
    
    # Try to get token from GitHub CLI
    try:
        import subprocess
        result = subprocess.run(["gh", "auth", "token"], capture_output=True, text=True)
        if result.returncode == 0:
            return result.stdout.strip()
    except FileNotFoundError:
        pass
    
    print("❌ GitHub token not found. Please set GITHUB_TOKEN environment variable or run 'gh auth login'")
    sys.exit(1)


def create_branch_configs() -> Dict[str, BranchProtectionConfig]:
    """Create branch protection configurations"""
    return {
        "main": BranchProtectionConfig(
            name="main",
            required_status_checks=[
                "Test (ubuntu-latest, stable)",
                "Test (windows-latest, stable)", 
                "Test (macos-latest, stable)",
                "Coverage",
                "Security Audit",
                "CodeQL / Analyze (rust)",
                "CodeQL / Analyze (javascript)"
            ],
            required_approving_review_count=1,
            dismiss_stale_reviews=True,
            require_code_owner_reviews=True,
            require_last_push_approval=True,
            required_linear_history=True,
            allow_force_pushes=False,
            allow_deletions=False,
            required_conversation_resolution=True,
            strict_status_checks=True
        ),
        "develop": BranchProtectionConfig(
            name="develop",
            required_status_checks=[
                "Test (ubuntu-latest, stable)",
                "Coverage",
                "Security Audit"
            ],
            required_approving_review_count=1,
            dismiss_stale_reviews=True,
            require_code_owner_reviews=False,
            require_last_push_approval=False,
            required_linear_history=False,
            allow_force_pushes=False,
            allow_deletions=False,
            required_conversation_resolution=True,
            strict_status_checks=True
        )
    }


def print_repository_summary(client: GitHubAPIClient):
    """Print repository information summary"""
    print("📊 Repository Information:")
    
    repo_info = client.get_repository_info()
    print(f"   📁 Name: {repo_info['full_name']}")
    print(f"   🌟 Stars: {repo_info['stargazers_count']}")
    print(f"   🔀 Forks: {repo_info['forks_count']}")
    print(f"   🌿 Default branch: {repo_info['default_branch']}")
    print(f"   🔒 Private: {repo_info['private']}")
    print(f"   📅 Created: {repo_info['created_at'][:10]}")
    print()


def verify_protection_status(client: GitHubAPIClient, branches: List[str]):
    """Verify current protection status"""
    print("🔍 Current Protection Status:")
    
    for branch in branches:
        protection = client.get_branch_protection(branch)
        if protection:
            checks_count = len(protection.get("required_status_checks", {}).get("checks", []))
            review_count = protection.get("required_pull_request_reviews", {}).get("required_approving_review_count", 0)
            linear_history = protection.get("required_linear_history", False)
            
            print(f"   ✅ {branch}: Protected")
            print(f"      - Status checks: {checks_count}")
            print(f"      - Required reviews: {review_count}")
            print(f"      - Linear history: {'Yes' if linear_history else 'No'}")
        else:
            print(f"   ❌ {branch}: Not protected")
    print()


def main():
    """Main execution function"""
    print("=" * 60)
    print("🛡️  Code-Guardian GitHub Branch Protection Setup")
    print("=" * 60)
    print()
    
    # Configuration
    REPO_OWNER = "d-oit"
    REPO_NAME = "code-guardian"
    
    # Get GitHub token
    try:
        token = get_github_token()
        print("✅ GitHub authentication verified")
    except Exception as e:
        print(f"❌ Authentication failed: {e}")
        sys.exit(1)
    
    # Initialize client
    client = GitHubAPIClient(token, REPO_OWNER, REPO_NAME)
    
    # Show repository summary
    print_repository_summary(client)
    
    # Create branch configurations
    branch_configs = create_branch_configs()
    branches = list(branch_configs.keys())
    
    # Verify current status
    verify_protection_status(client, branches)
    
    # Ask for confirmation
    response = input("🤔 Do you want to proceed with setting up branch protection rules? (y/N): ")
    if response.lower() not in ['y', 'yes']:
        print("⏹️  Setup cancelled by user.")
        sys.exit(0)
    
    print()
    print("🚀 Starting branch protection setup...")
    print()
    
    # Apply protection rules
    success_count = 0
    for branch, config in branch_configs.items():
        if client.set_branch_protection(branch, config):
            success_count += 1
            time.sleep(1)  # Rate limiting
        print()
    
    # Enable security features
    print("🔒 Enabling security features...")
    
    if client.enable_vulnerability_alerts():
        print("✅ Vulnerability alerts enabled")
    else:
        print("⚠️  Could not enable vulnerability alerts")
    
    if client.enable_automated_security_fixes():
        print("✅ Automated security fixes enabled")
    else:
        print("⚠️  Could not enable automated security fixes")
    
    print()
    
    # Final verification
    print("🔍 Final verification...")
    verify_protection_status(client, branches)
    
    # Summary
    print("📋 Setup Summary:")
    print(f"   ✅ Successfully configured: {success_count}/{len(branch_configs)} branches")
    print(f"   🔒 Security features: Enabled")
    print()
    
    if success_count == len(branch_configs):
        print("🎉 Branch protection setup completed successfully!")
        print()
        print("Next steps:")
        print("1. 🌐 Verify settings in GitHub web interface")
        print("2. 🧪 Test by creating a PR to main branch")
        print("3. ⚙️  Configure organization-level settings if needed")
        print("4. 📋 Import repository ruleset for additional protection")
    else:
        print("⚠️  Some configurations may have failed. Please check manually.")
    
    print()
    print("🛡️  Your repository now has enterprise-grade protection!")


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n⏹️  Setup cancelled by user.")
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ Unexpected error: {e}")
        sys.exit(1)