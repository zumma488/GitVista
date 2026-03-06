# AGENTS.md

## Cursor Cloud specific instructions

### Project overview

GitVista 是基于 **Tauri 2 + Vue 3** 的轻量级跨平台 Git GUI 桌面应用（Rust 后端）。详见 `README.md`。

**完整开发计划和进度**请参阅 `DEVELOPMENT_PLAN.md`，其中包含已完成功能清单、待开发功能及建议版本路线。

### Services

| Service | How to run | Notes |
|---------|-----------|-------|
| Tauri dev (full app) | `npx tauri dev` | Starts both Vite frontend (port 5173) and Tauri desktop window |
| Vite frontend only | `npm run dev` | Frontend-only at `http://localhost:5173`; Tauri API calls will fail |

### Key commands

| Task | Command |
|------|---------|
| Type-check | `npx vue-tsc --build` |
| Rust check | `cargo check` (from `src-tauri/`) |
| Lint | `npm run lint` |
| Frontend build | `npx vite build` |
| Full dev mode | `npx tauri dev` |

### Development workflow for new features

1. **Rust**: `src-tauri/src/git.rs` 添加 git 命令封装
2. **Register**: `src-tauri/src/lib.rs` 添加 `#[tauri::command]` 并注册到 `invoke_handler`
3. **Types**: `src/types/index.ts` 添加 TypeScript interface（如需）
4. **Store**: `src/stores/repo.ts` 添加 action，调用 `invoke()`
5. **UI**: `src/components/repo/*.vue` 添加界面交互
6. **Verify**: `npx vue-tsc --build` → `cargo check` → `npx tauri dev`

### Pre-built backend commands (ready for UI)

`stores/repo.ts` 中已导出但尚未在 UI 中使用的 action（P3 后端已实现）：
- `renameBranch(oldName, newName)` — 分支重命名
- `forcePush()` — 强制推送 (`--force-with-lease`)
- `pullRebase()` — Pull with rebase

`lib.rs` 中已注册但 store 中未封装的命令：
- `init_repo(path)` — 初始化新仓库

### Non-obvious caveats

- **Rust toolchain**: 需要 Rust >= 1.85（依赖 edition 2024）。如 `cargo build` 报 `edition2024` 错误，执行 `rustup update stable && rustup default stable`。
- **Linux 系统库**: Tauri 需要 `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `librsvg2-dev`，不在 `npm install` 范围内。
- **端口冲突**: Tauri `devUrl` 固定为 `http://localhost:5173`，启动前先杀掉占用 5173 端口的进程。
- **Display**: `npx tauri dev` 需要显示服务器（X11/Wayland），需 `DISPLAY` 环境变量。
- **无测试文件**: 项目无 unit test，`npx vitest run` 会因找不到测试文件而 exit 1。
- **CI 触发**: 云代理 bot token 推送 tag 不触发 GitHub Actions。发版需要用户本机执行 `git push origin <tag>`。
- **Windows CMD 闪现**: 已修复 — `git.rs` 中所有 `Command` 调用在 Windows 上使用 `CREATE_NO_WINDOW` 标志。
