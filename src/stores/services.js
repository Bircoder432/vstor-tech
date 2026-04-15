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

const API_BASE_URL = import.meta.env.VITE_API_URL || (import.meta.env.DEV ? "/api" : "http://localhost:3001");

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
      icon: { type: "vue-component", name: "Code" },
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
      icon: { type: "vue-component", name: "Terminal" },
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
      icon: { type: "vue-component", name: "Database" },
      status: "active",
      link: "",
    },
  ]);

  const isLoading = ref(false);
  const error = ref(null);

  const addService = (service) => {
    const id = Math.max(...services.value.map((s) => s.id), 0) + 1;
    services.value.push({ ...service, id });
    saveToApi();
  };

  const removeService = (id) => {
    services.value = services.value.filter((s) => s.id !== id);
    saveToApi();
  };

  const updateService = (id, updates) => {
    const index = services.value.findIndex((s) => s.id === id);
    if (index !== -1) {
      services.value[index] = { ...services.value[index], ...updates };
      saveToApi();
    }
  };

  // Получение описания на текущем языке
  const getDescription = (service, locale) => {
    if (typeof service.description === "object") {
      return service.description[locale] || service.description.en || "";
    }
    return service.description || "";
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
      
      if (data.services && Array.isArray(data.services)) {
        services.value = data.services.map((s) => ({
          ...s,
          description:
            typeof s.description === "string"
              ? { en: s.description, ru: s.description }
              : s.description,
        }));
      }
    } catch (e) {
      console.error("Failed to load services from API:", e);
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
        body: JSON.stringify({ services: services.value }),
      });
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return true;
    } catch (e) {
      console.error("Failed to save services to API:", e);
      error.value = e.message;
      throw e;
    } finally {
      isLoading.value = false;
    }
  };

  return {
    services,
    availableIcons,
    isLoading,
    error,
    addService,
    removeService,
    updateService,
    getDescription,
    loadFromApi,
    saveToApi,
  };
});
