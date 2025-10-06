import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

/**
 * WorkflowAutomationPlugin
 *
 * Automates workflows on session start and provides post-edit checks.
 *
 * Workflows supported:
 * - build: Runs cargo build
 * - test: Runs cargo test
 * - lint: Runs cargo clippy
 * - format: Runs cargo fmt
 * - check: Runs cargo check
 * - ci: Runs full CI pipeline (check, test, lint, format)
 */
export const WorkflowAutomationPlugin = async ({ client }) => {
  // Define workflow commands
  const workflows = {
    build: [{ cmd: "cargo build", desc: "Building project" }],
    test: [{ cmd: "cargo test", desc: "Running tests" }],
    lint: [{ cmd: "cargo clippy", desc: "Linting code" }],
    format: [{ cmd: "cargo fmt", desc: "Formatting code" }],
    check: [{ cmd: "cargo check", desc: "Checking code" }],
    ci: [
      { cmd: "cargo check", desc: "Checking code" },
      { cmd: "cargo test", desc: "Running tests" },
      { cmd: "cargo clippy", desc: "Linting code" },
      { cmd: "cargo fmt --check", desc: "Checking formatting" },
    ],
  };

  /**
   * Runs a workflow by executing its commands sequentially.
   * @param {string} workflowName - The name of the workflow to run.
   */
  const runWorkflow = async (workflowName) => {
    const steps = workflows[workflowName];
    if (!steps) {
      await client.tui.showToast({
        body: { message: `Unknown workflow: ${workflowName}`, variant: "error" },
      });
      return;
    }

    for (const step of steps) {
      try {
        await client.tui.showToast({
          body: { message: step.desc, variant: "info" },
        });
        await execAsync(step.cmd);
      } catch (error) {
        await client.tui.showToast({
          body: { message: `${step.desc} failed: ${error.message}`, variant: "error" },
        });
        return; // Stop on first failure
      }
    }

    await client.tui.showToast({
      body: { message: `Workflow ${workflowName} completed successfully`, variant: "success" },
    });
  };

  return {
    "tool.execute.after": async (input, output) => {
      // After editing code, auto-run check
      if (input.tool === "edit" && output.success) {
        try {
          await execAsync("cargo check");
        } catch (error) {
          await client.tui.showToast({
            body: { message: `Cargo check failed: ${error.message}`, variant: "error" },
          });
        }
      }
    },
    event: async ({ event }) => {
      // On session start for workflows, initialize
      if (event.type === "session.start" && event.properties?.workflow) {
        const workflow = event.properties.workflow;
        await runWorkflow(workflow);
      }
    },
  };
};