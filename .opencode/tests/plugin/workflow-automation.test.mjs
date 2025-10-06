import { WorkflowAutomationPlugin } from '../../plugin/workflow-automation.js';
import { exec } from 'child_process';

jest.mock('child_process', () => ({
  exec: jest.fn(),
}));

describe('WorkflowAutomationPlugin', () => {
  let mockProject, mockClient, mockDirectory, mockWorktree;

  beforeEach(() => {
    mockProject = {};
    mockClient = {
      tui: {
        showToast: jest.fn(),
      },
    };
    mockDirectory = '/test/dir';
    mockWorktree = {};
    exec.mockClear();
  });

  test('should initialize plugin correctly', async () => {
    const plugin = await WorkflowAutomationPlugin({
      project: mockProject,
      client: mockClient,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    expect(typeof plugin.event).toBe('function');
    expect(typeof plugin['tool.execute.after']).toBe('function');
  });

  test('tool.execute.after should run cargo check on edit success', async () => {
    exec.mockImplementation((cmd, callback) => callback(null, 'stdout', 'stderr'));

    const plugin = await WorkflowAutomationPlugin({
      project: mockProject,
      client: mockClient,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    const input = { tool: 'edit' };
    const output = { success: true };

    await plugin['tool.execute.after'](input, output);

    expect(exec).toHaveBeenCalledWith('cargo check', expect.any(Function));
  });

  test('tool.execute.after should show toast on cargo check failure', async () => {
    exec.mockImplementation((cmd, callback) => callback(new Error('Command failed'), null, 'stderr'));

    const plugin = await WorkflowAutomationPlugin({
      project: mockProject,
      client: mockClient,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    const input = { tool: 'edit' };
    const output = { success: true };

    await plugin['tool.execute.after'](input, output);

    expect(mockClient.tui.showToast).toHaveBeenCalledWith({
      body: { message: 'Cargo check failed: Command failed', variant: 'error' },
    });
  });

  test('tool.execute.after should do nothing if edit not successful', async () => {
    const plugin = await WorkflowAutomationPlugin({
      project: mockProject,
      client: mockClient,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    const input = { tool: 'edit' };
    const output = { success: false };

    await plugin['tool.execute.after'](input, output);

    expect(exec).not.toHaveBeenCalled();
    expect(mockClient.tui.showToast).not.toHaveBeenCalled();
  });

  test('tool.execute.after should do nothing if not edit tool', async () => {
    const plugin = await WorkflowAutomationPlugin({
      project: mockProject,
      client: mockClient,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    const input = { tool: 'other' };
    const output = { success: true };

    await plugin['tool.execute.after'](input, output);

    expect(exec).not.toHaveBeenCalled();
  });

  test('event handler should run workflow on session.start with workflow', async () => {
    exec.mockImplementation((cmd, callback) => callback(null, 'stdout', 'stderr'));

    const plugin = await WorkflowAutomationPlugin({
      project: mockProject,
      client: mockClient,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    const mockEvent = {
      type: 'session.start',
      properties: {
        workflow: 'test',
      },
    };

    await plugin.event({ event: mockEvent });

    expect(exec).toHaveBeenCalledWith('cargo test', expect.any(Function));
    expect(mockClient.tui.showToast).toHaveBeenCalledWith({
      body: { message: 'Running tests', variant: 'info' },
    });
    expect(mockClient.tui.showToast).toHaveBeenCalledWith({
      body: { message: 'Workflow test completed successfully', variant: 'success' },
    });
  });

  test('event handler should do nothing if not session.start or no workflow', async () => {
    const plugin = await WorkflowAutomationPlugin({
      project: mockProject,
      client: mockClient,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    await plugin.event({ event: { type: 'other' } });
    await plugin.event({ event: { type: 'session.start' } });

    expect(exec).not.toHaveBeenCalled();
  });

  test('should handle errors in cargo check gracefully', async () => {
    exec.mockImplementation((cmd, callback) => callback(new Error('Command failed'), null, 'stderr'));

    const plugin = await WorkflowAutomationPlugin({
      project: mockProject,
      client: mockClient,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    const input = { tool: 'edit' };
    const output = { success: true };

    await plugin['tool.execute.after'](input, output);

    expect(mockClient.tui.showToast).toHaveBeenCalledWith({
      body: { message: 'Cargo check failed: Command failed', variant: 'error' },
    });
  });
});