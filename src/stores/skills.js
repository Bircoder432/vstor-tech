// src/stores/skills.js — для консистентности тоже добавлю availableIcons если нужно
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
import { useAuthStore } from "./auth";
import { appConfig } from "../config/env";

const API_URL = appConfig.apiUrl;

export const useSkillsStore = defineStore("skills", () => {
  // Иконки для навыков
  const availableIcons = {
    FileCode,
    FileType,
    Terminal,
    Database,
    GitBranch,
    Container,
    Monitor,
  };

  const skillCategories = [
    { value: "language", label: "Programming Language" },
    { value: "frontend", label: "Frontend" },
    { value: "backend", label: "Backend" },
    { value: "database", label: "Database" },
    { value: "devops", label: "DevOps" },
    { value: "tools", label: "Tools" },
    { value: "system", label: "System" },
  ];

  const skills = ref([]);
  const isLoading = ref(false);
  const error = ref(null);

  const loadFromApi = async () => {
    isLoading.value = true;
    try {
      const response = await fetch(`${API_URL}/content`);
      const data = await response.json();
      if (data.skills) {
        skills.value = data.skills;
      }
    } catch (e) {
      console.error("Failed to load skills:", e);
      error.value = e.message;
    } finally {
      isLoading.value = false;
    }
  };

  const saveSkills = async () => {
    const authStore = useAuthStore();
    try {
      const response = await fetch(`${API_URL}/content`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Auth-Token": authStore.token,
        },
        body: JSON.stringify({ skills: skills.value }),
      });
      if (!response.ok) throw new Error("Failed to save");
      return true;
    } catch (e) {
      console.error("Failed to save skills:", e);
      throw e;
    }
  };

  const addSkill = async (skill) => {
    const id = Math.max(...skills.value.map((s) => s.id), 0) + 1;
    skills.value.push({ ...skill, id });
    await saveSkills();
  };

  const removeSkill = async (id) => {
    skills.value = skills.value.filter((s) => s.id !== id);
    await saveSkills();
  };

  const updateSkill = async (id, updates) => {
    const index = skills.value.findIndex((s) => s.id === id);
    if (index !== -1) {
      skills.value[index] = { ...skills.value[index], ...updates };
      await saveSkills();
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

  return {
    skills,
    skillCategories,
    availableIcons, // добавил для консистентности
    isLoading,
    error,
    loadFromApi,
    addSkill,
    removeSkill,
    updateSkill,
    getDescription,
    getLevelColor,
  };
});
