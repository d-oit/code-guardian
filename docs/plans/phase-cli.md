# Phase 5: CLI

## Overview
Build the command-line interface using clap for user interaction.

## Sub-goals
- Implement commands: scan, history, report, compare
- Integrate core, storage, output modules

## Action Sequence
1. Build CLI commands with clap
   - Precond: Core, Storage, Output modules implemented
   - Effect: CLI module implemented: true
   - Effort: Medium
   - Description: Define subcommands, parse args, orchestrate scanning, storage, formatting.

## Dependencies
- Phases 1,2,3,4

## Testing Recommendations
- CLI integration tests with assert_cmd
- Test argument parsing and error handling
- End-to-end tests simulating user workflows

## Potential Optimizations
- Auto-completion for shells
- Progress bars for long scans