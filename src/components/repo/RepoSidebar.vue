<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRepoStore } from '@/stores/repo'
import {
  GitBranch, FileText, History, Plus, Trash2,
  Search, Archive, Play, ArrowDownToLine, GitMerge, Download, PenLine, LogIn,
} from 'lucide-vue-next'
import ContextMenu from '@/components/ContextMenu.vue'
import CreateBranchDialog from '@/components/CreateBranchDialog.vue'
import type { MenuItem } from '@/components/ContextMenu.vue'
import type { BranchInfo } from '@/types'

import Accordion from 'primevue/accordion'
import AccordionPanel from 'primevue/accordionpanel'
import AccordionHeader from 'primevue/accordionheader'
import AccordionContent from 'primevue/accordioncontent'
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import VirtualScroller from 'primevue/virtualscroller'

const repo = useRepoStore()
const createBranchDialog = ref<InstanceType<typeof CreateBranchDialog>>()

const activePanels = ref(['local']) // 默认展开本地分支

const branchSearch = ref('')
const showStashInput = ref(false)
const stashMessage = ref('')

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

async function handleCreateBranch(name: string) {
  await repo.createBranch(name)
}

async function handleStashSave() {
  await repo.stashSave(stashMessage.value.trim() || undefined)
  stashMessage.value = ''
  showStashInput.value = false
}

async function handleMerge(branch: string) {
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

// ===== 拖拽改变宽度 =====
const MIN_WIDTH = 200
const MAX_WIDTH = 800
const DEFAULT_WIDTH = 346 // 260px 的基础上加 1/3
const sidebarWidth = ref(DEFAULT_WIDTH)
const isResizing = ref(false)

const savedWidth = localStorage.getItem('repo_sidebar_width')
if (savedWidth) {
  sidebarWidth.value = parseInt(savedWidth, 10)
}

function startResize() {
  isResizing.value = true
  document.addEventListener('mousemove', handleResize)
  document.addEventListener('mouseup', stopResize)
  // 防止拖拽时选中文本
  document.body.style.userSelect = 'none'
  document.body.style.cursor = 'col-resize'
}

function handleResize(e: MouseEvent) {
  if (!isResizing.value) return
  // sidebar 在左侧，鼠标 X 坐标差不多就是宽度（不考虑复杂的嵌套偏移，这里近似即可）
  let newWidth = e.clientX
  if (newWidth < MIN_WIDTH) newWidth = MIN_WIDTH
  if (newWidth > MAX_WIDTH) newWidth = MAX_WIDTH
  sidebarWidth.value = newWidth
}

function stopResize() {
  isResizing.value = false
  document.removeEventListener('mousemove', handleResize)
  document.removeEventListener('mouseup', stopResize)
  document.body.style.userSelect = ''
  document.body.style.cursor = ''
  localStorage.setItem('repo_sidebar_width', sidebarWidth.value.toString())
}

import { onMounted, onUnmounted } from 'vue'
const windowHeight = ref(window.innerHeight)

function updateWindowHeight() {
  windowHeight.value = window.innerHeight
}

onMounted(() => {
  window.addEventListener('resize', updateWindowHeight)
})

onUnmounted(() => {
  window.removeEventListener('resize', updateWindowHeight)
})
</script>

<template>
  <div class="sidebar" :style="{ width: `${sidebarWidth}px` }">
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
      <InputText
        v-model="branchSearch"
        class="search-input w-full"
        placeholder="搜索分支..."
      />
    </div>

    <div class="sidebar-scrollable flex-1 overflow-y-auto">
      <Accordion multiple v-model:value="activePanels" class="custom-accordion">

        <!-- 本地分支 -->
        <AccordionPanel value="local">
          <AccordionHeader>
            <div class="flex items-center w-full section-header-custom">
              <span class="section-title">本地分支</span>
              <Button
                variant="text" severity="secondary"
                class="!p-1 h-6 w-6 ml-auto section-action"
                title="新建分支"
                @click.stop="createBranchDialog?.show()"
              >
                <Plus :size="12" />
              </Button>
            </div>
          </AccordionHeader>
          <AccordionContent>
            <VirtualScroller
              v-if="filteredLocalBranches.length > 0"
              :items="filteredLocalBranches"
              :itemSize="28"
              class="branch-list-scroller"
              :style="{ height: Math.min(filteredLocalBranches.length * 28, windowHeight * 0.4) + 'px' }"
            >
              <template #item="{ item: branch }">
                <div
                  class="branch-item"
                  :class="{ current: branch.is_current }"
                  @click="!branch.is_current && repo.checkoutBranch(branch.name)"
                  @contextmenu.prevent="showBranchCtxMenu($event, branch)"
                >
                  <GitBranch :size="13" />
                  <span class="branch-name" :title="branch.name">{{ branch.name }}</span>
                  <Button
                    v-if="!branch.is_current"
                    variant="text" severity="danger"
                    class="!p-1 h-6 w-6 branch-delete"
                    title="删除分支"
                    @click.stop="repo.deleteBranch(branch.name)"
                  >
                    <Trash2 :size="11" />
                  </Button>
                </div>
              </template>
            </VirtualScroller>
            <div v-if="filteredLocalBranches.length === 0 && branchSearch" class="branch-empty">
              无匹配分支
            </div>
          </AccordionContent>
        </AccordionPanel>

        <!-- 远程分支 -->
        <AccordionPanel value="remote" v-if="repo.remoteBranches.length > 0">
          <AccordionHeader>
            <span class="section-title">远程分支</span>
          </AccordionHeader>
          <AccordionContent>
            <VirtualScroller
              v-if="filteredRemoteBranches.length > 0"
              :items="filteredRemoteBranches"
              :itemSize="28"
              class="branch-list-scroller"
              :style="{ height: Math.min(filteredRemoteBranches.length * 28, windowHeight * 0.4) + 'px' }"
            >
              <template #item="{ item: branch }">
                <div
                  class="branch-item remote"
                  title="点击检出到本地"
                  @click="checkoutRemoteBranch(branch.name)"
                >
                  <Download :size="13" />
                  <span class="branch-name" :title="branch.name">{{ branch.name }}</span>
                </div>
              </template>
            </VirtualScroller>
            <div v-if="filteredRemoteBranches.length === 0 && branchSearch" class="branch-empty">
              无匹配分支
            </div>
          </AccordionContent>
        </AccordionPanel>

        <!-- 合并分支 -->
        <AccordionPanel value="merge">
          <AccordionHeader>
            <div class="flex items-center gap-1">
              <GitMerge :size="12" class="mr-1" />
              <span class="section-title">合并分支</span>
            </div>
          </AccordionHeader>
          <AccordionContent>
            <VirtualScroller
              v-if="mergableBranches.length > 0"
              :items="mergableBranches"
              :itemSize="28"
              class="branch-list-scroller"
              :style="{ height: Math.min(mergableBranches.length * 28, windowHeight * 0.4) + 'px' }"
            >
              <template #item="{ item: branch }">
                <div
                  class="branch-item"
                  @click="handleMerge(branch.name)"
                >
                  <GitBranch :size="13" />
                  <span class="branch-name" :title="branch.name">{{ branch.name }}</span>
                  <span class="merge-hint">合并到当前</span>
                </div>
              </template>
            </VirtualScroller>
            <div v-if="mergableBranches.length === 0" class="branch-empty">无可合并分支</div>
          </AccordionContent>
        </AccordionPanel>

        <!-- Stash 暂存 -->
        <AccordionPanel value="stash">
          <AccordionHeader>
            <div class="flex items-center w-full section-header-custom">
              <span class="section-title">Stash</span>
              <span v-if="repo.stashes.length > 0" class="stash-count ml-1">{{ repo.stashes.length }}</span>
              <Button
                variant="text" severity="secondary"
                class="!p-1 h-6 w-6 ml-auto section-action"
                title="暂存当前更改"
                @click.stop="showStashInput = !showStashInput"
              >
                <Plus :size="12" />
              </Button>
            </div>
          </AccordionHeader>
          <AccordionContent>
            <div v-if="showStashInput" class="new-branch-input mb-2 mt-1 px-3">
              <InputText
                v-model="stashMessage"
                class="w-full !text-xs !py-1 !px-2"
                placeholder="stash 备注（可选）"
                @keyup.enter="handleStashSave"
                @keyup.escape="showStashInput = false"
              />
            </div>

<!-- 暂时不对 Stash 进行虚拟化，因为多数人的 stash 不会达到成百上千那么多，保留原样 -->
            <div class="branch-list">
              <div
                v-for="stash in repo.stashes"
                :key="stash.index"
                class="stash-item"
              >
                <Archive :size="13" />
                <span class="branch-name" :title="stash.message">{{ stash.message }}</span>
                <div class="stash-actions">
                  <Button variant="text" severity="secondary" class="!p-1 h-5 w-5" title="应用 (保留)" @click.stop="repo.stashApply(stash.index)">
                    <Play :size="11" />
                  </Button>
                  <Button variant="text" severity="secondary" class="!p-1 h-5 w-5" title="弹出 (删除)" @click.stop="repo.stashPop(stash.index)">
                    <ArrowDownToLine :size="11" />
                  </Button>
                  <Button variant="text" severity="danger" class="!p-1 h-5 w-5" title="删除" @click.stop="repo.stashDrop(stash.index)">
                    <Trash2 :size="11" />
                  </Button>
                </div>
              </div>
              <div v-if="repo.stashes.length === 0" class="branch-empty">暂无 stash</div>
            </div>
          </AccordionContent>
        </AccordionPanel>

      </Accordion>
    </div>

    <ContextMenu ref="branchCtxMenu" :items="branchCtxMenuItems" />
    <CreateBranchDialog ref="createBranchDialog" @submit="handleCreateBranch" />

    <!-- 拖拽调整宽度的把手 -->
    <div
      class="sidebar-resizer"
      :class="{ active: isResizing }"
      @mousedown.prevent="startResize"
    ></div>
  </div>
</template>

<style scoped>
.sidebar {
  /* width: var(--sidebar-width); <- 改为行内样式动态设置 */
  position: relative;
  height: 100%;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-default);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  flex-shrink: 0;
  will-change: width;
}

.sidebar-resizer {
  position: absolute;
  top: 0;
  right: -3px; /* 稍微伸出右侧以增加命中区 */
  width: 6px;
  height: 100%;
  cursor: col-resize;
  z-index: 10;
  transition: background-color 0.15s;
}

.sidebar-resizer:hover, .sidebar-resizer.active {
  background-color: var(--accent-blue);
  opacity: 0.5;
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
  flex-shrink: 0;
}

.branch-search:focus-within {
  border-color: var(--accent-blue);
}

.search-icon {
  color: var(--text-muted);
  flex-shrink: 0;
}

.search-input {
  /* 覆盖 PrimeVue InputText 的默认样式及背景 */
  background: transparent !important;
  border: none !important;
  box-shadow: none !important;
  padding: 0 !important;
  outline: none !important;
  color: var(--text-primary) !important;
  font-size: 12px !important;
  min-width: 0;
}

.search-input::placeholder {
  color: var(--text-muted);
}

.branch-empty {
  padding: 8px 12px 8px 28px;
  font-size: 12px;
  color: var(--text-muted);
}

.sidebar-scrollable {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  /* 美化主滚动条 */
  scrollbar-width: thin;
  scrollbar-color: var(--border-default) transparent;
}

.sidebar-scrollable::-webkit-scrollbar {
  width: 4px;
}

.sidebar-scrollable::-webkit-scrollbar-thumb {
  background-color: var(--border-default);
  border-radius: 4px;
}

.branch-list-scroller {
  /* 使用 max-height 以配合 autoSize 实现自适应不溢出 */
  max-height: 40vh;
  overflow-x: hidden;
  /* 美化内部滚动条 */
  scrollbar-width: thin;
  scrollbar-color: var(--border-default) transparent;
}

.branch-list-scroller::-webkit-scrollbar {
  width: 4px;
}

.branch-list-scroller::-webkit-scrollbar-thumb {
  background-color: var(--border-default);
  border-radius: 4px;
}

/* Accordion 覆写 */
:deep(.p-accordionpanel) {
  border: none !important;
  border-bottom: 1px solid var(--border-muted) !important;
  background: transparent !important;
}

:deep(.p-accordionheader) {
  padding: 8px 12px !important;
  background: transparent !important;
  border: none !important;
  color: var(--text-secondary) !important;
}

:deep(.p-accordionheader:hover) {
  color: var(--text-primary) !important;
}

:deep(.p-accordioncontent-content) {
  padding: 0 0 8px 0 !important;
  background: transparent !important;
}

.section-title {
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--text-secondary);
}

.section-header-custom {
  position: relative;
}

.section-action {
  opacity: 0;
  transition: opacity 0.15s;
}

:deep(.p-accordionheader:hover) .section-action {
  opacity: 1;
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
}

.branch-item:hover .branch-delete {
  opacity: 1;
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
