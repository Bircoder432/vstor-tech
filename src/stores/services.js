// src/stores/services.js
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
import { useAuthStore } from "./auth";
import { appConfig } from "../config/env";

const API_URL = "http://80.64.17.81:3001/api";

export const useServicesStore = defineStore("services", () => {
  // Компоненты иконок для шаблонов
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

  const services = ref([]);
  const isLoading = ref(false);
  const error = ref(null);

  const loadFromApi = async () => {
    isLoading.value = true;
    try {
      const response = await fetch(`${API_URL}/content`);
      const data = await response.json();
      if (data.services) {
        services.value = data.services;
      }
    } catch (e) {
      console.error("Failed to load services:", e);
      error.value = e.message;
    } finally {
      isLoading.value = false;
    }
  };

  const saveServices = async () => {
    const authStore = useAuthStore();
    try {
      const response = await fetch(`${API_URL}/content`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Auth-Token": authStore.token,
        },
        body: JSON.stringify({ services: services.value }),
      });
      if (!response.ok) throw new Error("Failed to save");
      return true;
    } catch (e) {
      console.error("Failed to save services:", e);
      throw e;
    }
  };

  const addService = async (service) => {
    const id = Math.max(...services.value.map((s) => s.id), 0) + 1;
    services.value.push({ ...service, id });
    await saveServices();
  };

  const removeService = async (id) => {
    services.value = services.value.filter((s) => s.id !== id);
    await saveServices();
  };

  const updateService = async (id, updates) => {
    const index = services.value.findIndex((s) => s.id === id);
    if (index !== -1) {
      services.value[index] = { ...services.value[index], ...updates };
      await saveServices();
    }
  };

  const getDescription = (service, locale) => {
    if (typeof service.description === "object") {
      return service.description[locale] || service.description.en || "";
    }
    return service.description || "";
  };

  return {
    services,
    availableIcons, // <-- вернул
    isLoading,
    error,
    loadFromApi,
    addService,
    removeService,
    updateService,
    getDescription,
  };
});
