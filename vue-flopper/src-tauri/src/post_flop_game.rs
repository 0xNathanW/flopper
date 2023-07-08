use std::sync::Mutex;
use tauri::State;
use serde::Serialize;
use gto::{
    postflop::PostFlopGame as PostFlopGameInternal, 
    action::{BetSizingsStreet, ActionTree}, 
    exploitability::compute_exploitability, 
    solve::{solve_step as solve_step_internal, finalise}
};
use poker::{Range, Board, Card};
use super::game_utill::*;

#[derive(Debug, Default)]
pub struct PostFlopGame(Mutex<PostFlopGameInternal>);

#[derive(Serialize)]
pub struct ResultsGame {
    current_player: String,
    num_actions: usize,
    empty: i32,
    eqr_base: [i32; 2],
    weights: [Vec<f64>; 2],
    normaliser: [Vec<f64>; 2],
    equity: [Vec<f64>; 2],
    ev: [Vec<f64>; 2],
    eqr: [Vec<f64>; 2],
    strategy: Vec<f64>,
    action_ev: Vec<f64>,
}

#[derive(Serialize)]
pub struct ChanceReportGame {
    status: Vec<i32>,
    combos: [Vec<f64>; 2],
    equity: [Vec<f64>; 2],
    ev: [Vec<f64>; 2],
    eqr: [Vec<f64>; 2],
    strategy: Vec<f64>,
}

#[tauri::command(async)]
pub fn build_game_tree(
    game: State<'_, PostFlopGame>,
    board: Vec<u8>,
    range_oop: Vec<i32>,
    range_ip: Vec<i32>,

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
    
) -> Option<String> {
    
    let range_oop = Range::new_from_grid(
        range_oop
        .iter()
        .map(|x| *x as f32 / 100.0)
        .collect::<Vec<f32>>()
        .try_into()
        .unwrap()
    );
    let range_ip = Range::new_from_grid(
        range_ip
        .iter()
        .map(|x| *x as f32 / 100.0)
        .collect::<Vec<f32>>()
        .try_into()
        .unwrap()
    );
    
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
    let action_tree = ActionTree::new(config).unwrap();
    let board = Board::from_vec(board.iter().map(|x| Card::from(*x)).collect::<Vec<Card>>()).unwrap();

    let post_flop_game = PostFlopGameInternal::new([range_oop, range_ip], board, action_tree).map_err(|e| e.to_string());
    if let Err(e) = post_flop_game {
        return Some(e);
    }
    *game.0.lock().unwrap() = post_flop_game.unwrap();
    None
}

#[tauri::command]
pub fn memory_usage_game(game: State<'_, PostFlopGame>) -> (u64, u64) {
    game.0.lock().unwrap().memory_usage()
}

#[tauri::command]
pub fn allocate_memory_game(game: State<'_, PostFlopGame>, compression: bool) {
    game.0.lock().unwrap().allocate_memory(compression)
}

#[tauri::command(async)]
pub fn exploitability_game(game: State<'_, PostFlopGame>, pool: State<'_, Mutex<rayon::ThreadPool>>) -> f32 {
    let g = game.0.lock().unwrap();
    let p = pool.lock().unwrap();
    p.install(|| compute_exploitability(&*g))
}

#[tauri::command(async)]
pub fn solve_step_game(
    game: State<'_, PostFlopGame>, 
    pool: State<'_, Mutex<rayon::ThreadPool>>,
    i: u32,
) {
    let g = game.0.lock().unwrap();
    let p = pool.lock().unwrap();
    p.install(|| solve_step_internal(&*g, i));
}

#[tauri::command(async)]
pub fn finalise_game(game: State<'_, PostFlopGame>, pool: State<'_, Mutex<rayon::ThreadPool>>) {
    let p = pool.lock().unwrap();
    p.install(|| finalise(&mut *game.0.lock().unwrap()));
}

#[tauri::command]
pub fn hands_game(game: State<'_, PostFlopGame>) -> [Vec<u16>; 2] {
    let g = game.0.lock().unwrap();
    let map_cards = |player: usize| {
        g.hands(player)
        .iter()
        .map(|&x| (x.0.0 as u16) | (x.1.0 as u16) << 8)
        .collect::<Vec<u16>>() 
    };
    [map_cards(0), map_cards(1)]
}

#[tauri::command]
pub fn apply_history_game(game: State<'_, PostFlopGame>, history: Vec<usize>) {
    let mut g = game.0.lock().unwrap();
    g.apply_history(&history);
}

#[tauri::command]
pub fn total_bet_amount_game(game: State<'_, PostFlopGame>, append: Vec<isize>) -> [i32; 2] {
    
    let mut g = game.0.lock().unwrap();
    if append.is_empty() {
        return g.total_bet_amount();
    }

    let history = g.history().to_vec();
    for &action in &append {
        g.play(if action == -1 { usize::MAX } else { action as usize });
    }

    let out = g.total_bet_amount();
    g.apply_history(&history);
    out
}

#[tauri::command]
pub fn possible_cards_game(game: State<'_, PostFlopGame>) -> u64 {
    game.0.lock().unwrap().possible_cards()
}

#[tauri::command]
pub fn results_game(game: State<'_, PostFlopGame>) -> ResultsGame {
    
    let mut g = game.0.lock().unwrap();
    let total_bet_amount = g.total_bet_amount();
    let pot_base = g.tree_config().starting_pot + total_bet_amount.iter().min().unwrap();
    let eqr_base = [pot_base + total_bet_amount[0], pot_base + total_bet_amount[1]];
    
    let truncate = |&w: &f32| if w < 0.0005 { 0.0 } else { round(w as f64) };
    let weights = [
        g.weights(0).iter().map(truncate).collect::<Vec<_>>(),
        g.weights(1).iter().map(truncate).collect::<Vec<_>>(),
    ];

    let empty = |player: usize| weights[player].iter().all(|&w| w == 0.0);
    let empty_flag = empty(0) as i32 + 2 * empty(1) as i32;

    let mut normaliser = [vec![], vec![]];
    let mut equity = [vec![], vec![]];
    let mut ev = [vec![], vec![]];
    let mut eqr = [vec![], vec![]];

    if empty_flag > 0 {
        normaliser[0].extend(weights[0].iter());
        normaliser[1].extend(weights[1].iter());
    
    } else {

        g.cache_normalised_weights();
        normaliser[0].extend(round_iter(g.normalised_weights(0).iter()));
        normaliser[1].extend(round_iter(g.normalised_weights(1).iter()));

        let raw_equity = [g.equity(0), g.equity(1)];
        equity[0].extend(round_iter(raw_equity[0].iter()));
        equity[1].extend(round_iter(raw_equity[1].iter()));

        let raw_ev = [g.expected_values(0), g.expected_values(1)];
        ev[0].extend(round_iter(raw_ev[0].iter()));
        ev[1].extend(round_iter(raw_ev[1].iter()));
        
        for player in 0..2 {
            let pot = eqr_base[player] as f64;

            for (&eq, &ev) in raw_equity[player].iter().zip(raw_ev[player].iter()) {
                let (eq, ev) = (eq as f64, ev as f64);
                if eq < 5e-7 {
                    eqr[player].push(ev / 0.0);
                } else {
                    eqr[player].push(round(ev / (eq * pot)));
                }
            }
        }
    }

    let mut strategy = vec![];
    let mut action_ev = vec![];

    if !g.is_terminal_node() && !g.is_chance_node() {
        strategy.extend(round_iter(g.strategy().iter()));
        if empty_flag == 0 {
            action_ev.extend(round_iter(g.expected_values_detail(g.current_player()).iter()));
        }
    }

    ResultsGame {
        current_player: current_player(&g),
        num_actions: num_actions(&g),
        empty: empty_flag,
        eqr_base,
        weights,
        normaliser,
        equity,
        ev,
        eqr,
        strategy,
        action_ev,
    }
}

#[tauri::command]
pub fn chance_report_game(game: State<'_, PostFlopGame>, append: Vec<isize>, num_actions: usize) -> ChanceReportGame {
    
    let mut g = game.0.lock().unwrap();
    let history = g.history().to_vec();
    let mut status = vec![0; 52];
    let mut combos = [vec![0.0; 52], vec![0.0; 52]];
    let mut equity = [vec![0.0; 52], vec![0.0; 52]];
    let mut ev = [vec![0.0; 52], vec![0.0; 52]];
    let mut eqr = [vec![0.0; 52], vec![0.0; 52]];
    let mut strategy = vec![0.0; num_actions * 52];
    let possible_cards = g.possible_cards();

    for chance in 0..52 {
        if possible_cards & (1 << chance) == 0 {
            continue;
        }

        g.play(chance);
        for &action in &append[1..] {
            g.play(if action == -1 { usize::MAX } else { action as usize });
        }

        let truncate = |&w: &f32| if w < 0.0005 { 0.0 } else { w };
        let weights = [
            g.weights(0).iter().map(truncate).collect::<Vec<_>>(),
            g.weights(1).iter().map(truncate).collect::<Vec<_>>(),
        ];

        combos[0][chance] = round(weights[0].iter().fold(0.0, |acc, &w| acc + w as f64));
        combos[1][chance] = round(weights[1].iter().fold(0.0, |acc, &w| acc + w as f64));

        let empty = |player: usize| weights[player].iter().all(|&w| w == 0.0);
        let empty_flag = [empty(0), empty(1)];

        g.cache_normalised_weights();
        let normaliser = [g.normalised_weights(0), g.normalised_weights(1)];

        if !g.is_terminal_node() {
            let current_player = g.current_player();
            
            if !empty_flag[current_player] {
                let temp_strategy = g.strategy();
                let num_hands = g.hands(current_player).len();
            
                let ws = if empty_flag[current_player ^ 1] {
                    &weights[current_player]
                } else {
                    normaliser[current_player]
                };

                for action in 0..num_actions {
                    let slice = &temp_strategy[action * num_hands..(action + 1) * num_hands];
                    let strategy_summary = weighted_average(slice, ws);
                    strategy[action * 52 + chance] = round(strategy_summary);
                }    
            }
        }

        if empty_flag[0] || empty_flag[1] {
            status[chance] = 1;
            g.apply_history(&history);
            continue;
        }

        status[chance] = 2;
        let total_bet_amount = g.total_bet_amount();
        let pot_base = g.tree_config().starting_pot + total_bet_amount.iter().min().unwrap();

        for player in 0..2 {
            let pot = (pot_base + total_bet_amount[player]) as f32;
            let temp_equity = weighted_average(&g.weights(player), normaliser[player]);
            let temp_ev = weighted_average(&g.expected_values(player), normaliser[player]);
            equity[player][chance] = round(temp_equity);
            ev[player][chance] = round(temp_ev);
            eqr[player][chance] = round(temp_ev / (pot as f64 * temp_equity));
        }

        g.apply_history(&history);
    }

    ChanceReportGame {
        status,
        combos,
        equity,
        ev,
        eqr,
        strategy,
    }
}

#[tauri::command]
pub fn actions_after_game(game: State<'_, PostFlopGame>, append: Vec<isize>) -> Vec<String> {
    
    let mut g = game.0.lock().unwrap();
    if append.is_empty() {
        return actions(&g);
    }

    let history = g.history().to_vec();
    for &action in &append {
        g.play(if action == -1 { usize::MAX } else { action as usize });
    }

    let out = actions(&g);
    g.apply_history(&history);
    out
}