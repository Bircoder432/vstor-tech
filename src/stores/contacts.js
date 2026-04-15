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

  const addContact = (contact) => {
    const id = Math.max(...contacts.value.map((c) => c.id), 0) + 1;
    const contactType = contactTypes.find((t) => t.value === contact.type);
    contacts.value.push({
      ...contact,
      id,
      icon: contactType?.icon || "Globe",
      label: contact.label || contactType?.label,
    });
    saveToStorage();
  };

  const removeContact = (id) => {
    contacts.value = contacts.value.filter((c) => c.id !== id);
    saveToStorage();
  };

  const updateContact = (id, updates) => {
    const index = contacts.value.findIndex((c) => c.id === id);
    if (index !== -1) {
      contacts.value[index] = { ...contacts.value[index], ...updates };
      saveToStorage();
    }
  };

  const saveToStorage = () => {
    localStorage.setItem("portfolio_contacts", JSON.stringify(contacts.value));
  };

  const loadFromStorage = () => {
    const saved = localStorage.getItem("portfolio_contacts");
    if (saved) {
      contacts.value = JSON.parse(saved);
    }
  };

  return {
    contacts,
    availableIcons,
    contactTypes,
    addContact,
    removeContact,
    updateContact,
    loadFromStorage,
    saveToStorage,
  };
});
