// src/stores/content.js
import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { i18n } from "../i18n";
import { appConfig } from "../config/env";
import { useAuthStore } from "./auth";

const API_URL = "https://api.vstor-tech.ru/portfolio/api";

export const useContentStore = defineStore("content", () => {
  const homeContent = ref({
    en: "",
    ru: "",
  });
  const githubUsername = ref(appConfig.githubUsername);
  const isLoading = ref(false);
  const error = ref(null);

  const currentHomeContent = computed(() => {
    const locale = i18n.global.locale.value;
    return homeContent.value[locale] || homeContent.value.en || "";
  });

  const loadFromApi = async () => {
    isLoading.value = true;
    error.value = null;
    try {
      const response = await fetch(`${API_URL}/content`);
      if (!response.ok) throw new Error(`HTTP ${response.status}`);
      const data = await response.json();

      if (data.homeContent) {
        homeContent.value = {
          en: data.homeContent.en || "",
          ru: data.homeContent.ru || "",
        };
      }
      if (data.githubUsername) {
        githubUsername.value = data.githubUsername;
      }
    } catch (e) {
      console.error("Failed to load content:", e);
      error.value = e.message;
    } finally {
      isLoading.value = false;
    }
  };

  const saveToApi = async () => {
    const authStore = useAuthStore();
    if (!authStore.token) throw new Error("Not authenticated");

    isLoading.value = true;
    error.value = null;

    try {
      const response = await fetch(`${API_URL}/content`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Auth-Token": authStore.token,
        },
        body: JSON.stringify({
          homeContent: homeContent.value,
          githubUsername: githubUsername.value,
        }),
      });

      if (!response.ok) {
        const data = await response.json();
        throw new Error(data.error || `HTTP ${response.status}`);
      }

      return true;
    } catch (e) {
      console.error("Failed to save:", e);
      error.value = e.message;
      throw e;
    } finally {
      isLoading.value = false;
    }
  };

  const updateHomeContent = (content, locale = null) => {
    const target = locale || i18n.global.locale.value;
    homeContent.value[target] = String(content);
  };

  const updateGithubUsername = (username) => {
    githubUsername.value = String(username);
  };

  return {
    homeContent,
    currentHomeContent,
    githubUsername,
    isLoading,
    error,
    loadFromApi,
    saveToApi,
    updateHomeContent,
    updateGithubUsername,
  };
});
