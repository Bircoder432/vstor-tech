<template>
    <component
        :is="renderComponent"
        v-if="isVueComponent"
        :size="size"
        :class="iconClass"
    />
    <img
        v-else-if="isImage"
        :src="imageSrc"
        :alt="alt"
        :width="size"
        :height="size"
        :class="iconClass"
        @error="handleError"
    />
    <div
        v-else
        class="icon-fallback"
        :style="{ width: size + 'px', height: size + 'px' }"
    >
        <Code :size="size * 0.6" />
    </div>
</template>

<script setup>
import { computed, ref } from "vue";
import { Code } from "lucide-vue-next";
import { useIconSourcesStore } from "../stores/iconSources";

const props = defineProps({
    icon: {
        type: [Object, String],
        required: true,
    },
    size: {
        type: Number,
        default: 24,
    },
    alt: {
        type: String,
        default: "icon",
    },
});

const iconSources = useIconSourcesStore();
const loadError = ref(false);

const isVueComponent = computed(() => {
    if (loadError.value) return false;

    if (typeof props.icon === "string") {
        return !!iconSources.getVueIcon(props.icon);
    }

    if (typeof props.icon === "object") {
        // Если это строка (имя Lucide иконки)
        if (props.icon.component && typeof props.icon.component === "object") {
            return true;
        }
        if (props.icon.type === "vue-component" || !props.icon.type) {
            // Проверяем, есть ли компонент в Lucide
            if (props.icon.name && iconSources.getVueIcon(props.icon.name)) {
                return true;
            }
        }
    }

    return false;
});

const isImage = computed(() => {
    if (loadError.value) return false;

    if (typeof props.icon === "object") {
        return (
            props.icon.type === "svg-url" ||
            props.icon.type === "base64" ||
            props.icon.type === "external" ||
            props.icon.type === "custom" ||
            !!props.icon.url
        );
    }

    return false;
});

const renderComponent = computed(() => {
    if (typeof props.icon === "string") {
        return iconSources.getVueIcon(props.icon);
    }

    if (props.icon.component) {
        return props.icon.component;
    }

    if (props.icon.name) {
        return iconSources.getVueIcon(props.icon.name);
    }

    return null;
});

const imageSrc = computed(() => {
    if (typeof props.icon === "object") {
        return (
            props.icon.url ||
            props.icon.data ||
            iconSources.getIconUrl(props.icon)
        );
    }
    return null;
});

const iconClass = computed(() => {
    return "icon-rendered";
});

const handleError = () => {
    loadError.value = true;
};
</script>

<style scoped>
.icon-rendered {
    display: inline-flex;
    align-items: center;
    justify-content: center;
}

.icon-fallback {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
    background: var(--bg-secondary);
    border: 1px solid var(--border);
}

img.icon-rendered {
    object-fit: contain;
    filter: grayscale(100%) brightness(200%);
}
</style>
