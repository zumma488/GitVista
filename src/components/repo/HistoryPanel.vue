<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRepoStore } from '@/stores/repo'
import { GitCommitHorizontal, Code2, User, Calendar, Search, CherryIcon, Undo2, Copy } from 'lucide-vue-next'
import ContextMenu from '@/components/ContextMenu.vue'
import type { MenuItem } from '@/components/ContextMenu.vue'

const repo = useRepoStore()
const searchQuery = ref('')
let searchTimeout: ReturnType<typeof setTimeout> | null = null

function handleSearch() {
  if (searchTimeout) clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => {
    if (searchQuery.value.trim()) {
      repo.searchCommits(searchQuery.value.trim())
    } else {
      repo.refreshHistory()
    }
  }, 300)
}

watch(searchQuery, handleSearch)

function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)

  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes} 分钟前`
  if (hours < 24) return `${hours} 小时前`
  if (days < 7) return `${days} 天前`
  return date.toLocaleDateString('zh-CN', { year: 'numeric', month: '2-digit', day: '2-digit' })
}

function statusColor(status: string): string {
  switch (status) {
    case 'modified': return 'var(--accent-yellow)'
    case 'added': return 'var(--accent-green)'
    case 'deleted': return 'var(--accent-red)'
    case 'renamed': return 'var(--accent-purple)'
    default: return 'var(--text-muted)'
  }
}

function statusIcon(status: string): string {
  switch (status) {
    case 'modified': return 'M'
    case 'added': return 'A'
    case 'deleted': return 'D'
    case 'renamed': return 'R'
    default: return '?'
  }
}

const commitCtxMenu = ref<InstanceType<typeof ContextMenu>>()
const commitCtxMenuItems = ref<MenuItem[]>([])

function showCommitCtxMenu(e: MouseEvent, hash: string) {
  commitCtxMenuItems.value = [
    { label: 'Cherry-pick', icon: CherryIcon, action: () => repo.cherryPick(hash) },
    { label: 'Revert', icon: Undo2, action: () => repo.revertCommit(hash) },
    { label: '', action: () => {}, divider: true },
    { label: '复制提交哈希', icon: Copy, action: () => navigator.clipboard.writeText(hash) },
  ]
  commitCtxMenu.value?.open(e)
}
</script>

<template>
  <div class="history-panel">
    <!-- 搜索栏 -->
    <div class="history-search">
      <Search :size="13" class="search-icon" />
      <input
        v-model="searchQuery"
        class="search-input"
        placeholder="搜索提交信息..."
      />
    </div>

    <!-- 提交列表 -->
    <div class="commit-list">
      <div
        v-for="commit in repo.commits"
        :key="commit.hash"
        class="commit-item"
        :class="{ selected: repo.selectedCommitHash === commit.hash }"
        @click="repo.selectCommit(commit.hash)"
        @contextmenu.prevent="showCommitCtxMenu($event, commit.hash)"
      >
        <div class="commit-dot" />
        <div class="commit-line" />
        <div class="commit-content">
          <div class="commit-header">
            <span class="commit-message">{{ commit.message }}</span>
            <span class="commit-hash">{{ commit.short_hash }}</span>
          </div>
          <div class="commit-meta">
            <span class="commit-author">
              <User :size="11" />
              {{ commit.author }}
            </span>
            <span class="commit-date">
              <Calendar :size="11" />
              {{ formatDate(commit.date) }}
            </span>
          </div>
        </div>
      </div>
      <div v-if="repo.commits.length === 0" class="empty-hint">暂无提交历史</div>
    </div>

    <!-- 提交详情 -->
    <div v-if="repo.selectedCommitHash" class="commit-detail">
      <div class="detail-header">
        <GitCommitHorizontal :size="14" />
        <span>提交详情</span>
        <span class="detail-hash">{{ repo.selectedCommitHash.substring(0, 10) }}</span>
        <button
          class="btn-icon detail-action"
          title="Cherry-pick 此提交"
          @click="repo.cherryPick(repo.selectedCommitHash!)"
        >
          <CherryIcon :size="13" />
        </button>
        <button
          class="btn-icon detail-action"
          title="Revert 此提交"
          @click="repo.revertCommit(repo.selectedCommitHash!)"
        >
          <Undo2 :size="13" />
        </button>
      </div>
      <div class="detail-files">
        <div
          v-for="file in repo.selectedCommitFiles"
          :key="file.path"
          class="detail-file"
          @dblclick="repo.openInVscode(file.path)"
        >
          <span class="file-status-badge" :style="{ color: statusColor(file.status) }">
            {{ statusIcon(file.status) }}
          </span>
          <span class="detail-file-path">{{ file.path }}</span>
          <button
            class="btn-icon file-action"
            title="在 VSCode 中查看"
            @click.stop="repo.openInVscode(file.path)"
          >
            <Code2 :size="12" />
          </button>
        </div>
        <div v-if="repo.selectedCommitFiles.length === 0" class="empty-hint">无文件变更</div>
      </div>
    </div>

    <ContextMenu ref="commitCtxMenu" :items="commitCtxMenuItems" />
  </div>
</template>

<style scoped>
.history-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.history-search {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 8px 12px;
  padding: 5px 8px;
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  flex-shrink: 0;
  transition: border-color 0.15s;
}

.history-search:focus-within {
  border-color: var(--accent-blue);
}

.search-icon {
  color: var(--text-muted);
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  background: transparent;
  border: none;
  outline: none;
  color: var(--text-primary);
  font-size: 12px;
  min-width: 0;
}

.search-input::placeholder {
  color: var(--text-muted);
}

.commit-list {
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.commit-item {
  display: flex;
  align-items: flex-start;
  padding: 8px 16px 8px 24px;
  position: relative;
  cursor: pointer;
  transition: background-color 0.1s;
}

.commit-item:hover {
  background: var(--bg-hover);
}

.commit-item.selected {
  background: var(--bg-active);
}

.commit-dot {
  position: absolute;
  left: 12px;
  top: 14px;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--accent-blue);
  z-index: 1;
  flex-shrink: 0;
}

.commit-item:first-child .commit-dot {
  background: var(--accent-green);
}

.commit-line {
  position: absolute;
  left: 15px;
  top: 22px;
  bottom: 0;
  width: 2px;
  background: var(--border-default);
}

.commit-item:last-child .commit-line {
  display: none;
}

.commit-content {
  flex: 1;
  min-width: 0;
  margin-left: 8px;
}

.commit-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.commit-message {
  flex: 1;
  font-size: 13px;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.commit-hash {
  flex-shrink: 0;
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--accent-blue);
  background: rgba(56, 139, 253, 0.1);
  padding: 1px 6px;
  border-radius: var(--radius-sm);
}

.commit-meta {
  display: flex;
  gap: 12px;
  margin-top: 2px;
}

.commit-author,
.commit-date {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  color: var(--text-muted);
}

.empty-hint {
  padding: 24px;
  text-align: center;
  color: var(--text-muted);
  font-size: 12px;
}

.commit-detail {
  flex-shrink: 0;
  max-height: 35%;
  border-top: 1px solid var(--border-default);
  display: flex;
  flex-direction: column;
}

.detail-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: var(--bg-tertiary);
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.detail-hash {
  font-family: var(--font-mono);
  color: var(--accent-blue);
  font-weight: 400;
  margin-left: auto;
}

.detail-action {
  color: var(--text-muted);
}

.detail-action:hover {
  color: var(--accent-blue) !important;
}

.detail-files {
  flex: 1;
  overflow-y: auto;
}

.detail-file {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 12px;
  font-size: 12px;
  cursor: pointer;
  transition: background-color 0.1s;
}

.detail-file:hover {
  background: var(--bg-hover);
}

.file-status-badge {
  flex-shrink: 0;
  width: 16px;
  text-align: center;
  font-weight: 700;
  font-size: 11px;
  font-family: var(--font-mono);
}

.detail-file-path {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--text-primary);
}

.file-action {
  opacity: 0;
  transition: opacity 0.15s;
}

.detail-file:hover .file-action {
  opacity: 1;
}
</style>
