import { onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useRepoStore } from '@/stores/repo'

export function useShortcuts() {
  const router = useRouter()
  const repo = useRepoStore()

  function isRepoPage() {
    return router.currentRoute.value.name === 'repo'
  }

  function handleKeydown(e: KeyboardEvent) {
    const isCtrl = e.ctrlKey || e.metaKey
    const tag = (e.target as HTMLElement)?.tagName

    // 忽略在输入框中的快捷键（除了特定组合）
    if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') {
      // 允许 Ctrl+P 在输入框中也生效
      if (!(isCtrl && e.key === 'p')) return
    }

    // Ctrl+S — 暂存全部
    if (isCtrl && e.key === 's' && !e.shiftKey) {
      e.preventDefault()
      if (isRepoPage() && repo.unstagedFiles.length > 0) {
        repo.stageAll()
      }
      return
    }

    // Ctrl+Shift+Z — 撤销上次提交
    if (isCtrl && e.shiftKey && e.key === 'Z') {
      e.preventDefault()
      if (isRepoPage()) {
        repo.undoLastCommit()
      }
      return
    }

    // Ctrl+P — 聚焦项目搜索框
    if (isCtrl && e.key === 'p') {
      e.preventDefault()
      if (router.currentRoute.value.name === 'projects') {
        const searchInput = document.getElementById('project-search-input')
        searchInput?.focus()
      }
      return
    }
  }

  onMounted(() => {
    document.addEventListener('keydown', handleKeydown)
  })

  onUnmounted(() => {
    document.removeEventListener('keydown', handleKeydown)
  })
}
