<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Minus, Square, X, Copy } from 'lucide-vue-next'

const appWindow = getCurrentWindow()
const isMaximized = ref(false)

async function syncMaximized() {
  isMaximized.value = await appWindow.isMaximized()
}

let unlisten: (() => void) | null = null

onMounted(async () => {
  await syncMaximized()
  unlisten = await appWindow.onResized(syncMaximized)
})

onUnmounted(() => {
  unlisten?.()
})

async function handleMinimize() {
  await appWindow.minimize()
}

async function handleToggleMaximize() {
  await appWindow.toggleMaximize()
}

async function handleClose() {
  await appWindow.close()
}
</script>

<template>
  <div class="titlebar">
    <div class="titlebar-drag" data-tauri-drag-region>
      <span class="titlebar-title" data-tauri-drag-region>GitVista</span>
    </div>
    <div class="titlebar-controls">
      <button
        class="titlebar-btn"
        title="最小化"
        @click="handleMinimize"
      >
        <Minus :size="16" />
      </button>
      <button
        class="titlebar-btn"
        :title="isMaximized ? '还原' : '最大化'"
        @click="handleToggleMaximize"
      >
        <Copy v-if="isMaximized" :size="12" class="restore-icon" />
        <Square v-else :size="12" />
      </button>
      <button
        class="titlebar-btn titlebar-btn-close"
        title="关闭"
        @click="handleClose"
      >
        <X :size="16" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  height: var(--titlebar-height);
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: var(--bg-canvas);
  flex-shrink: 0;
  position: relative;
  z-index: 9000;
}

.titlebar-drag {
  flex: 1;
  height: 100%;
  display: flex;
  align-items: center;
  -webkit-app-region: drag;
}

.titlebar-title {
  padding-left: 12px;
  font-size: 12px;
  color: var(--text-muted);
  pointer-events: none;
  user-select: none;
}

.titlebar-controls {
  display: flex;
  height: 100%;
  -webkit-app-region: no-drag;
}

.titlebar-btn {
  width: 46px;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
  border: none;
  background: transparent;
  cursor: pointer;
  transition: background-color 0.12s, color 0.12s;
  -webkit-app-region: no-drag;
}

.titlebar-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.titlebar-btn-close:hover {
  background: #e81123;
  color: #fff;
}

.restore-icon {
  transform: rotate(180deg);
}
</style>
