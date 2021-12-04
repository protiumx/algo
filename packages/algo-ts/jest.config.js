module.exports = {
  collectCoverageFrom: ["src/**/*.ts"],
  coverageDirectory: "coverage",
  coverageThreshold: {
    global: {
      branches: 100,
      functions: 100,
      lines: 100,
      statements: 100,
    },
  },
  globals: {
    "ts-jest": {
      babel: true,
      tsconfig: "tsconfig.json",
    },
  },
  moduleNameMapper: {
    "^$/(.*)$": "<rootDir>/src/$1",
  },
  moduleFileExtensions: ["json", "ts", "js"],
  testEnvironment: "node",
  testMatch: ["**/*.spec.ts"],
  transform: {
    "^.+\\.js$": "babel-jest",
    "^.+\\.ts$": "ts-jest",
  },
  transformIgnorePatterns: ["/node_modules/", "/dist"],
  verbose: false,
};
