// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// main.rs
use tauri::Builder;

mod commands;
mod config;
mod obsidian;
mod utils;
mod vault;

fn main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_total_count,
            commands::set_config,
            commands::get_config,
            commands::load_gitignore,
            commands::open_folder_dialog,
            commands::load_folder,
            commands::get_item_count,
            commands::init_vault,
            commands::open_in_obsidian
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
