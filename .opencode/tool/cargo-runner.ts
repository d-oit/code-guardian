import { tool } from "@opencode-ai/plugin";

export default tool({
  description: "Run a Cargo command and return output",
  args: {
    command: tool.schema.string().describe("Cargo subcommand, e.g., 'build'"),
  },
  async execute(args, ctx) {
    const result = await ctx.$`cargo ${args.command}`;
    return result.text();
  },
});