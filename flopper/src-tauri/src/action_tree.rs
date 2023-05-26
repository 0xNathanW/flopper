use std::sync::Mutex;
use tauri::State;
use gto::action::ActionTree as ActionTreeInternal;

#[derive(Default)]
pub struct ActionTree(Mutex<ActionTreeInternal>);

// Interfacing with JS requires passing strings.

#[tauri::command]
pub fn build_action_tree(
    game: State<'_, ActionTree>,
    starting_pot: i32,
    effective_stack: i32,
    rake: f64,
    rake_cap: f64,
    add_all_in_threshold: f64,
    force_all_in_threshold: f64,
    // TODO: bet sizings.
) {
    let config = gto::action::TreeConfig {
        initial_street: gto::Street::Flop,
        starting_pot,
        effective_stack,
        rake,
        rake_cap,
        // TODO: parsing of bet sizings from verified string.
        bet_sizings: gto::action::BetSizings::default(),
        add_all_in_threshold,
        force_all_in_threshold,
    };

    *game.0.lock().unwrap() = ActionTreeInternal::new(config).unwrap();
}

#[tauri::command]
pub fn is_terminal_node(game: State<'_, ActionTree>, node_id: usize) -> bool {
    game.0.lock().unwrap().is_terminal_node(node_id)
}

#[tauri::command]
pub fn is_chance_node(game: State<'_, ActionTree>, node_id: usize) -> bool {
    game.0.lock().unwrap().is_chance_node(node_id)
}

#[tauri::command]
p[bu ]