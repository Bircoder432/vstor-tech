import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { i18n } from "../i18n";
import { appConfig } from "../config/env";

const API_BASE_URL = import.meta.env.VITE_API_URL || "";

export const useContentStore = defineStore("content", () => {
  const homeContent = ref({
    en: `# > about_me

Full-stack developer specializing in building terminal-themed applications and developer tools. Passionate about clean code, minimal design, and maximum performance.

## > skills

- **Frontend:** Vue.js, React, TypeScript, TailwindCSS
- **Backend:** Node.js, Python, PostgreSQL
- **DevOps:** Docker, CI/CD, Linux
- **Tools:** Git, Vim, Terminal workflows

## > contact

- Email: dev@example.com
- GitHub: github.com/${appConfig.githubUsername}
- Location: Remote / UTC+3

---

*"Code is poetry written in logic."*`,
    ru: `# > обо_мне

Full-stack разработчик, специализирующийся на создании приложений в терминальном стиле и инструментов для разработчиков. Увлекаюсь чистым кодом, минималистичным дизайном и максимальной производительностью.

## > навыки

- **Frontend:** Vue.js, React, TypeScript, TailwindCSS
- **Backend:** Node.js, Python, PostgreSQL
- **DevOps:** Docker, CI/CD, Linux
- **Инструменты:** Git, Vim, Terminal workflows

## > контакты

- Email: dev@example.com
- GitHub: github.com/${appConfig.githubUsername}
- Локация: Удалённо / UTC+3

---

*"Код — это поэзия, написанная логикой."*`,
  });

  // Используем GitHub username из env или дефолт
  const githubUsername = ref(appConfig.githubUsername);
  const isLoading = ref(false);
  const error = ref(null);

  const currentHomeContent = computed(() => {
    const locale = i18n.global.locale.value;
    const content = homeContent.value[locale];
    if (typeof content === "string") return content;
    if (typeof homeContent.value.en === "string") return homeContent.value.en;
    return "";
  });

  const updateHomeContent = (content, locale = null) => {
    const targetLocale = locale || i18n.global.locale.value;
    homeContent.value[targetLocale] = String(content);
  };

  const updateGithubUsername = (username) => {
    githubUsername.value = String(username);
  };

  // Загрузка данных из API
  const loadFromApi = async () => {
    isLoading.value = true;
    error.value = null;
    try {
      const response = await fetch(`${API_BASE_URL}/content`);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data = await response.json();
      
      if (data.homeContent) {
        if (typeof data.homeContent === "string") {
          homeContent.value.en = data.homeContent;
          homeContent.value.ru = data.homeContent;
        } else {
          homeContent.value = { ...homeContent.value, ...data.homeContent };
        }
      }
      
      if (data.githubUsername) {
        githubUsername.value = data.githubUsername;
      }
    } catch (e) {
      console.error("Failed to load content from API:", e);
      error.value = e.message;
      // Fallback to default values already set
    } finally {
      isLoading.value = false;
    }
  };

  // Сохранение данных в API
  const saveToApi = async () => {
    isLoading.value = true;
    error.value = null;
    try {
      const response = await fetch(`${API_BASE_URL}/content`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Admin-Password": appConfig.adminPassword,
        },
        body: JSON.stringify({
          homeContent: homeContent.value,
          githubUsername: githubUsername.value,
        }),
      });
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return true;
    } catch (e) {
      console.error("Failed to save content to API:", e);
      error.value = e.message;
      throw e;
    } finally {
      isLoading.value = false;
    }
  };

  return {
    homeContent,
    currentHomeContent,
    githubUsername,
    isLoading,
    error,
    updateHomeContent,
    updateGithubUsername,
    loadFromApi,
    saveToApi,
  };
});
