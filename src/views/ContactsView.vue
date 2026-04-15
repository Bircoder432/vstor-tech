<template>
    <div class="contacts-view">
        <div class="section-header">
            <span class="section-prompt">~/{{ $t("contacts.title") }}</span>
            <span class="section-line"></span>
        </div>

        <h2 class="section-title">{{ $t("contacts.connect") }}</h2>

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
                        :is="contactsStore.availableIcons[contact.icon]"
                        :size="24"
                    />
                </div>
                <div class="contact-info">
                    <span class="contact-label">{{ contact.label }}</span>
                    <span class="contact-value">{{ contact.value }}</span>
                </div>
                <ExternalLink
                    v-if="contact.link"
                    :size="14"
                    class="contact-link-icon"
                />
            </a>
        </div>

        <div class="contacts-footer">
            <span class="footer-prompt">$</span>
            <span class="footer-text">
                {{ contactsStore.contacts.length }}
                {{ $t("contacts.title").toLowerCase() }} available
            </span>
        </div>
    </div>
</template>

<script setup>
import { onMounted } from "vue";
import { ExternalLink } from "lucide-vue-next";
import { useContactsStore } from "../stores/contacts";

const contactsStore = useContactsStore();

const getContactLink = (contact) => {
    if (contact.link) return contact.link;
    if (contact.type === "email") return `mailto:${contact.value}`;
    if (contact.type === "phone")
        return `tel:${contact.value.replace(/\s/g, "")}`;
    return "#";
};

onMounted(async () => {
    await contactsStore.loadFromApi();
});
</script>

<style scoped>
.contacts-view {
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

.section-title {
    color: var(--text-primary);
    font-size: 1.2rem;
    font-weight: normal;
    margin-bottom: 2rem;
    text-transform: lowercase;
}

.contacts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;
}

.contact-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.25rem;
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
    gap: 0.25rem;
    flex: 1;
}

.contact-label {
    font-size: 0.75rem;
    color: var(--text-secondary);
    text-transform: uppercase;
}

.contact-value {
    font-size: 0.95rem;
    color: var(--text-primary);
}

.contact-link-icon {
    color: var(--accent);
    opacity: 0.5;
}

.contacts-footer {
    margin-top: 2rem;
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
</style>
