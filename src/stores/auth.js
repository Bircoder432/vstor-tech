// src/stores/auth.js
import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { appConfig } from "../config/env";

const API_URL = "http://80.64.17.81:3001/api";

export const useAuthStore = defineStore("auth", () => {
  const isAuthenticated = ref(false);
  const isLoading = ref(false);
  const currentIP = ref("");
  const ipAllowed = ref(false);
  const authToken = ref(localStorage.getItem("portfolio_auth_token") || "");
  const error = ref(null);

  // Проверяем токен при инициализации
  const init = async () => {
    if (authToken.value) {
      await checkStatus();
    } else {
      await checkIP();
    }
  };

  const checkIP = async () => {
    try {
      const response = await fetch(`${API_URL}/auth/status`);
      const data = await response.json();
      currentIP.value = data.currentIP;
      ipAllowed.value = data.ipAllowed;
      isAuthenticated.value = data.authenticated;
    } catch (e) {
      console.error("Failed to check IP:", e);
      ipAllowed.value = false;
    }
  };

  const checkStatus = async () => {
    if (!authToken.value) return;

    try {
      const response = await fetch(`${API_URL}/auth/status`, {
        headers: {
          "X-Auth-Token": authToken.value,
        },
      });
      const data = await response.json();
      currentIP.value = data.currentIP;
      ipAllowed.value = data.ipAllowed;
      isAuthenticated.value = data.authenticated;

      if (!data.authenticated) {
        logout();
      }
    } catch (e) {
      console.error("Failed to check status:", e);
      isAuthenticated.value = false;
    }
  };

  const login = async (password) => {
    isLoading.value = true;
    error.value = null;

    try {
      const response = await fetch(`${API_URL}/auth/login`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ password }),
      });

      const data = await response.json();

      if (data.success && data.token) {
        authToken.value = data.token;
        localStorage.setItem("portfolio_auth_token", data.token);
        isAuthenticated.value = true;
        return { success: true };
      } else {
        error.value = data.error || "Login failed";
        return { success: false, error: data.error };
      }
    } catch (e) {
      error.value = "Network error";
      return { success: false, error: e.message };
    } finally {
      isLoading.value = false;
    }
  };

  const logout = async () => {
    if (authToken.value) {
      try {
        await fetch(`${API_URL}/auth/logout`, {
          method: "POST",
          headers: {
            "X-Auth-Token": authToken.value,
          },
        });
      } catch (e) {
        console.error("Logout error:", e);
      }
    }

    authToken.value = "";
    localStorage.removeItem("portfolio_auth_token");
    isAuthenticated.value = false;
  };

  return {
    isAuthenticated: computed(() => isAuthenticated.value),
    isLoading: computed(() => isLoading.value),
    currentIP: computed(() => currentIP.value),
    ipAllowed: computed(() => ipAllowed.value),
    error: computed(() => error.value),
    token: computed(() => authToken.value),
    init,
    login,
    logout,
    checkStatus,
  };
});
