use crate::{evaluate::eval_7_2p2, prelude::*};
use super::{EquityParams, EquityResults, remove_dead};
use rayon::prelude::*;

pub fn equity_enumerate(params: EquityParams) -> Result<EquityResults> {

    let board_cards = params.board.as_vec();
    let (ranges, deck) = remove_dead(params.ranges, &board_cards)?;

    let results = if params.board.is_river_dealt() {
        enumerate_river(ranges, &board_cards, params.lookup)

    } else if params.board.is_turn_dealt() {
        enumerate_turn(ranges, &board_cards, deck, params.lookup)
    
    } else if params.board.is_flop_dealt() {
        enumerate_flop(ranges, &board_cards, deck, params.lookup)
    
    } else {
        enumerate_preflop(ranges, deck, params.lookup)
    };

    Ok(results)
}

fn enumerate_preflop(ranges: Vec<Vec<(Hand, f32)>>, deck: Deck, lookup: &[i32]) -> EquityResults {
    let results = (0..deck.len()).into_par_iter().map(|a| {
        let mut cards = [Card::default(); 7];
        cards[2] = deck[a];
        let mut local_results = EquityResults::new(ranges.len());

        for b in (a + 1)..deck.len() {
            cards[3] = deck[b];
            for c in (b + 1)..deck.len() {
                cards[4] = deck[c];
                for d in (c + 1)..deck.len() {
                    cards[5] = deck[d];
                    for e in (d + 1)..deck.len() {
                        cards[6] = deck[e];
                        enumerate_board(&ranges, &mut local_results, &mut cards, &lookup);
                    }
                }
            }
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

fn enumerate_flop(ranges: Vec<Vec<(Hand, f32)>>, board: &[Card], deck: Deck, lookup: &[i32]) -> EquityResults {

    let results = (0..deck.len()).into_par_iter().map(|a| {

        let mut cards = [Card::default(); 7];
        cards[5] = deck[a];
        cards[2..5].copy_from_slice(board);
        let mut local_results = EquityResults::new(ranges.len());

        for b in (a + 1)..deck.len() {
            cards[6] = deck[b];
            enumerate_board(&ranges, &mut local_results, &mut cards, &lookup);
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

fn enumerate_turn(ranges: Vec<Vec<(Hand, f32)>>, board: &[Card], deck: Deck, lookup: &[i32]) -> EquityResults {

    let results = (0..deck.len()).into_par_iter().map(|a| {
        let mut cards = [Card::default(); 7];
        cards[6] = deck[a];
        cards[2..6].copy_from_slice(board);
        let mut local_results = EquityResults::new(ranges.len());
        enumerate_board(&ranges, &mut local_results, &mut cards, &lookup);
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

fn enumerate_river(ranges: Vec<Vec<(Hand, f32)>>, board: &[Card], lookup: &[i32]) -> EquityResults {

    let mut results = EquityResults::new(ranges.len());
    let mut cards = [Card::default(); 7];
    cards[2] = board[0];
    cards[3] = board[1];
    cards[4] = board[2];
    cards[5] = board[3];
    cards[6] = board[4];
    enumerate_board(&ranges, &mut results, &mut cards, &lookup);

    results
}

fn enumerate_hands(
    ranges: &Vec<Vec<(Hand, f32)>>,
    range_idx: usize,
    used_cards: &mut u64,
    hands: &mut Vec<Hand>,
    board: &mut [Card; 7],
    lookup_table: &[i32],
    results: &mut EquityResults,
) {

    // Base case, one hand assigned to each player.
    if range_idx == ranges.len() {

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

        // If there is a single best hand, increment the win count for that hand.
        if best_idxs_count == 1 {
            results.wins[best_idxs[0]] += 1.0;
        } else {
            // For debugging, don't split the tie
            let tie_value = 1.0 / best_idxs_count as f64;
            for idx in 0..best_idxs_count {
                results.ties[best_idxs[idx]] += tie_value;
            }
        }

        results.total += 1.0;
        return;
    }

    for (hand, _) in &ranges[range_idx] {

        let hand_mask = 1 << hand.0.0 | 1 << hand.1.0;
        if *used_cards & hand_mask != 0 {
            continue;
        }

        *used_cards |= hand_mask;

        hands.push(*hand);
        enumerate_hands(ranges, range_idx + 1, used_cards, hands, board, lookup_table, results);
        hands.pop();

        *used_cards &= !hand_mask;
    }
}

fn enumerate_board(
    ranges: &Vec<Vec<(Hand, f32)>>,
    results: &mut EquityResults,
    board: &mut [Card; 7],
    lookup_table: &[i32],
) {
    let mut hands = Vec::with_capacity(ranges.len());
    let mut used_cards = 0_u64;
    for card in board[2..].iter() {
        used_cards |= 1 << card.0;
    }

    enumerate_hands(ranges, 0, &mut used_cards, &mut hands, board, lookup_table, results);
}
