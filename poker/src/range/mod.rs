use std::{ops::Deref, fmt::Debug};
use crate::{hand::Hand, card::*};

mod parser;
pub use parser::*;

#[derive(Clone)]
pub struct Range {
    name:   String,
    hands:  [bool; 1326],
}

impl Default for Range {
    fn default() -> Self {
        Range {
            name:   String::from(""),
            hands:  [false; 1326],
        }
    }
}

impl Deref for Range {
    type Target = [bool; 1326];

    fn deref(&self) -> &Self::Target {
        &self.hands
    }
}

impl Range {

    pub fn new_from_grid(elems: Vec<bool>) -> Self {

        assert!(elems.len() == 169);
        let mut hands: [bool; 1326] = [false; 1326];

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

    pub fn get_hand(&self, hand: &Hand) -> bool {
        self[hand.idx()]
    }

    pub fn set_hand(&mut self, hand: &Hand, weight: bool) {
        self.hands[hand.idx()] = weight;
    }

    // Same as hand_combos, but excludes hands with cards in the dead mask.
    pub fn hand_combos(&self, dead: u64) -> Vec<Hand> {

        let mut hands = Vec::new();
        for i in 0..52 {
            for j in (i + 1)..52 {

                let hand = Hand(Card(i), Card(j));
                let hand_mask = hand.mask();
                let weight = self[hand.idx()];

                if weight && hand_mask & dead == 0 {
                    hands.push(hand);
                }
            }
        }
        
        hands
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
                    let weight = self.get_hand(&Hand(Card::new(*i, SUITS[0]), Card::new(*j, SUITS[0])));
                    if weight {
                        s.push_str(" 1 |");
                    } else {
                        s.push_str("    |");
                    }

                } else if i > j {
                    let weight = self.get_hand(&Hand(Card::new(*i, SUITS[0]), Card::new(*j, SUITS[0])));
                    if weight {
                        s.push_str(" 1 |");
                    } else {
                        s.push_str("    |");
                    }

                } else {
                    let weight = self.get_hand(&Hand(Card::new(*j, SUITS[0]), Card::new(*i, SUITS[0])));
                    if weight {
                        s.push_str(" 1 |");
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
    fn test_parse_range() {

        let a = [
            true,  true,  true,  false, false, false, false, false, false, false, false, false, false, 
            true,  true,  true,  false, false, false, false, false, false, false, false, false, false, 
            true,  true,  true,  false, false, false, false, false, false, false, false, false, false, 
            false, false, false, false, false, false, false, false, false, false, false, false, false, 
            false, false, false, false, false, false, false, false, false, false, false, false, false, 
            false, false, false, false, false, false, false, false, false, false, false, false, false, 
            false, false, false, false, false, false, false, false, false, false, false, false, false, 
            false, false, false, false, false, false, false, false, false, false, false, false, false, 
            false, false, false, false, false, false, false, false, false, false, false, false, false, 
            false, false, false, false, false, false, false, false, false, false, false, false, false, 
            false, false, false, false, false, false, false, false, false, false, false, false, false, 
            false, false, false, false, false, false, false, false, false, false, false, false, false, 
            false, false, false, false, false, false, false, false, false, false, false, false, false,
        ];
        let range = super::Range::new_from_grid(a.to_vec());

        let b = "QQ+, AQs+, KQs, AQo+, KQo";
        let range_2 = super::Range::from_str(b).unwrap();

        assert_eq!(range.hands, range_2.hands);
    }
}
