<script setup lang="ts">
import { onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useRepoStore } from '@/stores/repo'
import RepoToolbar from '@/components/repo/RepoToolbar.vue'
import RepoSidebar from '@/components/repo/RepoSidebar.vue'
import ChangesPanel from '@/components/repo/ChangesPanel.vue'
import HistoryPanel from '@/components/repo/HistoryPanel.vue'
import StatusBar from '@/components/repo/StatusBar.vue'
import { Loader2 } from 'lucide-vue-next'

const route = useRoute()
const router = useRouter()
const repo = useRepoStore()

onMounted(() => {
  const repoPath = route.query.path as string
  if (!repoPath) {
    router.push({ name: 'projects' })
    return
  }
  repo.loadRepo(repoPath)
})

// 定时刷新文件状态
let refreshTimer: ReturnType<typeof setInterval> | null = null

onMounted(() => {
  refreshTimer = setInterval(() => {
    if (repo.path && !repo.operating) {
      repo.refreshFiles()
    }
  }, 5000)
})

watch(
  () => route.query.path,
  (newPath) => {
    if (newPath && typeof newPath === 'string') {
      repo.loadRepo(newPath)
    }
  },
)

import { onUnmounted } from 'vue'
onUnmounted(() => {
  if (refreshTimer) clearInterval(refreshTimer)
})
</script>

<template>
  <div class="repo-page">
    <RepoToolbar />

    <div class="repo-body">
      <RepoSidebar />

      <main class="repo-main">
        <div v-if="repo.loading" class="loading-state">
          <Loader2 :size="32" class="spinning" />
          <span>加载中...</span>
        </div>
        <template v-else>
          <ChangesPanel v-if="repo.activeTab === 'changes'" />
          <HistoryPanel v-if="repo.activeTab === 'history'" />
        </template>
      </main>
    </div>

    <StatusBar />
  </div>
</template>

<style scoped>
.repo-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
}

.repo-body {
  flex: 1;
  display: flex;
  min-height: 0;
}

.repo-main {
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

.loading-state {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  color: var(--text-muted);
}

.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
