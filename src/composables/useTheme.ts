import { ref, onMounted } from 'vue'

type Theme = 'dark' | 'light'

const STORAGE_KEY = 'gitvista-theme'

const currentTheme = ref<Theme>('dark')

function applyTheme(theme: Theme) {
  currentTheme.value = theme
  document.documentElement.dataset.theme = theme === 'light' ? 'light' : ''
  localStorage.setItem(STORAGE_KEY, theme)
}

export function useTheme() {
  onMounted(() => {
    const saved = localStorage.getItem(STORAGE_KEY) as Theme | null
    applyTheme(saved || 'dark')
  })

  function toggleTheme() {
    applyTheme(currentTheme.value === 'dark' ? 'light' : 'dark')
  }

  return {
    currentTheme,
    toggleTheme,
  }
}
