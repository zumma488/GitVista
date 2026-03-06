<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useProjectsStore } from '@/stores/projects'
import { useRepoStore } from '@/stores/repo'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import {
  FolderGit2,
  Plus,
  Download,
  Trash2,
  GitBranch,
  Clock,
  FolderOpen,
  ExternalLink,
  LayoutGrid,
  List,
} from 'lucide-vue-next'

type ViewMode = 'card' | 'list'

const router = useRouter()
const store = useProjectsStore()
const repo = useRepoStore()

const viewMode = ref<ViewMode>((localStorage.getItem('gitvista-view-mode') as ViewMode) || 'card')
const showCloneDialog = ref(false)
const cloneUrl = ref('')
const cloneTarget = ref('')
const cloning = ref(false)

function setViewMode(mode: ViewMode) {
  viewMode.value = mode
  localStorage.setItem('gitvista-view-mode', mode)
}

onMounted(() => {
  store.loadProjects()
})

function getAvatarColor(name: string): string {
  const colors = ['#58a6ff', '#3fb950', '#f85149', '#d29922', '#bc8cff', '#f0883e', '#79c0ff', '#56d364']
  let hash = 0
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash)
  }
  return colors[Math.abs(hash) % colors.length]!
}

function formatTime(dateStr: string | null): string {
  if (!dateStr) return ''
  const date = new Date(dateStr)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)

  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes} 分钟前`
  if (hours < 24) return `${hours} 小时前`
  if (days < 30) return `${days} 天前`
  return date.toLocaleDateString('zh-CN')
}

async function handleAddProject() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择 Git 仓库文件夹',
    })
    if (selected) {
      await store.addProject(selected as string)
      repo.showToast('success', '项目添加成功')
    }
  } catch (e) {
    repo.showToast('error', `${e}`)
  }
}

async function handleRemoveProject(path: string) {
  try {
    await store.removeProject(path)
    repo.showToast('info', '项目已移除')
  } catch (e) {
    repo.showToast('error', `${e}`)
  }
}

async function handleOpenProject(path: string) {
  await store.updateLastOpened(path)
  router.push({ name: 'repo', query: { path } })
}

async function handleClone() {
  if (!cloneUrl.value.trim() || !cloneTarget.value.trim()) {
    repo.showToast('error', '请填写仓库地址和目标路径')
    return
  }
  cloning.value = true
  try {
    await invoke('clone_repo', { url: cloneUrl.value, target: cloneTarget.value })
    await store.addProject(cloneTarget.value)
    showCloneDialog.value = false
    cloneUrl.value = ''
    cloneTarget.value = ''
    repo.showToast('success', '克隆成功')
  } catch (e) {
    repo.showToast('error', `${e}`)
  } finally {
    cloning.value = false
  }
}

async function selectCloneTarget() {
  const selected = await open({ directory: true, multiple: false, title: '选择克隆目标文件夹' })
  if (selected) {
    cloneTarget.value = selected as string
  }
}
</script>

<template>
  <div class="project-list-page">
    <!-- 顶部 Header -->
    <header class="page-header">
      <div class="brand">
        <FolderGit2 :size="32" class="brand-icon" />
        <div>
          <h1 class="brand-title">GitVista</h1>
          <p class="brand-subtitle">轻量级 Git 图形化管理工具</p>
        </div>
      </div>
      <div class="header-actions">
        <div class="view-toggle">
          <button
            class="toggle-btn"
            :class="{ active: viewMode === 'card' }"
            title="卡片视图"
            @click="setViewMode('card')"
          >
            <LayoutGrid :size="15" />
          </button>
          <button
            class="toggle-btn"
            :class="{ active: viewMode === 'list' }"
            title="列表视图"
            @click="setViewMode('list')"
          >
            <List :size="15" />
          </button>
        </div>
        <span class="header-divider" />
        <button class="btn btn-primary" @click="handleAddProject">
          <Plus :size="16" />
          打开本地仓库
        </button>
        <button class="btn btn-secondary" @click="showCloneDialog = true">
          <Download :size="16" />
          克隆远程仓库
        </button>
      </div>
    </header>

    <!-- 项目内容区 -->
    <main class="project-content">
      <div v-if="store.projects.length === 0 && !store.loading" class="empty-state">
        <FolderGit2 :size="64" class="empty-icon" />
        <h2>还没有任何项目</h2>
        <p>点击「打开本地仓库」添加一个 Git 项目，或「克隆远程仓库」开始</p>
      </div>

      <!-- 卡片视图 -->
      <div v-else-if="viewMode === 'card'" class="project-grid">
        <div
          v-for="project in store.projects"
          :key="project.path"
          class="project-card"
          @click="handleOpenProject(project.path)"
        >
          <div class="card-avatar" :style="{ background: getAvatarColor(project.name) }">
            {{ project.name.charAt(0).toUpperCase() }}
          </div>
          <div class="card-body">
            <div class="card-header">
              <h3 class="card-title" :title="project.name">{{ project.name }}</h3>
              <button
                class="btn-icon card-delete"
                title="移除项目"
                @click.stop="handleRemoveProject(project.path)"
              >
                <Trash2 :size="14" />
              </button>
            </div>
            <p class="card-path" :title="project.path">{{ project.path }}</p>
            <div class="card-meta">
              <span
                v-if="store.projectInfoMap[project.path]"
                class="badge badge-blue card-branch"
              >
                <GitBranch :size="12" />
                <span class="branch-text">{{ store.projectInfoMap[project.path]!.current_branch }}</span>
              </span>
              <span v-if="project.last_opened" class="card-time">
                <Clock :size="12" />
                {{ formatTime(project.last_opened) }}
              </span>
            </div>
          </div>
          <div class="card-arrow">
            <ExternalLink :size="16" />
          </div>
        </div>
      </div>

      <!-- 列表视图 -->
      <div v-else class="project-list">
        <div class="list-header">
          <span class="list-col-name">项目名称</span>
          <span class="list-col-path">路径</span>
          <span class="list-col-branch">分支</span>
          <span class="list-col-time">最近打开</span>
          <span class="list-col-actions" />
        </div>
        <div
          v-for="project in store.projects"
          :key="project.path"
          class="list-row"
          @click="handleOpenProject(project.path)"
        >
          <span class="list-col-name">
            <span class="list-avatar" :style="{ background: getAvatarColor(project.name) }">
              {{ project.name.charAt(0).toUpperCase() }}
            </span>
            <span class="list-name" :title="project.name">{{ project.name }}</span>
          </span>
          <span class="list-col-path" :title="project.path">{{ project.path }}</span>
          <span class="list-col-branch">
            <span
              v-if="store.projectInfoMap[project.path]"
              class="badge badge-blue list-branch"
            >
              <GitBranch :size="12" />
              <span class="branch-text">{{ store.projectInfoMap[project.path]!.current_branch }}</span>
            </span>
          </span>
          <span class="list-col-time">
            <template v-if="project.last_opened">
              <Clock :size="12" />
              {{ formatTime(project.last_opened) }}
            </template>
          </span>
          <span class="list-col-actions">
            <button
              class="btn-icon list-delete"
              title="移除项目"
              @click.stop="handleRemoveProject(project.path)"
            >
              <Trash2 :size="14" />
            </button>
          </span>
        </div>
      </div>
    </main>

    <!-- 克隆弹窗 -->
    <Teleport to="body">
      <div v-if="showCloneDialog" class="modal-overlay" @click.self="showCloneDialog = false">
        <div class="modal">
          <h2 class="modal-title">克隆远程仓库</h2>
          <div class="form-group">
            <label>仓库地址</label>
            <input
              v-model="cloneUrl"
              type="text"
              placeholder="https://github.com/user/repo.git"
              class="input"
            />
          </div>
          <div class="form-group">
            <label>克隆到</label>
            <div class="input-row">
              <input
                v-model="cloneTarget"
                type="text"
                placeholder="选择本地目标路径"
                class="input"
                readonly
              />
              <button class="btn btn-secondary" @click="selectCloneTarget">
                <FolderOpen :size="14" />
                浏览
              </button>
            </div>
          </div>
          <div class="modal-actions">
            <button class="btn btn-secondary" @click="showCloneDialog = false">取消</button>
            <button class="btn btn-primary" :disabled="cloning" @click="handleClone">
              {{ cloning ? '克隆中...' : '开始克隆' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.project-list-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-canvas);
}

/* ===== Header ===== */
.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 32px;
  border-bottom: 1px solid var(--border-default);
  background: var(--bg-primary);
  flex-shrink: 0;
}

.brand {
  display: flex;
  align-items: center;
  gap: 12px;
}

.brand-icon {
  color: var(--accent-blue);
}

.brand-title {
  font-size: 22px;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.2;
}

.brand-subtitle {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 2px;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.header-divider {
  width: 1px;
  height: 24px;
  background: var(--border-default);
  margin: 0 4px;
}

/* ===== 布局切换 ===== */
.view-toggle {
  display: flex;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.toggle-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 28px;
  color: var(--text-muted);
  transition: background-color 0.15s, color 0.15s;
}

.toggle-btn:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.toggle-btn.active {
  color: var(--accent-blue);
  background: var(--bg-active);
}

/* ===== 内容区 ===== */
.project-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px 32px;
}

/* ===== 空状态 ===== */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 12px;
  color: var(--text-secondary);
}

.empty-icon {
  opacity: 0.3;
  margin-bottom: 8px;
}

.empty-state h2 {
  font-size: 18px;
  color: var(--text-primary);
}

.empty-state p {
  font-size: 14px;
  max-width: 400px;
  text-align: center;
}

/* ===== 卡片视图 ===== */
.project-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
  gap: 12px;
}

.project-card {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: border-color 0.2s, transform 0.15s, box-shadow 0.2s;
  min-width: 0;
}

.project-card:hover {
  border-color: var(--accent-blue);
  box-shadow: 0 0 0 1px var(--accent-blue);
  transform: translateY(-1px);
}

.card-avatar {
  flex-shrink: 0;
  width: 44px;
  height: 44px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: 700;
  color: #fff;
}

.card-body {
  flex: 1;
  min-width: 0;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.card-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  min-width: 0;
}

.card-delete {
  flex-shrink: 0;
  opacity: 0;
  color: var(--text-muted);
  transition: opacity 0.15s, color 0.15s;
}

.project-card:hover .card-delete {
  opacity: 1;
}

.card-delete:hover {
  color: var(--accent-red) !important;
}

.card-path {
  font-size: 12px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  margin: 4px 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-meta {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-top: 4px;
  min-width: 0;
}

.card-branch {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  max-width: 180px;
  min-width: 0;
}

.branch-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-time {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.card-arrow {
  flex-shrink: 0;
  color: var(--text-muted);
  opacity: 0;
  transition: opacity 0.15s;
}

.project-card:hover .card-arrow {
  opacity: 1;
}

/* ===== 列表视图 ===== */
.project-list {
  display: flex;
  flex-direction: column;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.list-header {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border-default);
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.02em;
}

.list-row {
  display: flex;
  align-items: center;
  padding: 10px 16px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-muted);
  cursor: pointer;
  transition: background-color 0.12s;
}

.list-row:last-child {
  border-bottom: none;
}

.list-row:hover {
  background: var(--bg-hover);
}

.list-col-name {
  flex: 2;
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 10px;
}

.list-col-path {
  flex: 3;
  min-width: 0;
  font-size: 12px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  padding-right: 12px;
}

.list-col-branch {
  flex: 1.5;
  min-width: 0;
  padding-right: 12px;
}

.list-col-time {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--text-muted);
  white-space: nowrap;
}

.list-col-actions {
  flex-shrink: 0;
  width: 32px;
  display: flex;
  justify-content: center;
}

.list-avatar {
  flex-shrink: 0;
  width: 30px;
  height: 30px;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 700;
  color: #fff;
}

.list-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.list-branch {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  max-width: 100%;
  min-width: 0;
}

.list-delete {
  opacity: 0;
  color: var(--text-muted);
  transition: opacity 0.15s, color 0.15s;
}

.list-row:hover .list-delete {
  opacity: 1;
}

.list-delete:hover {
  color: var(--accent-red) !important;
}

/* ===== 弹窗 ===== */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 24px;
  width: 480px;
  max-width: 90vw;
  box-shadow: var(--shadow-lg);
}

.modal-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 20px;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 6px;
}

.input {
  width: 100%;
  padding: 8px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s;
}

.input:focus {
  border-color: var(--accent-blue);
}

.input-row {
  display: flex;
  gap: 8px;
}

.input-row .input {
  flex: 1;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 20px;
}
</style>
