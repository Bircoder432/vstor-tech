<template>
    <nav class="terminal-nav">
        <router-link
            v-for="item in navItems"
            :key="item.path"
            :to="item.path"
            class="nav-link"
            :class="{ active: $route.path === item.path }"
        >
            <span class="bracket">[</span>
            {{ $t(item.labelKey) }}
            <span class="bracket">]</span>
        </router-link>
        <div class="nav-spacer"></div>
        <LanguageSwitcher />
        <span class="timestamp">{{ currentTime }}</span>
    </nav>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed } from "vue";
import { useI18n } from "vue-i18n";
import LanguageSwitcher from "./LanguageSwitcher.vue";

const { t } = useI18n();

// Только основные разделы (contacts теперь на главной)
const navItems = computed(() => [
    { path: "/", labelKey: "nav.home" },
    { path: "/services", labelKey: "nav.services" },
    { path: "/projects", labelKey: "nav.projects" },
]);

const currentTime = ref("");
let interval;

const updateTime = () => {
    const now = new Date();
    currentTime.value = now.toISOString().replace("T", " ").slice(0, 19);
};

onMounted(() => {
    updateTime();
    interval = setInterval(updateTime, 1000);
});

onUnmounted(() => {
    clearInterval(interval);
});
</script>

<style scoped>
.terminal-nav {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: var(--bg-primary);
    border-bottom: 1px solid var(--border);
    font-size: 0.85rem;
}

.nav-link {
    color: var(--text-secondary);
    text-decoration: none;
    padding: 0.25rem 0.5rem;
    transition: all 0.2s;
    text-transform: lowercase;
}

.nav-link:hover,
.nav-link.active {
    color: var(--accent);
    background: var(--bg-card);
}

.nav-link.active .bracket {
    color: var(--accent);
}

.bracket {
    color: var(--border);
    transition: color 0.2s;
}

.nav-spacer {
    flex: 1;
}

.timestamp {
    color: var(--text-secondary);
    font-size: 0.75rem;
    margin-left: 1rem;
}
</style>
