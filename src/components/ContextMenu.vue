<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
import type { Component } from 'vue'

export interface MenuItem {
  label: string
  icon?: Component
  action: () => void
  danger?: boolean
  divider?: boolean
}

defineProps<{
  items: MenuItem[]
}>()

const emit = defineEmits<{
  close: []
}>()

const visible = ref(false)
const x = ref(0)
const y = ref(0)
const menuRef = ref<HTMLElement>()

function open(event: MouseEvent) {
  x.value = event.clientX
  y.value = event.clientY
  visible.value = true

  nextTick(() => {
    if (!menuRef.value) return
    const rect = menuRef.value.getBoundingClientRect()
    const vw = window.innerWidth
    const vh = window.innerHeight

    if (x.value + rect.width > vw) {
      x.value = vw - rect.width - 4
    }
    if (y.value + rect.height > vh) {
      y.value = vh - rect.height - 4
    }
  })
}

function close() {
  visible.value = false
  emit('close')
}

function handleClick(item: MenuItem) {
  if (item.divider) return
  item.action()
  close()
}

function handleClickOutside(e: MouseEvent) {
  if (menuRef.value && !menuRef.value.contains(e.target as Node)) {
    close()
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    close()
  }
}

onMounted(() => {
  document.addEventListener('mousedown', handleClickOutside, true)
  document.addEventListener('keydown', handleKeydown, true)
})

onUnmounted(() => {
  document.removeEventListener('mousedown', handleClickOutside, true)
  document.removeEventListener('keydown', handleKeydown, true)
})

defineExpose({ open, close })
</script>

<template>
  <Teleport to="body">
    <div
      v-if="visible"
      ref="menuRef"
      class="context-menu"
      :style="{ left: x + 'px', top: y + 'px' }"
    >
      <template v-for="(item, idx) in items" :key="idx">
        <div v-if="item.divider" class="context-menu-divider" />
        <button
          v-else
          class="context-menu-item"
          :class="{ danger: item.danger }"
          @click="handleClick(item)"
        >
          <component v-if="item.icon" :is="item.icon" :size="14" class="context-menu-icon" />
          <span>{{ item.label }}</span>
        </button>
      </template>
    </div>
  </Teleport>
</template>

<style scoped>
.context-menu {
  position: fixed;
  z-index: 10000;
  min-width: 180px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  padding: 4px 0;
  animation: ctx-fade-in 0.1s ease;
}

@keyframes ctx-fade-in {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 6px 12px;
  font-size: 12px;
  color: var(--text-primary);
  background: transparent;
  border: none;
  cursor: pointer;
  transition: background-color 0.1s;
  text-align: left;
}

.context-menu-item:hover {
  background: var(--bg-hover);
}

.context-menu-item.danger {
  color: var(--accent-red);
}

.context-menu-item.danger:hover {
  background: rgba(248, 81, 73, 0.12);
}

.context-menu-icon {
  flex-shrink: 0;
  opacity: 0.8;
}

.context-menu-divider {
  height: 1px;
  margin: 4px 8px;
  background: var(--border-default);
}
</style>
