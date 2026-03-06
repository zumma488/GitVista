# GitVista 开发计划

## 项目概述

GitVista 是基于 **Tauri 2 + Vue 3** 的轻量级跨平台 Git 图形化管理工具。

- **前端**：Vue 3 + Pinia + Vue Router + Lucide Icons + TypeScript
- **后端**：Rust (Tauri 2)，通过 `std::process::Command` 调用系统 `git` CLI
- **数据存储**：`~/.gitvista/projects.json`

---

## 开发进度

### ✅ 已完成

#### 🚨 Bugfix

| 功能 | 说明 | 涉及文件 |
|------|------|----------|
| ~~窗口控制按钮失效~~ | 拆分 `data-tauri-drag-region` 拖拽区域与按钮区域，按钮添加 `@mousedown.stop.prevent` | `src/components/TitleBar.vue` |
| ~~CMD 窗口闪现~~ | 所有 `Command::new()` 调用在 Windows 上添加 `CREATE_NO_WINDOW` 创建标志 | `src-tauri/src/git.rs` |

#### 🔴 P0 — 核心体验补全

| 功能 | 说明 | 涉及文件 |
|------|------|----------|
| ~~Stash 暂存管理~~ | 创建/应用/弹出/删除 stash，侧边栏 Stash 面板 | `git.rs`, `lib.rs`, `repo.ts`, `types/index.ts`, `RepoSidebar.vue` |
| ~~分支合并~~ | 侧边栏"合并分支"区域，选择目标分支执行 merge | `git.rs`, `lib.rs`, `repo.ts`, `RepoSidebar.vue` |

#### 🟡 P1 — 常用工作流增强

| 功能 | 说明 | 涉及文件 |
|------|------|----------|
| ~~远程分支检出~~ | 点击远程分支直接 checkout 到本地 | `RepoSidebar.vue` |
| ~~提交历史搜索~~ | History 面板搜索框，按关键词过滤提交记录 | `git.rs`, `lib.rs`, `repo.ts`, `HistoryPanel.vue` |
| ~~撤销上次提交~~ | 提交区域 Undo 按钮 (`git reset --soft HEAD~1`) | `git.rs`, `lib.rs`, `repo.ts`, `ChangesPanel.vue` |
| ~~修改提交 (Amend)~~ | 提交区域 Amend 切换模式 | `git.rs`, `lib.rs`, `repo.ts`, `ChangesPanel.vue` |
| ~~Cherry-pick~~ | 提交详情中 Cherry-pick 按钮 | `git.rs`, `lib.rs`, `repo.ts`, `HistoryPanel.vue` |
| ~~Revert 回退~~ | 提交详情中 Revert 按钮 | `git.rs`, `lib.rs`, `repo.ts`, `HistoryPanel.vue` |
| ~~丢弃所有更改~~ | 未暂存区域垃圾桶按钮，接入已有后端 `discard_all_changes` | `repo.ts`, `ChangesPanel.vue` |

#### 其他已完成功能

| 功能 | 说明 |
|------|------|
| 无边框窗口 | 自定义标题栏（最小化/最大化/关闭），`decorations: false` |
| 首页双布局 | 卡片/列表视图切换，localStorage 持久化偏好 |
| 侧边栏分支搜索 | 实时过滤本地/远程分支 |
| 工具栏简化 | 顶部仅静态显示当前分支名，分支切换移至侧边栏 |

---

### ⬜ 待开发

#### 🟢 P2 — 体验优化（下一步）

| 序号 | 功能 | 说明 | 复杂度 | 涉及文件（建议） |
|------|------|------|--------|-----------------|
| 1 | **右键上下文菜单** | 文件右键：暂存/取消暂存/丢弃/在VSCode打开；分支右键：切换/删除/重命名；提交右键：Cherry-pick/Revert/复制哈希 | ⭐⭐ | 新建 `src/components/ContextMenu.vue`，修改 `ChangesPanel.vue`、`RepoSidebar.vue`、`HistoryPanel.vue` |
| 2 | **键盘快捷键** | `Ctrl+S` 暂存全部、`Ctrl+Shift+Z` 撤销提交、`Ctrl+Enter` 提交（已有）、`Ctrl+P` 搜索项目等 | ⭐⭐ | `App.vue` 或新建 `composables/useShortcuts.ts` |
| 3 | **深色/浅色主题切换** | 添加浅色主题 CSS 变量，标题栏或设置中切换 | ⭐⭐ | `src/assets/base.css`（添加 `[data-theme="light"]`）、`TitleBar.vue` 或新建设置面板 |
| 4 | **项目排序/搜索** | 首页项目列表：按名称/最近打开排序，输入关键词搜索过滤 | ⭐ | `ProjectListView.vue` |
| 5 | **项目收藏/置顶** | 项目卡片/列表行增加星标按钮，收藏项目置顶显示 | ⭐ | `ProjectListView.vue`、`stores/projects.ts`、`src-tauri/src/store.rs`（持久化 `favorite` 字段） |

#### 🔵 P3 — 高级功能

> **注意**：分支重命名、强制推送、Pull Rebase、初始化仓库的**后端命令已实现**（在 `git.rs` 和 `lib.rs` 中），只需要接入前端 UI。

| 序号 | 功能 | 说明 | 复杂度 | 后端状态 |
|------|------|------|--------|---------|
| 1 | **分支重命名** | 分支右键或编辑按钮，调用 `rename_branch` | ⭐ | ✅ 已实现 |
| 2 | **Force Push** | 工具栏 Push 按钮长按或下拉选择强制推送，调用 `force_push` | ⭐ | ✅ 已实现 |
| 3 | **Pull with Rebase** | 工具栏 Pull 按钮下拉选择 rebase 模式，调用 `pull_rebase` | ⭐ | ✅ 已实现 |
| 4 | **文件历史** | 查看单个文件的提交历史 | ⭐⭐ | ⬜ 需新增 |
| 5 | **初始化仓库** | 首页"初始化新仓库"按钮，调用 `init_repo` | ⭐ | ✅ 已实现 |
| 6 | **远程仓库管理** | 添加/移除/编辑 remote | ⭐⭐ | ⬜ 需新增 |
| 7 | **Tag 管理** | 创建/删除/推送标签 | ⭐⭐ | ⬜ 需新增 |
| 8 | **多编辑器支持** | 设置页面可配置外部编辑器（不局限于 VSCode） | ⭐ | ⬜ 需新增 |
| 9 | **多语言 / i18n** | 支持中英文切换 | ⭐⭐ | ⬜ 需新增 |

#### ⚪ P4 — 暂缓（复杂度 ⭐⭐⭐）

| 功能 | 说明 | 原因 |
|------|------|------|
| 提交关系图 | 用线条可视化分支/合并关系 | 需要自定义 Canvas/SVG 绘制，复杂度高 |
| 交互式 Rebase | 交互式变基 UI | 需要复杂的拖拽排序 + 多步操作 UI |
| Blame 逐行追溯 | 查看每行代码的最后修改者 | 需要自定义代码查看器 |
| Diff 查看器 | 文件变更行级对比 | 用户明确排除 |
| 合并冲突处理 | 冲突文件标记 + 编辑器 | 用户明确排除 |

---

## 建议版本路线

```
v0.4.0  →  P2: 右键菜单 + 键盘快捷键 + 主题切换 + 项目排序/搜索/收藏
v0.5.0  →  P3: 分支重命名 + Force Push + Pull Rebase + 初始化仓库（接入已有后端）
v0.6.0  →  P3: 文件历史 + Tag 管理 + 远程仓库管理
v0.7.0  →  P3: 多编辑器支持 + 多语言 i18n
v1.0.0  →  稳定版发布
```

---

## 架构说明

### 新增功能的标准开发流程

1. **Rust 后端** (`src-tauri/src/git.rs`)：添加 git 命令封装函数
2. **命令注册** (`src-tauri/src/lib.rs`)：添加 `#[tauri::command]` 并注册到 `invoke_handler`
3. **类型定义** (`src/types/index.ts`)：如有新数据结构，添加 TypeScript interface
4. **Store** (`src/stores/repo.ts`)：添加 store action，调用 `invoke()`，处理错误和 toast
5. **组件** (`src/components/repo/*.vue`)：添加 UI 交互，调用 store action
6. **验证**：`npx vue-tsc --build` + `cargo check` + `npx tauri dev` 手动测试

### 已预埋的后端命令（P3 可直接使用）

| Store Action | Tauri Command | Git 命令 |
|-------------|---------------|----------|
| `renameBranch(old, new)` | `rename_branch` | `git branch -m <old> <new>` |
| `forcePush()` | `force_push` | `git push --force-with-lease` |
| `pullRebase()` | `pull_rebase` | `git pull --rebase` |
| — | `init_repo` | `git init <path>` |
| `searchCommits(query)` | `search_commits` | `git log --grep <query> -i` |

这些 action 已在 `stores/repo.ts` 中定义并导出，前端 UI 只需调用即可。

---

## CI/CD 说明

- 推送 `v*` 标签到 `main` 分支会触发 GitHub Actions 多平台构建
- 构建完成后自动创建 GitHub Release 并上传安装包
- Workflow 文件：`.github/workflows/build.yml`
- **注意**：云代理的 bot token 推送标签不会触发 Actions，需要用户本机执行 `git push origin <tag>` 来触发
