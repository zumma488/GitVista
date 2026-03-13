<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useProjectsStore } from '@/stores/projects'
import { useRepoStore } from '@/stores/repo'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import Button from 'primevue/button'
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
  Search,
  Pin,
  ArrowUp,
  ArrowDown,
} from 'lucide-vue-next'

type ViewMode = 'card' | 'list'
type SortMode = 'default' | 'name' | 'recent' | 'custom'

const router = useRouter()
const store = useProjectsStore()
const repo = useRepoStore()

const viewMode = ref<ViewMode>((localStorage.getItem('gitvista-view-mode') as ViewMode) || 'card')
const showCloneDialog = ref(false)
const cloneUrl = ref('')
const cloneTarget = ref('')
const cloning = ref(false)
const searchQuery = ref('')
const sortMode = ref<SortMode>((localStorage.getItem('gitvista-sort-mode') as SortMode) || 'default')
const customOrder = ref<string[]>(JSON.parse(localStorage.getItem('gitvista-custom-order') || '[]'))

// 监听项目数量变化，保持 customOrder 同步（只追加新项目和移除被删项目，保持当前顺序）
import { watch } from 'vue'
watch(() => store.projects, (projects) => {
  const currentPaths = new Set(projects.map(p => p.path))
  const filtered = customOrder.value.filter(p => currentPaths.has(p))
  const existingPaths = new Set(filtered)

  for (const p of projects) {
    if (!existingPaths.has(p.path)) {
      filtered.push(p.path)
    }
  }

  if (filtered.length !== customOrder.value.length ||
      filtered.some((p, i) => p !== customOrder.value[i])) {
    customOrder.value = filtered
    saveCustomOrder()
  }
}, { deep: true })

function saveCustomOrder() {
  localStorage.setItem('gitvista-custom-order', JSON.stringify(customOrder.value))
}

const filteredProjects = computed(() => {
  let list = store.projects
  const q = searchQuery.value.trim().toLowerCase()
  if (q) {
    list = list.filter(p => p.name.toLowerCase().includes(q) || p.path.toLowerCase().includes(q))
  }

  // 如果是自定义排序模式
  if (sortMode.value === 'custom') {
    return [...list].sort((a, b) => {
      const indexA = customOrder.value.indexOf(a.path)
      const indexB = customOrder.value.indexOf(b.path)
      if (indexA === -1 && indexB === -1) return 0
      if (indexA === -1) return 1
      if (indexB === -1) return -1
      return indexA - indexB
    })
  }

  // 非自定义排序模式（默认加入时间 / 最近打开 / 名称）
  const pinned = list.filter(p => p.favorite)
  const unpinned = list.filter(p => !p.favorite)

  const sortFn = (a: typeof list[0], b: typeof list[0]) => {
    if (sortMode.value === 'name') {
      return a.name.localeCompare(b.name)
    }
    if (sortMode.value === 'recent') {
      const ta = a.last_opened ? new Date(a.last_opened).getTime() : 0
      const tb = b.last_opened ? new Date(b.last_opened).getTime() : 0
      return tb - ta
    }
    // default: 按加入时间倒序（最新加入的排最前）
    const ta = a.added_at ? new Date(a.added_at).getTime() : 0
    const tb = b.added_at ? new Date(b.added_at).getTime() : 0
    return tb - ta
  }

  // 分别对两组应用相同的排序规则，并合并结果
  return [...pinned.sort(sortFn), ...unpinned.sort(sortFn)]
})

function setViewMode(mode: ViewMode) {
  viewMode.value = mode
  localStorage.setItem('gitvista-view-mode', mode)
}

function setSortMode(mode: SortMode) {
  sortMode.value = mode
  localStorage.setItem('gitvista-sort-mode', mode)

  // 如果首次切换到自定义排序且没有数据，按照当前列表的顺序初始化 customOrder
  if (mode === 'custom' && customOrder.value.length === 0) {
    customOrder.value = store.projects.map(p => p.path)
    saveCustomOrder()
  }
}

async function handleTogglePin(path: string) {
  await store.toggleFavorite(path)
}

function handleMoveUp(path: string) {
  const currentIndex = customOrder.value.indexOf(path)
  if (currentIndex > 0) {
    const newOrder = [...customOrder.value]
    const item = newOrder.splice(currentIndex, 1)[0]!
    newOrder.splice(currentIndex - 1, 0, item)
    customOrder.value = newOrder
    saveCustomOrder()
  }
}

function handleMoveDown(path: string) {
  const currentIndex = customOrder.value.indexOf(path)
  if (currentIndex !== -1 && currentIndex < customOrder.value.length - 1) {
    const newOrder = [...customOrder.value]
    const item = newOrder.splice(currentIndex, 1)[0]!
    newOrder.splice(currentIndex + 1, 0, item)
    customOrder.value = newOrder
    saveCustomOrder()
  }
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
      const isRepo = await invoke<boolean>('is_git_repo', { path: selected as string })
      if (!isRepo) {
        repo.showToast('error', '所选文件夹不是一个有效的 Git 仓库')
        return
      }
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
        <Button size="small" @click="handleAddProject">
          <Plus :size="16" class="mr-2" />
          打开本地仓库
        </Button>
        <Button size="small" severity="secondary" variant="outlined" @click="showCloneDialog = true">
          <Download :size="16" class="mr-2" />
          克隆远程仓库
        </Button>
      </div>
    </header>

    <!-- 搜索和排序栏 -->
    <div v-if="store.projects.length > 0" class="toolbar-bar">
      <div class="search-box">
        <Search :size="14" class="search-icon" />
        <input
          id="project-search-input"
          v-model="searchQuery"
          class="search-input"
          placeholder="搜索项目..."
        />
      </div>
      <div class="sort-control">
        <Button
          size="small"
          variant="text"
          :severity="sortMode === 'default' ? 'primary' : 'secondary'"
          @click="setSortMode('default')"
        >默认排序</Button>
        <Button
          size="small"
          variant="text"
          :severity="sortMode === 'recent' ? 'primary' : 'secondary'"
          @click="setSortMode('recent')"
        >最近打开</Button>
        <Button
          size="small"
          variant="text"
          :severity="sortMode === 'name' ? 'primary' : 'secondary'"
          @click="setSortMode('name')"
        >名称</Button>
        <Button
          size="small"
          variant="text"
          :severity="sortMode === 'custom' ? 'primary' : 'secondary'"
          @click="setSortMode('custom')"
        >自定义排序</Button>
      </div>
    </div>

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
          v-for="project in filteredProjects"
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
              <div class="card-header-actions">
                <template v-if="sortMode === 'custom'">
                  <button
                    class="btn-icon card-move"
                    title="上移"
                    @click.stop="handleMoveUp(project.path)"
                  >
                    <ArrowUp :size="14" />
                  </button>
                  <button
                    class="btn-icon card-move"
                    title="下移"
                    @click.stop="handleMoveDown(project.path)"
                  >
                    <ArrowDown :size="14" />
                  </button>
                </template>
                <button
                  v-else
                  class="btn-icon card-pin"
                  :class="{ pinned: project.favorite }"
                  :title="project.favorite ? '取消置顶' : '置顶'"
                  @click.stop="handleTogglePin(project.path)"
                >
                  <Pin :size="14" />
                </button>
                <button
                  class="btn-icon card-delete"
                  title="移除项目"
                  @click.stop="handleRemoveProject(project.path)"
                >
                  <Trash2 :size="14" />
                </button>
              </div>
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
          v-for="project in filteredProjects"
          :key="project.path"
          class="list-row"
          @click="handleOpenProject(project.path)"
        >
          <span class="list-col-name">
            <template v-if="sortMode === 'custom'">
              <div class="list-move-actions">
                 <button
                   class="btn-icon list-move"
                   title="上移"
                   @click.stop="handleMoveUp(project.path)"
                 >
                   <ArrowUp :size="12" />
                 </button>
                 <button
                   class="btn-icon list-move"
                   title="下移"
                   @click.stop="handleMoveDown(project.path)"
                 >
                   <ArrowDown :size="12" />
                 </button>
              </div>
            </template>
            <button
              v-else
              class="btn-icon list-pin"
              :class="{ pinned: project.favorite }"
              :title="project.favorite ? '取消置顶' : '置顶'"
              @click.stop="handleTogglePin(project.path)"
            >
              <Pin :size="13" />
            </button>
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
        <div v-if="filteredProjects.length === 0 && searchQuery" class="search-empty">
          无匹配的项目
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
              <Button severity="secondary" variant="outlined" @click="selectCloneTarget">
                <FolderOpen :size="14" class="mr-2" />
                浏览
              </Button>
            </div>
          </div>
          <div class="modal-actions">
            <Button severity="secondary" variant="text" @click="showCloneDialog = false">取消</Button>
            <Button :disabled="cloning" @click="handleClone">
              {{ cloning ? '克隆中...' : '开始克隆' }}
            </Button>
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
  background: var(--p-content-background);
}

/* ===== 搜索排序栏 ===== */
.toolbar-bar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 32px;
  background: var(--p-content-background);
  border-bottom: 1px solid var(--p-content-border-color);
  flex-shrink: 0;
}

.search-box {
  display: flex;
  align-items: center;
  gap: 6px;
  flex: 1;
  max-width: 320px;
  padding: 5px 10px;
  background: var(--p-content-hover-background);
  border: 1px solid var(--p-content-border-color);
  border-radius: var(--radius-md);
  transition: border-color 0.15s;
}

.search-box:focus-within {
  border-color: var(--p-primary-color);
}

.search-box .search-icon {
  color: var(--p-text-muted-color);
  flex-shrink: 0;
}

.search-box .search-input {
  flex: 1;
  background: transparent;
  border: none;
  outline: none;
  color: var(--p-text-color);
  font-size: 12px;
  min-width: 0;
}

.search-box .search-input::placeholder {
  color: var(--p-text-muted-color);
}

.sort-control {
  display: flex;
  align-items: center;
  gap: 4px;
}

.search-empty {
  padding: 24px;
  text-align: center;
  color: var(--p-text-muted-color);
  font-size: 13px;
}

/* ===== Header ===== */
.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 32px;
  border-bottom: 1px solid var(--p-content-border-color);
  background: var(--p-content-background);
  flex-shrink: 0;
}

.brand {
  display: flex;
  align-items: center;
  gap: 12px;
}

.brand-icon {
  color: var(--p-primary-color);
}

.brand-title {
  font-size: 22px;
  font-weight: 700;
  color: var(--p-text-color);
  line-height: 1.2;
}

.brand-subtitle {
  font-size: 12px;
  color: var(--p-text-muted-color);
  margin-top: 2px;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.mr-2 {
  margin-right: 0.5rem;
}

.header-divider {
  width: 1px;
  height: 24px;
  background: var(--p-content-border-color);
  margin: 0 4px;
}

/* ===== 布局切换 ===== */
.view-toggle {
  display: flex;
  background: var(--p-content-hover-background);
  border: 1px solid var(--p-content-border-color);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.toggle-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 28px;
  color: var(--p-text-muted-color);
  transition: background-color 0.15s, color 0.15s;
}

.toggle-btn:hover {
  color: var(--p-text-color);
  background: var(--p-content-hover-background);
}

.toggle-btn.active {
  color: var(--p-primary-inverse-color);
  background: var(--p-primary-color);
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
  color: var(--p-text-muted-color);
}

.empty-icon {
  opacity: 0.3;
  margin-bottom: 8px;
}

.empty-state h2 {
  font-size: 18px;
  color: var(--p-text-color);
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
  background: var(--p-content-hover-background);
  border: 1px solid var(--p-content-border-color);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: border-color 0.2s, transform 0.15s, box-shadow 0.2s;
  min-width: 0;
}

.project-card:hover {
  border-color: var(--p-primary-color);
  box-shadow: 0 0 0 1px var(--p-primary-color);
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
  color: var(--p-text-color);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  min-width: 0;
}

.card-delete {
  flex-shrink: 0;
  opacity: 0;
  color: var(--p-text-muted-color);
  transition: opacity 0.15s, color 0.15s;
}

.card-header-actions {
  display: flex;
  gap: 2px;
  flex-shrink: 0;
}

.card-pin,
.list-pin {
  color: var(--p-text-muted-color);
  opacity: 0;
  transition: opacity 0.15s, color 0.15s;
}

.card-pin.pinned,
.list-pin.pinned {
  opacity: 1;
  color: var(--p-primary-color);
  fill: var(--p-primary-color);
}

.card-move,
.list-move {
  color: var(--p-text-muted-color);
  opacity: 0;
  transition: opacity 0.15s, color 0.15s;
}

.project-card:hover .card-pin,
.list-row:hover .list-pin,
.project-card:hover .card-move,
.list-row:hover .list-move {
  opacity: 1;
}

.card-pin:hover,
.list-pin:hover {
  color: var(--p-primary-color) !important;
}

.card-move:hover,
.list-move:hover {
  color: var(--p-primary-color) !important;
}

.list-move-actions {
  display: flex;
  flex-direction: column;
  gap: 2px;
  margin-right: 4px;
}
.list-move-actions .btn-icon {
  width: 14px;
  height: 14px;
  padding: 0;
}

.project-card:hover .card-delete {
  opacity: 1;
}

.card-delete:hover {
  color: var(--accent-red) !important;
}

.card-path {
  font-size: 12px;
  color: var(--p-text-muted-color);
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
  color: var(--p-text-muted-color);
  flex-shrink: 0;
}

.card-arrow {
  flex-shrink: 0;
  color: var(--p-text-muted-color);
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
  border: 1px solid var(--p-content-border-color);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.list-header {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  background: var(--p-content-hover-background);
  border-bottom: 1px solid var(--p-content-border-color);
  font-size: 11px;
  font-weight: 600;
  color: var(--p-text-muted-color);
  text-transform: uppercase;
  letter-spacing: 0.02em;
}

.list-row {
  display: flex;
  align-items: center;
  padding: 10px 16px;
  background: var(--p-content-hover-background);
  border-bottom: 1px solid var(--p-content-border-color);
  cursor: pointer;
  transition: background-color 0.12s;
}

.list-row:last-child {
  border-bottom: none;
}

.list-row:hover {
  background: var(--p-content-hover-background);
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
  color: var(--p-text-muted-color);
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
  color: var(--p-text-muted-color);
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
  color: var(--p-text-color);
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
  color: var(--p-text-muted-color);
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
  background: var(--p-content-hover-background);
  border: 1px solid var(--p-content-border-color);
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
  color: var(--p-text-muted-color);
  margin-bottom: 6px;
}

.input {
  width: 100%;
  padding: 8px 12px;
  background: var(--p-content-background);
  border: 1px solid var(--p-content-border-color);
  border-radius: var(--radius-md);
  color: var(--p-text-color);
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s;
}

.input:focus {
  border-color: var(--p-primary-color);
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
