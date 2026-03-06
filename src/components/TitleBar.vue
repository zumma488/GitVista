<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'
import { Minus, Square, X, Copy, Sun, Moon } from 'lucide-vue-next'
import { useTheme } from '@/composables/useTheme'
import CloseConfirmDialog from '@/components/CloseConfirmDialog.vue'

const appWindow = getCurrentWindow()
const isMaximized = ref(false)
const { currentTheme, toggleTheme } = useTheme()
const closeDialog = ref<InstanceType<typeof CloseConfirmDialog>>()

async function syncMaximized() {
  isMaximized.value = await appWindow.isMaximized()
}

let unlisten: (() => void) | null = null
let unlistenClose: (() => void) | null = null

onMounted(async () => {
  await syncMaximized()
  unlisten = await appWindow.onResized(syncMaximized)
  // 监听后端拦截的关闭请求（Alt+F4、任务栏关闭等）
  unlistenClose = await listen('close-requested', () => {
    triggerClose()
  })
})

onUnmounted(() => {
  unlisten?.()
  unlistenClose?.()
})

async function handleMinimize() {
  await appWindow.minimize()
}

async function handleToggleMaximize() {
  await appWindow.toggleMaximize()
}

function triggerClose() {
  const savedAction = localStorage.getItem('close_action')
  if (savedAction === 'minimize') {
    minimizeToTray()
  } else if (savedAction === 'exit') {
    exitApp()
  } else {
    closeDialog.value?.show()
  }
}

async function minimizeToTray() {
  await appWindow.hide()
}

async function exitApp() {
  // 先解除关闭事件拦截，再让 Tauri 正常关闭
  await appWindow.destroy()
}
</script>

<template>
  <div class="titlebar" data-tauri-drag-region="true">
    <div class="titlebar-drag" data-tauri-drag-region="true">
      <span class="titlebar-title" data-tauri-drag-region="true">GitVista</span>
    </div>
    <div class="titlebar-controls" data-tauri-drag-region="false">
      <button
        class="titlebar-btn"
        :title="currentTheme === 'dark' ? '切换到浅色主题' : '切换到深色主题'"
        @click="toggleTheme"
        @mousedown.stop.prevent
      >
        <Sun v-if="currentTheme === 'dark'" :size="14" />
        <Moon v-else :size="14" />
      </button>
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
        @click="triggerClose"
      >
        <X :size="16" />
      </button>
    </div>
  </div>

  <CloseConfirmDialog
    ref="closeDialog"
    @minimize-to-tray="minimizeToTray"
    @exit="exitApp"
  />
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
