import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { appConfig } from "../config/env";

export const useAuthStore = defineStore("auth", () => {
  // Используем значения из .env или fallback
  const DEFAULT_PASSWORD = appConfig.adminPassword;
  const ALLOWED_IPS = appConfig.allowedIps;

  // Приватные диапазоны сетей
  const PRIVATE_RANGES = [
    /^10\./,
    /^172\.(1[6-9]|2[0-9]|3[01])\./,
    /^192\.168\./,
  ];

  const isAuthenticated = ref(false);
  const password = ref("");
  const currentIP = ref("");
  const ipAllowed = ref(false);

  const isPrivateIP = (ip) => {
    return PRIVATE_RANGES.some((range) => range.test(ip));
  };

  const fetchIP = async () => {
    try {
      // Проверяем hostname первым делом
      const hostname = window.location.hostname;

      if (
        hostname === "localhost" ||
        hostname === "127.0.0.1" ||
        hostname === "::1"
      ) {
        currentIP.value = "127.0.0.1";
        ipAllowed.value =
          ALLOWED_IPS.includes("127.0.0.1") ||
          ALLOWED_IPS.includes("localhost");
        return;
      }

      // Проверяем приватные сети по hostname
      if (isPrivateIP(hostname)) {
        currentIP.value = hostname;
        ipAllowed.value = true;
        return;
      }

      // Пробуем получить внешний IP через API
      const response = await fetch("https://api.ipify.org?format=json");
      const data = await response.json();
      currentIP.value = data.ip;

      // Проверяем в белом списке или в приватном диапазоне
      ipAllowed.value = ALLOWED_IPS.includes(data.ip) || isPrivateIP(data.ip);
    } catch (err) {
      // Если API недоступен, используем hostname
      const hostname = window.location.hostname;
      currentIP.value = hostname;

      ipAllowed.value =
        ALLOWED_IPS.includes(hostname) ||
        hostname === "localhost" ||
        hostname === "127.0.0.1" ||
        isPrivateIP(hostname);
    }
  };

  const checkAuth = () => {
    const saved = localStorage.getItem("portfolio_auth");
    if (saved) {
      const data = JSON.parse(saved);
      isAuthenticated.value =
        data.isAuthenticated && data.timestamp > Date.now() - 86400000;
    }
  };

  const login = (pass) => {
    if (!ipAllowed.value) return false;
    if (pass === DEFAULT_PASSWORD) {
      isAuthenticated.value = true;
      localStorage.setItem(
        "portfolio_auth",
        JSON.stringify({
          isAuthenticated: true,
          timestamp: Date.now(),
        }),
      );
      return true;
    }
    return false;
  };

  const logout = () => {
    isAuthenticated.value = false;
    localStorage.removeItem("portfolio_auth");
  };

  return {
    isAuthenticated: computed(() => isAuthenticated.value),
    currentIP: computed(() => currentIP.value),
    ipAllowed: computed(() => ipAllowed.value),
    allowedIpsList: ALLOWED_IPS,
    login,
    logout,
    checkAuth,
    fetchIP,
  };
});
