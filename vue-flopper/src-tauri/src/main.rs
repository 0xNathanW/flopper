// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod action_tree;
use action_tree::*;

fn main() {
    tauri::Builder::default()
        .manage(ActionTree::default())
        .invoke_handler(tauri::generate_handler![
            build_action_tree,
            num_nodes,
            to_root,
            get_actions,
            get_added_lines,
            get_removed_lines,
            get_invalid_terminals,
            play,
            remove_current_node,
            apply_history,
            add_bet_action,
            total_bet_amount,
            is_terminal_node,
            is_chance_node,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



