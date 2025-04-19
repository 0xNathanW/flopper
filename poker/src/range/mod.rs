use std::{collections::{HashMap, HashSet}, ops::Deref};
use crate::{card::*, hand::Hand, Board, isomorphism::valid_suit_permutations};

mod parser;
pub use parser::*;

pub struct Range {
    name:   String,
    hands:  [f32; 1326],
}

impl Default for Range {
    fn default() -> Self {
        Range {
            name:   String::from(""),
            hands:  [0.0; 1326],
        }
    }
}

impl Deref for Range {
    type Target = [f32; 1326];

    fn deref(&self) -> &Self::Target {
        &self.hands
    }
}

impl Range {

    pub fn new_from_grid(elems: Vec<f32>) -> Self {

        assert!(elems.len() == 169);
        let mut hands = [0.0; 1326];

        for i in 0..13 {
            for j in 0..13 {
                

                if i == j {
                    let rank = Rank::from(i as u8);
                    for idx in pair_idxs(rank) {
                        hands[idx] = elems[(12 - i) * 13 + (12 - j)];
                    }
                
                } else if i < j {
                    let rank_1 = Rank::from(i as u8);
                    let rank_2 = Rank::from(j as u8);
                    for idx in suited_idxs(rank_1, rank_2) {
                        hands[idx] = elems[(12 - i) * 13 + (12 - j)];
                    }
                
                } else {
                    let rank_1 = Rank::from(i as u8);
                    let rank_2 = Rank::from(j as u8);
                    for idx in offsuit_idxs(rank_1, rank_2) {
                        hands[idx] = elems[(12 - i) * 13 + (12 - j)];
                    }
                }
            }
        }

        Range {
            name:   String::from(""),
            hands,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_hand_weight(&self, hand: &Hand) -> f32 {
        self[hand.idx()]
    }

    pub fn get_avg_weight(&self, idxs: &[usize]) -> f32 {
        let mut sum = 0.0;
        for idx in idxs {
            sum += self[*idx];
        }
        sum / idxs.len() as f32
    }

    pub fn set_hand_weight(&mut self, hand: &Hand, weight: f32) {
        self.hands[hand.idx()] = weight;
    }

    // Returns all hands in the range with their weights.
    pub fn hand_combos(&self, dead_mask: u64) -> Vec<(Hand, f32)> {

        let mut hands = Vec::new();
        for i in 0..52 {
            for j in (i + 1)..52 {
                let hand = Hand(Card(i), Card(j));
                let hand_mask = hand.mask();
                let weight = self[hand.idx()];

                if weight > 0.0 && hand_mask & dead_mask == 0 {
                    hands.push((hand, weight));
                }
            }
        }
        hands
    }

    fn pre_flop_hand_combos_isomorphic_suits(&self) -> Vec<(Hand, f32)> {
        let mut canonical_hands = HashMap::new();
        let hand_combos = self.hand_combos(0);

        for (mut hand, weight) in hand_combos.into_iter() {
            if weight <= 0.0 {
                continue;
            }

            hand.canonicalise();
            let entry = canonical_hands.entry(hand).or_insert((hand, 0.0, 0));
            entry.1 += weight;
            entry.2 += 1;
        }

        let mut result = Vec::new();
        for (_, (hand, weight, count)) in canonical_hands {
            // Rejig the weights, average.
            result.push((hand, weight / count as f32))
        }

        result
    }

    pub fn hand_combos_isomorphic_suits(&self, board: &Board) -> Vec<(Hand, f32)> {
        
        if !board.is_flop_dealt() {
            return self.pre_flop_hand_combos_isomorphic_suits();
        }

        let mut canonical_hands = HashMap::new();
        let suits_on_board: HashSet<_> = board.as_vec().iter().map(|c| c.suit()).collect();

        let suit_permutations = valid_suit_permutations(&suits_on_board);
        
        for (mut hand, weight) in self.hand_combos(board.mask()) {
            hand.canonicalise_with_constraints(&suit_permutations);
            let entry = canonical_hands.entry(hand).or_insert((hand, 0.0, 0));
            
            entry.1 += weight;
            entry.2 += 1;
        }

        let mut result = Vec::new();
        for (_, (hand, weight, count)) in canonical_hands {
            result.push((hand, weight / count as f32))
        }

        result
    }
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let mut s = String::new();
        s.push_str(" |");
        RANKS.iter().rev().for_each(|r| s.push_str(&format!(" {}  |", r)));
        s.push('\n');

        for i in RANKS.iter().rev() {
            s.push_str(&format!("{}|", i));
            
            for j in RANKS.iter().rev() {
                
                if i == j {
                    let weight = self.get_avg_weight(&pair_idxs(*i));
                    if weight > 0.0 {
                        s.push_str(&format!("{:.2}|", weight));
                    } else {
                        s.push_str("    |");
                    }

                } else if i > j {
                    let weight = self.get_avg_weight(&suited_idxs(*i, *j));
                    if weight > 0.0 {
                        s.push_str(&format!("{:.2}|", weight));
                    } else {
                        s.push_str("    |");
                    }

                } else {
                    let weight = self.get_avg_weight(&offsuit_idxs(*j, *i));
                    if weight > 0.0 {
                        s.push_str(&format!("{:.2}|", weight));
                    } else {
                        s.push_str("    |");
                    }
                }
            }
            s.push('\n');
        }

        write!(f, "{}", s)?;
        Ok(())
    }
}

// Indexes of paired hands.
fn pair_idxs(rank: Rank) -> Vec<usize> {
    
    let mut idxs = Vec::with_capacity(6);
    for i in 0..4 {
        for j in i+1..4 {
        
            let idx = Hand(
                Card::new(rank, SUITS[i]),
                Card::new(rank, SUITS[j]),
            ).idx();
            
            idxs.push(idx);
        }
    }
    idxs
}

// Indexes of suited hands.
fn suited_idxs(rank_1: Rank, rank_2: Rank) -> Vec<usize> {
    
    let mut idxs = Vec::with_capacity(4);
    for a in SUITS.iter() {

        let idx = Hand(
            Card::new(rank_1, *a),
            Card::new(rank_2, *a),
        ).idx();
        
        idxs.push(idx);
    }
    idxs
}

// Indexes of offsuit hands.
fn offsuit_idxs(rank_1: Rank, rank_2: Rank) -> Vec<usize> {
    
    let mut idxs = Vec::with_capacity(12);
    for a in SUITS.iter() {
        for b in SUITS.iter() {

            if a != b {
                let idx = Hand(
                    Card::new(rank_1, *a),
                    Card::new(rank_2, *b),
                ).idx();
                
                idxs.push(idx);
            }
        }
    }
    idxs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = [
            1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
            1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
            1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let range = Range::new_from_grid(a.to_vec());

        let b = "QQ+, AQs+, KQs, AQo+, KQo";
        let range_2 = Range::from_str(b).unwrap();

        assert_eq!(range.hands, range_2.hands);
    }

    #[test]
    fn test_preflop_hand_combos_isomorphic_suits() {
        let range = Range::from_str("AKs").unwrap();
        
        let original_combos = range.hand_combos(0);
        assert_eq!(original_combos.len(), 4);
        
        let preprocessed = range.pre_flop_hand_combos_isomorphic_suits();
        assert_eq!(preprocessed.len(), 1);
        
        let canonical_hand = preprocessed[0].0;
        assert_eq!(canonical_hand.0.suit(), Suit::Spades);
        assert_eq!(canonical_hand.1.suit(), Suit::Spades);
    }

    #[test]
    fn test_preprocessing_with_board() {
        let range = Range::from_str("AKs, AQo").unwrap();
        
        let preprocessed = range.pre_flop_hand_combos_isomorphic_suits();
        assert_eq!(preprocessed.len(), 2);
        
        let original_combos = range.hand_combos(0);
        let board = Board::from_vec(vec![
            Card::from_str("Ah").unwrap(),
            Card::from_str("Th").unwrap(),
            Card::from_str("7h").unwrap(),
        ]).unwrap();
        
        let preprocessed_with_board = range.hand_combos_isomorphic_suits(&board);
        
        println!("Original combos: {}, With board preprocessing: {}", original_combos.len(), preprocessed_with_board.len());
        
        assert!(!preprocessed_with_board.is_empty());
        
        for (hand, _) in &preprocessed_with_board {
            if hand.0.rank() == Rank::Ace && hand.1.rank() == Rank::King && hand.0.suit() == hand.1.suit() {
                assert_ne!(hand.0.suit(), Suit::Hearts);
            }
        }
    }
}