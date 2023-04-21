use std::vec;
use rayon::prelude::*;
use crate::{card::Card, deck::Deck};
use crate::evaluate::{rank_hand_7, load_lookup_table};
use super::{
    check_board, 
    setup_cards, 
    EquityError, 
    EquityResults
};

pub fn equity_enumerate(mut ranges: Vec<Vec<(usize, usize)>>, board: Vec<Card>) -> Result<EquityResults, EquityError> {

    check_board(&board)?;
    let (board, deck) = setup_cards(&mut ranges, board)?;

    match board.len() {
        0 => enumerate_preflop(ranges, deck),
        3 => enumerate_flop(ranges, board, deck),
        4 => enumerate_turn(ranges, board, deck),
        5 => enumerate_river(ranges, board),
        _ => Err(EquityError::InvalidBoardSize(board.len())),
    }
}

fn enumerate_hands(
    ranges: &Vec<Vec<(usize, usize)>>,
    range_idx: usize,
    used_cards: &mut u64,
    hands: &mut Vec<(usize, usize)>,
    board: &mut [usize; 7],
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
            for idx in best_idxs {
                // Ties need to be halved??
                results.ties[idx] += 1;
            }
        }

        results.total += 1;
        return;
    }

    for hand in &ranges[range_idx] {
        if *used_cards & (1 << hand.0) != 0 || *used_cards & (1 << hand.1) != 0 {
            continue;
        }

        *used_cards |= 1 << hand.0;
        *used_cards |= 1 << hand.1;

        hands.push(*hand);
        enumerate_hands(ranges, range_idx + 1, used_cards, hands, board, lookup_table, results);
        hands.pop();

        *used_cards &= !(1 << hand.0);
        *used_cards &= !(1 << hand.1);
    }
}

fn enumerate_board(
    ranges: &Vec<Vec<(usize, usize)>>,
    results: &mut EquityResults,
    board: &mut [usize; 7],
    lookup_table: &[i32],
) {
    let mut hands = Vec::with_capacity(ranges.len());
    let mut used_cards = 0_u64;
    for card in board[2..].iter() {
        used_cards |= 1 << card;
    }

    enumerate_hands(ranges, 0, &mut used_cards, &mut hands, board, lookup_table, results);
}

fn enumerate_preflop(ranges: Vec<Vec<(usize, usize)>>, deck: Deck<usize>) -> Result<EquityResults, EquityError> {

    let lookup = load_lookup_table()?;
    let results = (0..deck.len()).into_par_iter().map(|a| {

        let mut cards = [0; 7];
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

fn enumerate_flop(ranges: Vec<Vec<(usize, usize)>>, board: Vec<usize>, deck: Deck<usize>) -> Result<EquityResults, EquityError> {

    let lookup = load_lookup_table()?;
    let results = (0..deck.len()).into_par_iter().map(|a| {

        let mut cards = [0; 7];
        let mut results = EquityResults::new(ranges.len());
        cards[2..5].copy_from_slice(&board);

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

fn enumerate_turn(ranges: Vec<Vec<(usize, usize)>>, board: Vec<usize>, deck: Deck<usize>) -> Result<EquityResults, EquityError> {

    let lookup = load_lookup_table()?;
    let results = (0..deck.len()).into_par_iter().map(|a| {
        
        let mut cards = [0; 7];
        let mut results = EquityResults::new(ranges.len());
        
        cards[2..6].copy_from_slice(&board);
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

fn enumerate_river(ranges: Vec<Vec<(usize, usize)>>, board: Vec<usize>) -> Result<EquityResults, EquityError> {

    let lookup = load_lookup_table()?;
    let mut results = EquityResults::new(ranges.len());
    let mut cards = [0; 7];
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
    use std::vec;

    use super::*;
    use crate::equity::assert_results_eq;
    use crate::hand::HandCombos;
    use crate::range::Range;

    #[test]
    fn test_enumerate_river_heads_up() {

        let range_1 = Range::from_str("66+,A8s+,KTs+,QTs+,JTs,ATo+,KJo+").unwrap();
        let range_2 = Range::from_str("22-99, A2o-A8o").unwrap();
        let ranges = vec![range_1.combos(), range_2.combos()];
        let board = Card::vec_from_str("Qh 4h 8c Qc 6s").unwrap();

        let results = equity_enumerate(ranges, board).unwrap();
        assert_results_eq(&results, vec![56, 44]);
    }

    #[test]
    fn test_enumerate_river_multi() {
        
        let range_1 = Range::from_str("A7s-A8s,K9s,JTs,ATo,KTo-KJo,QJo").unwrap();
        let range_2 = Range::from_str("66+,A8s+,KTs+,QTs+,JTs,ATo+,KJo+").unwrap();
        let range_3 = Range::from_str("22-TT,KTs+,QTs+,J9s+,T8s+,98s,87s,KJo+,QTo+,JTo,T9o,98o").unwrap();

        let ranges = vec![range_1.combos(), range_2.combos(), range_3.combos()];
        let board = Card::vec_from_str("6h 8s 4s 4d Qc").unwrap();
        
        let results = equity_enumerate(ranges, board).unwrap();
        assert_results_eq(&results, vec![13, 50, 37]);
    }

    #[test]
    fn test_enumerate_turn_multi() {

        let range_1 = Range::from_str("22-TT,KTs+,QTs+,J9s+,T8s+,98s,87s,KJo+,QTo+,JTo,T9o,98o").unwrap();
        let range_2 = Range::from_str("22-TT,K9s+,Q9s+,J8s+,T8s+,97s+,87s,76s,65s,KJo+,QTo+,JTo,T9o,98o").unwrap();
        let range_3 = Range::from_str("22+,A2s+,K2s+,Q2s+,J2s+,T5s+,96s+,87s,76s,A2o+,K3o+,Q6o+,J7o+,T8o+,97o+,87o").unwrap();

        let ranges = vec![range_1.combos(), range_2.combos(), range_3.combos()];
        let board = Card::vec_from_str("Td 6d Qc 8s").unwrap();
        
        let results = equity_enumerate(ranges, board).unwrap();
        assert_results_eq(&results, vec![37, 37, 26]);
    }

    #[test]
    fn test_enumerate_flop_heads_up() {
        let range_1 = Range::from_str("88+,ATs+,KTs+,QJs,AJo+,KQo");
        let range_2 = Range::from_str("55,K5s,Q7s,98s,A7o,Q9o,J9o");
        let ranges = vec![range_1.unwrap().combos(), range_2.unwrap().combos()];

        let board = Card::vec_from_str("Qh 4h 8c").unwrap();
        let results = equity_enumerate(ranges, board).unwrap();
        assert_results_eq(&results, vec![67, 33]);
    }

    #[test]
    fn test_enumerate_preflop_heads_up() {

        let range_1 = Range::from_str("88+,ATs+,KTs+,QJs,AJo+,KQo");
        let range_2 = Range::from_str("55,K5s,Q7s,98s,A7o,Q9o,J9o");
        let ranges = vec![range_1.unwrap().combos(), range_2.unwrap().combos()];

        let results = equity_enumerate(ranges, vec![]).unwrap();
        assert_results_eq(&results, vec![67, 33]);
    }
}