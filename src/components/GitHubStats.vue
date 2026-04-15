<template>
    <div class="github-stats">
        <div class="stats-header">
            <span class="label">{{ $t("home.stats") }}</span>
            <span class="loading" v-if="loading">{{
                $t("github.loading")
            }}</span>
        </div>

        <div v-if="error" class="error">
            <span class="error-icon">!</span>
            {{ $t("github.error") }}
        </div>

        <div v-else-if="stats" class="stats-grid">
            <div class="stat-item">
                <span class="stat-value">{{ stats.public_repos }}</span>
                <span class="stat-label">{{ $t("github.repositories") }}</span>
            </div>
            <div class="stat-item">
                <span class="stat-value">{{ stats.followers }}</span>
                <span class="stat-label">{{ $t("github.followers") }}</span>
            </div>
            <div class="stat-item">
                <span class="stat-value">{{ stats.following }}</span>
                <span class="stat-label">{{ $t("github.following") }}</span>
            </div>
            <div class="stat-item">
                <span class="stat-value">{{
                    formatDate(stats.created_at)
                }}</span>
                <span class="stat-label">{{ $t("github.joined") }}</span>
            </div>
        </div>

        <div class="stats-footer">
            <a
                :href="`https://github.com/${username}`"
                target="_blank"
                rel="noopener"
                class="github-link"
            >
                <Github :size="14" />
                github.com/{{ username }}
            </a>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { Github } from "lucide-vue-next";

const props = defineProps({
    username: {
        type: String,
        required: true,
    },
});

const stats = ref(null);
const loading = ref(true);
const error = ref(null);

const fetchStats = async () => {
    try {
        const response = await fetch(
            `https://api.github.com/users/${props.username}`,
        );
        if (!response.ok) throw new Error("Failed to fetch");
        stats.value = await response.json();
    } catch (err) {
        error.value = err.message;
    } finally {
        loading.value = false;
    }
};

const formatDate = (dateString) => {
    return new Date(dateString).getFullYear();
};

onMounted(fetchStats);
</script>

<style scoped>
.github-stats {
    background: var(--bg-card);
    border: 1px solid var(--border);
    margin-top: 2rem;
}

.stats-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border);
    font-size: 0.85rem;
}

.label {
    color: var(--accent);
    text-transform: lowercase;
}

.loading {
    color: var(--text-secondary);
    animation: pulse 1.5s infinite;
}

@keyframes pulse {
    0%,
    100% {
        opacity: 0.5;
    }
    50% {
        opacity: 1;
    }
}

.stats-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 1px;
    background: var(--border);
}

.stat-item {
    background: var(--bg-card);
    padding: 1.5rem 1rem;
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.stat-value {
    font-size: 1.5rem;
    color: var(--accent);
    font-weight: bold;
}

.stat-label {
    font-size: 0.75rem;
    color: var(--text-secondary);
    text-transform: lowercase;
}

.stats-footer {
    padding: 0.75rem 1rem;
    border-top: 1px solid var(--border);
    background: var(--bg-secondary);
}

.github-link {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--text-secondary);
    text-decoration: none;
    font-size: 0.8rem;
    transition: color 0.2s;
}

.github-link:hover {
    color: var(--accent);
}

.error {
    padding: 1rem;
    color: var(--error);
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.error-icon {
    width: 20px;
    height: 20px;
    border: 1px solid var(--error);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
}

@media (max-width: 600px) {
    .stats-grid {
        grid-template-columns: repeat(2, 1fr);
    }
}
</style>
