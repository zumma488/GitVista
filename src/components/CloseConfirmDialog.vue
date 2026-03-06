<script setup lang="ts">
import { ref } from 'vue'

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
  <Teleport to="body">
    <div v-if="visible" class="dialog-overlay" @click.self="visible = false">
      <div class="dialog">
        <div class="dialog-header">关闭窗口</div>
        <div class="dialog-body">
          <p class="dialog-text">请选择关闭窗口时的操作：</p>
          <div class="dialog-actions">
            <button class="btn btn-primary" @click="handleMinimize">
              最小化到托盘
            </button>
            <button class="btn btn-danger" @click="handleExit">
              退出程序
            </button>
          </div>
          <label class="remember-label">
            <input v-model="rememberChoice" type="checkbox" class="remember-checkbox" />
            <span>记住我的选择</span>
          </label>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
  animation: overlay-in 0.15s ease;
}

.dialog {
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  min-width: 360px;
  max-width: 420px;
  animation: dialog-in 0.2s ease;
}

.dialog-header {
  padding: 16px 20px 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.dialog-body {
  padding: 12px 20px 20px;
}

.dialog-text {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 16px;
}

.dialog-actions {
  display: flex;
  gap: 10px;
}

.dialog-actions .btn {
  flex: 1;
  justify-content: center;
  padding: 8px 16px;
}

.remember-label {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 14px;
  font-size: 12px;
  color: var(--text-muted);
  cursor: pointer;
  user-select: none;
}

.remember-checkbox {
  accent-color: var(--accent-blue);
  width: 14px;
  height: 14px;
  cursor: pointer;
}

@keyframes overlay-in {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes dialog-in {
  from {
    opacity: 0;
    transform: scale(0.95) translateY(-8px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}
</style>
