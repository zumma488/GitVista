import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Project, RepoInfo } from '@/types'

export const useProjectsStore = defineStore('projects', () => {
  const projects = ref<Project[]>([])
  const projectInfoMap = ref<Record<string, RepoInfo>>({})
  const loading = ref(false)

  async function loadProjects() {
    loading.value = true
    try {
      projects.value = await invoke<Project[]>('list_projects')
      for (const project of projects.value) {
        loadProjectInfo(project.path)
      }
    } finally {
      loading.value = false
    }
  }

  async function loadProjectInfo(path: string) {
    try {
      const info = await invoke<RepoInfo>('get_repo_info', { path })
      projectInfoMap.value[path] = info
    } catch {
      // 仓库可能已被删除或移动
    }
  }

  async function addProject(path: string): Promise<void> {
    projects.value = await invoke<Project[]>('add_project', { path })
    loadProjectInfo(path)
  }

  async function removeProject(path: string): Promise<void> {
    projects.value = await invoke<Project[]>('remove_project', { path })
    delete projectInfoMap.value[path]
  }

  async function updateLastOpened(path: string): Promise<void> {
    await invoke('update_last_opened', { path })
  }

  return {
    projects,
    projectInfoMap,
    loading,
    loadProjects,
    loadProjectInfo,
    addProject,
    removeProject,
    updateLastOpened,
  }
})
