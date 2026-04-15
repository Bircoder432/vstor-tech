<template>
    <div class="skill-card" :class="`level-${skill.level}`">
        <div class="skill-header">
            <div class="skill-icon">
                <IconRenderer :icon="skill.icon" :size="24" />
            </div>
            <div class="skill-info">
                <h3 class="skill-name">{{ skill.name }}</h3>
                <span class="skill-category">{{ categoryLabel }}</span>
            </div>
        </div>
        <div class="skill-level-bar">
            <div
                class="skill-level-fill"
                :style="{ width: levelPercent, background: levelColor }"
            ></div>
        </div>
        <p class="skill-description">{{ displayDescription }}</p>
    </div>
</template>

<script setup>
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import IconRenderer from "./IconRenderer.vue";

const props = defineProps({
    skill: {
        type: Object,
        required: true,
    },
    categoryLabel: {
        type: String,
        default: "",
    },
});

const { locale } = useI18n();

const levelPercent = computed(() => {
    const levels = {
        beginner: "25%",
        intermediate: "50%",
        advanced: "75%",
        expert: "100%",
    };
    return levels[props.skill.level] || "50%";
});

const levelColor = computed(() => {
    const colors = {
        beginner: "#888888",
        intermediate: "#ffaa00",
        advanced: "#00ff00",
        expert: "#00ffff",
    };
    return colors[props.skill.level] || "#888888";
});

const displayDescription = computed(() => {
    if (typeof props.skill.description === "object") {
        return (
            props.skill.description[locale.value] ||
            props.skill.description.en ||
            ""
        );
    }
    return props.skill.description || "";
});
</script>

<style scoped>
.skill-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    padding: 1.25rem;
    transition: all 0.2s;
}

.skill-card:hover {
    border-color: var(--accent);
    transform: translateY(-2px);
}

.skill-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1rem;
}

.skill-icon {
    color: var(--accent);
    display: flex;
    align-items: center;
}

.skill-icon img {
    filter: grayscale(100%) brightness(200%);
}

.skill-info {
    flex: 1;
}

.skill-name {
    font-size: 1rem;
    font-weight: normal;
    color: var(--text-primary);
    margin: 0;
    text-transform: lowercase;
}

.skill-category {
    font-size: 0.7rem;
    color: var(--text-secondary);
    text-transform: uppercase;
}

.skill-level-bar {
    height: 3px;
    background: var(--bg-secondary);
    margin-bottom: 1rem;
    position: relative;
}

.skill-level-fill {
    height: 100%;
    transition: width 0.3s ease;
}

.skill-description {
    font-size: 0.8rem;
    color: var(--text-secondary);
    margin: 0;
    line-height: 1.4;
}

.level-beginner {
    border-left: 2px solid #888888;
}
.level-intermediate {
    border-left: 2px solid #ffaa00;
}
.level-advanced {
    border-left: 2px solid #00ff00;
}
.level-expert {
    border-left: 2px solid #00ffff;
}
</style>
