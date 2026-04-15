// Получение env переменных с fallback значениями
export const getEnvVar = (key, defaultValue = "") => {
  // Проверяем import.meta.env (Vite)
  if (typeof import.meta.env !== "undefined" && import.meta.env[key]) {
    return import.meta.env[key];
  }

  // Проверяем process.env (Node.js/окружение)
  if (typeof process !== "undefined" && process.env && process.env[key]) {
    return process.env[key];
  }

  return defaultValue;
};

// Парсинг списка IP из строки
export const parseIpList = (ipString) => {
  if (!ipString) return ["127.0.0.1"];
  return ipString
    .split(",")
    .map((ip) => ip.trim())
    .filter(Boolean);
};

// Конфигурация приложения
export const appConfig = {
  // Admin
  adminPassword: getEnvVar("VITE_ADMIN_PASSWORD", "admin123"),
  allowedIps: parseIpList(
    getEnvVar("VITE_ALLOWED_IPS", "127.0.0.1,::1,localhost"),
  ),

  // GitHub
  githubUsername: getEnvVar("VITE_GITHUB_USERNAME", "octocat"),

  // App info
  appName: getEnvVar("VITE_APP_NAME", "Portfolio"),
  appVersion: getEnvVar("VITE_APP_VERSION", "1.0.0"),

  // Mode
  isDev: typeof import.meta.env !== "undefined" && import.meta.env.DEV,
  isProd: typeof import.meta.env !== "undefined" && import.meta.env.PROD,
  mode:
    typeof import.meta.env !== "undefined"
      ? import.meta.env.MODE
      : "development",
};

export default appConfig;
