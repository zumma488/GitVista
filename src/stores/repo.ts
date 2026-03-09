import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { RepoInfo, FileChange, BranchInfo, CommitInfo, CommitFile, StashEntry, Toast } from '@/types'

export const useRepoStore = defineStore('repo', () => {
  const path = ref('')
  const info = ref<RepoInfo | null>(null)
  const files = ref<FileChange[]>([])
  const branches = ref<BranchInfo[]>([])
  const commits = ref<CommitInfo[]>([])
  const activeTab = ref<'changes' | 'history'>('changes')
  const loading = ref(false)
  const operating = ref(false)
  const commitMessage = ref('')
  const selectedCommitHash = ref<string | null>(null)
  const selectedCommitFiles = ref<CommitFile[]>([])
  const stashes = ref<StashEntry[]>([])
  const toasts = ref<Toast[]>([])

  let toastId = 0

  const stagedFiles = computed(() => files.value.filter((f) => f.staged))
  const unstagedFiles = computed(() => files.value.filter((f) => !f.staged))
  const localBranches = computed(() => branches.value.filter((b) => !b.is_remote))
  const remoteBranches = computed(() => branches.value.filter((b) => b.is_remote))

  function showToast(type: Toast['type'], message: string) {
    const id = ++toastId
    toasts.value.push({ id, type, message })
    setTimeout(() => {
      toasts.value = toasts.value.filter((t) => t.id !== id)
    }, 3000)
  }

  async function loadRepo(repoPath: string) {
    path.value = repoPath
    loading.value = true
    try {
      await Promise.all([refreshInfo(), refreshFiles(), refreshBranches(), refreshHistory(), refreshStashes()])
    } finally {
      loading.value = false
    }
  }

  async function refreshInfo() {
    try {
      info.value = await invoke<RepoInfo>('get_repo_info', { path: path.value })
    } catch (e) {
      showToast('error', `获取仓库信息失败: ${e}`)
    }
  }

  async function refreshFiles() {
    try {
      files.value = await invoke<FileChange[]>('get_file_statuses', { path: path.value })
    } catch (e) {
      showToast('error', `获取文件状态失败: ${e}`)
    }
  }

  async function refreshBranches() {
    try {
      branches.value = await invoke<BranchInfo[]>('get_branches', { path: path.value })
    } catch (e) {
      showToast('error', `获取分支列表失败: ${e}`)
    }
  }

  async function refreshHistory() {
    try {
      commits.value = await invoke<CommitInfo[]>('get_commit_history', {
        path: path.value,
        count: 100,
      })
    } catch (e) {
      showToast('error', `获取提交历史失败: ${e}`)
    }
  }

  async function refresh() {
    await Promise.all([refreshInfo(), refreshFiles(), refreshBranches(), refreshHistory(), refreshStashes()])
  }

  async function stageFiles(filePaths: string[]) {
    operating.value = true
    try {
      await invoke('stage_files', { path: path.value, files: filePaths })
      await refreshFiles()
    } catch (e) {
      showToast('error', `暂存失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function stageAll() {
    operating.value = true
    try {
      await invoke('stage_all', { path: path.value })
      await refreshFiles()
    } catch (e) {
      showToast('error', `暂存全部失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function unstageFiles(filePaths: string[]) {
    operating.value = true
    try {
      await invoke('unstage_files', { path: path.value, files: filePaths })
      await refreshFiles()
    } catch (e) {
      showToast('error', `取消暂存失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function unstageAll() {
    operating.value = true
    try {
      await invoke('unstage_all', { path: path.value })
      await refreshFiles()
    } catch (e) {
      showToast('error', `取消暂存全部失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function commit() {
    if (!commitMessage.value.trim()) {
      showToast('error', '请输入提交信息')
      return
    }
    operating.value = true
    try {
      await invoke('git_commit', { path: path.value, message: commitMessage.value })
      commitMessage.value = ''
      showToast('success', '提交成功')
      await refresh()
    } catch (e) {
      showToast('error', `提交失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function pull() {
    operating.value = true
    try {
      const result = await invoke<string>('git_pull', { path: path.value })
      showToast('success', result || '拉取成功')
      await refresh()
    } catch (e) {
      showToast('error', `拉取失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function push() {
    operating.value = true
    try {
      const result = await invoke<string>('git_push', { path: path.value })
      showToast('success', result || '推送成功')
      await refresh()
    } catch (e) {
      showToast('error', `推送失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function fetch() {
    operating.value = true
    try {
      await invoke<string>('git_fetch', { path: path.value })
      showToast('success', '获取远程更新成功')
      await refresh()
    } catch (e) {
      showToast('error', `获取远程更新失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function fetchBranch(branch: string) {
    operating.value = true
    try {
      await invoke<string>('fetch_branch', { path: path.value, branch })
      showToast('success', `成功获取远程分支：${branch}`)
      await refresh()
    } catch (e) {
      showToast('error', `获取分支失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function checkoutBranch(branch: string) {
    operating.value = true
    try {
      await invoke('checkout_branch', { path: path.value, branch })
      showToast('success', `已切换到 ${branch}`)
      await refresh()
    } catch (e) {
      showToast('error', `切换分支失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function createBranch(name: string) {
    operating.value = true
    try {
      await invoke('create_branch', { path: path.value, name })
      showToast('success', `已创建并切换到 ${name}`)
      await refresh()
    } catch (e) {
      showToast('error', `创建分支失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function deleteBranch(name: string) {
    operating.value = true
    try {
      await invoke('delete_branch', { path: path.value, name })
      showToast('success', `已删除分支 ${name}`)
      await refreshBranches()
    } catch (e) {
      showToast('error', `删除分支失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function discardChanges(filePaths: string[]) {
    operating.value = true
    try {
      await invoke('discard_changes', { path: path.value, files: filePaths })
      await refreshFiles()
      showToast('success', '已丢弃更改')
    } catch (e) {
      showToast('error', `丢弃更改失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function selectCommit(hash: string) {
    selectedCommitHash.value = hash
    try {
      selectedCommitFiles.value = await invoke<CommitFile[]>('get_commit_files', {
        path: path.value,
        hash,
      })
    } catch (e) {
      showToast('error', `获取提交详情失败: ${e}`)
    }
  }

  async function openInVscode(file?: string) {
    try {
      await invoke('open_in_vscode', { repoPath: path.value, file: file || null })
    } catch (e) {
      showToast('error', `${e}`)
    }
  }

  async function openTerminal() {
    try {
      await invoke('open_terminal', { repoPath: path.value })
    } catch (e) {
      showToast('error', `${e}`)
    }
  }

  async function openRemote() {
    try {
      await invoke('open_remote', { repoPath: path.value })
    } catch (e) {
      showToast('error', `${e}`)
    }
  }

  // ===== Stash =====

  async function refreshStashes() {
    try {
      stashes.value = await invoke<StashEntry[]>('stash_list', { path: path.value })
    } catch (e) {
      showToast('error', `获取 stash 列表失败: ${e}`)
    }
  }

  async function stashSave(message?: string) {
    operating.value = true
    try {
      await invoke('stash_save', { path: path.value, message: message || null })
      showToast('success', '已暂存工作区更改')
      await Promise.all([refreshFiles(), refreshStashes()])
    } catch (e) {
      showToast('error', `Stash 失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function stashApply(index: number) {
    operating.value = true
    try {
      await invoke('stash_apply', { path: path.value, index })
      showToast('success', '已应用 stash')
      await Promise.all([refreshFiles(), refreshStashes()])
    } catch (e) {
      showToast('error', `应用 stash 失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function stashPop(index: number) {
    operating.value = true
    try {
      await invoke('stash_pop', { path: path.value, index })
      showToast('success', '已弹出 stash')
      await Promise.all([refreshFiles(), refreshStashes()])
    } catch (e) {
      showToast('error', `弹出 stash 失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function stashDrop(index: number) {
    operating.value = true
    try {
      await invoke('stash_drop', { path: path.value, index })
      showToast('success', '已删除 stash')
      await refreshStashes()
    } catch (e) {
      showToast('error', `删除 stash 失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  // ===== Merge =====

  async function mergeBranch(branch: string) {
    operating.value = true
    try {
      const result = await invoke<string>('merge_branch', { path: path.value, branch })
      showToast('success', result || `已合并 ${branch}`)
      await refresh()
    } catch (e) {
      showToast('error', `合并失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  // ===== 提交增强 =====

  async function commitAmend(message: string) {
    operating.value = true
    try {
      await invoke('commit_amend', { path: path.value, message })
      commitMessage.value = ''
      showToast('success', '已修改上次提交')
      await refresh()
    } catch (e) {
      showToast('error', `修改提交失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function undoLastCommit() {
    operating.value = true
    try {
      await invoke('undo_last_commit', { path: path.value })
      showToast('success', '已撤销上次提交，更改回到暂存区')
      await refresh()
    } catch (e) {
      showToast('error', `撤销失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function cherryPick(hash: string) {
    operating.value = true
    try {
      await invoke('cherry_pick', { path: path.value, hash })
      showToast('success', 'Cherry-pick 成功')
      await refresh()
    } catch (e) {
      showToast('error', `Cherry-pick 失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function revertCommit(hash: string) {
    operating.value = true
    try {
      await invoke('revert_commit', { path: path.value, hash })
      showToast('success', 'Revert 成功')
      await refresh()
    } catch (e) {
      showToast('error', `Revert 失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  // ===== 丢弃所有 =====

  async function discardAllChanges() {
    operating.value = true
    try {
      await invoke('discard_all_changes', { path: path.value })
      showToast('success', '已丢弃所有更改')
      await refreshFiles()
    } catch (e) {
      showToast('error', `丢弃失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  // ===== 搜索提交 =====

  async function searchCommits(query: string) {
    try {
      commits.value = await invoke<CommitInfo[]>('search_commits', { path: path.value, query, count: 100 })
    } catch (e) {
      showToast('error', `搜索失败: ${e}`)
    }
  }

  // ===== 分支增强 =====

  async function renameBranch(oldName: string, newName: string) {
    operating.value = true
    try {
      await invoke('rename_branch', { path: path.value, oldName, newName })
      showToast('success', `已重命名分支为 ${newName}`)
      await refresh()
    } catch (e) {
      showToast('error', `重命名失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function forcePush() {
    operating.value = true
    try {
      const result = await invoke<string>('force_push', { path: path.value })
      showToast('success', result || '强制推送成功')
      await refresh()
    } catch (e) {
      showToast('error', `强制推送失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  async function pullRebase() {
    operating.value = true
    try {
      const result = await invoke<string>('pull_rebase', { path: path.value })
      showToast('success', result || '拉取 (rebase) 成功')
      await refresh()
    } catch (e) {
      showToast('error', `拉取 (rebase) 失败: ${e}`)
    } finally {
      operating.value = false
    }
  }

  return {
    path,
    info,
    files,
    branches,
    commits,
    activeTab,
    loading,
    operating,
    commitMessage,
    selectedCommitHash,
    selectedCommitFiles,
    toasts,
    stagedFiles,
    unstagedFiles,
    localBranches,
    remoteBranches,
    showToast,
    loadRepo,
    refresh,
    refreshFiles,
    refreshHistory,
    stageFiles,
    stageAll,
    unstageFiles,
    unstageAll,
    commit,
    pull,
    push,
    fetch,
    fetchBranch,
    checkoutBranch,
    createBranch,
    deleteBranch,
    discardChanges,
    discardAllChanges,
    selectCommit,
    openInVscode,
    openTerminal,
    openRemote,
    stashes,
    refreshStashes,
    stashSave,
    stashApply,
    stashPop,
    stashDrop,
    mergeBranch,
    commitAmend,
    undoLastCommit,
    cherryPick,
    revertCommit,
    searchCommits,
    renameBranch,
    forcePush,
    pullRebase,
  }
})
