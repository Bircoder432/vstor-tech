<!-- src/views/AdminView.vue -->
<template>
    <div class="admin-view">
        <!-- Loading -->
        <div v-if="authStore.isLoading" class="loading-section">
            <span class="loading-spinner">◐</span>
            checking...
        </div>

        <!-- IP Denied -->
        <div v-else-if="!authStore.ipAllowed" class="access-denied">
            <div class="section-header">
                <span class="section-prompt">~/admin/error</span>
                <span class="section-line"></span>
            </div>
            <div class="denied-content">
                <span class="denied-icon">✗</span>
                <h2>{{ $t("common.access_denied") }}</h2>
                <p>Your IP: {{ authStore.currentIP }}</p>
                <p class="hint">This IP is not in the allowed list.</p>
            </div>
        </div>

        <!-- Login -->
        <div v-else-if="!authStore.isAuthenticated" class="login-section">
            <div class="section-header">
                <span class="section-prompt">~/admin/login</span>
                <span class="section-line"></span>
            </div>

            <div class="login-form">
                <div class="ip-info">
                    <span class="ip-label">IP:</span>
                    <span class="ip-value">{{ authStore.currentIP }}</span>
                    <span class="ip-status">(allowed)</span>
                </div>

                <div class="form-group">
                    <label class="form-label"
                        >{{ $t("admin.password") }}:</label
                    >
                    <input
                        v-model="loginPassword"
                        type="password"
                        class="form-input"
                        placeholder="********"
                        @keyup.enter="handleLogin"
                    />
                </div>

                <div v-if="authStore.error" class="login-error">
                    <span class="error-icon">✗</span>
                    {{ authStore.error }}
                </div>

                <button
                    class="btn btn-primary"
                    @click="handleLogin"
                    :disabled="authStore.isLoading"
                >
                    <Lock :size="16" />
                    {{
                        authStore.isLoading ? "..." : $t("admin.authenticate")
                    }}()
                </button>
            </div>
        </div>

        <!-- Admin Panel -->
        <template v-else>
            <div class="admin-header">
                <div class="section-header">
                    <span class="section-prompt">~/admin/panel</span>
                    <span class="section-line"></span>
                </div>
                <button
                    class="btn btn-secondary logout-btn"
                    @click="handleLogout"
                >
                    <LogOut :size="16" />
                    {{ $t("admin.logout") }}()
                </button>
            </div>

            <div class="admin-tabs">
                <button
                    v-for="tab in tabs"
                    :key="tab.id"
                    class="tab-btn"
                    :class="{ active: activeTab === tab.id }"
                    @click="activeTab = tab.id"
                >
                    {{ tab.label }}
                </button>
            </div>

            <!-- Content Tab -->
            <div v-if="activeTab === 'content'" class="admin-section">
                <div class="form-group">
                    <label class="form-label"
                        >{{ $t("admin.github_username") }}:</label
                    >
                    <input
                        v-model="githubUsername"
                        type="text"
                        class="form-input"
                    />
                </div>

                <div class="content-tabs">
                    <button
                        class="content-tab-btn"
                        :class="{ active: contentLocale === 'en' }"
                        @click="contentLocale = 'en'"
                    >
                        [EN]
                    </button>
                    <button
                        class="content-tab-btn"
                        :class="{ active: contentLocale === 'ru' }"
                        @click="contentLocale = 'ru'"
                    >
                        [RU]
                    </button>
                </div>

                <div class="form-group">
                    <label class="form-label">
                        {{
                            contentLocale === "en"
                                ? $t("admin.home_content_en")
                                : $t("admin.home_content_ru")
                        }}:
                    </label>
                    <textarea
                        v-model="homeContent[contentLocale]"
                        class="form-textarea"
                        rows="15"
                    ></textarea>
                </div>

                <div class="form-actions">
                    <button
                        class="btn btn-primary"
                        @click="saveContent"
                        :disabled="contentStore.isLoading"
                    >
                        <Save :size="16" />
                        {{
                            contentStore.isLoading ? "..." : $t("admin.save")
                        }}()
                    </button>
                    <button class="btn btn-secondary" @click="resetContent">
                        <RotateCcw :size="16" />
                        {{ $t("admin.reset") }}()
                    </button>
                </div>
            </div>

            <!-- Skills Tab -->
            <div v-if="activeTab === 'skills'" class="admin-section">
                <div class="skills-admin">
                    <div class="add-skill-form">
                        <h3 class="form-title">add_skill()</h3>

                        <div class="form-group">
                            <label class="form-label">name:</label>
                            <input
                                v-model="newSkill.name"
                                type="text"
                                class="form-input"
                            />
                        </div>

                        <div class="form-group">
                            <label class="form-label">category:</label>
                            <select
                                v-model="newSkill.category"
                                class="form-input"
                            >
                                <option
                                    v-for="cat in skillsStore.skillCategories"
                                    :key="cat.value"
                                    :value="cat.value"
                                >
                                    {{ cat.label }}
                                </option>
                            </select>
                        </div>

                        <div class="form-group">
                            <label class="form-label">level:</label>
                            <select v-model="newSkill.level" class="form-input">
                                <option value="beginner">beginner</option>
                                <option value="intermediate">
                                    intermediate
                                </option>
                                <option value="advanced">advanced</option>
                                <option value="expert">expert</option>
                            </select>
                        </div>

                        <div class="form-group">
                            <label class="form-label">description (en):</label>
                            <input
                                v-model="newSkill.description.en"
                                type="text"
                                class="form-input"
                            />
                        </div>

                        <div class="form-group">
                            <label class="form-label">description (ru):</label>
                            <input
                                v-model="newSkill.description.ru"
                                type="text"
                                class="form-input"
                            />
                        </div>

                        <button class="btn btn-primary" @click="addSkill">
                            <Plus :size="16" />
                            add_skill()
                        </button>
                    </div>

                    <div class="skills-list">
                        <h3 class="form-title">current_skills[]</h3>
                        <div
                            v-for="skill in skillsStore.skills"
                            :key="skill.id"
                            class="skill-item"
                        >
                            <div class="skill-info">
                                <span class="skill-name">{{ skill.name }}</span>
                                <span class="skill-meta"
                                    >{{ skill.category }} •
                                    {{ skill.level }}</span
                                >
                            </div>
                            <button
                                class="btn btn-danger"
                                @click="removeSkill(skill.id)"
                            >
                                <Trash2 :size="14" />
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Services Tab -->
            <div v-if="activeTab === 'services'" class="admin-section">
                <div class="services-admin">
                    <div class="add-service-form">
                        <h3 class="form-title">
                            {{ $t("admin.add_service") }}()
                        </h3>

                        <div class="form-group">
                            <label class="form-label"
                                >{{ $t("admin.title_label") }}:</label
                            >
                            <input
                                v-model="newService.title"
                                type="text"
                                class="form-input"
                            />
                        </div>

                        <div class="form-group">
                            <label class="form-label"
                                >{{ $t("admin.description_en") }}:</label
                            >
                            <textarea
                                v-model="newService.description.en"
                                class="form-textarea"
                                rows="2"
                            ></textarea>
                        </div>

                        <div class="form-group">
                            <label class="form-label"
                                >{{ $t("admin.description_ru") }}:</label
                            >
                            <textarea
                                v-model="newService.description.ru"
                                class="form-textarea"
                                rows="2"
                            ></textarea>
                        </div>

                        <div class="form-group">
                            <label class="form-label"
                                >{{ $t("admin.link") }}:</label
                            >
                            <input
                                v-model="newService.link"
                                type="url"
                                class="form-input"
                            />
                        </div>

                        <button class="btn btn-primary" @click="addService">
                            <Plus :size="16" />
                            {{ $t("admin.add_service") }}()
                        </button>
                    </div>

                    <div class="services-list">
                        <h3 class="form-title">
                            {{ $t("admin.current_services") }}[]
                        </h3>
                        <div
                            v-for="service in servicesStore.services"
                            :key="service.id"
                            class="service-item"
                        >
                            <div class="service-info">
                                <span class="service-name">{{
                                    service.title
                                }}</span>
                                <span
                                    v-if="service.link"
                                    class="service-link"
                                    >{{ service.link }}</span
                                >
                            </div>
                            <button
                                class="btn btn-danger"
                                @click="removeService(service.id)"
                            >
                                <Trash2 :size="14" />
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Contacts Tab -->
            <div v-if="activeTab === 'contacts'" class="admin-section">
                <div class="contacts-admin">
                    <div class="add-contact-form">
                        <h3 class="form-title">
                            {{ $t("admin.add_contact") }}()
                        </h3>

                        <div class="form-group">
                            <label class="form-label"
                                >{{ $t("admin.contact_type") }}:</label
                            >
                            <select
                                v-model="newContact.type"
                                class="form-input"
                            >
                                <option
                                    v-for="type in contactsStore.contactTypes"
                                    :key="type.value"
                                    :value="type.value"
                                >
                                    {{ type.label }}
                                </option>
                            </select>
                        </div>

                        <div class="form-group">
                            <label class="form-label"
                                >{{ $t("admin.contact_value") }}:</label
                            >
                            <input
                                v-model="newContact.value"
                                type="text"
                                class="form-input"
                            />
                        </div>

                        <div class="form-group">
                            <label class="form-label"
                                >{{ $t("admin.contact_label") }}:</label
                            >
                            <input
                                v-model="newContact.label"
                                type="text"
                                class="form-input"
                            />
                        </div>

                        <div class="form-group">
                            <label class="form-label"
                                >{{ $t("admin.link") }}:</label
                            >
                            <input
                                v-model="newContact.link"
                                type="url"
                                class="form-input"
                            />
                            <span class="form-hint"
                                >Optional - for social media links</span
                            >
                        </div>

                        <button class="btn btn-primary" @click="addContact">
                            <Plus :size="16" />
                            {{ $t("admin.add_contact") }}()
                        </button>
                    </div>

                    <div class="contacts-list">
                        <h3 class="form-title">
                            {{ $t("admin.current_contacts") }}[]
                        </h3>
                        <div
                            v-for="contact in contactsStore.contacts"
                            :key="contact.id"
                            class="contact-item"
                        >
                            <div class="contact-info">
                                <span class="contact-name">{{
                                    contact.label
                                }}</span>
                                <span class="contact-value-small">{{
                                    contact.value
                                }}</span>
                            </div>
                            <button
                                class="btn btn-danger"
                                @click="removeContact(contact.id)"
                            >
                                <Trash2 :size="14" />
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <div v-if="saved" class="save-notification">
                <Check :size="16" />
                saved successfully
            </div>
        </template>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import {
    Lock,
    LogOut,
    Save,
    RotateCcw,
    Plus,
    Trash2,
    Check,
} from "lucide-vue-next";
import { useContentStore } from "../stores/content";
import { useServicesStore } from "../stores/services";
import { useContactsStore } from "../stores/contacts";
import { useSkillsStore } from "../stores/skills";
import { useAuthStore } from "../stores/auth";

const { t } = useI18n();
const contentStore = useContentStore();
const servicesStore = useServicesStore();
const contactsStore = useContactsStore();
const skillsStore = useSkillsStore();
const authStore = useAuthStore();

const activeTab = ref("content");
const contentLocale = ref("en");
const saved = ref(false);
const loginPassword = ref("");

const tabs = [
    { id: "content", label: "content" },
    { id: "skills", label: "skills" },
    { id: "services", label: "services" },
    { id: "contacts", label: "contacts" },
];

const homeContent = ref({ en: "", ru: "" });
const githubUsername = ref("");

const newSkill = ref({
    name: "",
    category: "language",
    level: "intermediate",
    description: { en: "", ru: "" },
});

const newService = ref({
    title: "",
    description: { en: "", ru: "" },
    link: "",
});

const newContact = ref({
    type: "email",
    value: "",
    label: "",
    link: "",
});

onMounted(async () => {
    await authStore.init();
    if (authStore.isAuthenticated) {
        await loadData();
    }
});

const loadData = async () => {
    await Promise.all([
        contentStore.loadFromApi(),
        servicesStore.loadFromApi(),
        contactsStore.loadFromApi(),
        skillsStore.loadFromApi(),
    ]);
    homeContent.value = { ...contentStore.homeContent };
    githubUsername.value = contentStore.githubUsername;
};

const handleLogin = async () => {
    const result = await authStore.login(loginPassword.value);
    if (result.success) {
        loginPassword.value = "";
        await loadData();
    }
};

const handleLogout = async () => {
    await authStore.logout();
};

const saveContent = async () => {
    contentStore.updateHomeContent(homeContent.value.en, "en");
    contentStore.updateHomeContent(homeContent.value.ru, "ru");
    contentStore.updateGithubUsername(githubUsername.value);
    await contentStore.saveToApi();
    showNotification();
};

const resetContent = () => {
    homeContent.value = { ...contentStore.homeContent };
    githubUsername.value = contentStore.githubUsername;
};

const addSkill = async () => {
    if (!newSkill.value.name) return;
    await skillsStore.addSkill({ ...newSkill.value });
    newSkill.value = {
        name: "",
        category: "language",
        level: "intermediate",
        description: { en: "", ru: "" },
    };
    showNotification();
};

const removeSkill = async (id) => {
    await skillsStore.removeSkill(id);
    showNotification();
};

const addService = async () => {
    if (!newService.value.title) return;
    await servicesStore.addService({
        ...newService.value,
        icon: { type: "vue-component", name: "Code" },
        status: "active",
    });
    newService.value = {
        title: "",
        description: { en: "", ru: "" },
        link: "",
    };
    showNotification();
};

const removeService = async (id) => {
    await servicesStore.removeService(id);
    showNotification();
};

const addContact = async () => {
    if (!newContact.value.value) return;
    await contactsStore.addContact({ ...newContact.value });
    newContact.value = {
        type: "email",
        value: "",
        label: "",
        link: "",
    };
    showNotification();
};

const removeContact = async (id) => {
    await contactsStore.removeContact(id);
    showNotification();
};

const showNotification = () => {
    saved.value = true;
    setTimeout(() => (saved.value = false), 2000);
};
</script>

<style scoped>
/* Стили остаются такими же как в оригинале */
.admin-view {
    max-width: 900px;
}

.loading-section {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 3rem;
    color: var(--text-secondary);
}

.loading-spinner {
    animation: spin 1s linear infinite;
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

.access-denied {
    text-align: center;
    padding: 3rem;
}

.denied-content {
    background: var(--bg-secondary);
    border: 1px solid var(--error);
    padding: 2rem;
}

.denied-icon {
    font-size: 3rem;
    color: var(--error);
    display: block;
    margin-bottom: 1rem;
}

.hint {
    color: var(--text-secondary);
    font-size: 0.9rem;
    margin-top: 0.5rem;
}

.admin-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
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

.login-form {
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    padding: 2rem;
    max-width: 500px;
}

.ip-info {
    margin-bottom: 1.5rem;
    padding: 0.75rem;
    background: var(--bg-card);
    border: 1px solid var(--border);
}

.ip-label {
    color: var(--accent);
}

.ip-status {
    color: var(--accent);
    text-transform: uppercase;
    font-size: 0.75rem;
}

.form-group {
    margin-bottom: 1.5rem;
}

.form-label {
    display: block;
    color: var(--accent);
    font-size: 0.85rem;
    margin-bottom: 0.5rem;
    text-transform: lowercase;
}

.form-input,
.form-textarea {
    width: 100%;
    background: var(--bg-card);
    border: 1px solid var(--border);
    color: var(--text-primary);
    padding: 0.75rem;
    font-family: inherit;
}

.form-input:focus,
.form-textarea:focus {
    outline: none;
    border-color: var(--accent);
}

.form-hint {
    display: block;
    margin-top: 0.5rem;
    font-size: 0.75rem;
    color: var(--text-secondary);
}

.form-actions {
    display: flex;
    gap: 1rem;
}

.btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    font-family: inherit;
    cursor: pointer;
    border: 1px solid;
    text-transform: lowercase;
    background: transparent;
}

.btn-primary {
    background: var(--accent);
    border-color: var(--accent);
    color: var(--bg-primary);
}

.btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.btn-secondary {
    border-color: var(--border);
    color: var(--text-secondary);
}

.btn-danger {
    border-color: var(--error);
    color: var(--error);
    padding: 0.5rem;
}

.admin-tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    border-bottom: 1px solid var(--border);
    padding-bottom: 0.5rem;
}

.tab-btn {
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-secondary);
    padding: 0.5rem 1.5rem;
    font-family: inherit;
    cursor: pointer;
}

.tab-btn.active,
.tab-btn:hover {
    border-color: var(--accent);
    color: var(--accent);
}

.content-tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
}

.content-tab-btn {
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-secondary);
    padding: 0.25rem 0.75rem;
}

.content-tab-btn.active {
    border-color: var(--accent);
    color: var(--accent);
}

.admin-section {
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    padding: 1.5rem;
}

.skills-admin,
.services-admin,
.contacts-admin {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
}

@media (max-width: 768px) {
    .skills-admin,
    .services-admin,
    .contacts-admin {
        grid-template-columns: 1fr;
    }
}

.form-title {
    color: var(--text-primary);
    font-size: 1rem;
    font-weight: normal;
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border);
}

.skill-item,
.service-item,
.contact-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background: var(--bg-card);
    border: 1px solid var(--border);
    margin-bottom: 0.5rem;
}

.skill-info,
.service-info,
.contact-info {
    display: flex;
    flex-direction: column;
}

.skill-name,
.service-name,
.contact-name {
    color: var(--text-primary);
}

.skill-meta,
.service-link,
.contact-value-small {
    color: var(--text-secondary);
    font-size: 0.75rem;
}

.save-notification {
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    background: var(--bg-card);
    border: 1px solid var(--accent);
    color: var(--accent);
    animation: slideIn 0.3s ease;
}

@keyframes slideIn {
    from {
        transform: translateX(100%);
        opacity: 0;
    }
    to {
        transform: translateX(0);
        opacity: 1;
    }
}
</style>
