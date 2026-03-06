<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useRepoStore } from '@/stores/repo'
import TitleBar from '@/components/TitleBar.vue'

const repo = useRepoStore()
</script>

<template>
  <TitleBar />
  <div class="app-content">
    <RouterView />
  </div>

  <div class="toast-container">
    <div
      v-for="toast in repo.toasts"
      :key="toast.id"
      class="toast"
      :class="`toast-${toast.type}`"
    >
      {{ toast.message }}
    </div>
  </div>
</template>

<style scoped>
.app-content {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
</style>
