# PR Create Command

## Description

This command facilitates the creation of pull requests on GitHub for the current branch, ensuring all changes are properly analyzed and documented.

## Agent Assignment

Assigned to: GitHub Agent

## Steps

1. Check the current branch status: Run `git status` to see untracked files, `git diff` for changes, and verify if the branch tracks a remote.

2. Analyze commits: Run `git log` and `git diff main...HEAD` to understand the commit history since diverging from the main branch.

3. Draft PR summary: Summarize the nature of changes, purpose, impact, and ensure no sensitive information.

4. Prepare for push: If not up to date, push the branch with `-u` flag.

5. Create PR: Use `gh pr create --title "Title" --body "Body"` with a concise summary in 1-2 bullet points focusing on the "why".

6. Return the PR URL.