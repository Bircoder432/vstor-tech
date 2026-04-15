<template>
    <div class="language-switcher">
        <button
            v-for="lang in SUPPORT_LOCALES"
            :key="lang"
            class="lang-btn"
            :class="{ active: currentLocale === lang }"
            @click="switchLanguage(lang)"
        >
            [{{ lang }}]
        </button>
    </div>
</template>

<script setup>
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { setLocale, SUPPORT_LOCALES } from "../i18n";

const { locale } = useI18n();

const currentLocale = computed(() => locale.value);

const switchLanguage = (lang) => {
    setLocale(lang);
};
</script>

<style scoped>
.language-switcher {
    display: flex;
    gap: 0.5rem;
    align-items: center;
}

.lang-btn {
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-secondary);
    padding: 0.25rem 0.5rem;
    font-family: inherit;
    font-size: 0.75rem;
    cursor: pointer;
    transition: all 0.2s;
    text-transform: lowercase;
}

.lang-btn:hover,
.lang-btn.active {
    border-color: var(--accent);
    color: var(--accent);
    background: var(--bg-card);
}
</style>
