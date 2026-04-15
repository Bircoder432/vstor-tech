import { createI18n } from "vue-i18n";

// Определение доступных локалей
export const SUPPORT_LOCALES = ["en", "ru"];

// Определение сообщений
const messages = {
  en: {
    nav: {
      home: "home",
      services: "services",
      projects: "projects",
    },
    home: {
      title: "home",
      about_me: "about_me",
      stats: "github_stats",
      connect: "connect_with_me",
    },
    services: {
      title: "services",
      found: "found",
      active_services: "active services",
      external_link: "external link",
    },
    projects: {
      title: "projects",
      search: "search repositories...",
      grep: "grep",
      showing: "showing",
      of: "of",
      repositories: "repositories",
      updated: "updated",
      stars: "stars",
      name: "name",
    },
    contacts: {
      email: "email",
      phone: "phone",
      location: "location",
      social: "social",
    },
    github: {
      loading: "fetching...",
      repositories: "repositories",
      followers: "followers",
      following: "following",
      joined: "joined",
      error: "Failed to load GitHub stats",
    },
    admin: {
      title: "admin",
      login: "login",
      password: "password",
      authenticate: "authenticate",
      logout: "logout",
      panel: "panel",
      content: "content",
      services: "services",
      github_username: "github username",
      home_content_en: "home content english (markdown)",
      home_content_ru: "home content russian (markdown)",
      save: "save changes",
      reset: "reset",
      add_service: "add new service",
      title_label: "title",
      description: "description",
      description_ru: "description (russian)",
      description_en: "description (english)",
      link: "link (optional)",
      icon: "icon",
      current_services: "current services",
      no_services: "No services configured",
      contacts_config: "contacts",
      add_contact: "add contact",
      contact_type: "type",
      contact_value: "value",
      contact_label: "label",
      current_contacts: "current contacts",
      no_contacts: "No contacts configured",
    },
    common: {
      loading: "loading...",
      error: "error",
      access_denied: "Access denied",
      invalid_ip: "Invalid IP address",
    },
  },
  ru: {
    nav: {
      home: "главная",
      services: "услуги",
      projects: "проекты",
    },
    home: {
      title: "главная",
      about_me: "обо_мне",
      stats: "статистика_github",
      connect: "связаться_со_мной",
    },
    services: {
      title: "услуги",
      found: "найдено",
      active_services: "активных услуг",
      external_link: "внешняя ссылка",
    },
    projects: {
      title: "проекты",
      search: "поиск репозиториев...",
      grep: "поиск",
      showing: "показано",
      of: "из",
      repositories: "репозиториев",
      updated: "обновлено",
      stars: "звёзды",
      name: "имя",
    },
    contacts: {
      email: "почта",
      phone: "телефон",
      location: "локация",
      social: "соцсети",
    },
    github: {
      loading: "загрузка...",
      repositories: "репозиториев",
      followers: "подписчиков",
      following: "подписок",
      joined: "регистрация",
      error: "Не удалось загрузить статистику GitHub",
    },
    admin: {
      title: "админ",
      login: "вход",
      password: "пароль",
      authenticate: "войти",
      logout: "выйти",
      panel: "панель",
      content: "контент",
      services: "услуги",
      github_username: "github пользователь",
      home_content_en: "контент главной английский (markdown)",
      home_content_ru: "контент главной русский (markdown)",
      save: "сохранить",
      reset: "сбросить",
      add_service: "добавить услугу",
      title_label: "название",
      description: "описание",
      description_ru: "описание (русский)",
      description_en: "описание (английский)",
      link: "ссылка (опционально)",
      icon: "иконка",
      current_services: "текущие услуги",
      no_services: "Услуги не настроены",
      contacts_config: "контакты",
      add_contact: "добавить контакт",
      contact_type: "тип",
      contact_value: "значение",
      contact_label: "подпись",
      current_contacts: "текущие контакты",
      no_contacts: "Контакты не настроены",
    },
    common: {
      loading: "загрузка...",
      error: "ошибка",
      access_denied: "Доступ запрещён",
      invalid_ip: "Неверный IP адрес",
    },
  },
};

// Определение предпочтительного языка пользователя
function getBrowserLocale() {
  const navigatorLocale = navigator.language || navigator.userLanguage;
  if (!navigatorLocale) return "en";

  const locale = navigatorLocale.split("-")[0];
  return SUPPORT_LOCALES.includes(locale) ? locale : "en";
}

// Получение сохранённой локали
function getSavedLocale() {
  const saved = localStorage.getItem("portfolio_locale");
  return saved && SUPPORT_LOCALES.includes(saved) ? saved : null;
}

// Создание i18n instance
export const i18n = createI18n({
  legacy: false,
  locale: getSavedLocale() || getBrowserLocale(),
  fallbackLocale: "en",
  messages,
  globalInjection: true,
});

// Функция для смены языка
export function setLocale(locale) {
  if (!SUPPORT_LOCALES.includes(locale)) return;

  i18n.global.locale.value = locale;
  localStorage.setItem("portfolio_locale", locale);
  document.querySelector("html").setAttribute("lang", locale);
}

export default i18n;
