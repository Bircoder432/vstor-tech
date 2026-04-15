// src/stores/contacts.js
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
import { useAuthStore } from "./auth";
import { appConfig } from "../config/env";

const API_URL = appConfig.apiUrl;

export const useContactsStore = defineStore("contacts", () => {
  // Компоненты иконок для использования в шаблонах
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

  const contacts = ref([]);
  const isLoading = ref(false);
  const error = ref(null);

  const loadFromApi = async () => {
    isLoading.value = true;
    try {
      const response = await fetch(`${API_URL}/content`);
      const data = await response.json();
      if (data.contacts) {
        contacts.value = data.contacts;
      }
    } catch (e) {
      console.error("Failed to load contacts:", e);
      error.value = e.message;
    } finally {
      isLoading.value = false;
    }
  };

  const saveContacts = async () => {
    const authStore = useAuthStore();
    try {
      const response = await fetch(`${API_URL}/content`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Auth-Token": authStore.token,
        },
        body: JSON.stringify({ contacts: contacts.value }),
      });
      if (!response.ok) throw new Error("Failed to save");
      return true;
    } catch (e) {
      console.error("Failed to save contacts:", e);
      throw e;
    }
  };

  const addContact = async (contact) => {
    const id = Math.max(...contacts.value.map((c) => c.id), 0) + 1;
    const contactType = contactTypes.find((t) => t.value === contact.type);
    contacts.value.push({
      ...contact,
      id,
      icon: contactType?.icon || "Globe",
      label: contact.label || contactType?.label,
    });
    await saveContacts();
  };

  const removeContact = async (id) => {
    contacts.value = contacts.value.filter((c) => c.id !== id);
    await saveContacts();
  };

  const updateContact = async (id, updates) => {
    const index = contacts.value.findIndex((c) => c.id === id);
    if (index !== -1) {
      contacts.value[index] = { ...contacts.value[index], ...updates };
      await saveContacts();
    }
  };

  return {
    contacts,
    availableIcons, // <-- вернул
    contactTypes,
    isLoading,
    error,
    loadFromApi,
    addContact,
    removeContact,
    updateContact,
  };
});
