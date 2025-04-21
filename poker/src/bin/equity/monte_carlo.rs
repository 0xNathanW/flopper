use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use std::time::Instant;
use poker::{equity::EquityResults, evaluate::eval_7_2p2, remove_dead, Board, Card, Deck, Hand, Range, error::Result};
use rayon::prelude::*;
use rand::prelude::*;
use signal_hook::flag;

// The number of matchups to simulate per board.
const MATCHUPS_PER_BOARD: usize = 10;

pub fn equity_monte_carlo(ranges: Vec<Range>, board: Board, lookup: &[i32], iterations: Option<usize>) -> Result<EquityResults> {

    let board_cards = board.as_vec();
    let (ranges, deck) = remove_dead(ranges, &board_cards)?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    flag::register(signal_hook::consts::SIGINT, r)?;

    let start_time = Instant::now();
    let results = if board.is_river_dealt() {
        monte_carlo_river(ranges, &board_cards, lookup)
    } else if board.is_turn_dealt() {
        monte_carlo_turn(ranges, &board_cards, deck, lookup, iterations, running)
    } else if board.is_flop_dealt() {
        monte_carlo_flop(ranges, &board_cards, deck, lookup, iterations, running)
    } else {
        monte_carlo_preflop(ranges, deck, lookup, iterations, running)
    };

    let duration = start_time.elapsed();
    println!("Simulation completed in {:.2?}", duration);
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

fn monte_carlo_river(ranges: Vec<Vec<(Hand, f32)>>, board: &[Card], lookup: &[i32]) -> EquityResults {
    let mut results = EquityResults::new(ranges.len());
    let mut cards = [Card::default(); 7];
    cards[2..7].copy_from_slice(board);
    
    let mut used_cards = 0u64;
    for card in board.iter() {
        used_cards |= 1 << card.0;
    }
    
    let mut rng = rand::thread_rng();
    monte_carlo_sample_hands(&ranges, &mut rng, &mut results, &mut cards, lookup, used_cards);
    
    results
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

#[cfg(test)]
mod tests {
    use poker::evaluate::load_lookup_table;
    use crate::print_output;

    use super::*;
    
    const LOOKUP_PATH: &str = "./data/lookup_table.bin";

    #[test]
    fn test_monte_carlo_preflop_heads_up() {
        let range_1 = Range::from_str("88+").unwrap();
        let range_2 = Range::from_str("55+").unwrap();
        let ranges = vec![range_1, range_2];

        let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
        // Run 10,000 iterations for testing
        let results = equity_monte_carlo(ranges, Board::default(), &lookup, Some(10_000_000)).unwrap();
        
        // Monte Carlo should get approximately the same results as enumeration
        // Results from enumerate test: 59%, 39%
        let win_pcts = results.win_pct();
        // Use a wide margin because of randomness
        assert!((win_pcts[0] - 59.0).abs() < 5.0, "Expected win percentage for player 1 to be close to 59%, got {}%", win_pcts[0]);
        assert!((win_pcts[1] - 39.0).abs() < 5.0, "Expected win percentage for player 2 to be close to 39%, got {}%", win_pcts[1]);
        print_output(vec!["88+".to_string(), "55+".to_string()], results);
    }
}
