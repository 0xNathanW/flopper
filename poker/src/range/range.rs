use std::{collections::{BTreeSet}, fmt::Debug};
use rand::Rng;

use crate::{card::{Rank, RANKS, Suit, Card}, hand::Hand};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum RangeHand {
    Pair(Rank),
    Suited(Rank, Rank),
    Offsuit(Rank, Rank),
}

impl From<Hand> for RangeHand {
    fn from(value: Hand) -> Self {
        let max_rank = value.0.rank().max(value.1.rank());
        let min_rank = value.0.rank().min(value.1.rank());

        // Pair
        if max_rank == min_rank {
            RangeHand::Pair(max_rank)
        }
        // Suited
        else if value.0.suit() == value.1.suit() {
            RangeHand::Suited(max_rank, min_rank)
        }
        // Offsuit
        else {
            RangeHand::Offsuit(max_rank, min_rank)
        }
    }
}

impl RangeHand {
    pub fn chen_score(self) -> i32 {
        match self {
            
            RangeHand::Pair(rank) => {
                let hand = Hand(
                    Card::new(rank, Suit::Clubs),
                    Card::new(rank, Suit::Diamonds)
                );
                hand.chen_score()
            },

            RangeHand::Suited(rank_1, rank_2) => {
                let hand = Hand(
                    Card::new(rank_1, Suit::Clubs),
                    Card::new(rank_2, Suit::Clubs)
                );
                hand.chen_score()
            },
            
            RangeHand::Offsuit(rank_1, rank_2) => {
                let hand = Hand(
                    Card::new(rank_1, Suit::Clubs),
                    Card::new(rank_2, Suit::Diamonds)
                );
                hand.chen_score()
            },
        }
    }
}

pub struct Range {
    pub name:       String,
    pub hands:          BTreeSet<RangeHand>,
    pub combo_counts:   Vec<(usize, RangeHand)>,
    pub total_combos:   usize,
}

impl Range {

    // Returns true if the range contains the given hand.
    pub fn contains(&self, hand: &Hand) -> bool {

        let max_rank = hand.0.rank().max(hand.1.rank());
        let min_rank = hand.0.rank().min(hand.1.rank());

        // Pair
        if max_rank == min_rank {
            self.hands.contains(&RangeHand::Pair(max_rank))
        }
        // Suited
        else if hand.0.suit() == hand.1.suit() {
            self.hands.contains(&RangeHand::Suited(max_rank, min_rank))
        }
        // Offsuit
        else {
            self.hands.contains(&RangeHand::Offsuit(max_rank, min_rank))
        }
    }

    pub fn random_hand<R: Rng>(&self, rng: &mut R) -> Hand {
        
        let n = rng.gen_range(0..self.total_combos);

        let mut spec = None;
        for hand in self.combo_counts.iter() {
            if n < hand.0 {
                spec = Some(hand.1);
                break;
            }
        }

        match spec.unwrap() {

            RangeHand::Pair(rank) => {
                let suits = random_suits(rng);
                Hand(Card::new(rank, suits.0), Card::new(rank, suits.1))
            }
        
            RangeHand::Suited(rank_1, rank_2) => {
                let suits = random_suits(rng);
                Hand(Card::new(rank_1, suits.0), Card::new(rank_2, suits.0))
            }

            RangeHand::Offsuit(rank_1, rank_2) => {
                let suits = random_suits(rng);
                Hand(Card::new(rank_1, suits.0), Card::new(rank_2, suits.1))
            }
        }
    }
}

// Print range as a table.
impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let mut s = String::new();
        s.push_str(" |");
        RANKS.iter().rev().for_each(|r| s.push_str(&format!("{}|", r)));
        s.push('\n');

        for i in RANKS.iter().rev() {

            s.push_str(&format!("{}|", i));
            for j in RANKS.iter().rev() {
                
                if i == j {
                    if let Some(_) = self.hands.get(&RangeHand::Pair(*i)) {
                        s.push_str("x|");
                    } else {
                        s.push_str(" |");
                    }

                } else if i > j {
                    if let Some(_) = self.hands.get(&RangeHand::Suited(*i, *j)) {
                        s.push_str("x|");
                    } else {
                        s.push_str(" |");
                    }

                } else {
                    if let Some(_) = self.hands.get(&RangeHand::Offsuit(*j, *i)) {
                        s.push_str("x|");
                    } else {
                        s.push_str(" |");
                    }
                }
            }
            s.push('\n');
        }

        write!(f, "{}", s)?;
        Ok(())
    }
}

fn random_suits<R: Rng>(rng: &mut R) -> (Suit, Suit) {
    let i = rng.gen_range(0..4);
    let j = loop {
        let j = rng.gen_range(0..4);
        if j != i {
            break j;
        }
    };
    (i.into(), j.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let range = Range::from_str("99+").unwrap();
        
        assert!(range.contains(&Hand::from_str("9d9s").unwrap()));
        assert!(range.contains(&Hand::from_str("TsTd").unwrap()));
        assert!(range.contains(&Hand::from_str("QcQh").unwrap()));
        assert!(range.contains(&Hand::from_str("AcAd").unwrap()));
        assert!(!range.contains(&Hand::from_str("8d8h").unwrap()));
        assert!(!range.contains(&Hand::from_str("2s2h").unwrap()));
    }

    #[test]
    fn test_range_hand_chen_score() {
        assert_eq!(RangeHand::Pair(Rank::Ace).chen_score(), 20);
        assert_eq!(RangeHand::Pair(Rank::Ten).chen_score(), 10);
        assert_eq!(RangeHand::Suited(Rank::Five, Rank::Seven).chen_score(), 6);
        assert_eq!(RangeHand::Offsuit(Rank::Two, Rank::Seven).chen_score(), -1);
        assert_eq!(RangeHand::Suited(Rank::Ace, Rank::King).chen_score(), 12);
    }
}