use tauri::{State, AppHandle};
use crate::db::models::Skill;
use crate::scanner::Scanner;
use crate::db::Db;
use std::collections::HashMap;

// A simple state struct to hold things if needed
pub struct AppState {
    pub db: Option<Db>,
    pub home_dir: String,
    pub config_dir: String,
}

#[tauri::command]
pub fn scan_all(state: State<'_, AppState>) -> Result<Vec<Skill>, String> {
    let custom_paths = vec![]; // We would fetch this from db settings
    let skills = Scanner::scan_all(&state.home_dir, &state.config_dir, &custom_paths);
    println!("Found skills: {:?}", skills); Ok(skills)
}

#[tauri::command]
pub fn get_skills() -> Result<Vec<Skill>, String> {
    // Should query from SQLite Db, returning empty for now
    Ok(vec![])
}

#[tauri::command]
pub fn get_skill(id: String) -> Result<Option<Skill>, String> {
    // Should query from SQLite Db
    Ok(None)
}

#[tauri::command]
pub fn save_skill(id: String, content: String, frontmatter: HashMap<String, String>) -> Result<Skill, String> {
    Err("Not implemented".into())
}

#[tauri::command]
pub fn create_skill(tool: String, name: String, content: String) -> Result<Skill, String> {
    Err("Not implemented".into())
}

#[tauri::command]
pub fn delete_skill(id: String) -> Result<(), String> {
    Err("Not implemented".into())
}

#[tauri::command]
pub fn get_skill_content(id: String) -> Result<String, String> {
    Err("Not implemented".into())
}

#[tauri::command]
pub fn start_watch(app: AppHandle) -> Result<(), String> {
    // In a real app we'd store the watcher in state to keep it alive
    // let watcher = crate::watcher::start_watcher(vec![], app);
    Ok(())
}

#[tauri::command]
pub fn stop_watch() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_collections() -> Result<Vec<crate::db::models::Collection>, String> {
    Ok(vec![])
}
