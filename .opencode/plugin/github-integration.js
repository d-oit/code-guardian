export const GitHubIntegrationPlugin = async ({ project, client, $, directory, worktree }) => {
  return {
    event: async ({ event }) => {
      // On session completion, check for PR and comment with summary
      if (event.type === "session.idle" && event.properties?.prNumber) {
        const summary = await client.session.summarize({ path: { id: event.properties.sessionId } });
        // Assume GitHub API integration here, e.g., post comment
        console.log(`Posting summary to PR #${event.properties.prNumber}: ${summary}`);
        // In real implementation, use GitHub API or webhook
      }
    },
    "tool.execute.after": async (input, output) => {
      // After cargo-test, if failures, notify
      if (input.tool === "cargo-runner" && input.args.command === "test" && output.includes("FAILED")) {
        await $`echo "Test failures detected - notify team"`;
      }
    },
  };
};