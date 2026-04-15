import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],

  // Пробрасываем env в билд
  define: {
    __API_URL__: JSON.stringify(
      process.env.VITE_API_URL || "http://localhost:3001/api",
    ),
  },

  // Или используем стандартный механизм Vite (рекомендуется)
  envPrefix: "VITE_",

  server: {
    port: 5173,
    proxy: {
      // Для разработки — проксируем API
      "/api": {
        target: process.env.VITE_API_URL || "http://localhost:3001",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, "/api"),
      },
    },
  },

  build: {
    // Копируем .env файлы в dist для проверки (опционально)
    copyPublicDir: true,
  },
});
