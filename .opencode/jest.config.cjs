module.exports = {
  preset: 'ts-jest/presets/default-esm',
  testEnvironment: 'node',
  roots: ['<rootDir>/tests'],
  testMatch: ['**/__tests__/**/*.test.(js|mjs|ts)', '**/?(*.)+(spec|test).(js|mjs|ts)'],
  transform: {
    '^.+\\.(ts)$': ['ts-jest', { useESM: true }],
    '^.+\\.(js|mjs)$': 'babel-jest',
  },
  extensionsToTreatAsEsm: ['.ts'],
  moduleFileExtensions: ['ts', 'js', 'mjs'],
  collectCoverageFrom: [
    'plugin/**/*.js',
    'tool/**/*.ts',
    '!**/*.d.ts',
  ],
  transformIgnorePatterns: [
    'node_modules/(?!(@opencode-ai/plugin))',
  ],
  moduleNameMapper: {
    '^@opencode-ai/plugin/(.*)$': '<rootDir>/node_modules/@opencode-ai/plugin/dist/$1.js',
  },
};