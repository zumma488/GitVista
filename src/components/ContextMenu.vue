<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Component } from 'vue'
import PrimeContextMenu from 'primevue/contextmenu'

export interface MenuItem {
  label: string
  icon?: Component
  action: () => void
  danger?: boolean
  divider?: boolean
}

const props = defineProps<{
  items: MenuItem[]
}>()

const menuRef = ref<InstanceType<typeof PrimeContextMenu>>()

const primeItems = computed(() => {
  return props.items.map((item, index) => {
    if (item.divider) {
      return { separator: true }
    }
    return {
      key: String(index),
      label: item.label,
      command: () => {
        item.action()
        close()
      },
      originalItem: item
    }
  })
})

function open(event: MouseEvent) {
  menuRef.value?.show(event)
}

function close() {
  menuRef.value?.hide()
}

defineExpose({ open, close })
</script>

<template>
  <PrimeContextMenu ref="menuRef" :model="primeItems" class="custom-ctx-menu">
    <template #item="{ item, props }">
      <a v-ripple class="flex items-center px-3 py-2 cursor-pointer w-full" v-bind="props.action" :class="{ 'text-red-500': item.originalItem?.danger }">
        <component :is="item.originalItem?.icon" v-if="item.originalItem?.icon" :size="14" style="margin-right: 8px;" />
        <span style="font-size: 12px;">{{ item.label }}</span>
      </a>
    </template>
  </PrimeContextMenu>
</template>

<style scoped>
.custom-ctx-menu :deep(.p-menuitem-link) {
  padding: 0 !important;
}
.custom-ctx-menu :deep(.p-menuitem-content) {
  transition: background-color 0.1s;
}
.custom-ctx-menu :deep(.p-menuitem-content:hover) {
  background: var(--p-content-hover-background) !important;
}
.text-red-500 {
  color: var(--p-red-500) !important;
}
.text-red-500:hover {
  background: rgba(248, 81, 73, 0.12) !important;
}
.flex { display: flex; }
.items-center { align-items: center; }
.cursor-pointer { cursor: pointer; }
.w-full { width: 100%; }
.px-3 { padding-left: 12px; padding-right: 12px; }
.py-2 { padding-top: 6px; padding-bottom: 6px; }
</style>
