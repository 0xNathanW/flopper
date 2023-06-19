use std::sync::Mutex;
use gto::action::Action;
use tauri::State;
use gto::action::ActionTree as ActionTreeInternal;
use gto::action::BetSizingsStreet;

mod action_util;
pub use action_util::*;

#[derive(Default)]
pub struct ActionTree(Mutex<ActionTreeInternal>);

#[tauri::command]
pub fn build_action_tree(
    game: State<'_, ActionTree>,
    board: Vec<u8>,

    starting_pot: i32,
    effective_stack: i32,
    rake: f64,
    rake_cap: f64,
    add_all_in_threshold: f64,
    force_all_in_threshold: f64,
    
    oop_bets_flop: &str,
    oop_raises_flop: &str,
    oop_bets_turn: &str,
    oop_raises_turn: &str,
    oop_bets_river: &str,
    oop_raises_river: &str,

    ip_bets_flop: &str,
    ip_raises_flop: &str,
    ip_bets_turn: &str,
    ip_raises_turn: &str,
    ip_bets_river: &str,
    ip_raises_river: &str,
) -> bool {

    let initial_street = match board.len() {
        3 => gto::Street::Flop,
        4 => gto::Street::Turn,
        5 => gto::Street::River,
        _ => unreachable!(),
    };

    let oop_bets_flop = BetSizingsStreet::from_str(oop_bets_flop, oop_raises_flop).unwrap();
    let oop_bets_turn = BetSizingsStreet::from_str(oop_bets_turn, oop_raises_turn).unwrap();
    let oop_bets_river = BetSizingsStreet::from_str(oop_bets_river, oop_raises_river).unwrap();

    let ip_bets_flop = BetSizingsStreet::from_str(ip_bets_flop, ip_raises_flop).unwrap();
    let ip_bets_turn = BetSizingsStreet::from_str(ip_bets_turn, ip_raises_turn).unwrap();
    let ip_bets_river = BetSizingsStreet::from_str(ip_bets_river, ip_raises_river).unwrap();

    let bet_sizings = gto::action::BetSizings {
        flop: [oop_bets_flop, ip_bets_flop],
        turn: [oop_bets_turn, ip_bets_turn],
        river: [oop_bets_river, ip_bets_river],
    };

    let config = gto::action::TreeConfig {
        initial_street,
        starting_pot,
        effective_stack,
        rake,
        rake_cap,
        bet_sizings,
        add_all_in_threshold,
        force_all_in_threshold,
    };

    *game.0.lock().unwrap() = ActionTreeInternal::new(config).unwrap();
    true
}

#[tauri::command]
pub fn num_nodes(game: State<'_, ActionTree>) -> usize {
    game.0.lock().unwrap().num_nodes()
}

#[tauri::command]
pub fn to_root(game: State<'_, ActionTree>) {
    game.0.lock().unwrap().history.clear();
}

#[tauri::command]
pub fn get_actions(game: State<'_, ActionTree>) -> Vec<String> {
    game.0.lock().unwrap()
        .available_actions()
        .iter()
        .cloned()
        .map(action_to_str)
        .collect::<Vec<_>>()
}

#[tauri::command]
pub fn get_added_lines(game: State<'_, ActionTree>) -> String {
    game.0.lock().unwrap()
        .added_lines
        .iter()
        .map(|l| encode_line(&l))
        .collect::<Vec<_>>()
        .join(",")
}

#[tauri::command]
pub fn get_removed_lines(game: State<'_, ActionTree>) -> String {
    game.0.lock().unwrap()
        .removed_lines
        .iter()
        .map(|l| encode_line(&l))
        .collect::<Vec<_>>()
        .join(",")
}

#[tauri::command]
pub fn get_invalid_terminals(game: State<'_, ActionTree>) -> String {
    game.0.lock().unwrap()
        .invalid_terminals()
        .iter()
        .map(|l| encode_line(&l))
        .collect::<Vec<_>>()
        .join(",")
}

#[tauri::command]
pub fn play(game: State<'_, ActionTree>, action: &str) -> i32 {
    let mut binding = game.0.lock().unwrap();
    let action = decode_action(&action);
    let available_actions = binding.available_actions();
    if let Some(idx) = available_actions.iter().position(|&a| a == action) {
        binding.play(action).unwrap();
        idx as i32
    } else {
        -1
    }
}

#[tauri::command]
pub fn remove_current_node(game: State<'_, ActionTree>) {
    game.0.lock().unwrap().remove_current_node().unwrap();
}

#[tauri::command]
pub fn apply_history(game: State<'_, ActionTree>, history: Vec<String>) {
    let line = history.iter().map(|s| decode_action(s)).collect::<Vec<_>>();
    game.0.lock().unwrap().apply_history(&line).expect("Invalid history");
}

#[tauri::command]
pub fn add_bet_action(game: State<'_, ActionTree>, amount: i32, raise: bool) {
    let action: Action = if raise {
        Action::Raise(amount)
    } else {
        Action::Bet(amount)
    };
    game.0.lock().unwrap().add_action(action).unwrap();
}

#[tauri::command]
pub fn total_bet_amount(game: State<'_, ActionTree>) -> [i32; 2] {
    game.0.lock().unwrap().total_bet_amount()
}

#[tauri::command]
pub fn is_terminal_node(game: State<'_, ActionTree>) -> bool {
    game.0.lock().unwrap().is_terminal_node()
}

#[tauri::command]
pub fn is_chance_node(game: State<'_, ActionTree>) -> bool {
    game.0.lock().unwrap().is_chance_node()
}
