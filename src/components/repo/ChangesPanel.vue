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
  RotateCcw,
  PenLine,
} from 'lucide-vue-next'
import type { FileChange } from '@/types'

const repo = useRepoStore()
const amendMode = ref(false)

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
    default: return 'var(--text-muted)'
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
        <button
          v-if="repo.stagedFiles.length > 0"
          class="btn-icon"
          title="取消暂存全部"
          @click="repo.unstageAll()"
        >
          <Minus :size="13" />
        </button>
      </div>
      <div class="file-list">
        <div
          v-for="file in repo.stagedFiles"
          :key="'staged-' + file.path"
          class="file-item"
          @dblclick="openFileInVscode(file)"
        >
          <span class="file-status" :style="{ color: statusColor(file.status) }">
            {{ statusIcon(file.status) }}
          </span>
          <span class="file-dir">{{ fileDir(file.path) }}</span>
          <span class="file-name">{{ fileName(file.path) }}</span>
          <div class="file-actions">
            <button class="btn-icon" title="在 VSCode 中查看" @click.stop="openFileInVscode(file)">
              <Code2 :size="12" />
            </button>
            <button class="btn-icon" title="取消暂存" @click.stop="repo.unstageFiles([file.path])">
              <Minus :size="12" />
            </button>
          </div>
        </div>
        <div v-if="repo.stagedFiles.length === 0" class="empty-hint">暂无已暂存的文件</div>
      </div>
    </div>

    <!-- 未暂存区域 -->
    <div class="file-section">
      <div class="section-header">
        <ChevronDown :size="14" />
        <span class="section-title">未暂存的更改</span>
        <span class="file-count">{{ repo.unstagedFiles.length }}</span>
        <button
          v-if="repo.unstagedFiles.length > 0"
          class="btn-icon"
          title="丢弃所有更改"
          @click="repo.discardAllChanges()"
        >
          <Trash2 :size="13" />
        </button>
        <button
          v-if="repo.unstagedFiles.length > 0"
          class="btn-icon"
          title="暂存全部"
          @click="repo.stageAll()"
        >
          <Plus :size="13" />
        </button>
      </div>
      <div class="file-list">
        <div
          v-for="file in repo.unstagedFiles"
          :key="'unstaged-' + file.path"
          class="file-item"
          @dblclick="openFileInVscode(file)"
        >
          <span class="file-status" :style="{ color: statusColor(file.status) }">
            {{ statusIcon(file.status) }}
          </span>
          <span class="file-dir">{{ fileDir(file.path) }}</span>
          <span class="file-name">{{ fileName(file.path) }}</span>
          <div class="file-actions">
            <button class="btn-icon" title="在 VSCode 中查看" @click.stop="openFileInVscode(file)">
              <Code2 :size="12" />
            </button>
            <button class="btn-icon" title="丢弃更改" @click.stop="repo.discardChanges([file.path])">
              <Undo2 :size="12" />
            </button>
            <button class="btn-icon" title="暂存" @click.stop="repo.stageFiles([file.path])">
              <Plus :size="12" />
            </button>
          </div>
        </div>
        <div v-if="repo.unstagedFiles.length === 0" class="empty-hint">工作区干净，没有更改</div>
      </div>
    </div>

    <!-- 提交区域 -->
    <div class="commit-area">
      <textarea
        v-model="repo.commitMessage"
        class="commit-input"
        :placeholder="amendMode ? '修改上次提交信息...' : '输入提交信息...'"
        rows="3"
        @keydown.ctrl.enter="amendMode ? repo.commitAmend(repo.commitMessage) : repo.commit()"
        @keydown.meta.enter="amendMode ? repo.commitAmend(repo.commitMessage) : repo.commit()"
      />
      <div class="commit-actions">
        <div class="commit-options">
          <button
            class="btn-icon"
            :class="{ active: amendMode }"
            title="修改上次提交 (Amend)"
            @click="amendMode = !amendMode"
          >
            <PenLine :size="13" />
          </button>
          <button
            class="btn-icon"
            title="撤销上次提交"
            :disabled="repo.operating"
            @click="repo.undoLastCommit()"
          >
            <RotateCcw :size="13" />
          </button>
        </div>
        <button
          v-if="amendMode"
          class="btn btn-secondary commit-btn"
          :disabled="!repo.commitMessage.trim() || repo.operating"
          @click="repo.commitAmend(repo.commitMessage)"
        >
          <PenLine :size="14" />
          Amend
        </button>
        <button
          v-else
          class="btn btn-primary commit-btn"
          :disabled="repo.stagedFiles.length === 0 || !repo.commitMessage.trim() || repo.operating"
          @click="repo.commit()"
        >
          <CheckCircle2 :size="14" />
          提交 ({{ repo.stagedFiles.length }} 个文件)
        </button>
      </div>
    </div>
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
  border-bottom: 1px solid var(--border-default);
}

.section-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  font-size: 12px;
  color: var(--text-secondary);
  background: var(--bg-tertiary);
  flex-shrink: 0;
}

.section-title {
  font-weight: 600;
  flex: 1;
}

.file-count {
  font-size: 11px;
  background: var(--bg-hover);
  padding: 0 6px;
  border-radius: 8px;
  color: var(--text-muted);
  line-height: 18px;
}

.file-list {
  flex: 1;
  overflow-y: auto;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 12px;
  font-size: 12px;
  cursor: pointer;
  transition: background-color 0.1s;
}

.file-item:hover {
  background: var(--bg-hover);
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
  color: var(--text-muted);
  font-size: 11px;
  white-space: nowrap;
}

.file-name {
  color: var(--text-primary);
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
}

.file-item:hover .file-actions {
  opacity: 1;
}

.empty-hint {
  padding: 16px;
  text-align: center;
  color: var(--text-muted);
  font-size: 12px;
}

.commit-area {
  flex-shrink: 0;
  padding: 12px;
  background: var(--bg-secondary);
}

.commit-input {
  width: 100%;
  padding: 8px 10px;
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 13px;
  resize: none;
  outline: none;
  font-family: var(--font-ui);
  transition: border-color 0.15s;
}

.commit-input:focus {
  border-color: var(--accent-blue);
}

.commit-input::placeholder {
  color: var(--text-muted);
}

.commit-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
}

.commit-options {
  display: flex;
  gap: 2px;
}

.commit-options .btn-icon.active {
  color: var(--accent-blue);
  background: var(--bg-active);
}

.commit-btn {
  flex: 1;
  justify-content: center;
  padding: 8px;
}
</style>
