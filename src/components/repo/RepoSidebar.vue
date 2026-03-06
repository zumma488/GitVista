<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRepoStore } from '@/stores/repo'
import {
  GitBranch, FileText, History, Plus, Trash2, ChevronDown, ChevronRight,
  Search, Archive, Play, ArrowDownToLine, GitMerge, Download, PenLine, LogIn,
} from 'lucide-vue-next'
import ContextMenu from '@/components/ContextMenu.vue'
import type { MenuItem } from '@/components/ContextMenu.vue'
import type { BranchInfo } from '@/types'

const repo = useRepoStore()
const showNewBranch = ref(false)
const newBranchName = ref('')
const branchesExpanded = ref(true)
const remoteBranchesExpanded = ref(false)
const stashesExpanded = ref(false)
const branchSearch = ref('')
const showStashInput = ref(false)
const stashMessage = ref('')
const showMergeSelect = ref(false)

const filteredLocalBranches = computed(() => {
  const q = branchSearch.value.trim().toLowerCase()
  if (!q) return repo.localBranches
  return repo.localBranches.filter(b => b.name.toLowerCase().includes(q))
})

const filteredRemoteBranches = computed(() => {
  const q = branchSearch.value.trim().toLowerCase()
  if (!q) return repo.remoteBranches
  return repo.remoteBranches.filter(b => b.name.toLowerCase().includes(q))
})

const mergableBranches = computed(() =>
  repo.localBranches.filter(b => !b.is_current)
)

async function handleCreateBranch() {
  if (!newBranchName.value.trim()) return
  await repo.createBranch(newBranchName.value.trim())
  newBranchName.value = ''
  showNewBranch.value = false
}

async function handleStashSave() {
  await repo.stashSave(stashMessage.value.trim() || undefined)
  stashMessage.value = ''
  showStashInput.value = false
  stashesExpanded.value = true
}

async function handleMerge(branch: string) {
  showMergeSelect.value = false
  await repo.mergeBranch(branch)
}

function checkoutRemoteBranch(remoteName: string) {
  const localName = remoteName.replace(/^origin\//, '')
  repo.checkoutBranch(localName)
}

const branchCtxMenu = ref<InstanceType<typeof ContextMenu>>()
const branchCtxMenuItems = ref<MenuItem[]>([])

function showBranchCtxMenu(e: MouseEvent, branch: BranchInfo) {
  const items: MenuItem[] = []
  if (!branch.is_current) {
    items.push({ label: '切换到此分支', icon: LogIn, action: () => repo.checkoutBranch(branch.name) })
  }
  items.push({ label: '重命名', icon: PenLine, action: () => {
    const newName = prompt('输入新分支名称', branch.name)
    if (newName && newName !== branch.name) {
      repo.renameBranch(branch.name, newName)
    }
  }})
  if (!branch.is_current) {
    items.push({ label: '', action: () => {}, divider: true })
    items.push({ label: '删除分支', icon: Trash2, action: () => repo.deleteBranch(branch.name), danger: true })
  }
  branchCtxMenuItems.value = items
  branchCtxMenu.value?.open(e)
}
</script>

<template>
  <div class="sidebar">
    <!-- 导航 Tab -->
    <div class="sidebar-tabs">
      <button
        class="sidebar-tab"
        :class="{ active: repo.activeTab === 'changes' }"
        @click="repo.activeTab = 'changes'"
      >
        <FileText :size="14" />
        变更
        <span v-if="repo.files.length > 0" class="tab-badge">{{ repo.files.length }}</span>
      </button>
      <button
        class="sidebar-tab"
        :class="{ active: repo.activeTab === 'history' }"
        @click="repo.activeTab = 'history'"
      >
        <History :size="14" />
        历史
      </button>
    </div>

    <!-- 分支搜索 -->
    <div class="branch-search">
      <Search :size="13" class="search-icon" />
      <input
        v-model="branchSearch"
        class="search-input"
        placeholder="搜索分支..."
      />
    </div>

    <!-- 本地分支 -->
    <div class="sidebar-section">
      <div class="section-header" @click="branchesExpanded = !branchesExpanded">
        <component :is="branchesExpanded ? ChevronDown : ChevronRight" :size="14" />
        <span class="section-title">本地分支</span>
        <button class="btn-icon section-action" title="新建分支" @click.stop="showNewBranch = !showNewBranch">
          <Plus :size="12" />
        </button>
      </div>

      <div v-if="showNewBranch" class="new-branch-input">
        <input
          v-model="newBranchName"
          class="input-sm"
          placeholder="分支名称"
          @keyup.enter="handleCreateBranch"
          @keyup.escape="showNewBranch = false"
        />
      </div>

      <div v-if="branchesExpanded" class="branch-list">
        <div
          v-for="branch in filteredLocalBranches"
          :key="branch.name"
          class="branch-item"
          :class="{ current: branch.is_current }"
          @click="!branch.is_current && repo.checkoutBranch(branch.name)"
          @contextmenu.prevent="showBranchCtxMenu($event, branch)"
        >
          <GitBranch :size="13" />
          <span class="branch-name">{{ branch.name }}</span>
          <button
            v-if="!branch.is_current"
            class="btn-icon branch-delete"
            title="删除分支"
            @click.stop="repo.deleteBranch(branch.name)"
          >
            <Trash2 :size="11" />
          </button>
        </div>
        <div v-if="filteredLocalBranches.length === 0 && branchSearch" class="branch-empty">
          无匹配分支
        </div>
      </div>
    </div>

    <!-- 远程分支 -->
    <div v-if="repo.remoteBranches.length > 0" class="sidebar-section">
      <div class="section-header" @click="remoteBranchesExpanded = !remoteBranchesExpanded">
        <component :is="remoteBranchesExpanded ? ChevronDown : ChevronRight" :size="14" />
        <span class="section-title">远程分支</span>
      </div>

      <div v-if="remoteBranchesExpanded" class="branch-list">
        <div
          v-for="branch in filteredRemoteBranches"
          :key="branch.name"
          class="branch-item remote"
          title="点击检出到本地"
          @click="checkoutRemoteBranch(branch.name)"
        >
          <Download :size="13" />
          <span class="branch-name">{{ branch.name }}</span>
        </div>
        <div v-if="filteredRemoteBranches.length === 0 && branchSearch" class="branch-empty">
          无匹配分支
        </div>
      </div>
    </div>

    <!-- 合并分支 -->
    <div class="sidebar-section">
      <div class="section-header" @click="showMergeSelect = !showMergeSelect">
        <GitMerge :size="14" />
        <span class="section-title">合并分支</span>
      </div>
      <div v-if="showMergeSelect" class="branch-list">
        <div
          v-for="branch in mergableBranches"
          :key="'merge-' + branch.name"
          class="branch-item"
          @click="handleMerge(branch.name)"
        >
          <GitBranch :size="13" />
          <span class="branch-name">{{ branch.name }}</span>
          <span class="merge-hint">合并到当前</span>
        </div>
        <div v-if="mergableBranches.length === 0" class="branch-empty">无可合并分支</div>
      </div>
    </div>

    <!-- Stash 暂存 -->
    <div class="sidebar-section sidebar-section-grow">
      <div class="section-header" @click="stashesExpanded = !stashesExpanded">
        <component :is="stashesExpanded ? ChevronDown : ChevronRight" :size="14" />
        <span class="section-title">Stash</span>
        <span v-if="repo.stashes.length > 0" class="stash-count">{{ repo.stashes.length }}</span>
        <button class="btn-icon section-action" title="暂存当前更改" @click.stop="showStashInput = !showStashInput">
          <Plus :size="12" />
        </button>
      </div>

      <div v-if="showStashInput" class="new-branch-input">
        <input
          v-model="stashMessage"
          class="input-sm"
          placeholder="stash 备注（可选）"
          @keyup.enter="handleStashSave"
          @keyup.escape="showStashInput = false"
        />
      </div>

      <div v-if="stashesExpanded" class="branch-list">
        <div
          v-for="stash in repo.stashes"
          :key="stash.index"
          class="stash-item"
        >
          <Archive :size="13" />
          <span class="branch-name">{{ stash.message }}</span>
          <div class="stash-actions">
            <button class="btn-icon" title="应用 (保留)" @click.stop="repo.stashApply(stash.index)">
              <Play :size="11" />
            </button>
            <button class="btn-icon" title="弹出 (删除)" @click.stop="repo.stashPop(stash.index)">
              <ArrowDownToLine :size="11" />
            </button>
            <button class="btn-icon" title="删除" @click.stop="repo.stashDrop(stash.index)">
              <Trash2 :size="11" />
            </button>
          </div>
        </div>
        <div v-if="repo.stashes.length === 0" class="branch-empty">暂无 stash</div>
      </div>
    </div>

    <ContextMenu ref="branchCtxMenu" :items="branchCtxMenuItems" />
  </div>
</template>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  height: 100%;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-default);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  flex-shrink: 0;
}

.sidebar-tabs {
  display: flex;
  border-bottom: 1px solid var(--border-default);
  flex-shrink: 0;
}

.sidebar-tab {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px 0;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  border-bottom: 2px solid transparent;
  transition: color 0.15s, border-color 0.15s;
}

.sidebar-tab:hover {
  color: var(--text-primary);
}

.sidebar-tab.active {
  color: var(--accent-blue);
  border-bottom-color: var(--accent-blue);
}

.tab-badge {
  background: var(--accent-blue);
  color: #fff;
  font-size: 10px;
  padding: 0 5px;
  border-radius: 8px;
  min-width: 16px;
  text-align: center;
  line-height: 16px;
}

/* ===== 分支搜索 ===== */
.branch-search {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 8px 12px;
  padding: 5px 8px;
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  transition: border-color 0.15s;
}

.branch-search:focus-within {
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

.branch-empty {
  padding: 8px 28px;
  font-size: 12px;
  color: var(--text-muted);
}

.sidebar-section {
  border-bottom: 1px solid var(--border-muted);
}

.section-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  cursor: pointer;
  color: var(--text-secondary);
  transition: color 0.15s;
}

.section-header:hover {
  color: var(--text-primary);
}

.section-title {
  flex: 1;
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.section-action {
  opacity: 0;
  transition: opacity 0.15s;
}

.section-header:hover .section-action {
  opacity: 1;
}

.new-branch-input {
  padding: 0 12px 8px;
}

.input-sm {
  width: 100%;
  padding: 4px 8px;
  background: var(--bg-primary);
  border: 1px solid var(--accent-blue);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  font-size: 12px;
  outline: none;
}

.branch-list {
  max-height: 200px;
  overflow-y: auto;
}

.branch-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 5px 12px 5px 28px;
  font-size: 12px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background-color 0.1s, color 0.1s;
}

.branch-item:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.branch-item.current {
  color: var(--accent-blue);
  font-weight: 500;
  cursor: default;
}

.branch-item.remote {
  color: var(--text-muted);
}

.branch-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.branch-delete {
  opacity: 0;
  transition: opacity 0.15s;
  color: var(--text-muted);
}

.branch-item:hover .branch-delete {
  opacity: 1;
}

.branch-delete:hover {
  color: var(--accent-red) !important;
}

.branch-item.remote {
  cursor: pointer;
}

.branch-item.remote:hover {
  color: var(--accent-blue);
}

.merge-hint {
  font-size: 10px;
  color: var(--text-muted);
  margin-left: auto;
  flex-shrink: 0;
}

.sidebar-section-grow {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.sidebar-section-grow .branch-list {
  flex: 1;
  overflow-y: auto;
}

.stash-count {
  font-size: 10px;
  background: var(--accent-purple);
  color: #fff;
  padding: 0 5px;
  border-radius: 8px;
  min-width: 16px;
  text-align: center;
  line-height: 16px;
}

.stash-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 5px 12px 5px 28px;
  font-size: 12px;
  color: var(--text-secondary);
  transition: background-color 0.1s;
}

.stash-item:hover {
  background: var(--bg-hover);
}

.stash-actions {
  display: flex;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.15s;
  flex-shrink: 0;
}

.stash-item:hover .stash-actions {
  opacity: 1;
}
</style>
