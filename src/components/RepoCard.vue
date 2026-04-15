<template>
    <div class="repo-card">
        <div class="repo-header">
            <GitBranch :size="16" class="repo-icon" />
            <a
                :href="repo.html_url"
                target="_blank"
                rel="noopener"
                class="repo-name"
            >
                {{ repo.name }}
            </a>
            <span
                class="repo-visibility"
                :class="repo.private ? 'private' : 'public'"
            >
                {{ repo.private ? "private" : "public" }}
            </span>
        </div>

        <p class="repo-description">
            {{ repo.description || "No description available" }}
        </p>

        <div class="repo-meta">
            <span class="meta-item" v-if="repo.language">
                <span
                    class="lang-color"
                    :style="{ background: getLangColor(repo.language) }"
                ></span>
                {{ repo.language }}
            </span>
            <span class="meta-item">
                <Star :size="14" />
                {{ repo.stargazers_count }}
            </span>
            <span class="meta-item">
                <GitFork :size="14" />
                {{ repo.forks_count }}
            </span>
            <span class="meta-item">
                updated: {{ formatDate(repo.updated_at) }}
            </span>
        </div>
    </div>
</template>

<script setup>
import { GitBranch, Star, GitFork } from "lucide-vue-next";

const props = defineProps({
    repo: {
        type: Object,
        required: true,
    },
});

const langColors = {
    JavaScript: "#f1e05a",
    TypeScript: "#2b7489",
    Vue: "#41b883",
    HTML: "#e34c26",
    CSS: "#563d7c",
    Python: "#3572A5",
    Java: "#b07219",
    "C++": "#f34b7d",
    Rust: "#dea584",
    Go: "#00ADD8",
};

const getLangColor = (lang) => langColors[lang] || "#888888";

const formatDate = (dateString) => {
    const date = new Date(dateString);
    return date.toLocaleDateString("en-US", {
        year: "numeric",
        month: "short",
        day: "numeric",
    });
};
</script>

<style scoped>
.repo-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    padding: 1.25rem;
    transition: all 0.2s;
}

.repo-card:hover {
    border-color: var(--accent);
}

.repo-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 0.75rem;
}

.repo-icon {
    color: var(--accent);
}

.repo-name {
    flex: 1;
    color: var(--text-primary);
    text-decoration: none;
    font-weight: normal;
    font-size: 1.1rem;
}

.repo-name:hover {
    color: var(--accent);
    text-decoration: underline;
}

.repo-visibility {
    font-size: 0.7rem;
    padding: 0.125rem 0.5rem;
    border: 1px solid var(--border);
    text-transform: uppercase;
}

.repo-visibility.public {
    border-color: var(--accent);
    color: var(--accent);
}

.repo-description {
    color: var(--text-secondary);
    font-size: 0.9rem;
    margin-bottom: 1rem;
    line-height: 1.4;
}

.repo-meta {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    font-size: 0.8rem;
    color: var(--text-secondary);
}

.meta-item {
    display: flex;
    align-items: center;
    gap: 0.375rem;
}

.lang-color {
    width: 12px;
    height: 12px;
    border-radius: 50%;
}
</style>
