import { defineStore } from "pinia";
import { ref } from "vue";
import {
  FileCode,
  FileType,
  Terminal,
  Database,
  GitBranch,
  Container,
  Monitor,
} from "lucide-vue-next";

const API_BASE_URL = import.meta.env.VITE_API_URL || (import.meta.env.DEV ? "/api" : "http://localhost:3001");

export const useSkillsStore = defineStore("skills", () => {
  const skillCategories = [
    { value: "language", label: "Programming Language" },
    { value: "frontend", label: "Frontend" },
    { value: "backend", label: "Backend" },
    { value: "database", label: "Database" },
    { value: "devops", label: "DevOps" },
    { value: "tools", label: "Tools" },
    { value: "system", label: "System" },
  ];

  const skills = ref([
    {
      id: 1,
      name: "JavaScript",
      category: "language",
      icon: { type: "vue-component", name: "FileCode" },
      level: "advanced",
      description: {
        en: "ES6+, async/await, closures, event loop",
        ru: "ES6+, async/await, замыкания, event loop",
      },
    },
    {
      id: 2,
      name: "TypeScript",
      category: "language",
      icon: { type: "vue-component", name: "FileType" },
      level: "advanced",
      description: {
        en: "Type systems, generics, decorators",
        ru: "Типизация, дженерики, декораторы",
      },
    },
    {
      id: 3,
      name: "Python",
      category: "language",
      icon: { type: "vue-component", name: "Terminal" },
      level: "intermediate",
      description: {
        en: "Django, FastAPI, data processing",
        ru: "Django, FastAPI, обработка данных",
      },
    },
    {
      id: 4,
      name: "Git",
      category: "tools",
      icon: { type: "vue-component", name: "GitBranch" },
      level: "advanced",
      description: {
        en: "Branching, rebasing, CI/CD integration",
        ru: "Ветвление, ребейзинг, интеграция CI/CD",
      },
    },
    {
      id: 5,
      name: "Docker",
      category: "devops",
      icon: { type: "vue-component", name: "Container" },
      level: "intermediate",
      description: {
        en: "Containerization, compose, multi-stage builds",
        ru: "Контейнеризация, compose, многоэтапные сборки",
      },
    },
    {
      id: 6,
      name: "Linux",
      category: "system",
      icon: { type: "vue-component", name: "Monitor" },
      level: "advanced",
      description: {
        en: "Bash, systemd, networking, security",
        ru: "Bash, systemd, сети, безопасность",
      },
    },
    {
      id: 7,
      name: "Vue.js",
      category: "frontend",
      icon: {
        type: "external",
        source: "simple-icons",
        name: "vuedotjs",
        url: "https://cdn.simpleicons.org/vuedotjs",
      },
      level: "advanced",
      description: {
        en: "Composition API, Pinia, Vue Router",
        ru: "Composition API, Pinia, Vue Router",
      },
    },
    {
      id: 8,
      name: "PostgreSQL",
      category: "database",
      icon: { type: "vue-component", name: "Database" },
      level: "intermediate",
      description: {
        en: "Query optimization, indexing, transactions",
        ru: "Оптимизация запросов, индексы, транзакции",
      },
    },
  ]);

  const isLoading = ref(false);
  const error = ref(null);

  const addSkill = (skill) => {
    const id = Math.max(...skills.value.map((s) => s.id), 0) + 1;
    skills.value.push({ ...skill, id });
    saveToApi();
  };

  const removeSkill = (id) => {
    skills.value = skills.value.filter((s) => s.id !== id);
    saveToApi();
  };

  const updateSkill = (id, updates) => {
    const index = skills.value.findIndex((s) => s.id === id);
    if (index !== -1) {
      skills.value[index] = { ...skills.value[index], ...updates };
      saveToApi();
    }
  };

  const getDescription = (skill, locale) => {
    if (typeof skill.description === "object") {
      return skill.description[locale] || skill.description.en || "";
    }
    return skill.description || "";
  };

  const getLevelColor = (level) => {
    const colors = {
      beginner: "#888888",
      intermediate: "#ffaa00",
      advanced: "#00ff00",
      expert: "#00ffff",
    };
    return colors[level] || "#888888";
  };

  // Загрузка данных из API
  const loadFromApi = async () => {
    isLoading.value = true;
    error.value = null;
    try {
      const response = await fetch(`${API_BASE_URL}/api/content`);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data = await response.json();
      
      if (data.skills && Array.isArray(data.skills)) {
        skills.value = data.skills;
      }
    } catch (e) {
      console.error("Failed to load skills from API:", e);
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
      const response = await fetch(`${API_BASE_URL}/api/content`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Admin-Password": localStorage.getItem("adminPassword") || "admin123",
        },
        body: JSON.stringify({ skills: skills.value }),
      });
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return true;
    } catch (e) {
      console.error("Failed to save skills to API:", e);
      error.value = e.message;
      throw e;
    } finally {
      isLoading.value = false;
    }
  };

  return {
    skills,
    skillCategories,
    isLoading,
    error,
    addSkill,
    removeSkill,
    updateSkill,
    getDescription,
    getLevelColor,
    loadFromApi,
    saveToApi,
  };
});
