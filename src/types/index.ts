export interface Project {
  name: string
  path: string
  last_opened: string | null
  favorite: boolean
}

export interface RepoInfo {
  current_branch: string
  remote_url: string | null
  is_clean: boolean
  head_message: string | null
}

export interface FileChange {
  path: string
  status: string
  staged: boolean
  original_path: string | null
}

export interface BranchInfo {
  name: string
  is_current: boolean
  is_remote: boolean
}

export interface CommitInfo {
  hash: string
  short_hash: string
  message: string
  author: string
  email: string
  date: string
  parents: string[]
}

export interface CommitFile {
  path: string
  status: string
}

export interface StashEntry {
  index: number
  message: string
}

export interface Toast {
  id: number
  type: 'success' | 'error' | 'info'
  message: string
}
