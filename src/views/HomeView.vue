<template>
    <div class="home-view">
        <div class="section-header">
            <span class="section-prompt">~/{{ $t("home.title") }}</span>
            <span class="section-line"></span>
        </div>

        <MarkdownRenderer :content="contentStore.currentHomeContent" />

        <!-- Skills Section - над статистикой GitHub -->
        <div class="skills-section">
            <div class="section-header skills-header">
                <span class="section-prompt">~/skills</span>
                <span class="section-line"></span>
            </div>

            <div class="skills-grid">
                <SkillCard
                    v-for="skill in skillsStore.skills"
                    :key="skill.id"
                    :skill="skill"
                    :category-label="getCategoryLabel(skill.category)"
                />
            </div>
        </div>

        <GitHubStats :username="contentStore.githubUsername" />

        <!-- Contacts Section -->
        <div class="contacts-section">
            <div class="section-header contacts-header">
                <span class="section-prompt">~/{{ $t("home.connect") }}</span>
                <span class="section-line"></span>
            </div>

            <!-- src/views/HomeView.vue — исправленный фрагмент контактов -->
            <div class="contacts-grid">
                <a
                    v-for="contact in contactsStore.contacts"
                    :key="contact.id"
                    :href="getContactLink(contact)"
                    :target="contact.link ? '_blank' : undefined"
                    :rel="contact.link ? 'noopener' : undefined"
                    class="contact-card"
                >
                    <div class="contact-icon">
                        <component
                            :is="
                                contactsStore.availableIcons[contact.icon] ||
                                contactsStore.availableIcons.Globe
                            "
                            :size="20"
                        />
                    </div>
                    <div class="contact-info">
                        <span class="contact-label">{{ contact.label }}</span>
                        <span class="contact-value">{{ contact.value }}</span>
                    </div>
                    <ExternalLink
                        v-if="
                            contact.link ||
                            contact.type === 'email' ||
                            contact.type === 'phone'
                        "
                        :size="12"
                        class="contact-link-icon"
                    />
                </a>
            </div>
        </div>
    </div>
</template>

<script setup>
import { onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { ExternalLink } from "lucide-vue-next";
import MarkdownRenderer from "../components/MarkdownRenderer.vue";
import GitHubStats from "../components/GitHubStats.vue";
import SkillCard from "../components/SkillCard.vue";
import { useContentStore } from "../stores/content";
import { useContactsStore } from "../stores/contacts";
import { useSkillsStore } from "../stores/skills";

const { t, locale } = useI18n();
const contentStore = useContentStore();
const contactsStore = useContactsStore();
const skillsStore = useSkillsStore();

const getContactLink = (contact) => {
    if (contact.link) return contact.link;
    if (contact.type === "email") return `mailto:${contact.value}`;
    if (contact.type === "phone")
        return `tel:${contact.value.replace(/\s/g, "")}`;
    return "#";
};

const getCategoryLabel = (categoryValue) => {
    const category = skillsStore.skillCategories.find(
        (c) => c.value === categoryValue,
    );
    return category ? category.label : categoryValue;
};

onMounted(async () => {
    await Promise.all([
        contentStore.loadFromApi(),
        contactsStore.loadFromApi(),
        skillsStore.loadFromApi(),
    ]);
});
</script>

<style scoped>
.home-view {
    max-width: 800px;
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

.skills-section {
    margin-top: 2rem;
    margin-bottom: 2rem;
}

.skills-header {
    margin-bottom: 1rem;
}

.skills-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1rem;
}

.contacts-section {
    margin-top: 2rem;
}

.contacts-header {
    margin-bottom: 1rem;
}

.contacts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 0.75rem;
}

.contact-card {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    background: var(--bg-card);
    border: 1px solid var(--border);
    text-decoration: none;
    color: inherit;
    transition: all 0.2s;
}

.contact-card:hover {
    border-color: var(--accent);
    transform: translateX(4px);
}

.contact-icon {
    color: var(--accent);
    display: flex;
    align-items: center;
}

.contact-info {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    flex: 1;
    min-width: 0;
}

.contact-label {
    font-size: 0.7rem;
    color: var(--text-secondary);
    text-transform: uppercase;
}

.contact-value {
    font-size: 0.85rem;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.contact-link-icon {
    color: var(--accent);
    opacity: 0.5;
    flex-shrink: 0;
}

@media (max-width: 600px) {
    .skills-grid {
        grid-template-columns: 1fr;
    }

    .contacts-grid {
        grid-template-columns: 1fr;
    }
}
</style>
