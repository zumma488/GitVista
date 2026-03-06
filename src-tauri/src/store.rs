use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub last_opened: Option<String>,
    #[serde(default)]
    pub favorite: bool,
}

fn store_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".gitvista")
}

fn store_path() -> PathBuf {
    store_dir().join("projects.json")
}

pub fn load_projects() -> Vec<Project> {
    let path = store_path();
    if !path.exists() {
        return Vec::new();
    }

    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn save_projects(projects: &[Project]) -> Result<(), String> {
    let dir = store_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("创建目录失败: {}", e))?;

    let json =
        serde_json::to_string_pretty(projects).map_err(|e| format!("序列化失败: {}", e))?;

    fs::write(store_path(), json).map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(())
}

pub fn add_project(name: &str, path: &str) -> Result<Vec<Project>, String> {
    let mut projects = load_projects();

    if projects.iter().any(|p| p.path == path) {
        return Err("该项目已存在".to_string());
    }

    projects.push(Project {
        name: name.to_string(),
        path: path.to_string(),
        last_opened: Some(chrono::Local::now().to_rfc3339()),
        favorite: false,
    });

    save_projects(&projects)?;
    Ok(projects)
}

pub fn remove_project(path: &str) -> Result<Vec<Project>, String> {
    let mut projects = load_projects();
    projects.retain(|p| p.path != path);
    save_projects(&projects)?;
    Ok(projects)
}

pub fn update_last_opened(path: &str) -> Result<(), String> {
    let mut projects = load_projects();
    if let Some(project) = projects.iter_mut().find(|p| p.path == path) {
        project.last_opened = Some(chrono::Local::now().to_rfc3339());
    }
    save_projects(&projects)?;
    Ok(())
}

pub fn toggle_favorite(path: &str) -> Result<Vec<Project>, String> {
    let mut projects = load_projects();
    if let Some(project) = projects.iter_mut().find(|p| p.path == path) {
        project.favorite = !project.favorite;
    }
    save_projects(&projects)?;
    Ok(projects)
}
