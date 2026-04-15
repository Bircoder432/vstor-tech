<template>
    <div class="projects-view">
        <div class="section-header">
            <span class="section-prompt">~/{{ $t("projects.title") }}</span>
            <span class="section-line"></span>
        </div>

        <div class="projects-controls">
            <div class="search-box">
                <span class="search-prompt">$ {{ $t("projects.grep") }}</span>
                <input
                    v-model="searchQuery"
                    type="text"
                    class="search-input"
                    :placeholder="$t('projects.search')"
                />
            </div>
            <div class="sort-controls">
                <button
                    v-for="option in sortOptions"
                    :key="option.value"
                    class="sort-btn"
                    :class="{ active: sortBy === option.value }"
                    @click="sortBy = option.value"
                >
                    {{ option.label }}
                </button>
            </div>
        </div>

        <div v-if="loading" class="loading-state">
            <span class="loading-spinner">◐</span>
            {{ $t("common.loading") }}
        </div>

        <div v-else-if="error" class="error-state">
            <span class="error-icon">✗</span>
            {{ error }}
        </div>

        <div v-else class="projects-grid">
            <RepoCard
                v-for="repo in filteredRepos"
                :key="repo.id"
                :repo="repo"
            />
        </div>

        <div class="projects-footer" v-if="!loading && !error">
            <span class="footer-prompt">$</span>
            <span class="footer-text">
                {{ $t("projects.showing") }} {{ filteredRepos.length }}
                {{ $t("projects.of") }} {{ repos.length }}
                {{ $t("projects.repositories") }}
            </span>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import RepoCard from "../components/RepoCard.vue";
import { useContentStore } from "../stores/content";

const { t } = useI18n();
const contentStore = useContentStore();

const repos = ref([]);
const loading = ref(true);
const error = ref(null);
const searchQuery = ref("");
const sortBy = ref("updated");

const sortOptions = computed(() => [
    { value: "updated", label: t("projects.updated") },
    { value: "stars", label: t("projects.stars") },
    { value: "name", label: t("projects.name") },
]);

const fetchRepos = async () => {
    try {
        const response = await fetch(
            `https://api.github.com/users/${contentStore.githubUsername}/repos?sort=updated&per_page=100`,
        );
        if (!response.ok) throw new Error("Failed to fetch");
        repos.value = await response.json();
    } catch (err) {
        error.value = "Failed to load repositories";
    } finally {
        loading.value = false;
    }
};

const filteredRepos = computed(() => {
    let result = repos.value;

    if (searchQuery.value) {
        const query = searchQuery.value.toLowerCase();
        result = result.filter(
            (repo) =>
                repo.name.toLowerCase().includes(query) ||
                (repo.description &&
                    repo.description.toLowerCase().includes(query)),
        );
    }

    result = [...result].sort((a, b) => {
        switch (sortBy.value) {
            case "stars":
                return b.stargazers_count - a.stargazers_count;
            case "name":
                return a.name.localeCompare(b.name);
            case "updated":
            default:
                return new Date(b.updated_at) - new Date(a.updated_at);
        }
    });

    return result;
});

onMounted(async () => {
    await contentStore.loadFromApi();
    fetchRepos();
});
</script>

<style scoped>
.projects-view {
    max-width: 1000px;
}

.section-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;
}

.section-prompt {
    color: var(--accent);
    font-size: 0.9rem;
    text-transform: lowercase;
}

.section-line {
    flex: 1;
    height: 1px;
    background: var(--border);
}

.projects-controls {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    margin-bottom: 1.5rem;
    padding: 1rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
}

.search-box {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
    min-width: 250px;
}

.search-prompt {
    color: var(--accent);
    font-size: 0.9rem;
}

.search-input {
    flex: 1;
    background: var(--bg-card);
    border: 1px solid var(--border);
    color: var(--text-primary);
    padding: 0.5rem;
    font-family: inherit;
    font-size: 0.9rem;
}

.search-input:focus {
    outline: none;
    border-color: var(--accent);
}

.sort-controls {
    display: flex;
    gap: 0.5rem;
}

.sort-btn {
    background: var(--bg-card);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    padding: 0.5rem 1rem;
    font-family: inherit;
    font-size: 0.8rem;
    cursor: pointer;
    transition: all 0.2s;
    text-transform: lowercase;
}

.sort-btn:hover,
.sort-btn.active {
    border-color: var(--accent);
    color: var(--accent);
}

.projects-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 1rem;
}

.loading-state,
.error-state {
    padding: 3rem;
    text-align: center;
    color: var(--text-secondary);
    border: 1px dashed var(--border);
}

.loading-spinner {
    display: inline-block;
    animation: spin 1s linear infinite;
    margin-right: 0.5rem;
    color: var(--accent);
}

@keyframes spin {
    from {
        transform: rotate(0deg);
    }
    to {
        transform: rotate(360deg);
    }
}

.error-state {
    color: var(--error);
    border-color: var(--error);
}

.projects-footer {
    margin-top: 1.5rem;
    padding: 1rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    font-size: 0.9rem;
}

.footer-prompt {
    color: var(--accent);
    margin-right: 0.5rem;
}

.footer-text {
    color: var(--text-secondary);
}

@media (max-width: 768px) {
    .projects-grid {
        grid-template-columns: 1fr;
    }
}
</style>
