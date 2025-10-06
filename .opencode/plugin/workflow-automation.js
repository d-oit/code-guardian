export const WorkflowAutomationPlugin = async ({ project, client, $, directory, worktree }) => {
  return {
    "tool.execute.after": async (input, output) => {
      // After editing code, auto-run check
      if (input.tool === "edit" && output.success) {
        const checkResult = await $`cargo check`.quiet();
        if (checkResult.exitCode !== 0) {
          await client.tui.showToast({ body: { message: "Cargo check failed - fix errors", variant: "error" } });
        }
      }
    },
    event: async ({ event }) => {
      // On session start for workflows, initialize
      if (event.type === "session.start" && event.properties?.workflow) {
        console.log(`Starting workflow: ${event.properties.workflow}`);
        // Could invoke specific agents here
      }
    },
  };
};