<template>
    <div class="icon-picker">
        <div class="picker-tabs">
            <button
                class="tab-btn"
                :class="{ active: activeTab === 'lucide' }"
                @click="activeTab = 'lucide'"
            >
                Lucide
            </button>
            <button
                class="tab-btn"
                :class="{ active: activeTab === 'external' }"
                @click="activeTab = 'external'"
            >
                External
            </button>
            <button
                class="tab-btn"
                :class="{ active: activeTab === 'upload' }"
                @click="activeTab = 'upload'"
            >
                Upload
            </button>
            <button
                class="tab-btn"
                :class="{ active: activeTab === 'url' }"
                @click="activeTab = 'url'"
            >
                URL
            </button>
        </div>

        <!-- Lucide Icons -->
        <div v-if="activeTab === 'lucide'" class="picker-content">
            <input
                v-model="searchQuery"
                type="text"
                class="search-input"
                placeholder="Search icons..."
            />
            <div class="icons-grid">
                <button
                    v-for="icon in filteredLucideIcons"
                    :key="icon.name"
                    class="icon-btn"
                    :class="{ active: isSelected(icon) }"
                    @click="selectIcon(icon)"
                >
                    <component :is="icon.component" :size="20" />
                    <span class="icon-name">{{ icon.name }}</span>
                </button>
            </div>
        </div>

        <!-- External Sources -->
        <div v-if="activeTab === 'external'" class="picker-content">
            <select v-model="selectedSource" class="form-input source-select">
                <option
                    v-for="source in iconSources.externalSources"
                    :key="source.id"
                    :value="source.id"
                >
                    {{ source.name }}
                </option>
            </select>
            <input
                v-model="externalIconName"
                type="text"
                class="form-input"
                placeholder="Icon name (e.g., javascript, docker, linux)"
            />
            <div v-if="externalPreview" class="external-preview">
                <img :src="externalPreview" alt="Preview" class="preview-img" />
                <button class="btn btn-primary" @click="selectExternalIcon">
                    Select
                </button>
            </div>
        </div>

        <!-- File Upload -->
        <div v-if="activeTab === 'upload'" class="picker-content">
            <div
                class="drop-zone"
                :class="{ dragging: isDragging }"
                @dragenter.prevent="isDragging = true"
                @dragleave.prevent="isDragging = false"
                @dragover.prevent
                @drop.prevent="handleDrop"
                @click="$refs.fileInput.click()"
            >
                <input
                    ref="fileInput"
                    type="file"
                    accept=".svg,.png,.jpg,.jpeg,.webp"
                    class="hidden"
                    @change="handleFileSelect"
                />
                <div class="drop-content">
                    <Upload :size="32" />
                    <p>Drop SVG/PNG here or click to browse</p>
                    <span class="hint">Max 100KB</span>
                </div>
            </div>
            <div v-if="uploadPreview" class="upload-preview">
                <img
                    :src="uploadPreview"
                    alt="Upload preview"
                    class="preview-img"
                />
                <button class="btn btn-primary" @click="selectUploadedIcon">
                    Select
                </button>
            </div>
        </div>

        <!-- Direct URL -->
        <div v-if="activeTab === 'url'" class="picker-content">
            <input
                v-model="iconUrl"
                type="url"
                class="form-input"
                placeholder="https://example.com/icon.svg"
            />
            <div v-if="urlPreview" class="url-preview">
                <img
                    :src="urlPreview"
                    alt="URL preview"
                    class="preview-img"
                    @error="urlError = true"
                />
                <p v-if="urlError" class="error-text">Failed to load image</p>
                <button v-else class="btn btn-primary" @click="selectUrlIcon">
                    Select
                </button>
            </div>
        </div>

        <!-- Selected Icon Display -->
        <div v-if="modelValue" class="selected-display">
            <span class="selected-label">Selected:</span>
            <IconRenderer :icon="modelValue" :size="24" />
            <button class="btn btn-danger btn-small" @click="clearSelection">
                <X :size="14" />
            </button>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, watch } from "vue";
import { Upload, X } from "lucide-vue-next";
import { useIconSourcesStore } from "../stores/iconSources";
import IconRenderer from "./IconRenderer.vue";

const props = defineProps({
    modelValue: {
        type: [Object, String],
        default: null,
    },
});

const emit = defineEmits(["update:modelValue"]);

const iconSources = useIconSourcesStore();

const activeTab = ref("lucide");
const searchQuery = ref("");
const selectedSource = ref(iconSources.externalSources[0]?.id || "");
const externalIconName = ref("");
const iconUrl = ref("");
const isDragging = ref(false);
const uploadPreview = ref(null);
const uploadedFileData = ref(null);
const urlError = ref(false);

// Lucide icons filtering
const filteredLucideIcons = computed(() => {
    const icons = Object.keys(iconSources.builtInLibraries.lucide.icons)
        .filter((name) => name !== "default" && name !== "createLucideIcon")
        .map((name) => ({
            name,
            component: iconSources.builtInLibraries.lucide.icons[name],
            type: "vue-component",
        }));

    if (!searchQuery.value) return icons.slice(0, 48);

    const query = searchQuery.value.toLowerCase();
    return icons
        .filter((icon) => icon.name.toLowerCase().includes(query))
        .slice(0, 48);
});

const externalPreview = computed(() => {
    if (!externalIconName.value || !selectedSource.value) return null;
    const source = iconSources.externalSources.find(
        (s) => s.id === selectedSource.value,
    );
    return source ? source.format(externalIconName.value) : null;
});

const urlPreview = computed(() => {
    return iconUrl.value || null;
});

const isSelected = (icon) => {
    if (!props.modelValue) return false;
    if (typeof props.modelValue === "string") {
        return props.modelValue === icon.name;
    }
    return (
        props.modelValue.name === icon.name &&
        props.modelValue.type === icon.type
    );
};

const selectIcon = (icon) => {
    emit("update:modelValue", {
        type: "vue-component",
        name: icon.name,
        component: icon.component,
    });
};

const selectExternalIcon = () => {
    if (!externalPreview.value) return;
    emit("update:modelValue", {
        type: "external",
        source: selectedSource.value,
        name: externalIconName.value,
        url: externalPreview.value,
    });
};

const handleFileSelect = (e) => {
    const file = e.target.files[0];
    if (file) processFile(file);
};

const handleDrop = (e) => {
    isDragging.value = false;
    const file = e.dataTransfer.files[0];
    if (file) processFile(file);
};

const processFile = async (file) => {
    if (file.size > 100 * 1024) {
        alert("File too large. Max 100KB.");
        return;
    }

    try {
        const reader = new FileReader();
        reader.onload = (e) => {
            uploadPreview.value = e.target.result;
            uploadedFileData.value = {
                name: file.name.replace(/\.[^/.]+$/, ""),
                data: e.target.result,
                type: file.type.includes("svg") ? "svg-url" : "base64",
            };
        };
        reader.readAsDataURL(file);
    } catch (err) {
        console.error("Failed to read file:", err);
    }
};

const selectUploadedIcon = () => {
    if (!uploadedFileData.value) return;
    const id = iconSources.addCustomIcon(uploadedFileData.value.name, {
        type: uploadedFileData.value.type,
        url: uploadedFileData.value.data,
        data: uploadedFileData.value.data,
    });
    iconSources.saveToStorage();
    emit("update:modelValue", {
        type: "custom",
        id: id,
        name: uploadedFileData.value.name,
    });
    uploadPreview.value = null;
    uploadedFileData.value = null;
};

const selectUrlIcon = () => {
    if (!iconUrl.value) return;
    emit("update:modelValue", {
        type: "svg-url",
        url: iconUrl.value,
        name: "custom-url",
    });
};

const clearSelection = () => {
    emit("update:modelValue", null);
    uploadPreview.value = null;
    uploadedFileData.value = null;
    iconUrl.value = "";
    externalIconName.value = "";
};

watch(iconUrl, () => {
    urlError.value = false;
});
</script>

<style scoped>
.icon-picker {
    border: 1px solid var(--border);
    background: var(--bg-card);
}

.picker-tabs {
    display: flex;
    border-bottom: 1px solid var(--border);
}

.tab-btn {
    flex: 1;
    padding: 0.5rem;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-family: inherit;
    font-size: 0.8rem;
    cursor: pointer;
    transition: all 0.2s;
    text-transform: lowercase;
}

.tab-btn:hover,
.tab-btn.active {
    color: var(--accent);
    background: var(--bg-secondary);
}

.picker-content {
    padding: 1rem;
}

.search-input {
    width: 100%;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    color: var(--text-primary);
    padding: 0.5rem;
    font-family: inherit;
    margin-bottom: 1rem;
}

.search-input:focus {
    outline: none;
    border-color: var(--accent);
}

.icons-grid {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 0.5rem;
    max-height: 200px;
    overflow-y: auto;
}

.icon-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    padding: 0.5rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s;
}

.icon-btn:hover,
.icon-btn.active {
    border-color: var(--accent);
    color: var(--accent);
}

.icon-name {
    font-size: 0.6rem;
    text-align: center;
    overflow: hidden;
    text-overflow: ellipsis;
    width: 100%;
}

.source-select {
    margin-bottom: 0.75rem;
}

.form-input {
    width: 100%;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    color: var(--text-primary);
    padding: 0.5rem;
    font-family: inherit;
    margin-bottom: 0.75rem;
}

.form-input:focus {
    outline: none;
    border-color: var(--accent);
}

.drop-zone {
    border: 2px dashed var(--border);
    padding: 2rem;
    text-align: center;
    cursor: pointer;
    transition: all 0.2s;
}

.drop-zone.dragging,
.drop-zone:hover {
    border-color: var(--accent);
    background: var(--bg-secondary);
}

.drop-content {
    color: var(--text-secondary);
}

.drop-content svg {
    margin-bottom: 0.5rem;
}

.hint {
    font-size: 0.75rem;
    opacity: 0.7;
}

.hidden {
    display: none;
}

.preview-img {
    max-width: 48px;
    max-height: 48px;
    margin: 0.5rem 0;
}

.external-preview,
.upload-preview,
.url-preview {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 1rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
}

.error-text {
    color: var(--error);
    font-size: 0.8rem;
}

.btn {
    padding: 0.5rem 1rem;
    font-family: inherit;
    cursor: pointer;
    border: 1px solid;
    text-transform: lowercase;
}

.btn-primary {
    background: var(--accent);
    border-color: var(--accent);
    color: var(--bg-primary);
}

.btn-danger {
    background: transparent;
    border-color: var(--error);
    color: var(--error);
}

.btn-small {
    padding: 0.25rem;
}

.selected-display {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background: var(--bg-secondary);
    border-top: 1px solid var(--border);
}

.selected-label {
    color: var(--text-secondary);
    font-size: 0.8rem;
}

@media (max-width: 600px) {
    .icons-grid {
        grid-template-columns: repeat(4, 1fr);
    }
}
</style>
