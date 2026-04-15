import { defineStore } from "pinia";
import { ref, computed } from "vue";
import * as lucideIcons from "lucide-vue-next";

export const useIconSourcesStore = defineStore("iconSources", () => {
  // Встроенные библиотеки
  const builtInLibraries = {
    lucide: {
      name: "Lucide",
      type: "vue-components",
      icons: lucideIcons,
    },
  };

  // Пользовательские иконки (загруженные)
  const customIcons = ref({});

  // Внешние источники SVG
  const externalSources = ref([
    {
      id: "simple-icons",
      name: "Simple Icons",
      type: "svg-url",
      baseUrl: "https://cdn.simpleicons.org",
      format: (name) =>
        `https://cdn.simpleicons.org/${name.toLowerCase().replace(/\s+/g, "")}`,
    },
    {
      id: "heroicons",
      name: "Heroicons",
      type: "svg-url",
      baseUrl: "https://unpkg.com/heroicons@2.0.13/24/outline",
      format: (name) =>
        `https://unpkg.com/heroicons@2.0.13/24/outline/${name.toLowerCase().replace(/\s+/g, "-")}.svg`,
    },
    {
      id: "svgrepo",
      name: "SVG Repo",
      type: "svg-url",
      baseUrl: "https://www.svgrepo.com",
      format: (name) =>
        `https://www.svgrepo.com/show/509053/${name.toLowerCase().replace(/\s+/g, "-")}.svg`,
    },
  ]);

  // Активный источник для новых иконок
  const activeSource = ref("lucide");

  // Добавить пользовательскую иконку
  const addCustomIcon = (name, source) => {
    const id = `custom-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    customIcons.value[id] = {
      id,
      name,
      ...source,
    };
    return id;
  };

  // Удалить пользовательскую иконку
  const removeCustomIcon = (id) => {
    delete customIcons.value[id];
  };

  // Получить URL иконки
  const getIconUrl = (iconRef) => {
    if (!iconRef) return null;

    // Если это объект с type
    if (typeof iconRef === "object") {
      switch (iconRef.type) {
        case "vue-component":
          return iconRef;

        case "svg-url":
          return iconRef.url;

        case "base64":
          return iconRef.data;

        case "external":
          const source = externalSources.value.find(
            (s) => s.id === iconRef.source,
          );
          if (source) {
            return source.format(iconRef.name);
          }
          return iconRef.url || null;

        default:
          return iconRef.url || null;
      }
    }

    // Если это строка (ID пользовательской иконки)
    if (typeof iconRef === "string" && customIcons.value[iconRef]) {
      return customIcons.value[iconRef];
    }

    return null;
  };

  // Получить Vue компонент (только для Lucide)
  const getVueIcon = (iconRef) => {
    if (typeof iconRef === "object" && iconRef.type === "vue-component") {
      return iconRef.component;
    }

    if (typeof iconRef === "string" && lucideIcons[iconRef]) {
      return lucideIcons[iconRef];
    }

    return null;
  };

  // Проверить, является ли иконка Vue компонентом
  const isVueIcon = (iconRef) => {
    return getVueIcon(iconRef) !== null;
  };

  // Получить все доступные иконки для выбора
  const getAvailableIcons = () => {
    const result = [];

    // Lucide иконки
    Object.keys(lucideIcons).forEach((name) => {
      result.push({
        id: `lucide-${name}`,
        name: name,
        library: "lucide",
        type: "vue-component",
        component: lucideIcons[name],
        preview: name,
      });
    });

    // Пользовательские иконки
    Object.values(customIcons.value).forEach((icon) => {
      result.push({
        id: icon.id,
        name: icon.name,
        library: "custom",
        type: icon.type,
        ...icon,
      });
    });

    return result;
  };

  // Поиск иконок по имени
  const searchIcons = (query) => {
    const all = getAvailableIcons();
    if (!query) return all.slice(0, 50); // Первые 50 по умолчанию

    const lowerQuery = query.toLowerCase();
    return all
      .filter((icon) => icon.name.toLowerCase().includes(lowerQuery))
      .slice(0, 50);
  };

  // Загрузить иконку из файла (FileReader)
  const loadIconFromFile = (file) => {
    return new Promise((resolve, reject) => {
      if (
        !file ||
        (!file.type.includes("svg") &&
          !file.type.includes("png") &&
          !file.type.includes("image"))
      ) {
        reject(new Error("Only SVG and PNG files are supported"));
        return;
      }

      const reader = new FileReader();
      reader.onload = (e) => {
        const id = addCustomIcon(file.name.replace(/\.[^/.]+$/, ""), {
          type: file.type.includes("svg") ? "svg-url" : "base64",
          url: e.target.result,
          data: e.target.result,
          mimeType: file.type,
        });
        resolve(id);
      };
      reader.onerror = reject;
      reader.readAsDataURL(file);
    });
  };

  // Сохранить/загрузить из localStorage
  const saveToStorage = () => {
    localStorage.setItem(
      "portfolio_custom_icons",
      JSON.stringify(customIcons.value),
    );
  };

  const loadFromStorage = () => {
    const saved = localStorage.getItem("portfolio_custom_icons");
    if (saved) {
      try {
        customIcons.value = JSON.parse(saved);
      } catch (e) {
        console.error("Failed to load custom icons:", e);
      }
    }
  };

  return {
    builtInLibraries,
    customIcons,
    externalSources,
    activeSource,
    addCustomIcon,
    removeCustomIcon,
    getIconUrl,
    getVueIcon,
    isVueIcon,
    getAvailableIcons,
    searchIcons,
    loadIconFromFile,
    saveToStorage,
    loadFromStorage,
  };
});
