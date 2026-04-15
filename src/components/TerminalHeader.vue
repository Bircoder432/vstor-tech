<template>
    <header class="terminal-header">
        <div class="window-controls">
            <span class="control close"></span>
            <span class="control minimize"></span>
            <span class="control maximize"></span>
        </div>
        <div class="title">VSTOR_TECH@1.2.3 — terminal</div>
        <div class="spacer"></div>

        <!-- Admin link - visible only for allowed IPs -->
        <router-link v-if="showAdmin" to="/system-panel" class="admin-link">
            [admin]
        </router-link>
    </header>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { appConfig } from "../config/env";

const appName = appConfig.appName;
const appVersion = appConfig.appVersion;

const ALLOWED_IPS = appConfig.allowedIps;
const showAdmin = ref(false);

const checkIP = async () => {
    try {
        // Для локальной разработки сразу разрешаем
        if (
            window.location.hostname === "localhost" ||
            window.location.hostname === "127.0.0.1"
        ) {
            showAdmin.value =
                ALLOWED_IPS.includes("127.0.0.1") ||
                ALLOWED_IPS.includes("localhost");
            return;
        }

        const isLocal =
            window.location.hostname.includes("192.168.") ||
            window.location.hostname.includes("10.") ||
            window.location.hostname.includes("172.");

        if (isLocal) {
            showAdmin.value = true;
            return;
        }

        const response = await fetch("https://api.ipify.org?format=json");
        const data = await response.json();
        showAdmin.value = ALLOWED_IPS.includes(data.ip);
    } catch (err) {
        const hostname = window.location.hostname;
        showAdmin.value =
            hostname === "localhost" ||
            hostname === "127.0.0.1" ||
            ALLOWED_IPS.includes(hostname);
    }
};

onMounted(() => {
    checkIP();
});
</script>

<style scoped>
.terminal-header {
    display: flex;
    align-items: center;
    padding: 0.75rem 1rem;
    background: var(--bg-secondary);
    border-bottom: 2px solid var(--border);
    gap: 1rem;
}

.window-controls {
    display: flex;
    gap: 0.5rem;
}

.control {
    width: 12px;
    height: 12px;
    border: 1px solid var(--border);
    background: var(--bg-card);
}

.close:hover {
    background: var(--error);
}
.minimize:hover {
    background: var(--warning);
}
.maximize:hover {
    background: var(--accent);
}

.title {
    font-size: 0.85rem;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 1px;
}

.spacer {
    flex: 1;
}

.admin-link {
    color: var(--text-secondary);
    text-decoration: none;
    font-size: 0.75rem;
    padding: 0.25rem 0.5rem;
    border: 1px solid var(--border);
    transition: all 0.2s;
    text-transform: lowercase;
}

.admin-link:hover {
    border-color: var(--accent);
    color: var(--accent);
    background: var(--bg-card);
}
</style>
