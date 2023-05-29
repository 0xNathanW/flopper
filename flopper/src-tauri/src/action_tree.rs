use std::sync::Mutex;
use tauri::State;
use gto::action::ActionTree as ActionTreeInternal;
use gto::action::BetSizingsStreet;

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
) {

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
}

// #[tauri::command]
// pub fn is_terminal_node(game: State<'_, ActionTree>, node_id: usize) -> bool {
//     game.0.lock().unwrap().is_terminal_node(node_id)
// }

// #[tauri::command]
// pub fn is_chance_node(game: State<'_, ActionTree>, node_id: usize) -> bool {
//     game.0.lock().unwrap().is_chance_node(node_id)
// }
