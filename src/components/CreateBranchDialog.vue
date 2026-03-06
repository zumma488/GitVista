<script setup lang="ts">
import { ref, nextTick } from 'vue'

const visible = ref(false)
const branchName = ref('')
const inputRef = ref<HTMLInputElement>()

const emit = defineEmits<{
  (e: 'submit', name: string): void
}>()

function show() {
  visible.value = true
  branchName.value = ''
  nextTick(() => {
    inputRef.value?.focus()
  })
}

function handleSubmit() {
  const name = branchName.value.trim()
  if (!name) return

  visible.value = false
  emit('submit', name)
}

function handleCancel() {
  visible.value = false
}

defineExpose({ show })
</script>

<template>
  <Teleport to="body">
    <div v-if="visible" class="dialog-overlay">
      <div class="dialog">
        <div class="dialog-header">新建分支</div>
        <div class="dialog-body">
          <p class="dialog-text">请输入新分支的名称：</p>
          <input
            ref="inputRef"
            v-model="branchName"
            type="text"
            class="branch-input"
            placeholder="分支名称"
            @keyup.enter="handleSubmit"
            @keyup.escape="handleCancel"
          />
          <div class="dialog-actions">
            <button class="btn btn-secondary" @click="handleCancel">
              取消
            </button>
            <button class="btn btn-primary" :disabled="!branchName.trim()" @click="handleSubmit">
              创建分支
            </button>
          </div>
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
  width: 360px;
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
  margin-bottom: 12px;
}

.branch-input {
  width: 100%;
  padding: 8px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s;
  margin-bottom: 16px;
}

.branch-input:focus {
  border-color: var(--accent-blue);
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
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
