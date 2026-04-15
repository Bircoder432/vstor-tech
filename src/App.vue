<template>
    <div class="terminal">
        <TerminalHeader />
        <TerminalNav />
        <main class="terminal-main">
            <router-view v-slot="{ Component }">
                <transition name="fade" mode="out-in">
                    <component :is="Component" />
                </transition>
            </router-view>
        </main>
        <footer class="terminal-footer">
            <span class="prompt">root@portfolio:~$</span>
            <span class="cursor">_</span>
        </footer>
    </div>
</template>

<script setup>
import TerminalHeader from "./components/TerminalHeader.vue";
import TerminalNav from "./components/TerminalNav.vue";
</script>

<style>
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

:root {
    --bg-primary: #0a0a0a;
    --bg-secondary: #111111;
    --bg-card: #1a1a1a;
    --text-primary: #ffffff;
    --text-secondary: #888888;
    --accent: #00ff00;
    --accent-dim: #00aa00;
    --border: #333333;
    --error: #ff0000;
    --warning: #ffaa00;
}

body {
    font-family: "Courier New", "Monaco", "Consolas", monospace;
    background: var(--bg-primary);
    color: var(--text-primary);
    line-height: 1.6;
    min-height: 100vh;
}

.terminal {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}

.terminal-main {
    flex: 1;
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
    width: 100%;
}

.terminal-footer {
    padding: 1rem 2rem;
    border-top: 2px solid var(--border);
    background: var(--bg-secondary);
    font-size: 0.9rem;
}

.prompt {
    color: var(--accent);
}

.cursor {
    animation: blink 1s infinite;
    color: var(--accent);
}

@keyframes blink {
    0%,
    50% {
        opacity: 1;
    }
    51%,
    100% {
        opacity: 0;
    }
}

.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}

/* Scrollbar */
::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

::-webkit-scrollbar-track {
    background: var(--bg-primary);
}

::-webkit-scrollbar-thumb {
    background: var(--border);
    border-radius: 0;
}

::-webkit-scrollbar-thumb:hover {
    background: var(--accent-dim);
}

/* Selection */
::selection {
    background: var(--accent);
    color: var(--bg-primary);
}
</style>
