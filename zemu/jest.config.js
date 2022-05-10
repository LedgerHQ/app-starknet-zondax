module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  transformIgnorePatterns: ['^.+\\.js$'],
  globalSetup: './jest/globalsetup.ts',
  globalTeardown: './jest/globalteardown.ts',
  setupFilesAfterEnv: ['./jest/setup.ts'],
  testMatch: ['**/__tests__/**/*.ts?(x)', '**/?(*.)+(spec|test).ts?(x)'],
}
