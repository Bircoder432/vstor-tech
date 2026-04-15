import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { i18n } from "../i18n";
import { appConfig } from "../config/env";

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

  const loadFromStorage = () => {
    const saved = localStorage.getItem("portfolio_content");
    if (saved) {
      try {
        const data = JSON.parse(saved);
        if (data.homeContent) {
          if (typeof data.homeContent === "string") {
            homeContent.value.en = data.homeContent;
            homeContent.value.ru = data.homeContent;
          } else {
            homeContent.value = { ...homeContent.value, ...data.homeContent };
          }
        }
        // Если в localStorage нет username, используем env
        githubUsername.value = data.githubUsername || appConfig.githubUsername;
      } catch (e) {
        console.error("Failed to load content:", e);
      }
    }
  };

  const saveToStorage = () => {
    localStorage.setItem(
      "portfolio_content",
      JSON.stringify({
        homeContent: homeContent.value,
        githubUsername: githubUsername.value,
      }),
    );
  };

  return {
    homeContent,
    currentHomeContent,
    githubUsername,
    updateHomeContent,
    updateGithubUsername,
    loadFromStorage,
    saveToStorage,
  };
});
