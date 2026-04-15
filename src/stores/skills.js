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
      icon: { type: "vue-component", name: "FileCode", component: FileCode },
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
      icon: { type: "vue-component", name: "FileType", component: FileType },
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
      icon: { type: "vue-component", name: "Terminal", component: Terminal },
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
      icon: { type: "vue-component", name: "GitBranch", component: GitBranch },
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
      icon: { type: "vue-component", name: "Container", component: Container },
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
      icon: { type: "vue-component", name: "Monitor", component: Monitor },
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
      icon: { type: "vue-component", name: "Database", component: Database },
      level: "intermediate",
      description: {
        en: "Query optimization, indexing, transactions",
        ru: "Оптимизация запросов, индексы, транзакции",
      },
    },
  ]);

  const addSkill = (skill) => {
    const id = Math.max(...skills.value.map((s) => s.id), 0) + 1;
    skills.value.push({ ...skill, id });
    saveToStorage();
  };

  const removeSkill = (id) => {
    skills.value = skills.value.filter((s) => s.id !== id);
    saveToStorage();
  };

  const updateSkill = (id, updates) => {
    const index = skills.value.findIndex((s) => s.id === id);
    if (index !== -1) {
      skills.value[index] = { ...skills.value[index], ...updates };
      saveToStorage();
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

  const saveToStorage = () => {
    // Не сохраняем компоненты Vue в localStorage
    const storable = skills.value.map((s) => ({
      ...s,
      icon:
        typeof s.icon === "object" && s.icon.component
          ? { ...s.icon, component: undefined }
          : s.icon,
    }));
    localStorage.setItem("portfolio_skills", JSON.stringify(storable));
  };

  const loadFromStorage = () => {
    const saved = localStorage.getItem("portfolio_skills");
    if (saved) {
      try {
        const parsed = JSON.parse(saved);
        // Восстанавливаем компоненты для Lucide иконок
        skills.value = parsed.map((s) => {
          if (s.icon && s.icon.type === "vue-component" && s.icon.name) {
            // Импортируем динамически или используем сопоставление
            const iconMap = {
              FileCode,
              FileType,
              Terminal,
              Database,
              GitBranch,
              Container,
              Monitor,
            };
            if (iconMap[s.icon.name]) {
              s.icon.component = iconMap[s.icon.name];
            }
          }
          return s;
        });
      } catch (e) {
        console.error("Failed to load skills:", e);
      }
    }
  };

  return {
    skills,
    skillCategories,
    addSkill,
    removeSkill,
    updateSkill,
    getDescription,
    getLevelColor,
    loadFromStorage,
    saveToStorage,
  };
});
