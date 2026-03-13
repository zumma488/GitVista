use serde::Serialize;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Serialize, Clone, Debug)]
pub struct RepoInfo {
    pub current_branch: String,
    pub remote_url: Option<String>,
    pub is_clean: bool,
    pub head_message: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct FileChange {
    pub path: String,
    pub status: String,
    pub staged: bool,
    pub original_path: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct CommitInfo {
    pub hash: String,
    pub short_hash: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub date: String,
    pub parents: Vec<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct CommitFile {
    pub path: String,
    pub status: String,
}

fn run_git(repo_path: &str, args: &[&str]) -> Result<String, String> {
    let mut cmd = Command::new("git");
    cmd.args(args)
        .current_dir(repo_path)
        .env("GIT_TERMINAL_PROMPT", "0");

    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let output = cmd
        .output()
        .map_err(|e| format!("无法执行 git 命令: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(if stderr.is_empty() { stdout } else { stderr })
    }
}

pub fn is_git_repo(path: &str) -> bool {
    run_git(path, &["rev-parse", "--is-inside-work-tree"]).is_ok()
}

pub fn get_repo_info(path: &str) -> Result<RepoInfo, String> {
    let branch = run_git(path, &["rev-parse", "--abbrev-ref", "HEAD"])
        .unwrap_or_else(|_| "unknown".to_string());

    let remote_url = run_git(path, &["remote", "get-url", "origin"]).ok();

    let status_output = run_git(path, &["status", "--porcelain"])?;
    let is_clean = status_output.is_empty();

    let head_message = run_git(path, &["log", "-1", "--format=%s"]).ok();

    Ok(RepoInfo {
        current_branch: branch,
        remote_url,
        is_clean,
        head_message,
    })
}

pub fn get_file_statuses(path: &str) -> Result<Vec<FileChange>, String> {
    let output = run_git(path, &["status", "--porcelain"])?;
    let mut changes = Vec::new();

    for line in output.lines() {
        if line.len() < 3 {
            continue;
        }

        let index_char = line.as_bytes()[0] as char;
        let worktree_char = line.as_bytes()[1] as char;
        let file_part = &line[3..];

        let (actual_path, original) = if file_part.contains(" -> ") {
            let parts: Vec<&str> = file_part.splitn(2, " -> ").collect();
            (parts[1].to_string(), Some(parts[0].to_string()))
        } else {
            (file_part.to_string(), None)
        };

        if index_char == '?' && worktree_char == '?' {
            changes.push(FileChange {
                path: actual_path,
                status: "untracked".to_string(),
                staged: false,
                original_path: None,
            });
            continue;
        }

        if index_char == '!' {
            continue;
        }

        // 暂存区变更
        if index_char != ' ' && index_char != '?' {
            let status = match index_char {
                'M' => "modified",
                'A' => "added",
                'D' => "deleted",
                'R' => "renamed",
                'C' => "copied",
                'U' => "conflicted",
                _ => "unknown",
            };
            changes.push(FileChange {
                path: actual_path.clone(),
                status: status.to_string(),
                staged: true,
                original_path: original.clone(),
            });
        }

        // 工作区变更
        if worktree_char != ' ' && worktree_char != '?' {
            let status = match worktree_char {
                'M' => "modified",
                'D' => "deleted",
                'U' => "conflicted",
                _ => "unknown",
            };
            changes.push(FileChange {
                path: actual_path.clone(),
                status: status.to_string(),
                staged: false,
                original_path: original,
            });
        }
    }

    Ok(changes)
}

pub fn get_branches(path: &str) -> Result<Vec<BranchInfo>, String> {
    let output = run_git(path, &["branch", "-a", "--no-color"])?;
    let mut branches = Vec::new();

    for line in output.lines() {
        let is_current = line.starts_with('*');
        let name = line.trim_start_matches('*').trim().to_string();

        if name.contains("HEAD detached") || name.contains("->") {
            continue;
        }

        let is_remote = name.starts_with("remotes/");
        let display_name = if is_remote {
            name.trim_start_matches("remotes/").to_string()
        } else {
            name
        };

        branches.push(BranchInfo {
            name: display_name,
            is_current,
            is_remote,
        });
    }

    Ok(branches)
}

#[derive(Serialize, Clone, Debug)]
pub struct BranchStat {
    pub name: String,
    pub ahead: u32,
    pub behind: u32,
}

/// 一次性获取所有本地分支相对于 HEAD 的 ahead/behind 数
/// 使用单条 git for-each-ref 命令（Git ≥ 2.31），避免 N 次调用
pub fn get_branch_stats(path: &str) -> Result<Vec<BranchStat>, String> {
    let format = "%(refname:short)\x1f%(ahead-behind:HEAD)";
    let output = run_git(
        path,
        &["for-each-ref", &format!("--format={}", format), "refs/heads/"],
    )?;

    let mut stats = Vec::new();
    for line in output.lines() {
        let parts: Vec<&str> = line.splitn(2, '\x1f').collect();
        if parts.len() == 2 {
            let name = parts[0].trim().to_string();
            let counts: Vec<&str> = parts[1].trim().split_whitespace().collect();
            if counts.len() == 2 {
                let ahead = counts[0].parse::<u32>().unwrap_or(0);
                let behind = counts[1].parse::<u32>().unwrap_or(0);
                stats.push(BranchStat { name, ahead, behind });
            }
        }
    }

    Ok(stats)
}

pub fn get_commit_history(path: &str, count: usize) -> Result<Vec<CommitInfo>, String> {
    let format = "%H\x1f%h\x1f%s\x1f%an\x1f%ae\x1f%aI\x1f%P";
    let count_str = format!("-{}", count);
    let output = run_git(path, &["log", &count_str, &format!("--format={}", format)])?;
    let mut commits = Vec::new();

    for line in output.lines() {
        let parts: Vec<&str> = line.split('\x1f').collect();
        if parts.len() >= 6 {
            commits.push(CommitInfo {
                hash: parts[0].to_string(),
                short_hash: parts[1].to_string(),
                message: parts[2].to_string(),
                author: parts[3].to_string(),
                email: parts[4].to_string(),
                date: parts[5].to_string(),
                parents: if parts.len() > 6 && !parts[6].is_empty() {
                    parts[6].split(' ').map(|s| s.to_string()).collect()
                } else {
                    vec![]
                },
            });
        }
    }

    Ok(commits)
}

pub fn stage_files(path: &str, files: &[String]) -> Result<(), String> {
    let mut args = vec!["add".to_string(), "--".to_string()];
    args.extend(files.iter().cloned());
    let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run_git(path, &refs)?;
    Ok(())
}

pub fn stage_all(path: &str) -> Result<(), String> {
    run_git(path, &["add", "-A"])?;
    Ok(())
}

pub fn unstage_files(path: &str, files: &[String]) -> Result<(), String> {
    let mut args = vec!["restore".to_string(), "--staged".to_string(), "--".to_string()];
    args.extend(files.iter().cloned());
    let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run_git(path, &refs)?;
    Ok(())
}

pub fn unstage_all(path: &str) -> Result<(), String> {
    run_git(path, &["reset", "HEAD"])?;
    Ok(())
}

pub fn commit(path: &str, message: &str) -> Result<String, String> {
    run_git(path, &["commit", "-m", message])
}

pub fn pull(path: &str) -> Result<String, String> {
    run_git(path, &["pull"])
}

pub fn push(path: &str) -> Result<String, String> {
    run_git(path, &["push"])
}

pub fn fetch_all(path: &str) -> Result<String, String> {
    run_git(path, &["fetch", "--all"])
}

pub fn fetch_branch(path: &str, branch: &str) -> Result<String, String> {
    run_git(path, &["fetch", "origin", branch])
}

pub fn checkout_branch(path: &str, branch: &str) -> Result<(), String> {
    run_git(path, &["checkout", branch])?;
    Ok(())
}

pub fn create_branch(path: &str, name: &str) -> Result<(), String> {
    run_git(path, &["checkout", "-b", name])?;
    Ok(())
}

pub fn delete_branch(path: &str, name: &str) -> Result<(), String> {
    run_git(path, &["branch", "-d", name])?;
    Ok(())
}

pub fn discard_changes(path: &str, files: &[String]) -> Result<(), String> {
    for file in files {
        let result = run_git(path, &["checkout", "--", file]);
        if result.is_err() {
            let full_path = std::path::Path::new(path).join(file);
            let _ = std::fs::remove_file(full_path);
        }
    }
    Ok(())
}

pub fn discard_all_changes(path: &str) -> Result<(), String> {
    run_git(path, &["checkout", "--", "."])?;
    run_git(path, &["clean", "-fd"])?;
    Ok(())
}

pub fn get_commit_files(path: &str, hash: &str) -> Result<Vec<CommitFile>, String> {
    let result = run_git(path, &["diff-tree", "--no-commit-id", "-r", "--name-status", hash]);

    let output = match result {
        Ok(ref o) if !o.is_empty() => o.clone(),
        _ => run_git(path, &["show", "--name-status", "--format=", hash])?,
    };

    parse_commit_files(&output)
}

fn parse_commit_files(output: &str) -> Result<Vec<CommitFile>, String> {
    let mut files = Vec::new();
    for line in output.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(2, '\t').collect();
        if parts.len() == 2 {
            let status = match parts[0].chars().next().unwrap_or('M') {
                'M' => "modified",
                'A' => "added",
                'D' => "deleted",
                'R' => "renamed",
                'C' => "copied",
                _ => "unknown",
            };
            files.push(CommitFile {
                path: parts[1].to_string(),
                status: status.to_string(),
            });
        }
    }
    Ok(files)
}

pub fn open_in_vscode(repo_path: &str, file: Option<&str>) -> Result<(), String> {
    let target = match file {
        Some(f) => std::path::Path::new(repo_path)
            .join(f)
            .to_string_lossy()
            .to_string(),
        None => repo_path.to_string(),
    };

    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", "code", &target]);
        cmd.creation_flags(CREATE_NO_WINDOW);
        cmd.spawn()
            .map_err(|e| format!("无法打开 VSCode: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("code")
            .arg(&target)
            .spawn()
            .map_err(|e| format!("无法打开 VSCode: {}", e))?;
    }

    Ok(())
}

pub fn open_terminal(repo_path: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "cmd.exe", "/K", "cd", "/d", repo_path])
            .spawn()
            .map_err(|e| format!("无法打开终端: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        // 简单处理，对于 macOS 可以调用 open -a Terminal，Linux 视环境而定
        Command::new("open")
            .args(["-a", "Terminal", repo_path])
            .spawn()
            .map_err(|e| format!("无法打开终端: {}", e))?;
    }

    Ok(())
}

pub fn open_remote(repo_path: &str) -> Result<(), String> {
    let output = run_git(repo_path, &["remote", "get-url", "origin"])?;
    let mut url = output.trim().to_string();
    
    // 如果是 ssh 格式类似 git@github.com:user/repo.git，则替换为 https 链接
    if url.starts_with("git@") {
        url = url.replace(":", "/").replace("git@", "https://");
    }
    if url.ends_with(".git") {
        url = url.trim_end_matches(".git").to_string();
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", &url])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| format!("无法在浏览器中打开远端: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("无法在浏览器中打开远端: {}", e))?;
    }

    Ok(())
}

// ===== Stash 操作 =====

#[derive(Serialize, Clone, Debug)]
pub struct StashEntry {
    pub index: usize,
    pub message: String,
}

pub fn stash_list(path: &str) -> Result<Vec<StashEntry>, String> {
    let output = run_git(path, &["stash", "list", "--format=%gd\x1f%s"])?;
    let mut entries = Vec::new();
    for line in output.lines() {
        let parts: Vec<&str> = line.split('\x1f').collect();
        if parts.len() >= 2 {
            let idx_str = parts[0].trim_start_matches("stash@{").trim_end_matches('}');
            if let Ok(index) = idx_str.parse::<usize>() {
                entries.push(StashEntry {
                    index,
                    message: parts[1].to_string(),
                });
            }
        }
    }
    Ok(entries)
}

pub fn stash_save(path: &str, message: Option<&str>) -> Result<String, String> {
    match message {
        Some(msg) if !msg.is_empty() => run_git(path, &["stash", "push", "-m", msg]),
        _ => run_git(path, &["stash", "push"]),
    }
}

pub fn stash_apply(path: &str, index: usize) -> Result<String, String> {
    run_git(path, &["stash", "apply", &format!("stash@{{{}}}", index)])
}

pub fn stash_pop(path: &str, index: usize) -> Result<String, String> {
    run_git(path, &["stash", "pop", &format!("stash@{{{}}}", index)])
}

pub fn stash_drop(path: &str, index: usize) -> Result<String, String> {
    run_git(path, &["stash", "drop", &format!("stash@{{{}}}", index)])
}

// ===== Merge 操作 =====

pub fn merge_branch(path: &str, branch: &str) -> Result<String, String> {
    run_git(path, &["merge", branch])
}

// ===== 提交操作增强 =====

pub fn commit_amend(path: &str, message: &str) -> Result<String, String> {
    run_git(path, &["commit", "--amend", "-m", message])
}

pub fn undo_last_commit(path: &str) -> Result<String, String> {
    run_git(path, &["reset", "--soft", "HEAD~1"])
}

pub fn cherry_pick(path: &str, hash: &str) -> Result<String, String> {
    run_git(path, &["cherry-pick", hash])
}

pub fn revert_commit(path: &str, hash: &str) -> Result<String, String> {
    run_git(path, &["revert", "--no-edit", hash])
}

// ===== 分支操作增强 =====

pub fn rename_branch(path: &str, old_name: &str, new_name: &str) -> Result<(), String> {
    run_git(path, &["branch", "-m", old_name, new_name])?;
    Ok(())
}

pub fn force_push(path: &str) -> Result<String, String> {
    run_git(path, &["push", "--force-with-lease"])
}

pub fn pull_rebase(path: &str) -> Result<String, String> {
    run_git(path, &["pull", "--rebase"])
}

pub fn init_repo(path: &str) -> Result<String, String> {
    let mut cmd = Command::new("git");
    cmd.args(["init", path]);

    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let result = cmd.output().map_err(|e| format!("初始化失败: {}", e))?;

    if result.status.success() {
        Ok("仓库初始化成功".to_string())
    } else {
        Err(String::from_utf8_lossy(&result.stderr).trim().to_string())
    }
}

// ===== 提交搜索 =====

pub fn search_commits(path: &str, query: &str, count: usize) -> Result<Vec<CommitInfo>, String> {
    let format = "%H\x1f%h\x1f%s\x1f%an\x1f%ae\x1f%aI\x1f%P";
    let count_str = format!("-{}", count);
    let output = run_git(path, &["log", &count_str, &format!("--format={}", format), "--grep", query, "-i"])?;
    let mut commits = Vec::new();
    for line in output.lines() {
        let parts: Vec<&str> = line.split('\x1f').collect();
        if parts.len() >= 6 {
            commits.push(CommitInfo {
                hash: parts[0].to_string(),
                short_hash: parts[1].to_string(),
                message: parts[2].to_string(),
                author: parts[3].to_string(),
                email: parts[4].to_string(),
                date: parts[5].to_string(),
                parents: if parts.len() > 6 && !parts[6].is_empty() {
                    parts[6].split(' ').map(|s| s.to_string()).collect()
                } else {
                    vec![]
                },
            });
        }
    }
    Ok(commits)
}

pub fn clone_repo(url: &str, target: &str) -> Result<String, String> {
    let mut cmd = Command::new("git");
    cmd.args(["clone", url, target]);

    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let output = cmd
        .output()
        .map_err(|e| format!("克隆失败: {}", e))?;

    if output.status.success() {
        Ok("克隆成功".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}
