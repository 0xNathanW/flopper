// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod action_tree;
use action_tree::*;

fn main() {
    tauri::Builder::default()
        .manage(ActionTree::default())
        .invoke_handler(tauri::generate_handler![
            build_action_tree,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



