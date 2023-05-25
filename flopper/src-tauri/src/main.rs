// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use poker::Range;
use gto::{postflop::PostFlopGame, action::ActionTree};

fn main() {
    tauri::Builder::default()
        .manage(SolverInternal::default())
        .invoke_handler(tauri::generate_handler![
            build_action_tree
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Default)]
pub struct SolverInternal {
    game: PostFlopGame,
    action_tree: ActionTree,
}


#[derive(Default)]
pub struct Solver(Mutex<SolverInternal>);

#[tauri::command]
fn build_action_tree(
    game: &mut Solver,
    starting_pot: i32,
    effective_stack: i32,
    rake: f64,
    rake_cap: f64,
    add_all_in_threshold: f64,
    force_all_in_threshold: f64,
) {
    let config = gto::action::TreeConfig {
        initial_street: gto::Street::Flop,
        starting_pot,
        effective_stack,
        rake,
        rake_cap,
        bet_sizings: gto::action::BetSizings::default(),
        add_all_in_threshold,
        force_all_in_threshold,
    };

    game.0.lock().unwrap().action_tree = ActionTree::new(config).unwrap();
}