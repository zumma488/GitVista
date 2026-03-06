<script setup lang="ts">
import { useRepoStore } from '@/stores/repo'
import { GitBranch, FolderOpen, Check, AlertCircle, Loader2 } from 'lucide-vue-next'

const repo = useRepoStore()
</script>

<template>
  <div class="statusbar">
    <div class="statusbar-left">
      <span class="status-item">
        <GitBranch :size="12" />
        {{ repo.info?.current_branch || '...' }}
      </span>
      <span v-if="repo.info" class="status-item">
        <component :is="repo.info.is_clean ? Check : AlertCircle" :size="12" />
        {{ repo.info.is_clean ? '工作区干净' : `${repo.files.length} 个文件变更` }}
      </span>
    </div>
    <div class="statusbar-right">
      <span v-if="repo.operating" class="status-item">
        <Loader2 :size="12" class="spinning" />
        操作中...
      </span>
      <span class="status-item path" :title="repo.path">
        <FolderOpen :size="12" />
        {{ repo.path }}
      </span>
    </div>
  </div>
</template>

<style scoped>
.statusbar {
  height: var(--statusbar-height);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  background: var(--bg-canvas);
  border-top: 1px solid var(--border-default);
  flex-shrink: 0;
  font-size: 11px;
}

.statusbar-left,
.statusbar-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 4px;
  color: var(--text-muted);
}

.status-item.path {
  max-width: 300px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-family: var(--font-mono);
}

.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
