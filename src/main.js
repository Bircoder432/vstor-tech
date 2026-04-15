import { createApp } from "vue";
import { createPinia } from "pinia";
import { createRouter, createWebHistory } from "vue-router";
import App from "./App.vue";
import i18n from "./i18n";

import HomeView from "./views/HomeView.vue";
import ServicesView from "./views/ServicesView.vue";
import ProjectsView from "./views/ProjectsView.vue";
import AdminView from "./views/AdminView.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", component: HomeView },
    { path: "/services", component: ServicesView },
    { path: "/projects", component: ProjectsView },
    { path: "/system-panel", component: AdminView, meta: { hidden: true } },
  ],
});

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.use(i18n);

// Загрузка пользовательских иконок
import { useIconSourcesStore } from "./stores/iconSources";
const iconSources = useIconSourcesStore();
iconSources.loadFromStorage();

app.mount("#app");
