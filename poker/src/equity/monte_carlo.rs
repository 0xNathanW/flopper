use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use crate::{evaluate::eval_7_2p2, prelude::*};
use super::{remove_dead, EquityParams, EquityResults, ProgressReporter};
use rayon::prelude::*;
use signal_hook::flag;

pub fn equity_monte_carlo(equity_params: EquityParams, iterations: Option<u64>) -> Result<EquityResults> {

    let board = equity_params.board.as_vec();
    let (ranges, deck) = remove_dead(equity_params.ranges, &board)?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    flag::register(signal_hook::consts::SIGINT, r)?;

    let params = MonteCarloParams {
        ranges,
        deck,
        board,
        lookup: equity_params.lookup,
        iterations,
        reporter: equity_params.reporter,
        running,
    };

    let results = if equity_params.board.is_river_dealt() {
        monte_carlo_river(params)
    } else if equity_params.board.is_turn_dealt() {
        monte_carlo_turn(params)
    } else if equity_params.board.is_flop_dealt() {
        monte_carlo_flop(params)
    } else {
        monte_carlo_preflop(params)
    };

    Ok(results)
}

struct MonteCarloParams<'a> {
    ranges:     Vec<Vec<Hand>>,
    deck:       Deck,
    board:      Vec<Card>,
    lookup:     &'a [i32],
    iterations: Option<u64>,
    reporter:   Option<&'a dyn ProgressReporter>,
    running:    Arc<AtomicBool>,
}

fn monte_carlo_preflop(params: MonteCarloParams) -> EquityResults {

    let num_threads = rayon::current_num_threads() as u64;
    let iterations_per_thread = params.iterations.map(|i| (i + num_threads - 1) / num_threads);
    
    let results = (0..num_threads).into_par_iter().map(|_| {

        let mut local_results = EquityResults::new(params.ranges.len());
        let mut cards = [Card::default(); 7];
        let mut iteration = 0;
        
        while params.running.load(Ordering::Relaxed) && (iterations_per_thread.is_none() || iteration < iterations_per_thread.unwrap()) {
            
            let mut available_deck = params.deck.clone();
            available_deck.shuffle();
            let first_five = &available_deck.as_ref()[0..5];
            cards[2..7].copy_from_slice(first_five);
            
            let mut used_cards = 0u64;
            for card in &cards[2..7] {
                used_cards |= 1 << card.0;
            }
            
            monte_carlo_sample_hands(&params.ranges, &mut local_results, &mut cards, params.lookup, used_cards);            
            
            iteration += 1;
            if let Some(reporter) = params.reporter {
                reporter.board_complete();
            }
        }
        local_results
    }).collect::<Vec<EquityResults>>();

    EquityResults::combine(results)
}

fn monte_carlo_flop(params: MonteCarloParams) -> EquityResults {
    let num_threads = rayon::current_num_threads() as u64;
    let iterations_per_thread = params.iterations.map(|i| (i + num_threads - 1) / num_threads);
    
    let results = (0..num_threads).into_par_iter().map(|_| {
        let mut local_deck = params.deck.clone();
        let mut local_results = EquityResults::new(params.ranges.len());
        let mut cards = [Card::default(); 7];
        cards[2..5].copy_from_slice(&params.board);
        let mut iteration = 0;
        
        while params.running.load(Ordering::Relaxed) && (iterations_per_thread.is_none() || iteration < iterations_per_thread.unwrap()) {
            
            local_deck.shuffle();
            
            cards[5] = local_deck[0];
            cards[6] = local_deck[1];
            
            let mut used_cards = 0u64;
            for card in cards[2..].iter() {
                used_cards |= 1 << card.0;
            }
            
            monte_carlo_sample_hands(&params.ranges, &mut local_results, &mut cards, params.lookup, used_cards);
            
            iteration += 1;
            if let Some(reporter) = params.reporter {
                reporter.board_complete();
            }
        }
        
        local_results
    }).collect::<Vec<EquityResults>>();
    
    EquityResults::combine(results)
}

fn monte_carlo_turn(params: MonteCarloParams) -> EquityResults {
    let num_threads = rayon::current_num_threads() as u64;
    let iterations_per_thread = params.iterations.map(|i| (i + num_threads - 1) / num_threads);
    
    let results = (0..num_threads).into_par_iter().map(|_| {
        
        let mut local_deck = params.deck.clone();
        let mut local_results = EquityResults::new(params.ranges.len());
        let mut cards = [Card::default(); 7];
        cards[2..6].copy_from_slice(&params.board);
        let mut iteration = 0;
        
        while params.running.load(Ordering::Relaxed) && (iterations_per_thread.is_none() || iteration < iterations_per_thread.unwrap()) {
            
            local_deck.shuffle();
            cards[6] = local_deck[0];
            
            let mut used_cards = 0u64;
            for card in cards[2..].iter() {
                used_cards |= 1 << card.0;
            }
            
            monte_carlo_sample_hands(&params.ranges, &mut local_results, &mut cards, params.lookup, used_cards);            
            iteration += 1;
            if let Some(reporter) = params.reporter {
                reporter.board_complete();
            }
        }
        
        local_results
    }).collect::<Vec<EquityResults>>();
    
    EquityResults::combine(results)
}

fn monte_carlo_river(params: MonteCarloParams) -> EquityResults {
    let num_threads = rayon::current_num_threads() as u64;
    let iterations_per_thread = params.iterations.map(|i | (i + num_threads - 1) / num_threads);

    let results = (0..num_threads).into_par_iter().map(|_| {
        let mut local_results = EquityResults::new(params.ranges.len());
        let mut cards = [Card::default(); 7];
        cards[2..7].copy_from_slice(&params.board);
        let mut iteration = 0;
        
        let mut used_cards = 0u64;
        for card in &params.board {
            used_cards |= 1 << card.0;
        }
        
        while params.running.load(Ordering::Relaxed) && (iterations_per_thread.is_none() || iteration < iterations_per_thread.unwrap()) {
            monte_carlo_sample_hands(&params.ranges, &mut local_results, &mut cards, params.lookup, used_cards);
            iteration += 1;
            if let Some(reporter) = params.reporter {
                reporter.board_complete();
            }
        }
        
        local_results
    }).collect::<Vec<EquityResults>>();
    
    EquityResults::combine(results)
}

fn monte_carlo_sample_hands(
    ranges: &Vec<Vec<Hand>>,
    results: &mut EquityResults,
    board: &mut [Card; 7],
    lookup_table: &[i32],
    used_cards: u64,
) {
    
    let mut hands = Vec::with_capacity(ranges.len());
    let mut current_used_cards = used_cards;
    
    for range in ranges {
        let mut valid_hands = Vec::new();
        
        for hand in range {
            let hand_mask = 1 << hand.0.0 | 1 << hand.1.0;
            if current_used_cards & hand_mask == 0 {
                valid_hands.push(hand);
            }
        }
        
        if valid_hands.is_empty() {
            return;
        }
        
        let idx = fastrand::usize(0..valid_hands.len());
        let selected_hand = valid_hands[idx];
        hands.push(selected_hand);
        current_used_cards |= 1 << selected_hand.0.0 | 1 << selected_hand.1.0;
    }
    
    if hands.len() == ranges.len() {
        let mut best_idxs = [0; 8];
        let mut best_idxs_count = 0;
        let mut best_rank = 0;
        
        for (i, &hand) in hands.iter().enumerate() {
            board[0] = hand.0;
            board[1] = hand.1;
            
            let rank = eval_7_2p2(board, lookup_table);
            if rank > best_rank {
                best_idxs[0] = i;
                best_idxs_count = 1;
                best_rank = rank;
            } else if rank == best_rank {
                best_idxs[best_idxs_count] = i;
                best_idxs_count += 1;
            }
        }
        
        if best_idxs_count == 1 {
            results.wins[best_idxs[0]] += 1.0;
        } else {
            let tie_value = 1.0 / best_idxs_count as f64;
            for idx in 0..best_idxs_count {
                results.ties[best_idxs[idx]] += tie_value;
            }
        }
        
        results.total += 1.0;
    }
}