<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useRepoStore } from '@/stores/repo'
import Button from 'primevue/button'
import {
  ArrowLeft,
  GitBranch,
  ArrowDownToLine,
  ArrowUpFromLine,
  RefreshCw,
  Code2,
} from 'lucide-vue-next'

const router = useRouter()
const repo = useRepoStore()

function goBack() {
  router.push({ name: 'projects' })
}

function repoName(): string {
  const parts = repo.path.split('/')
  return parts[parts.length - 1] || repo.path
}
</script>

<template>
  <div class="toolbar">
    <div class="toolbar-left">
      <Button variant="text" severity="secondary" aria-label="返回项目列表" title="返回项目列表" @click="goBack" class="!px-2 !py-2">
        <ArrowLeft :size="18" />
      </Button>
      <span class="toolbar-divider" />
      <span class="repo-name">{{ repoName() }}</span>

      <span class="current-branch" :title="repo.info?.current_branch">
        <GitBranch :size="14" />
        <span class="current-branch-text">{{ repo.info?.current_branch || '...' }}</span>
      </span>
    </div>

    <div class="toolbar-right">
      <Button
        severity="secondary"
        variant="outlined"
        :disabled="repo.operating"
        title="Fetch"
        @click="repo.fetch()"
        class="!py-1 !px-3 h-8 text-sm"
      >
        <RefreshCw :size="14" :class="{ spinning: repo.operating }" class="mr-2" />
        Fetch
      </Button>
      <Button
        severity="secondary"
        variant="outlined"
        :disabled="repo.operating"
        title="Pull"
        @click="repo.pull()"
        class="!py-1 !px-3 h-8 text-sm"
      >
        <ArrowDownToLine :size="14" class="mr-2" />
        Pull
      </Button>
      <Button
        severity="secondary"
        variant="outlined"
        :disabled="repo.operating"
        title="Push"
        @click="repo.push()"
        class="!py-1 !px-3 h-8 text-sm"
      >
        <ArrowUpFromLine :size="14" class="mr-2" />
        Push
      </Button>
      <span class="toolbar-divider" />
      <Button
        variant="text"
        severity="secondary"
        title="在 VSCode 中打开"
        @click="repo.openInVscode()"
        class="!py-1 !px-3 h-8 text-sm"
      >
        <Code2 :size="14" class="mr-2" />
        VSCode
      </Button>
    </div>
  </div>
</template>

<style scoped>
.toolbar {
  height: var(--toolbar-height);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-default);
  flex-shrink: 0;
}

.toolbar-left,
.toolbar-right {
  display: flex;
  align-items: center;
  gap: 6px;
}

.mr-2 { margin-right: 0.5rem; }
.h-8 { height: 2rem; }
.text-sm { font-size: 0.875rem; }

.toolbar-divider {
  width: 1px;
  height: 20px;
  background: var(--border-default);
  margin: 0 4px;
}

.repo-name {
  font-weight: 600;
  font-size: 14px;
  color: var(--text-primary);
  margin-right: 4px;
}

.current-branch {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  font-size: 12px;
  color: var(--text-primary);
  max-width: 520px;
}

.current-branch-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
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
