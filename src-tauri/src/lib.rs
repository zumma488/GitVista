mod git;
mod store;

use store::Project;
use tauri::Emitter;

#[tauri::command]
fn list_projects() -> Vec<Project> {
    store::load_projects()
}

#[tauri::command]
fn add_project(path: String) -> Result<Vec<Project>, String> {
    if !git::is_git_repo(&path) {
        return Err("所选文件夹不是一个 Git 仓库".to_string());
    }

    let name = std::path::Path::new(&path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());

    store::add_project(&name, &path)
}

#[tauri::command]
fn remove_project(path: String) -> Result<Vec<Project>, String> {
    store::remove_project(&path)
}

#[tauri::command]
fn update_last_opened(path: String) -> Result<(), String> {
    store::update_last_opened(&path)
}

#[tauri::command]
fn is_git_repo(path: String) -> bool {
    git::is_git_repo(&path)
}

#[tauri::command]
fn get_repo_info(path: String) -> Result<git::RepoInfo, String> {
    git::get_repo_info(&path)
}

#[tauri::command]
fn get_file_statuses(path: String) -> Result<Vec<git::FileChange>, String> {
    git::get_file_statuses(&path)
}

#[tauri::command]
fn get_branches(path: String) -> Result<Vec<git::BranchInfo>, String> {
    git::get_branches(&path)
}

#[tauri::command]
fn get_commit_history(path: String, count: Option<usize>) -> Result<Vec<git::CommitInfo>, String> {
    git::get_commit_history(&path, count.unwrap_or(50))
}

#[tauri::command]
fn stage_files(path: String, files: Vec<String>) -> Result<(), String> {
    git::stage_files(&path, &files)
}

#[tauri::command]
fn stage_all(path: String) -> Result<(), String> {
    git::stage_all(&path)
}

#[tauri::command]
fn unstage_files(path: String, files: Vec<String>) -> Result<(), String> {
    git::unstage_files(&path, &files)
}

#[tauri::command]
fn unstage_all(path: String) -> Result<(), String> {
    git::unstage_all(&path)
}

#[tauri::command]
fn git_commit(path: String, message: String) -> Result<String, String> {
    git::commit(&path, &message)
}

#[tauri::command]
fn git_pull(path: String) -> Result<String, String> {
    git::pull(&path)
}

#[tauri::command]
fn git_push(path: String) -> Result<String, String> {
    git::push(&path)
}

#[tauri::command]
fn git_fetch(path: String) -> Result<String, String> {
    git::fetch_all(&path)
}

#[tauri::command]
fn checkout_branch(path: String, branch: String) -> Result<(), String> {
    git::checkout_branch(&path, &branch)
}

#[tauri::command]
fn create_branch(path: String, name: String) -> Result<(), String> {
    git::create_branch(&path, &name)
}

#[tauri::command]
fn delete_branch(path: String, name: String) -> Result<(), String> {
    git::delete_branch(&path, &name)
}

#[tauri::command]
fn discard_changes(path: String, files: Vec<String>) -> Result<(), String> {
    git::discard_changes(&path, &files)
}

#[tauri::command]
fn discard_all_changes(path: String) -> Result<(), String> {
    git::discard_all_changes(&path)
}

#[tauri::command]
fn get_commit_files(path: String, hash: String) -> Result<Vec<git::CommitFile>, String> {
    git::get_commit_files(&path, &hash)
}

#[tauri::command]
fn open_in_vscode(repo_path: String, file: Option<String>) -> Result<(), String> {
    git::open_in_vscode(&repo_path, file.as_deref())
}

#[tauri::command]
fn clone_repo(url: String, target: String) -> Result<String, String> {
    git::clone_repo(&url, &target)
}

#[tauri::command]
fn stash_list(path: String) -> Result<Vec<git::StashEntry>, String> {
    git::stash_list(&path)
}

#[tauri::command]
fn stash_save(path: String, message: Option<String>) -> Result<String, String> {
    git::stash_save(&path, message.as_deref())
}

#[tauri::command]
fn stash_apply(path: String, index: usize) -> Result<String, String> {
    git::stash_apply(&path, index)
}

#[tauri::command]
fn stash_pop(path: String, index: usize) -> Result<String, String> {
    git::stash_pop(&path, index)
}

#[tauri::command]
fn stash_drop(path: String, index: usize) -> Result<String, String> {
    git::stash_drop(&path, index)
}

#[tauri::command]
fn merge_branch(path: String, branch: String) -> Result<String, String> {
    git::merge_branch(&path, &branch)
}

#[tauri::command]
fn commit_amend(path: String, message: String) -> Result<String, String> {
    git::commit_amend(&path, &message)
}

#[tauri::command]
fn undo_last_commit(path: String) -> Result<String, String> {
    git::undo_last_commit(&path)
}

#[tauri::command]
fn cherry_pick(path: String, hash: String) -> Result<String, String> {
    git::cherry_pick(&path, &hash)
}

#[tauri::command]
fn revert_commit(path: String, hash: String) -> Result<String, String> {
    git::revert_commit(&path, &hash)
}

#[tauri::command]
fn rename_branch(path: String, old_name: String, new_name: String) -> Result<(), String> {
    git::rename_branch(&path, &old_name, &new_name)
}

#[tauri::command]
fn force_push(path: String) -> Result<String, String> {
    git::force_push(&path)
}

#[tauri::command]
fn pull_rebase(path: String) -> Result<String, String> {
    git::pull_rebase(&path)
}

#[tauri::command]
fn init_repo(path: String) -> Result<String, String> {
    git::init_repo(&path)
}

#[tauri::command]
fn search_commits(path: String, query: String, count: Option<usize>) -> Result<Vec<git::CommitInfo>, String> {
    git::search_commits(&path, &query, count.unwrap_or(50))
}

#[tauri::command]
fn toggle_favorite(path: String) -> Result<Vec<Project>, String> {
    store::toggle_favorite(&path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // --- 系统托盘 ---
            use tauri::{
                Manager,
                menu::{Menu, MenuItem},
                tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
            };

            let show_i = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("GitVista")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // 拦截窗口关闭请求，通知前端让其决定
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.emit("close-requested", ());
            }
        })
        .invoke_handler(tauri::generate_handler![
            list_projects,
            add_project,
            remove_project,
            update_last_opened,
            is_git_repo,
            get_repo_info,
            get_file_statuses,
            get_branches,
            get_commit_history,
            stage_files,
            stage_all,
            unstage_files,
            unstage_all,
            git_commit,
            git_pull,
            git_push,
            git_fetch,
            checkout_branch,
            create_branch,
            delete_branch,
            discard_changes,
            discard_all_changes,
            get_commit_files,
            open_in_vscode,
            clone_repo,
            stash_list,
            stash_save,
            stash_apply,
            stash_pop,
            stash_drop,
            merge_branch,
            commit_amend,
            undo_last_commit,
            cherry_pick,
            revert_commit,
            rename_branch,
            force_push,
            pull_rebase,
            init_repo,
            search_commits,
            toggle_favorite,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
