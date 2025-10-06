/**
 * GitHub Integration Plugin for opencode.
 * Handles posting session summaries to PR comments and notifying on cargo-test failures.
 */
export const GitHubIntegrationPlugin = async ({ client, $ }) => {
  let currentPrNumber = null;

  return {
    event: async ({ event }) => {
      // On session idle, store PR number and post summary as PR comment if available
      if (event.type === "session.idle") {
        currentPrNumber = event.properties?.prNumber;
        if (currentPrNumber) {
          try {
            const summary = await client.session.summarize({ path: { id: event.properties.sessionId } });
             await $`gh pr comment ${currentPrNumber} --body "${JSON.stringify(summary)}"`;
          } catch (error) {
            console.error("Failed to post PR comment:", error.message);
          }
        }
      }
    },
    "tool.execute.after": async (input, output) => {
      // After cargo-test, if failures detected, notify via issue or PR comment
      if (input.tool === "cargo-runner" && input.args?.command === "test" && output.includes("FAILED")) {
        try {
          if (currentPrNumber) {
            await $`gh pr comment ${currentPrNumber} --body "Cargo test failures detected. Please review the test output."`;
          } else {
            await $`gh issue create --title "Cargo test failure" --body "Test failures detected in cargo test. Please check the CI logs."`;
          }
        } catch (error) {
          console.error("Failed to notify on test failure:", error.message);
        }
      }
    },
  };
};