use std::vec;
use rayon::prelude::*;
use crate::{board::Board, card::Card, deck::Deck, hand::Hand, range::Range, evaluate::rank_hand_7, equity::{setup_cards, EquityResults}, error::Result};

pub fn equity_enumerate(ranges: Vec<Range>, board: Board, lookup: &[i32]) -> Result<EquityResults> {

    let board_cards = board.as_vec();
    let (ranges, deck) = setup_cards(ranges, &board_cards)?;

    if board.is_river_dealt() {
        enumerate_river(ranges, &board_cards, lookup)

    } else if board.is_turn_dealt() {
        enumerate_turn(ranges, &board_cards, deck, lookup)
    
    } else if board.is_flop_dealt() {
        enumerate_flop(ranges, &board_cards, deck, lookup)
    
    } else {
        enumerate_preflop(ranges, deck, lookup)
    }
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

    if range_idx == ranges.len() {

        let mut best_idxs = vec![];
        let mut best_rank = 0;
        for (i, &hand) in hands.iter().enumerate() {

            board[0] = hand.0;
            board[1] = hand.1;
            
            let rank = rank_hand_7(board, lookup_table);
            if rank > best_rank {
                best_idxs.clear();
                best_idxs.push(i);
                best_rank = rank;
            } else if rank == best_rank {
                best_idxs.push(i);
            }
        }

        if best_idxs.len() == 1 {
            results.wins[best_idxs[0]] += 1;
        } else {
            // print!("Tie (rank: {:?})", HandRank::from(best_rank));
            for idx in best_idxs {
                // print!(" {:?}", hands[idx]);
                // Ties need to be halved??
                results.ties[idx] += 1;
            }
            // println!();
        }
        results.total += 1;
        return;
    }

    for (hand, _weight) in &ranges[range_idx] {
        if *used_cards & (1 << hand.0.0) != 0 || *used_cards & (1 << hand.1.0) != 0 {
            continue;
        }

        *used_cards |= 1 << hand.0.0;
        *used_cards |= 1 << hand.1.0;

        hands.push(*hand);
        enumerate_hands(ranges, range_idx + 1, used_cards, hands, board, lookup_table, results);
        hands.pop();

        *used_cards &= !(1 << hand.0.0);
        *used_cards &= !(1 << hand.1.0);
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

fn enumerate_preflop(ranges: Vec<Vec<(Hand, f32)>>, deck: Deck, lookup: &[i32]) -> Result<EquityResults> {

    let results = (0..deck.len()).into_par_iter().map(|a| {

        let mut cards = [Card::default(); 7];
        let mut results = EquityResults::new(ranges.len());

        for b in (a + 1)..deck.len() {
            for c in (b + 1)..deck.len() {
                for d in (c + 1)..deck.len() {
                    for e in (d + 1)..deck.len() {

                        cards[2] = deck[a];
                        cards[3] = deck[b];
                        cards[4] = deck[c];
                        cards[5] = deck[d];
                        cards[6] = deck[e];

                        enumerate_board(&ranges, &mut results, &mut cards, &lookup);
                    }
                }
            }
        }

        results
    }).collect::<Vec<EquityResults>>();

    let mut total = EquityResults::new(ranges.len());
    for result in results {
        total.wins.iter_mut().zip(result.wins.iter()).for_each(|(a, b)| *a += b);
        total.ties.iter_mut().zip(result.ties.iter()).for_each(|(a, b)| *a += b);
        total.total += result.total;
    }

    Ok(total)
}

fn enumerate_flop(ranges: Vec<Vec<(Hand, f32)>>, board: &[Card], deck: Deck, lookup: &[i32]) -> Result<EquityResults> {

    let results = (0..deck.len()).into_par_iter().map(|a| {

        let mut cards = [Card::default(); 7];
        cards[2..5].copy_from_slice(board);
        let mut results = EquityResults::new(ranges.len());

        for b in (a + 1)..deck.len() {

            cards[5] = deck[a];
            cards[6] = deck[b];

            enumerate_board(&ranges, &mut results, &mut cards, &lookup);
        }

        results
    }).collect::<Vec<EquityResults>>();

    let mut total = EquityResults::new(ranges.len());
    for result in results {
        total.wins.iter_mut().zip(result.wins.iter()).for_each(|(a, b)| *a += b);
        total.ties.iter_mut().zip(result.ties.iter()).for_each(|(a, b)| *a += b);
        total.total += result.total;
    }

    Ok(total)
}

fn enumerate_turn(ranges: Vec<Vec<(Hand, f32)>>, board: &[Card], deck: Deck, lookup: &[i32]) -> Result<EquityResults> {

    let results = (0..deck.len()).into_par_iter().map(|a| {
        
        let mut cards = [Card::default(); 7];
        let mut results = EquityResults::new(ranges.len());
        
        cards[2..6].copy_from_slice(board);
        cards[6] = deck[a];
        
        enumerate_board(&ranges, &mut results, &mut cards, &lookup);
        results
    }).collect::<Vec<EquityResults>>();

    let mut total = EquityResults::new(ranges.len());
    for result in results {
        total.wins.iter_mut().zip(result.wins.iter()).for_each(|(a, b)| *a += b);
        total.ties.iter_mut().zip(result.ties.iter()).for_each(|(a, b)| *a += b);
        total.total += result.total;
    }
    Ok(total)
}

fn enumerate_river(ranges: Vec<Vec<(Hand, f32)>>, board: &[Card], lookup: &[i32]) -> Result<EquityResults> {

    let mut results = EquityResults::new(ranges.len());
    let mut cards = [Card::default(); 7];
    cards[2] = board[0];
    cards[3] = board[1];
    cards[4] = board[2];
    cards[5] = board[3];
    cards[6] = board[4];

    enumerate_board(&ranges, &mut results, &mut cards, &lookup);

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{equity::assert_results_eq, range::Range, evaluate::load_lookup_table};
    
    const LOOKUP_PATH: &str = "./data/lookup_table.bin";

    #[test]
    fn test_enumerate_river_heads_up() {

        let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
        let range_1 = Range::from_str("66+,A8s+,KTs+,QTs+,JTs,ATo+,KJo+").unwrap();
        let range_2 = Range::from_str("22-99,A2o-A8o").unwrap();
        let ranges = vec![range_1, range_2];
        let board = Board::from_str("Qh 4h 8c Qc 6s").unwrap();

        let results = equity_enumerate(ranges, board, &lookup).unwrap();
        assert_results_eq(&results, vec![56, 44]);
    }

    #[test]
    fn test_enumerate_river_multi() {
        
        let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
        let range_1 = Range::from_str("A7s-A8s,K9s,JTs,ATo,KTo-KJo,QJo").unwrap();
        let range_2 = Range::from_str("66+,A8s+,KTs+,QTs+,JTs,ATo+,KJo+").unwrap();
        let range_3 = Range::from_str("22-TT,KTs+,QTs+,J9s+,T8s+,98s,87s,KJo+,QTo+,JTo,T9o,98o").unwrap();

        let ranges = vec![range_1, range_2, range_3];
        let board = Board::from_str("6h 8s 4s 4d Qc").unwrap();
        
        let results = equity_enumerate(ranges, board, &lookup).unwrap();
        assert_results_eq(&results, vec![13, 50, 37]);
    }


    #[test]
    fn test_enumerate_turn_multi() {

        let range_1 = Range::from_str("22-TT,KTs+,QTs+,J9s+,T8s+,98s,87s,KJo+,QTo+,JTo,T9o,98o").unwrap();
        let range_2 = Range::from_str("22-TT,K9s+,Q9s+,J8s+,T8s+,97s+,87s,76s,65s,KJo+,QTo+,JTo,T9o,98o").unwrap();
        let range_3 = Range::from_str("22+,A2s+,K2s+,Q2s+,J2s+,T5s+,96s+,87s,76s,A2o+,K3o+,Q6o+,J7o+,T8o+,97o+,87o").unwrap();

        let ranges = vec![range_1, range_2, range_3];
        let board = Board::from_str("Td 6d Qc 8s").unwrap();
        
        let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
        let results = equity_enumerate(ranges, board, &lookup).unwrap();
        assert_results_eq(&results, vec![37, 37, 26]);
    }

    #[test]
    fn test_enumerate_flop_heads_up() {
        let range_1 = Range::from_str("88+,ATs+,KTs+,QJs,AJo+,KQo").unwrap();
        let range_2 = Range::from_str("55,K5s,Q7s,98s,A7o,Q9o,J9o").unwrap();
        let ranges = vec![range_1, range_2];

        let board = Board::from_str("Qh 4h 8c").unwrap();
        let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
        let results = equity_enumerate(ranges, board, &lookup).unwrap();
        assert_results_eq(&results, vec![67, 33]);
    }

    #[test]
    fn test_enumerate_preflop_heads_up() {

        let range_1 = Range::from_str("88+,ATs+,KTs+,QJs,AJo+,KQo").unwrap();
        let range_2 = Range::from_str("55,K5s,Q7s,98s,A7o,Q9o,J9o").unwrap();
        let ranges = vec![range_1, range_2];

        let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
        let results = equity_enumerate(ranges, Board::default(), &lookup).unwrap();
        assert_results_eq(&results, vec![67, 33]);
    }
}