# Issue Management Command

## Description
This command file provides standardized instructions for managing GitHub issues and discussions within the OpenCode ecosystem. It covers commenting on issues, closing issues, and adding comments to discussions using the GitHub CLI (`gh`). This ensures consistent and efficient handling of repository interactions.

## Agent Assignment
GitHub

## Steps

### Commenting on an Issue
1. Identify the repository and issue number (e.g., issue #123 in the `owner/repo` repository).
2. Ensure you are authenticated with GitHub CLI: `gh auth login` (if not already done).
3. Run the following command to add a comment:
   ```
   gh issue comment 123 --body "Your detailed comment here."
   ```
4. Verify the comment appears on the issue page.

### Closing an Issue
1. Identify the repository and issue number.
2. Ensure you have the necessary permissions to close issues.
3. Run the following command to close the issue:
   ```
   gh issue close 123
   ```
4. Optionally, add a comment when closing:
   ```
   gh issue close 123 --comment "Reason for closing: Issue resolved."
   ```
5. Confirm the issue status changes to closed.

### Adding to Discussions
1. Identify the repository, discussion category, and discussion number or URL.
2. Note that GitHub CLI has limited direct support for discussions; use the API for commenting.
3. Run the following command to add a comment to a discussion:
   ```
   gh api repos/{owner}/{repo}/discussions/{discussion_number}/comments -f body="Your comment here."
   ```
   - Replace `{owner}`, `{repo}`, and `{discussion_number}` with actual values (e.g., `octocat/Hello-World` and `1`).
4. For creating new discussions or more complex operations, refer to the GitHub CLI documentation or use the web interface.

### Additional Notes
- Always review the issue or discussion context before taking action.
- Use clear and concise comments to maintain professionalism.
- If operations fail, check permissions, authentication, and repository access.
- For bulk operations or automation, consider scripting with these commands.