// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod action_tree;
mod action_util;
mod post_flop_game;
mod game_utill;

use std::{sync::Mutex, env};
use post_flop_game::*;
use action_tree::*;
use rayon::{ThreadPoolBuilder, ThreadPool};
use sysinfo::{System, SystemExt};

fn main() {

    tauri::Builder::default()
        .manage(ActionTree::default())
        .manage(PostFlopGame::default())
        .manage(Mutex::new(ThreadPoolBuilder::new().build().unwrap()))
        .invoke_handler(tauri::generate_handler![

            os_name,
            memory,
            set_num_threads,
            
            // action tree
            build_action_tree,
            num_nodes_action_tree,
            to_root_action_tree,
            get_actions_action_tree,
            get_added_lines_action_tree,
            get_removed_lines_action_tree,
            get_invalid_terminals_action_tree,
            play_action_tree,
            remove_current_node_action_tree,
            apply_history_action_tree,
            add_bet_action_action_tree,
            total_bet_amount_action_tree,
            is_terminal_node_action_tree,
            is_chance_node_action_tree,

            // post flop game
            build_game_tree,
            exploitability_game,
            memory_usage_game,
            allocate_memory_game,
            solve_step_game,
            finalise_game,
            hands_game,
            apply_history_game,
            total_bet_amount_game,
            possible_cards_game,
            results_game,
            actions_after_game,
            chance_report_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn os_name() -> String {
    "windows".to_string()
}

#[cfg(target_os = "linux")]
#[tauri::command]
fn os_name() -> String {
    "linux".to_string()
}

#[cfg(target_os = "macos")]
#[tauri::command]
fn os_name() -> String {
    "macos".to_string()
}

#[tauri::command]
fn memory() -> (u64, u64) {
    let mut sys = System::new_all();
    sys.refresh_memory();
    (sys.available_memory(), sys.total_memory())
}

#[tauri::command]
fn set_num_threads(pool: tauri::State<Mutex<ThreadPool>>, num: usize) {
    *pool.lock().unwrap() = ThreadPoolBuilder::new().num_threads(num).build().unwrap();
}