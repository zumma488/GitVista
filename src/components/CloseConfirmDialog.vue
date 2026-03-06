<script setup lang="ts">
import { ref } from 'vue'
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'

const visible = ref(false)
const rememberChoice = ref(false)

const emit = defineEmits<{
  (e: 'minimize-to-tray'): void
  (e: 'exit'): void
}>()

function show() {
  visible.value = true
  rememberChoice.value = false
}

function handleMinimize() {
  if (rememberChoice.value) {
    localStorage.setItem('close_action', 'minimize')
  }
  visible.value = false
  emit('minimize-to-tray')
}

function handleExit() {
  if (rememberChoice.value) {
    localStorage.setItem('close_action', 'exit')
  }
  visible.value = false
  emit('exit')
}

defineExpose({ show })
</script>

<template>
  <Dialog v-model:visible="visible" modal header="关闭窗口" :style="{ width: '360px', maxWidth: '420px' }" :draggable="false">
    <div class="pt-2">
      <p class="text-sm text-surface-500 mb-4">请选择关闭窗口时的操作：</p>
      <div class="flex gap-3 mb-4">
        <Button label="最小化到托盘" severity="primary" class="flex-1" @click="handleMinimize" />
        <Button label="退出程序" severity="danger" class="flex-1" @click="handleExit" />
      </div>
      <div class="flex items-center gap-2 mt-4">
        <Checkbox v-model="rememberChoice" binary inputId="remember" />
        <label for="remember" class="text-sm text-surface-500 cursor-pointer select-none">记住我的选择</label>
      </div>
    </div>
  </Dialog>
</template>

<style scoped>
.pt-2 { padding-top: 0.5rem; }
.mb-4 { margin-bottom: 1rem; }
.mt-4 { margin-top: 1rem; }
.flex { display: flex; }
.flex-1 { flex: 1; justify-content: center; }
.items-center { align-items: center; }
.gap-2 { gap: 0.5rem; }
.gap-3 { gap: 0.75rem; }
.cursor-pointer { cursor: pointer; }
.select-none { user-select: none; }
.text-sm { font-size: 13px; }
.text-surface-500 { color: var(--text-secondary); }
</style>
