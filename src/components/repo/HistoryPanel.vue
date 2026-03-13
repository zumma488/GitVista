<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRepoStore } from '@/stores/repo'
import { GitCommitHorizontal, Code2, User, Calendar, Search, CherryIcon, Undo2, Copy } from 'lucide-vue-next'
import ContextMenu from '@/components/ContextMenu.vue'
import type { MenuItem } from '@/components/ContextMenu.vue'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import VirtualScroller from 'primevue/virtualscroller'

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
    default: return 'var(--p-text-muted-color)'
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
      <InputText
        v-model="searchQuery"
        class="search-input w-full"
        placeholder="搜索提交信息..."
      />
    </div>

    <!-- 提交列表 -->
    <div class="commit-list-container">
      <VirtualScroller
        v-if="repo.commits.length > 0"
        :items="repo.commits"
        :itemSize="56"
        class="h-full w-full scroller-custom"
      >
        <template v-slot:item="{ item: commit, options }">
          <div
            class="commit-item"
            :class="{
              selected: repo.selectedCommitHash === commit.hash,
              'is-first': options.index === 0,
              'is-last': options.index === repo.commits.length - 1
            }"
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
        </template>
      </VirtualScroller>
      <div v-else class="empty-hint">暂无提交历史</div>
    </div>

    <!-- 提交详情 -->
    <div v-if="repo.selectedCommitHash" class="commit-detail">
      <div class="detail-header">
        <GitCommitHorizontal :size="14" />
        <span>提交详情</span>
        <span class="detail-hash">{{ repo.selectedCommitHash.substring(0, 10) }}</span>
        <Button
          variant="text" severity="secondary"
          class="!p-1 h-6 w-6 detail-action ml-2"
          title="Cherry-pick 此提交"
          @click="repo.cherryPick(repo.selectedCommitHash!)"
        >
          <CherryIcon :size="13" />
        </Button>
        <Button
          variant="text" severity="secondary"
          class="!p-1 h-6 w-6 detail-action"
          title="Revert 此提交"
          @click="repo.revertCommit(repo.selectedCommitHash!)"
        >
          <Undo2 :size="13" />
        </Button>
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
          <Button
            variant="text" severity="secondary"
            class="!p-1 h-6 w-6 file-action"
            title="在 VSCode 中查看"
            @click.stop="repo.openInVscode(file.path)"
          >
            <Code2 :size="12" />
          </Button>
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
  background: var(--p-content-background);
  border: 1px solid var(--p-content-border-color);
  border-radius: var(--radius-md);
  flex-shrink: 0;
  transition: border-color 0.15s;
}

.history-search:focus-within {
  border-color: var(--p-primary-color);
}

.search-icon {
  color: var(--p-text-muted-color);
  flex-shrink: 0;
}

.search-input {
  /* 覆盖 PrimeVue InputText 的默认样式及背景 */
  background: transparent !important;
  border: none !important;
  box-shadow: none !important;
  padding: 0 !important;
  outline: none !important;
  color: var(--p-text-color) !important;
  font-size: 13px !important;
  min-width: 0;
}

.search-input::placeholder {
  color: var(--p-text-muted-color);
}

.commit-list-container {
  flex: 1;
  min-height: 0;
}

.scroller-custom {
  border: none !important;
}

.h-full { height: 100%; }
.w-full { width: 100%; box-sizing: border-box; }
.h-6 { height: 1.5rem; }
.w-6 { width: 1.5rem; }
.ml-2 { margin-left: 0.5rem; }

.commit-item {
  display: flex;
  align-items: flex-start;
  padding: 8px 16px 8px 24px;
  position: relative;
  cursor: pointer;
  transition: background-color 0.1s;
  height: 56px;
  box-sizing: border-box;
}

.commit-item:hover {
  background: var(--p-content-hover-background);
}

.commit-item.selected {
  background: var(--p-highlight-background);
}

.commit-dot {
  position: absolute;
  left: 12px;
  top: 14px;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--p-primary-color);
  z-index: 1;
  flex-shrink: 0;
}

.commit-item.is-first .commit-dot {
  background: var(--accent-green);
}

.commit-line {
  position: absolute;
  left: 15px;
  top: 22px;
  bottom: 0;
  width: 2px;
  background: var(--p-content-border-color);
}

.commit-item.is-last .commit-line {
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
  color: var(--p-text-color);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.commit-hash {
  flex-shrink: 0;
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--p-primary-color);
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
  color: var(--p-text-muted-color);
}

.empty-hint {
  padding: 24px;
  text-align: center;
  color: var(--p-text-muted-color);
  font-size: 12px;
}

.commit-detail {
  flex-shrink: 0;
  max-height: 45%;
  border-top: 1px solid var(--p-content-border-color);
  display: flex;
  flex-direction: column;
}

.detail-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: var(--p-content-hover-background);
  font-size: 12px;
  font-weight: 600;
  color: var(--p-text-muted-color);
  flex-shrink: 0;
}

.detail-hash {
  font-family: var(--font-mono);
  color: var(--p-primary-color);
  font-weight: 400;
  margin-left: auto;
}

.detail-action {
  color: var(--p-text-muted-color);
}

.detail-action:hover {
  color: var(--p-text-color) !important;
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
  background: var(--p-content-hover-background);
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
