<template>
    <div class="services-view">
        <div class="section-header">
            <span class="section-prompt">~/{{ $t("services.title") }}</span>
            <span class="section-line"></span>
        </div>

        <div class="services-grid">
            <TerminalCard
                v-for="service in servicesStore.services"
                :key="service.id"
                :title="service.title"
                :description="service.description"
                :icon="servicesStore.availableIcons[service.icon]"
                :status="service.status"
                :link="service.link"
                :clickable="!!service.link"
            >
                <template #meta>
                    <span class="service-id"
                        >ID: {{ service.id.toString().padStart(4, "0") }}</span
                    >
                    <span v-if="service.link" class="service-link-indicator">
                        → {{ $t("services.external_link") }}
                    </span>
                </template>
            </TerminalCard>
        </div>

        <div class="services-count">
            <span class="count-prompt">$</span>
            <span class="count-text">
                {{ $t("services.found") }} {{ servicesStore.services.length }}
                {{ $t("services.active_services") }}
            </span>
        </div>
    </div>
</template>

<script setup>
import { onMounted } from "vue";
import TerminalCard from "../components/TerminalCard.vue";
import { useServicesStore } from "../stores/services";

const servicesStore = useServicesStore();

onMounted(() => {
    servicesStore.loadFromStorage();
});
</script>

<style scoped>
.services-view {
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

.services-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;
}

.service-id {
    color: var(--text-secondary);
}

.service-link-indicator {
    color: var(--accent);
    font-size: 0.75rem;
}

.services-count {
    margin-top: 2rem;
    padding: 1rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    font-size: 0.9rem;
}

.count-prompt {
    color: var(--accent);
    margin-right: 0.5rem;
}

.count-text {
    color: var(--text-secondary);
}
</style>
