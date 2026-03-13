import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useSettings, type ThemeMode } from './useSettings'

type ResolvedTheme = 'dark' | 'light'

const currentTheme = ref<ResolvedTheme>('dark')

const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

function resolveTheme(mode: ThemeMode): ResolvedTheme {
  if (mode === 'system') {
    return mediaQuery.matches ? 'dark' : 'light'
  }
  return mode
}

function applyTheme(theme: ResolvedTheme) {
  currentTheme.value = theme
  if (theme === 'dark') {
    document.documentElement.classList.add('app-dark')
    document.documentElement.dataset.theme = 'dark'
  } else {
    document.documentElement.classList.remove('app-dark')
    document.documentElement.dataset.theme = 'light'
  }
}

export function useTheme() {
  const { settings } = useSettings()

  function handleSystemChange() {
    if (settings.value.theme === 'system') {
      applyTheme(resolveTheme('system'))
    }
  }

  onMounted(() => {
    applyTheme(resolveTheme(settings.value.theme))
    mediaQuery.addEventListener('change', handleSystemChange)
  })

  onUnmounted(() => {
    mediaQuery.removeEventListener('change', handleSystemChange)
  })

  // 当设置中的主题变更时，立即应用
  watch(() => settings.value.theme, (mode) => {
    applyTheme(resolveTheme(mode))
  })

  function toggleTheme() {
    const next: ResolvedTheme = currentTheme.value === 'dark' ? 'light' : 'dark'
    settings.value = { ...settings.value, theme: next }
  }

  return {
    currentTheme,
    toggleTheme,
  }
}
