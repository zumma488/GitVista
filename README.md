# GitVista

轻量级 Git 图形化管理工具，基于 **Tauri 2 + Vue 3** 构建的跨平台桌面应用。

![License](https://img.shields.io/badge/license-MIT-blue)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-brightgreen)
![Tauri](https://img.shields.io/badge/Tauri-2.x-blue)
![Vue](https://img.shields.io/badge/Vue-3.x-green)

## 功能特性

### 项目管理

- 项目列表首页，卡片式展示所有 Git 仓库
- 打开本地已有的 Git 仓库
- 克隆远程仓库到本地
- 显示每个项目的当前分支、最近打开时间
- 移除项目（仅从列表移除，不删除本地文件）

### 文件变更

- 查看已暂存 / 未暂存的文件变更列表
- 单文件或批量暂存（Stage）/ 取消暂存（Unstage）
- 丢弃工作区更改
- 双击文件在 **VSCode** 中查看 Diff
- 每 5 秒自动刷新文件状态

### 提交与同步

- 输入提交信息并提交（支持 `Ctrl+Enter` 快捷提交）
- Pull / Push / Fetch 远程操作
- 操作状态实时反馈（Toast 通知）

### 分支管理

- 查看本地分支和远程分支
- 创建新分支
- 切换分支
- 删除分支

### 提交历史

- 时间线展示提交记录（哈希、信息、作者、时间）
- 点击提交查看变更文件列表
- 双击变更文件在 VSCode 中打开

## 界面预览

```
┌──────────────────────────────────────────────────────┐
│  ← 仓库名  [main ▾]   Fetch  Pull  Push    VSCode   │
├────────────┬─────────────────────────────────────────┤
│  变更 | 历史 │  已暂存的更改 (3)              [−]     │
│            │   M src/app.vue                         │
│  本地分支   │   A src/utils.ts                       │
│  ● main    │   D old-file.js                         │
│    dev     │                                         │
│    feature │  未暂存的更改 (2)              [+]       │
│            │   M README.md                           │
│  远程分支   │   ? new-file.ts                        │
│   origin/… │                                         │
│            │  ┌─────────────────────────────┐        │
│            │  │ 输入提交信息...               │        │
│            │  └─────────────────────────────┘        │
│            │  [ ✓ 提交 (3 个文件) ]                   │
├────────────┴─────────────────────────────────────────┤
│  ● main  │  3 个文件变更  │  /path/to/repo           │
└──────────────────────────────────────────────────────┘
```

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端框架 | Vue 3 (Composition API + `<script setup>`) |
| 状态管理 | Pinia |
| 路由 | Vue Router |
| 图标 | Lucide Icons |
| 桌面框架 | Tauri 2 |
| 后端语言 | Rust |
| Git 操作 | 通过 `std::process::Command` 调用系统 `git` CLI |
| 数据存储 | 本地 JSON 文件 (`~/.gitvista/projects.json`) |
| 构建工具 | Vite |

## 项目结构

```
├── src/                          # Vue 前端
│   ├── views/
│   │   ├── ProjectListView.vue   # 首页 - 项目列表
│   │   └── RepoView.vue          # 仓库管理主界面
│   ├── components/repo/
│   │   ├── RepoToolbar.vue       # 顶部工具栏
│   │   ├── RepoSidebar.vue       # 左侧边栏（分支管理）
│   │   ├── ChangesPanel.vue      # 文件变更面板
│   │   ├── HistoryPanel.vue      # 提交历史面板
│   │   └── StatusBar.vue         # 底部状态栏
│   ├── stores/
│   │   ├── projects.ts           # 项目列表状态
│   │   └── repo.ts               # 仓库操作状态
│   └── types/
│       └── index.ts              # TypeScript 类型定义
├── src-tauri/                    # Rust 后端
│   └── src/
│       ├── git.rs                # Git CLI 封装
│       ├── store.rs              # 项目持久化存储
│       ├── lib.rs                # Tauri 命令注册
│       └── main.rs               # 入口
└── .github/workflows/
    └── build.yml                 # 多平台自动构建
```

## 开发指南

### 环境要求

- [Node.js](https://nodejs.org/) >= 22
- [Rust](https://www.rust-lang.org/tools/install) >= 1.77
- [Git](https://git-scm.com/)
- **Linux 额外依赖：**

  ```bash
  sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev librsvg2-dev
  ```

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npx tauri dev
```

启动后会同时运行 Vite 前端热重载和 Tauri 桌面窗口。

### 构建生产包

```bash
npx tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`：

| 平台 | 格式 |
|------|------|
| Windows | `.msi`, `.exe` (NSIS) |
| macOS | `.dmg`, `.app` |
| Linux | `.deb`, `.rpm`, `.AppImage` |

### CI/CD 自动构建

项目已配置 GitHub Actions 多平台构建，推送版本标签即可自动触发：

```bash
git tag v0.1.0
git push origin v0.1.0
```

构建完成后在 GitHub Actions 页面的 **Artifacts** 区域下载各平台安装包。

## 许可证

MIT
