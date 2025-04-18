use std::{ops::Deref, fmt::Debug};
use crate::{hand::Hand, card::*};

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
    pub fn hand_combos(&self) -> Vec<(Hand, f32)> {
        
        let mut hands = Vec::new();
        for i in 0..1326 {

            let weight = self[i];
            if weight > 0.0 {
                let hand = Hand::from_idx(i);
                hands.push((hand, weight));
            }
        }
        hands
    }

    // Same as hand_combos, but excludes hands with cards in the dead mask.
    pub fn hand_combos_dead(&self, dead: u64) -> Vec<(Hand, f32)> {

        let mut hands = Vec::new();
        for i in 0..52 {
            for j in (i + 1)..52 {

                let hand = Hand(Card(i), Card(j));
                let hand_mask = hand.mask();
                let weight = self[hand.idx()];

                if weight > 0.0  && hand_mask & dead == 0 {
                    hands.push((hand, weight));
                }
            }
        }
        
        hands
    }

    // Returns true if swapping two suits in the range does not change the hand weights.
    pub fn suit_isomorphistic(&self, suits: [Suit; 2]) -> bool {
        
        let swap_suit = |suit| {
            if suit == suits[0] {
                suits[1]
            } else if suit == suits[1] {
                suits[0]
            } else {
                suit
            }
        };

        for a in (0..52).into_iter().map(|i| Card(i)) {
            for b in ((a.0 + 1)..52).into_iter().map(|i| Card(i)) {
                
                let a_swapped = a.swap_suit(swap_suit(a.suit()));
                let b_swapped = b.swap_suit(swap_suit(b.suit()));

                let weight = self.get_hand_weight(&Hand(a, b));
                let weight_swapped = self.get_hand_weight(&Hand(a_swapped, b_swapped));
                
                // If weights are different, then the range is not suit isomorphistic.
                if weight != weight_swapped {
                    return false;
                }
            }
        }

        true
    }
    
}

impl Debug for Range {
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
pub fn pair_idxs(rank: Rank) -> Vec<usize> {
    
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
pub fn suited_idxs(rank_1: Rank, rank_2: Rank) -> Vec<usize> {
    
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
pub fn offsuit_idxs(rank_1: Rank, rank_2: Rank) -> Vec<usize> {
    
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
        let range = super::Range::new_from_grid(a.to_vec());

        let b = "QQ+, AQs+, KQs, AQo+, KQo";
        let range_2 = super::Range::from_str(b).unwrap();

        assert_eq!(range.hands, range_2.hands);
    }
}