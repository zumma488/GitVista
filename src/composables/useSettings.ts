import { ref, watch } from 'vue'

export type ViewMode = 'card' | 'list'
export type SortMode = 'default' | 'name' | 'recent' | 'custom'
export type ThemeMode = 'dark' | 'light' | 'system'
export type PullMode = 'merge' | 'rebase'
export type CloseAction = 'ask' | 'minimize' | 'exit'

interface Settings {
  theme: ThemeMode
  viewMode: ViewMode
  sortMode: SortMode
  pullMode: PullMode
  commitHistoryCount: number
  closeAction: CloseAction
  customGitPath: string
}

const STORAGE_KEY = 'gitvista-settings'

const defaults: Settings = {
  theme: 'dark',
  viewMode: 'card',
  sortMode: 'default',
  pullMode: 'merge',
  commitHistoryCount: 100,
  closeAction: 'ask',
  customGitPath: '',
}

function loadSettings(): Settings {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (raw) {
      return { ...defaults, ...JSON.parse(raw) }
    }
  } catch {
    // ignore
  }

  // 迁移旧版 localStorage 的分散存储
  const legacy: Partial<Settings> = {}
  const oldTheme = localStorage.getItem('gitvista-theme')
  if (oldTheme === 'dark' || oldTheme === 'light') legacy.theme = oldTheme
  const oldView = localStorage.getItem('gitvista-view-mode')
  if (oldView === 'card' || oldView === 'list') legacy.viewMode = oldView
  const oldSort = localStorage.getItem('gitvista-sort-mode')
  if (['default', 'name', 'recent', 'custom'].includes(oldSort || '')) legacy.sortMode = oldSort as SortMode
  const oldClose = localStorage.getItem('close_action')
  if (oldClose === 'minimize') legacy.closeAction = 'minimize'
  else if (oldClose === 'exit') legacy.closeAction = 'exit'

  return { ...defaults, ...legacy }
}

function saveSettings(settings: Settings) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings))
}

// 全局单例状态
const settings = ref<Settings>(loadSettings())

export function useSettings() {
  // 监听变化自动保存
  watch(settings, (val) => {
    saveSettings(val)
  }, { deep: true })

  function update<K extends keyof Settings>(key: K, value: Settings[K]) {
    settings.value = { ...settings.value, [key]: value }
  }

  function resetAll() {
    settings.value = { ...defaults }
    // 清理旧版 localStorage
    localStorage.removeItem('gitvista-theme')
    localStorage.removeItem('gitvista-view-mode')
    localStorage.removeItem('gitvista-sort-mode')
    localStorage.removeItem('close_action')
    localStorage.removeItem('gitvista-custom-order')
  }

  return {
    settings,
    update,
    resetAll,
    defaults,
  }
}
