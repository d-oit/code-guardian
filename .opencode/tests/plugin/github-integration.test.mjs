import { GitHubIntegrationPlugin } from '../../plugin/github-integration.js';

describe('GitHubIntegrationPlugin', () => {
  let mockProject, mockClient, mock$, mockDirectory, mockWorktree;

  beforeEach(() => {
    mockProject = {};
    mockClient = {
      session: {
        summarize: jest.fn().mockResolvedValue('Test summary'),
      },
    };
    mock$ = jest.fn();
    mockDirectory = '/test/dir';
    mockWorktree = '/test/worktree';
  });

  test('should initialize plugin correctly', async () => {
    const plugin = await GitHubIntegrationPlugin({
      project: mockProject,
      client: mockClient,
      $: mock$,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    expect(typeof plugin.event).toBe('function');
    expect(typeof plugin['tool.execute.after']).toBe('function');
  });

  test('event handler should post summary on session.idle with prNumber', async () => {
    const plugin = await GitHubIntegrationPlugin({
      project: mockProject,
      client: mockClient,
      $: mock$,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    const mockEvent = {
      type: 'session.idle',
      properties: {
        prNumber: 123,
        sessionId: 'session-456',
      },
    };

    await plugin.event({ event: mockEvent });

    expect(mockClient.session.summarize).toHaveBeenCalledWith({
      path: { id: 'session-456' },
    });
    expect(mock$).toHaveBeenCalledWith(["gh pr comment ", " --body \"", "\""], 123, "Test summary");
  });

  test('event handler should do nothing if not session.idle or no prNumber', async () => {
    const plugin = await GitHubIntegrationPlugin({
      project: mockProject,
      client: mockClient,
      $: mock$,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    await plugin.event({ event: { type: 'other' } });
    await plugin.event({ event: { type: 'session.idle' } });

    expect(mockClient.session.summarize).not.toHaveBeenCalled();
  });

  test('tool.execute.after should notify on test failures', async () => {
    const plugin = await GitHubIntegrationPlugin({
      project: mockProject,
      client: mockClient,
      $: mock$,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    const input = {
      tool: 'cargo-runner',
      args: { command: 'test' },
    };
    const output = 'Some output with FAILED';

    await plugin['tool.execute.after'](input, output);

    expect(mock$).toHaveBeenCalledWith(["gh issue create --title \"Cargo test failure\" --body \"Test failures detected in cargo test. Please check the CI logs.\""]);
  });

  test('tool.execute.after should do nothing if not cargo-runner test or no FAILED', async () => {
    const plugin = await GitHubIntegrationPlugin({
      project: mockProject,
      client: mockClient,
      $: mock$,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    await plugin['tool.execute.after']({ tool: 'other' }, 'output');
    await plugin['tool.execute.after']({ tool: 'cargo-runner', args: { command: 'build' } }, 'output');
    await plugin['tool.execute.after']({ tool: 'cargo-runner', args: { command: 'test' } }, 'passed');

    expect(mock$).not.toHaveBeenCalled();
  });

  test('should handle errors in summarize gracefully', async () => {
    mockClient.session.summarize.mockRejectedValue(new Error('API error'));

    const plugin = await GitHubIntegrationPlugin({
      project: mockProject,
      client: mockClient,
      $: mock$,
      directory: mockDirectory,
      worktree: mockWorktree,
    });

    const mockEvent = {
      type: 'session.idle',
      properties: {
        prNumber: 123,
        sessionId: 'session-456',
      },
    };

    const consoleErrorSpy = jest.spyOn(console, 'error').mockImplementation();

    await plugin.event({ event: mockEvent });

    expect(consoleErrorSpy).toHaveBeenCalledWith('Failed to post PR comment:', 'API error');

    consoleErrorSpy.mockRestore();
  });
});