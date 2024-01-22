// commands.rs

#[tauri::command]
pub fn open_folder_dialog(app: tauri::AppHandle) {
    crate::utils::open_folder_dialog(app);
}

#[tauri::command]
pub fn load_gitignore(path: String) -> Vec<String> {
    // Call function from utils.rs
    crate::utils::load_gitignore(path)
}

#[tauri::command]
pub fn open_in_obsidian(path: String) {
    // Call function from obsidian.rs
    crate::obsidian::open_obsidian(path);
}

#[tauri::command]
pub fn set_config(key: String, value: String) {
    crate::config::set_config(key, value);
}

#[tauri::command]
pub fn get_config(key: String) -> Option<String> {
    crate::config::get_config(key)
}

#[tauri::command]
pub fn init_vault(path: String) {
    crate::vault::init_vault(path);
}

#[tauri::command]
pub fn load_folder(path: String) -> Vec<String> {
    crate::utils::load_folder(path)
}

#[tauri::command]
pub fn get_item_count(path: String) -> i32 {
    crate::utils::get_item_count(path)
}

#[tauri::command]
pub fn get_total_count(path: String, filter: Vec<String>) -> i32 {
    crate::utils::get_total_count(path, filter)
}
