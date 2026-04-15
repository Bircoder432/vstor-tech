<template>
    <div class="markdown-content" v-html="renderedContent"></div>
</template>

<script setup>
import { computed } from "vue";
import { marked } from "marked";

const props = defineProps({
    content: {
        type: String,
        default: "",
    },
});

const renderedContent = computed(() => {
    return marked(props.content);
});
</script>

<style scoped>
.markdown-content :deep(h1) {
    color: var(--accent);
    font-size: 1.5rem;
    margin-bottom: 1rem;
    border-bottom: 1px solid var(--border);
    padding-bottom: 0.5rem;
}

.markdown-content :deep(h2) {
    color: var(--text-primary);
    font-size: 1.2rem;
    margin-top: 1.5rem;
    margin-bottom: 0.75rem;
}

.markdown-content :deep(p) {
    margin-bottom: 1rem;
    color: var(--text-secondary);
}

.markdown-content :deep(ul) {
    margin-left: 1.5rem;
    margin-bottom: 1rem;
}

.markdown-content :deep(li) {
    color: var(--text-secondary);
    margin-bottom: 0.25rem;
}

.markdown-content :deep(code) {
    background: var(--bg-card);
    color: var(--accent);
    padding: 0.125rem 0.375rem;
    font-family: inherit;
}

.markdown-content :deep(pre) {
    background: var(--bg-card);
    border: 1px solid var(--border);
    padding: 1rem;
    overflow-x: auto;
    margin-bottom: 1rem;
}

.markdown-content :deep(pre code) {
    background: transparent;
    padding: 0;
}

.markdown-content :deep(a) {
    color: var(--accent);
    text-decoration: none;
}

.markdown-content :deep(a:hover) {
    text-decoration: underline;
}

.markdown-content :deep(blockquote) {
    border-left: 2px solid var(--accent);
    padding-left: 1rem;
    margin-left: 0;
    color: var(--text-secondary);
    font-style: italic;
}
</style>
