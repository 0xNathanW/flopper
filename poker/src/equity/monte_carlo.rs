use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use crate::{evaluate::eval_7_2p2, prelude::*};
use super::{EquityParams, EquityResults, remove_dead};
use rayon::prelude::*;
use rand::prelude::*;
use signal_hook::flag;

pub fn equity_monte_carlo(params: EquityParams, iterations: Option<usize>) -> Result<EquityResults> {

    let board_cards = params.board.as_vec();
    let (ranges, deck) = remove_dead(params.ranges, &board_cards)?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    flag::register(signal_hook::consts::SIGINT, r)?;

    let results = if params.board.is_river_dealt() {
        monte_carlo_river(ranges, &board_cards, params.lookup, iterations, running)
    } else if params.board.is_turn_dealt() {
        monte_carlo_turn(ranges, &board_cards, deck, params.lookup, iterations, running)
    } else if params.board.is_flop_dealt() {
        monte_carlo_flop(ranges, &board_cards, deck, params.lookup, iterations, running)
    } else {
        monte_carlo_preflop(ranges, deck, params.lookup, iterations, running)
    };

    Ok(results)
}

fn monte_carlo_preflop(
    ranges: Vec<Vec<(Hand, f32)>>,
    deck: Deck,
    lookup: &[i32],
    iterations: Option<usize>,
    running: Arc<AtomicBool>,
) -> EquityResults {
    let num_threads = rayon::current_num_threads();
    let iterations_per_thread = iterations.map(|i| (i + num_threads - 1) / num_threads);
    
    let results = (0..num_threads).into_par_iter().map(|_| {

        let mut rng = rand::thread_rng();
        let mut local_results = EquityResults::new(ranges.len());
        let mut cards = [Card::default(); 7];
        let mut iteration = 0;
        
        while running.load(Ordering::Relaxed) && (iterations_per_thread.is_none() || iteration < iterations_per_thread.unwrap()) {
            
            let mut available_deck = deck.clone();
            available_deck.shuffle(&mut rng);
            let first_five = &available_deck.as_ref()[0..5];
            cards[2..7].copy_from_slice(first_five);
            
            let mut used_cards = 0u64;
            for card in &cards[2..7] {
                used_cards |= 1 << card.0;
            }
            
            monte_carlo_sample_hands(&ranges, &mut rng, &mut local_results, &mut cards, lookup, used_cards);            
            iteration += 1;
        }
        
        local_results
    }).collect::<Vec<EquityResults>>();
    
    // Combine results from all threads
    let mut total = EquityResults::new(ranges.len());
    for result in results {
        total.wins.iter_mut().zip(result.wins.iter()).for_each(|(a, b)| *a += b);
        total.ties.iter_mut().zip(result.ties.iter()).for_each(|(a, b)| *a += b);
        total.total += result.total;
    }
    
    total
}

fn monte_carlo_flop(
ranges: Vec<Vec<(Hand, f32)>>,
    board: &[Card],
    deck: Deck,
    lookup: &[i32],
    iterations: Option<usize>,
    running: Arc<AtomicBool>,
) -> EquityResults {
    let num_threads = rayon::current_num_threads();
    let iterations_per_thread = iterations.map(|i| (i + num_threads - 1) / num_threads);
    
    let results = (0..num_threads).into_par_iter().map(|_| {
        let mut local_deck = deck.clone();
        let mut rng = rand::thread_rng();
        let mut local_results = EquityResults::new(ranges.len());
        let mut cards = [Card::default(); 7];
        cards[2..5].copy_from_slice(board);
        let mut iteration = 0;
        
        while running.load(Ordering::Relaxed) &&  (iterations_per_thread.is_none() || iteration < iterations_per_thread.unwrap()) {
            
            local_deck.shuffle(&mut rng);
            
            cards[5] = local_deck[0];
            cards[6] = local_deck[1];
            
            let mut used_cards = 0u64;
            for card in cards[2..].iter() {
                used_cards |= 1 << card.0;
            }
            
            monte_carlo_sample_hands(&ranges, &mut rng, &mut local_results, &mut cards, lookup, used_cards);
            
            iteration += 1;
        }
        
        local_results
    }).collect::<Vec<EquityResults>>();
    
    // Combine results from all threads
    let mut total = EquityResults::new(ranges.len());
    for result in results {
        total.wins.iter_mut().zip(result.wins.iter()).for_each(|(a, b)| *a += b);
        total.ties.iter_mut().zip(result.ties.iter()).for_each(|(a, b)| *a += b);
        total.total += result.total;
    }
    
    total
}

fn monte_carlo_turn(
    ranges: Vec<Vec<(Hand, f32)>>,
    board: &[Card],
    deck: Deck,
    lookup: &[i32],
    iterations: Option<usize>,
    running: Arc<AtomicBool>,
) -> EquityResults {
    let num_threads = rayon::current_num_threads();
    let iterations_per_thread = iterations.map(|i| (i + num_threads - 1) / num_threads);
    
    let results = (0..num_threads).into_par_iter().map(|_| {
        
        let mut local_deck = deck.clone();
        let mut rng = rand::thread_rng();
        let mut local_results = EquityResults::new(ranges.len());
        let mut cards = [Card::default(); 7];
        cards[2..6].copy_from_slice(board);
        let mut iteration = 0;
        
        while running.load(Ordering::Relaxed) && (iterations_per_thread.is_none() || iteration < iterations_per_thread.unwrap()) {
            
            local_deck.shuffle(&mut rng);
            
            // Set up the river card
            cards[6] = local_deck[0];
            
            // Run simulation for this random river
            let mut used_cards = 0u64;
            for card in cards[2..].iter() {
                used_cards |= 1 << card.0;
            }
            
            monte_carlo_sample_hands(&ranges, &mut rng, &mut local_results, &mut cards, lookup, used_cards);
            
            iteration += 1;
        }
        
        local_results
    }).collect::<Vec<EquityResults>>();
    
    let mut total = EquityResults::new(ranges.len());
    for result in results {
        total.wins.iter_mut().zip(result.wins.iter()).for_each(|(a, b)| *a += b);
        total.ties.iter_mut().zip(result.ties.iter()).for_each(|(a, b)| *a += b);
        total.total += result.total;
    }
    
    total
}

fn monte_carlo_river(
    ranges: Vec<Vec<(Hand, f32)>>,
    board: &[Card],
    lookup: &[i32],
    iterations: Option<usize>,
    running: Arc<AtomicBool>,
) -> EquityResults {
    let num_threads = rayon::current_num_threads();
    let iterations_per_thread = iterations.map(|i| (i + num_threads - 1) / num_threads);
    
    let results = (0..num_threads).into_par_iter().map(|_| {
        let mut rng = rand::thread_rng();
        let mut local_results = EquityResults::new(ranges.len());
        let mut cards = [Card::default(); 7];
        cards[2..7].copy_from_slice(board);
        let mut iteration = 0;
        
        let mut used_cards = 0u64;
        for card in board.iter() {
            used_cards |= 1 << card.0;
        }
        
        while running.load(Ordering::Relaxed) && (iterations_per_thread.is_none() || iteration < iterations_per_thread.unwrap()) {
            monte_carlo_sample_hands(&ranges, &mut rng, &mut local_results, &mut cards, lookup, used_cards);
            iteration += 1;
        }
        
        local_results
    }).collect::<Vec<EquityResults>>();
    
    // Combine results from all threads
    let mut total = EquityResults::new(ranges.len());
    for result in results {
        total.wins.iter_mut().zip(result.wins.iter()).for_each(|(a, b)| *a += b);
        total.ties.iter_mut().zip(result.ties.iter()).for_each(|(a, b)| *a += b);
        total.total += result.total;
    }
    
    total
}

fn monte_carlo_sample_hands(
    ranges: &Vec<Vec<(Hand, f32)>>,
    rng: &mut impl Rng,
    results: &mut EquityResults,
    board: &mut [Card; 7],
    lookup_table: &[i32],
    used_cards: u64,
) {
    
    let mut hands = Vec::with_capacity(ranges.len());
    let mut current_used_cards = used_cards;
    
    for range in ranges {
        let mut valid_hands = Vec::new();
        
        for (hand, _) in range {
            let hand_mask = 1 << hand.0.0 | 1 << hand.1.0;
            if current_used_cards & hand_mask == 0 {
                valid_hands.push(*hand);
            }
        }
        
        if valid_hands.is_empty() {
            return;
        }
        
        let idx = rng.gen_range(0..valid_hands.len());
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
