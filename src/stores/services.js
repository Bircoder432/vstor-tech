import { defineStore } from "pinia";
import { ref } from "vue";
import {
  Code,
  Terminal,
  Database,
  Globe,
  Cpu,
  Shield,
  Smartphone,
  Cloud,
  ExternalLink,
} from "lucide-vue-next";

export const useServicesStore = defineStore("services", () => {
  const availableIcons = {
    Code,
    Terminal,
    Database,
    Globe,
    Cpu,
    Shield,
    Smartphone,
    Cloud,
    ExternalLink,
  };

  const services = ref([
    {
      id: 1,
      title: "Web Development",
      description: {
        en: "Full-stack web applications using modern frameworks and best practices. From concept to deployment.",
        ru: "Full-stack веб-приложения с использованием современных фреймворков. От концепции до развёртывания.",
      },
      icon: "Code",
      status: "active",
      link: "https://github.com",
    },
    {
      id: 2,
      title: "CLI Tools",
      description: {
        en: "Custom command-line tools and automation scripts to streamline your development workflow.",
        ru: "Инструменты командной строки и скрипты автоматизации для оптимизации рабочего процесса.",
      },
      icon: "Terminal",
      status: "active",
      link: "",
    },
    {
      id: 3,
      title: "Database Design",
      description: {
        en: "Scalable database architecture and optimization for high-performance applications.",
        ru: "Масштабируемая архитектура баз данных и оптимизация для высоконагруженных приложений.",
      },
      icon: "Database",
      status: "active",
      link: "",
    },
  ]);

  const addService = (service) => {
    const id = Math.max(...services.value.map((s) => s.id), 0) + 1;
    services.value.push({ ...service, id });
    saveToStorage();
  };

  const removeService = (id) => {
    services.value = services.value.filter((s) => s.id !== id);
    saveToStorage();
  };

  const updateService = (id, updates) => {
    const index = services.value.findIndex((s) => s.id === id);
    if (index !== -1) {
      services.value[index] = { ...services.value[index], ...updates };
      saveToStorage();
    }
  };

  // Получение описания на текущем языке
  const getDescription = (service, locale) => {
    if (typeof service.description === "object") {
      return service.description[locale] || service.description.en || "";
    }
    return service.description || "";
  };

  const saveToStorage = () => {
    localStorage.setItem("portfolio_services", JSON.stringify(services.value));
  };

  const loadFromStorage = () => {
    const saved = localStorage.getItem("portfolio_services");
    if (saved) {
      const parsed = JSON.parse(saved);
      // Миграция старого формата (строка) в новый (объект)
      services.value = parsed.map((s) => ({
        ...s,
        description:
          typeof s.description === "string"
            ? { en: s.description, ru: s.description }
            : s.description,
      }));
    }
  };

  return {
    services,
    availableIcons,
    addService,
    removeService,
    updateService,
    getDescription,
    loadFromStorage,
    saveToStorage,
  };
});
