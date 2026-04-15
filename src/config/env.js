// src/config/env.js
export const getEnvVar = (key, defaultValue = "") => {
  if (typeof import.meta.env !== "undefined" && import.meta.env[key]) {
    return import.meta.env[key];
  }
  if (typeof process !== "undefined" && process.env && process.env[key]) {
    return process.env[key];
  }
  return defaultValue;
};

export const appConfig = {
  apiUrl: getEnvVar("VITE_API_URL", "http://localhost:3001/api"),
  githubUsername: getEnvVar("VITE_GITHUB_USERNAME", "octocat"),
  appName: getEnvVar("VITE_APP_NAME", "Portfolio"),
  appVersion: getEnvVar("VITE_APP_VERSION", "1.0.0"),
  isDev: typeof import.meta.env !== "undefined" && import.meta.env.DEV,
  isProd: typeof import.meta.env !== "undefined" && import.meta.env.PROD,
};

export default appConfig;
