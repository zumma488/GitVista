<script setup lang="ts">
import { ref } from 'vue'
import { useRepoStore } from '@/stores/repo'
import {
  ChevronDown,
  Plus,
  Minus,
  Undo2,
  Code2,
  CheckCircle2,
  Trash2,
  PenLine,
} from 'lucide-vue-next'
import type { FileChange } from '@/types'
import ContextMenu from '@/components/ContextMenu.vue'
import type { MenuItem } from '@/components/ContextMenu.vue'
import Button from 'primevue/button'
import Textarea from 'primevue/textarea'
import VirtualScroller from 'primevue/virtualscroller'
import Menu from 'primevue/menu'
import { MoreVertical } from 'lucide-vue-next'
import { useConfirm } from 'primevue/useconfirm'

const repo = useRepoStore()
const amendMode = ref(false)
const ctxMenu = ref<InstanceType<typeof ContextMenu>>()
const ctxMenuItems = ref<MenuItem[]>([])
const moreMenuInfo = ref()
const confirm = useConfirm()

const moreMenuOptions = ref([
  {
    label: '危险操作',
    items: [
      {
        label: '修改上次提交 (Amend)',
        icon: 'pi pi-pencil',
        command: (event: { originalEvent: Event }) => {
          confirm.require({
            target: event.originalEvent.currentTarget as HTMLElement,
            message: '确定要进入修改上次提交模式吗？',
            icon: 'pi pi-exclamation-triangle',
            acceptProps: { label: '确认', severity: 'danger' },
            rejectProps: { label: '取消', severity: 'secondary', variant: 'outlined' },
            accept: () => { amendMode.value = true }
          })
        }
      },
      {
        label: '撤销上次提交',
        icon: 'pi pi-undo',
        command: (event: { originalEvent: Event }) => {
          confirm.require({
            target: event.originalEvent.currentTarget as HTMLElement,
            message: '确定要撤销上次提交吗？（更改将退回至暂存区）',
            icon: 'pi pi-exclamation-triangle',
            acceptProps: { label: '确认', severity: 'danger' },
            rejectProps: { label: '取消', severity: 'secondary', variant: 'outlined' },
            accept: () => { repo.undoLastCommit() }
          })
        }
      }
    ]
  }
])

const toggleMoreMenu = (event: Event) => {
  moreMenuInfo.value.toggle(event)
}

function showStagedCtxMenu(e: MouseEvent, file: FileChange) {
  ctxMenuItems.value = [
    { label: '取消暂存', icon: Minus, action: () => repo.unstageFiles([file.path]) },
    { label: '在 VSCode 中打开', icon: Code2, action: () => repo.openInVscode(file.path) },
  ]
  ctxMenu.value?.open(e)
}

function showUnstagedCtxMenu(e: MouseEvent, file: FileChange) {
  ctxMenuItems.value = [
    { label: '暂存', icon: Plus, action: () => repo.stageFiles([file.path]) },
    { label: '在 VSCode 中打开', icon: Code2, action: () => repo.openInVscode(file.path) },
    { label: '', action: () => {}, divider: true },
    { label: '丢弃更改', icon: Trash2, action: () => repo.discardChanges([file.path]), danger: true },
  ]
  ctxMenu.value?.open(e)
}

function statusIcon(status: string): string {
  switch (status) {
    case 'modified': return 'M'
    case 'added': return 'A'
    case 'deleted': return 'D'
    case 'renamed': return 'R'
    case 'untracked': return 'U'
    case 'conflicted': return 'C'
    default: return '?'
  }
}

function statusColor(status: string): string {
  switch (status) {
    case 'modified': return 'var(--accent-yellow)'
    case 'added':
    case 'untracked': return 'var(--accent-green)'
    case 'deleted': return 'var(--accent-red)'
    case 'renamed': return 'var(--accent-purple)'
    case 'conflicted': return 'var(--accent-orange)'
    default: return 'var(--p-text-muted-color)'
  }
}

function fileName(path: string): string {
  const parts = path.split('/')
  return parts[parts.length - 1] ?? path
}

function fileDir(path: string): string {
  const parts = path.split('/')
  if (parts.length <= 1) return ''
  return parts.slice(0, -1).join('/') + '/'
}

function openFileInVscode(file: FileChange) {
  repo.openInVscode(file.path)
}
</script>

<template>
  <div class="changes-panel">
    <!-- 已暂存区域 -->
    <div class="file-section">
      <div class="section-header">
        <ChevronDown :size="14" />
        <span class="section-title">已暂存的更改</span>
        <span class="file-count">{{ repo.stagedFiles.length }}</span>
        <Button
          variant="text" severity="secondary"
          title="取消暂存全部"
          :disabled="repo.stagedFiles.length === 0"
          @click="repo.unstageAll()"
          class="!p-1 h-6 w-6"
        >
          <Minus :size="13" />
        </Button>
      </div>
      <div class="file-list-container">
        <VirtualScroller v-if="repo.stagedFiles.length > 0" :items="repo.stagedFiles" :itemSize="28" class="h-full w-full scroller-custom">
          <template v-slot:item="{ item: file }">
            <div
              class="file-item"
              @dblclick="openFileInVscode(file)"
              @contextmenu.prevent="showStagedCtxMenu($event, file)"
            >
              <span class="file-status" :style="{ color: statusColor(file.status) }">
                {{ statusIcon(file.status) }}
              </span>
              <span class="file-dir">{{ fileDir(file.path) }}</span>
              <span class="file-name">{{ fileName(file.path) }}</span>
              <div class="file-actions">
                <Button variant="text" severity="secondary" class="!p-1 h-6 w-6" title="在 VSCode 中查看" @click.stop="openFileInVscode(file)">
                  <Code2 :size="12" />
                </Button>
                <Button variant="text" severity="secondary" class="!p-1 h-6 w-6" title="取消暂存" @click.stop="repo.unstageFiles([file.path])">
                  <Minus :size="12" />
                </Button>
              </div>
            </div>
          </template>
        </VirtualScroller>
        <div v-else class="empty-hint">暂无已暂存的文件</div>
      </div>
    </div>

    <!-- 未暂存区域 -->
    <div class="file-section">
      <div class="section-header">
        <ChevronDown :size="14" />
        <span class="section-title">未暂存的更改</span>
        <span class="file-count">{{ repo.unstagedFiles.length }}</span>
        <Button
          variant="text" severity="secondary"
          title="丢弃所有更改"
          :disabled="repo.unstagedFiles.length === 0"
          @click="repo.discardAllChanges()"
          class="!p-1 h-6 w-6"
        >
          <Trash2 :size="13" />
        </Button>
        <Button
          variant="text" severity="secondary"
          title="暂存全部"
          :disabled="repo.unstagedFiles.length === 0"
          @click="repo.stageAll()"
          class="!p-1 h-6 w-6"
        >
          <Plus :size="13" />
        </Button>
      </div>
      <div class="file-list-container">
        <VirtualScroller v-if="repo.unstagedFiles.length > 0" :items="repo.unstagedFiles" :itemSize="28" class="h-full w-full scroller-custom">
          <template v-slot:item="{ item: file }">
            <div
              class="file-item"
              @dblclick="openFileInVscode(file)"
              @contextmenu.prevent="showUnstagedCtxMenu($event, file)"
            >
              <span class="file-status" :style="{ color: statusColor(file.status) }">
                {{ statusIcon(file.status) }}
              </span>
              <span class="file-dir">{{ fileDir(file.path) }}</span>
              <span class="file-name">{{ fileName(file.path) }}</span>
              <div class="file-actions">
                <Button variant="text" severity="secondary" class="!p-1 h-6 w-6" title="在 VSCode 中查看" @click.stop="openFileInVscode(file)">
                  <Code2 :size="12" />
                </Button>
                <Button variant="text" severity="secondary" class="!p-1 h-6 w-6" title="丢弃更改" @click.stop="repo.discardChanges([file.path])">
                  <Undo2 :size="12" />
                </Button>
                <Button variant="text" severity="secondary" class="!p-1 h-6 w-6" title="暂存" @click.stop="repo.stageFiles([file.path])">
                  <Plus :size="12" />
                </Button>
              </div>
            </div>
          </template>
        </VirtualScroller>
        <div v-else class="empty-hint">工作区干净，没有更改</div>
      </div>
    </div>

    <!-- 提交区域 -->
    <div class="commit-area">
      <Textarea
        v-model="repo.commitMessage"
        class="commit-input w-full"
        :placeholder="amendMode ? '修改上次提交信息...' : '输入提交信息...'"
        rows="3"
        autoResize
        @keydown.ctrl.enter="amendMode ? repo.commitAmend(repo.commitMessage) : repo.commit()"
        @keydown.meta.enter="amendMode ? repo.commitAmend(repo.commitMessage) : repo.commit()"
      />
      <div class="commit-actions flex items-center gap-2 mt-2">
        <div class="commit-options flex gap-1">
          <Button
            variant="text"
            severity="secondary"
            title="更多选项"
            @click="toggleMoreMenu"
            class="!p-2"
          >
            <MoreVertical :size="14" />
          </Button>
          <Menu ref="moreMenuInfo" :model="moreMenuOptions" :popup="true" />
        </div>
        <Button
          v-if="amendMode"
          severity="secondary"
          :disabled="!repo.commitMessage.trim() || repo.operating"
          @click="repo.commitAmend(repo.commitMessage)"
          class="flex-1"
        >
          <PenLine :size="14" class="mr-2" />
          Amend
        </Button>
        <Button
          v-else
          :disabled="repo.stagedFiles.length === 0 || !repo.commitMessage.trim() || repo.operating"
          @click="repo.commit()"
          class="flex-1"
        >
          <CheckCircle2 :size="14" class="mr-2" />
          提交 ({{ repo.stagedFiles.length }} 个文件)
        </Button>
      </div>
    </div>

    <ContextMenu ref="ctxMenu" :items="ctxMenuItems" />
  </div>
</template>

<style scoped>
.changes-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.file-section {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  border-bottom: 1px solid var(--p-content-border-color);
}

.section-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  font-size: 12px;
  color: var(--p-text-muted-color);
  background: var(--p-content-hover-background);
  flex-shrink: 0;
}

.section-title {
  font-weight: 600;
  flex: 1;
}

.file-count {
  font-size: 11px;
  background: var(--p-content-hover-background);
  padding: 0 6px;
  border-radius: 8px;
  color: var(--p-text-muted-color);
  line-height: 18px;
}

.file-list-container {
  flex: 1;
  min-height: 0; /* Important for flex child with scrolling */
}

/* 覆盖 VirtualScroller 的默认样式，去边框，高度直接用 100% */
.scroller-custom {
  border: none !important;
}

.h-full { height: 100%; }
.w-full { width: 100%; box-sizing: border-box; }
.h-6 { height: 1.5rem; }
.w-6 { width: 1.5rem; }
.flex-1 { flex: 1; justify-content: center; }
.flex { display: flex; }
.items-center { align-items: center; }
.gap-1 { gap: 0.25rem; }
.gap-2 { gap: 0.5rem; }
.mt-2 { margin-top: 0.5rem; }
.mr-2 { margin-right: 0.5rem; }

.file-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 0 12px;
  font-size: 12px;
  height: 28px;
  cursor: pointer;
  transition: background-color 0.1s;
}

.file-item:hover {
  background: var(--p-content-hover-background);
}

.file-status {
  flex-shrink: 0;
  width: 16px;
  text-align: center;
  font-weight: 700;
  font-size: 11px;
  font-family: var(--font-mono);
}

.file-dir {
  color: var(--p-text-muted-color);
  font-size: 11px;
  white-space: nowrap;
}

.file-name {
  color: var(--p-text-color);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
}

.file-actions {
  display: flex;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.15s;
  flex-shrink: 0;
  align-items: center;
}

.file-item:hover .file-actions {
  opacity: 1;
}

.empty-hint {
  padding: 16px;
  text-align: center;
  color: var(--p-text-muted-color);
  font-size: 12px;
}

.commit-area {
  flex-shrink: 0;
  padding: 12px;
  background: var(--p-content-hover-background);
}

.commit-input {
  background: var(--p-content-background);
  border: 1px solid var(--p-content-border-color);
  color: var(--p-text-color);
  font-size: 13px;
  font-family: var(--font-ui);
}

.commit-input:focus {
  border-color: var(--p-primary-color);
}

.commit-input::placeholder {
  color: var(--p-text-muted-color);
}
</style>
