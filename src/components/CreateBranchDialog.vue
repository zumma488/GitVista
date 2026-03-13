<script setup lang="ts">
import { ref, nextTick } from 'vue'
import Dialog from 'primevue/dialog'
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'

const visible = ref(false)
const branchName = ref('')
const inputRef = ref()

const emit = defineEmits<{
  (e: 'submit', name: string): void
}>()

function show() {
  visible.value = true
  branchName.value = ''
  nextTick(() => {
    // PrimeVue InputText element is likely available via $el
    inputRef.value?.$el?.focus()
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
  <Dialog v-model:visible="visible" modal header="新建分支" :style="{ width: '360px' }" :draggable="false">
    <div class="pt-2">
      <p class="text-sm text-surface-500 mb-3">请输入新分支的名称：</p>
      <InputText
        ref="inputRef"
        v-model="branchName"
        class="w-full mb-4"
        placeholder="分支名称"
        @keyup.enter="handleSubmit"
      />
      <div class="flex justify-end gap-2 mt-2">
        <Button label="取消" severity="secondary" variant="text" @click="handleCancel" />
        <Button label="创建分支" :disabled="!branchName.trim()" @click="handleSubmit" />
      </div>
    </div>
  </Dialog>
</template>

<style scoped>
.pt-2 { padding-top: 0.5rem; }
.mb-3 { margin-bottom: 0.75rem; }
.mb-4 { margin-bottom: 1rem; }
.mt-2 { margin-top: 0.5rem; }
.w-full { width: 100%; box-sizing: border-box; }
.flex { display: flex; }
.justify-end { justify-content: flex-end; }
.gap-2 { gap: 0.5rem; }
.text-sm { font-size: 13px; }
.text-surface-500 { color: var(--p-text-muted-color); }
</style>
