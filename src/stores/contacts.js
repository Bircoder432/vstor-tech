import { defineStore } from "pinia";
import { ref } from "vue";
import {
  Mail,
  Phone,
  MapPin,
  Github,
  Linkedin,
  Twitter,
  Globe,
  MessageCircle,
} from "lucide-vue-next";
import { appConfig } from "../config/env";

const API_BASE_URL = import.meta.env.VITE_API_URL || "";

export const useContactsStore = defineStore("contacts", () => {
  const availableIcons = {
    Mail,
    Phone,
    MapPin,
    Github,
    Linkedin,
    Twitter,
    Globe,
    MessageCircle,
  };

  const contactTypes = [
    { value: "email", label: "Email", icon: "Mail" },
    { value: "phone", label: "Phone", icon: "Phone" },
    { value: "location", label: "Location", icon: "MapPin" },
    { value: "github", label: "GitHub", icon: "Github" },
    { value: "linkedin", label: "LinkedIn", icon: "Linkedin" },
    { value: "twitter", label: "Twitter", icon: "Twitter" },
    { value: "website", label: "Website", icon: "Globe" },
    { value: "telegram", label: "Telegram", icon: "MessageCircle" },
  ];

  const contacts = ref([
    {
      id: 1,
      type: "email",
      value: "dev@example.com",
      label: "Email",
      icon: "Mail",
    },
    {
      id: 2,
      type: "github",
      value: "github.com/username",
      label: "GitHub",
      icon: "Github",
      link: "https://github.com/username",
    },
    {
      id: 3,
      type: "location",
      value: "Remote / UTC+3",
      label: "Location",
      icon: "MapPin",
    },
  ]);

  const isLoading = ref(false);
  const error = ref(null);

  // Вспомогательная функция для сохранения полного контента
  const saveFullContent = async (partialData) => {
    isLoading.value = true;
    error.value = null;
    try {
      // Сначала загружаем текущие данные
      const getResponse = await fetch(`${API_BASE_URL}/content`);
      if (!getResponse.ok) {
        throw new Error(`HTTP error! status: ${getResponse.status}`);
      }
      const currentData = await getResponse.json();
      
      // Объединяем с новыми данными
      const updatedData = {
        ...currentData,
        ...partialData,
      };
      
      // Отправляем полный объект
      const response = await fetch(`${API_BASE_URL}/content`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Admin-Password": appConfig.adminPassword,
        },
        body: JSON.stringify(updatedData),
      });
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return true;
    } catch (e) {
      console.error("Failed to save contacts to API:", e);
      error.value = e.message;
      throw e;
    } finally {
      isLoading.value = false;
    }
  };

  const addContact = (contact) => {
    const id = Math.max(...contacts.value.map((c) => c.id), 0) + 1;
    const contactType = contactTypes.find((t) => t.value === contact.type);
    contacts.value.push({
      ...contact,
      id,
      icon: contactType?.icon || "Globe",
      label: contact.label || contactType?.label,
    });
    saveFullContent({ contacts: contacts.value });
  };

  const removeContact = (id) => {
    contacts.value = contacts.value.filter((c) => c.id !== id);
    saveFullContent({ contacts: contacts.value });
  };

  const updateContact = (id, updates) => {
    const index = contacts.value.findIndex((c) => c.id === id);
    if (index !== -1) {
      contacts.value[index] = { ...contacts.value[index], ...updates };
      saveFullContent({ contacts: contacts.value });
    }
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
      
      if (data.contacts && Array.isArray(data.contacts)) {
        contacts.value = data.contacts;
      }
    } catch (e) {
      console.error("Failed to load contacts from API:", e);
      error.value = e.message;
      // Fallback to default values already set
    } finally {
      isLoading.value = false;
    }
  };

  return {
    contacts,
    availableIcons,
    contactTypes,
    isLoading,
    error,
    addContact,
    removeContact,
    updateContact,
    loadFromApi,
  };
});
