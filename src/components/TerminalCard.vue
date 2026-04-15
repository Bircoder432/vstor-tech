<template>
    <component
        :is="link ? 'a' : 'div'"
        :href="link || undefined"
        :target="link ? '_blank' : undefined"
        :rel="link ? 'noopener' : undefined"
        class="terminal-card"
        :class="{ interactive: clickable || link }"
        @click="handleClick"
    >
        <div class="card-header">
            <span class="card-icon" v-if="icon">
                <component :is="icon" :size="20" />
            </span>
            <h3 class="card-title">{{ title }}</h3>
            <span class="card-status" v-if="status" :class="status">{{
                status
            }}</span>
            <ExternalLink v-if="link" :size="14" class="link-indicator" />
        </div>
        <div class="card-body">
            <p class="card-description">{{ displayDescription }}</p>
            <div class="card-meta" v-if="$slots.meta">
                <slot name="meta" />
            </div>
        </div>
        <div class="card-footer" v-if="$slots.footer">
            <slot name="footer" />
        </div>
    </component>
</template>

<script setup>
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { ExternalLink } from "lucide-vue-next";

const props = defineProps({
    title: String,
    description: [String, Object], // Может быть строкой или объектом с переводами
    icon: Object,
    status: String,
    clickable: Boolean,
    link: String,
});

const { locale } = useI18n();

const emit = defineEmits(["click"]);

const displayDescription = computed(() => {
    if (typeof props.description === "object" && props.description !== null) {
        return props.description[locale.value] || props.description.en || "";
    }
    return props.description || "";
});

const handleClick = () => {
    if (!props.link && props.clickable) emit("click");
};
</script>

<style scoped>
.terminal-card {
    display: block;
    background: var(--bg-card);
    border: 1px solid var(--border);
    transition: all 0.2s;
    text-decoration: none;
    color: inherit;
}

.terminal-card.interactive {
    cursor: pointer;
}

.terminal-card.interactive:hover {
    border-color: var(--accent);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 255, 0, 0.1);
}

.card-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    border-bottom: 1px solid var(--border);
    background: var(--bg-secondary);
}

.card-icon {
    color: var(--accent);
    display: flex;
    align-items: center;
}

.card-title {
    flex: 1;
    font-size: 1rem;
    font-weight: normal;
    color: var(--text-primary);
    text-transform: lowercase;
}

.card-status {
    font-size: 0.7rem;
    padding: 0.125rem 0.5rem;
    border: 1px solid var(--border);
    text-transform: uppercase;
}

.card-status.active {
    border-color: var(--accent);
    color: var(--accent);
}

.link-indicator {
    color: var(--accent);
    opacity: 0.7;
}

.card-body {
    padding: 1rem;
}

.card-description {
    color: var(--text-secondary);
    font-size: 0.9rem;
    line-height: 1.5;
}

.card-meta {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px dashed var(--border);
    font-size: 0.8rem;
    color: var(--text-secondary);
}

.card-footer {
    padding: 0.75rem 1rem;
    border-top: 1px solid var(--border);
    background: var(--bg-secondary);
    font-size: 0.8rem;
}
</style>
