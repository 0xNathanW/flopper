use thiserror::Error;
use regex::Regex;
use crate::card::{Card, CardParseError, Rank, Suit};

#[derive(Error, Debug)]
pub enum HandParseError {
    #[error("Invalid number of cards in hand. expected 2")]
    InvalidNumberOfCards,
    #[error("Card error: {0}")]
    CardError(#[from] CardParseError),
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord)]
pub struct Hand(pub Card, pub Card);

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for Hand {}

impl std::hash::Hash for Hand {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let sorted = self.sorted();
        sorted.0.hash(state);
        sorted.1.hash(state);
    }
}

impl Hand {
    pub fn from_str(s: &str) -> Result<Hand, HandParseError> {
        
        let re = Regex::new(r"(?i)[2-9TJQKA][c|s|h|d]").unwrap();

        let a = re.find_iter(s)
            .nth(0)
            .ok_or(HandParseError::InvalidNumberOfCards)?
            .as_str();
        let a = Card::from_str(a)?;

        let b = re.find_iter(s)
            .nth(1)
            .ok_or(HandParseError::InvalidNumberOfCards)?
            .as_str();
        let b = Card::from_str(b)?;
        
        Ok(Hand(a, b))
    }

    pub fn random() -> Hand {
        let first = Card::random();
        let mut second;
        loop {
            second = Card::random();
            if first != second {
                break;
            }
        }
        Hand(first, second)
    }

    pub fn pocket_pair(&self) -> bool {
        self.0.rank() == self.1.rank()
    }

    pub fn suited(&self) -> bool {
        self.0.suit() == self.1.suit()
    }

    pub fn high_card(&self) -> Rank {
        self.0.rank().max(self.1.rank())
    } 

    #[inline]
    fn sorted(&self) -> Hand {
        let mut hand = *self;
        if (hand.1.rank() != hand.0.rank() && hand.1.rank() > hand.0.rank()) || 
           (hand.1.rank() == hand.0.rank() && hand.1.suit() > hand.0.suit()) {
            std::mem::swap(&mut hand.0, &mut hand.1);
        }
        hand
    }

    // Returns index of hand in the range array.
    pub fn idx(&self) -> usize {
        let sorted = self.sorted();
        let high = sorted.0.0;
        let low = sorted.1.0;

        low as usize * (101 - low as usize) / 2 + high as usize - 1
    }

    pub fn from_idx(idx: usize) -> Hand {
        let card1 = (103 - (103.0 * 103.0 - 8.0 * idx as f64).sqrt().ceil() as u16) / 2;
        let card2 = idx as u16 - card1 * (101 - card1) / 2 + 1;
        Hand(Card(card1 as u8), Card(card2 as u8))
    }

    pub fn mask(&self) -> u64 {
        (1 << self.0.0) | (1 << self.1.0)
    }

    pub fn chen_score(&self) -> i32 {
        let sorted = self.sorted();
        
        let mut base = sorted.0.max(sorted.1).chen_score();
        let gap = ((sorted.0.rank() as i8 - sorted.1.rank() as i8).abs() as u8).saturating_sub(1);

        if sorted.pocket_pair() {
            base = 5.0_f32.max(base * 2.0);
        }
        if sorted.suited() {
            base += 2.0;
        }

        // Subtract points if their is a gap between the two cards.
        // Add 1 point if there is a 0 or 1 card gap and both cards are lower than a Q. (e.g. JT, 75, 32 etc, this bonus point does not apply to pocket pairs).
        base -= match gap {
            0 => 0.0,
            1 => {
                if sorted.0.rank().max(sorted.1.rank()) < Rank::Queen {
                    0.0
                } else {
                    1.0
                }
            },
            2 => {
                if sorted.0.rank().max(sorted.1.rank()) < Rank::Queen {
                    1.0
                } else {
                    2.0
                }
            },
            3 => 4.0,
            _ => 5.0,
        };

        base.ceil() as i32
    }

    pub fn canonicalise_without_constraints(&mut self) {
        *self = self.sorted();

        if self.0.suit() == self.1.suit() {
            if self.0.suit() == Suit::Spades {
                return
            } else {
                self.0.swap_suit(Suit::Spades);
                self.1.swap_suit(Suit::Spades);
            }
        } else {
            self.0.swap_suit(Suit::Spades);
            self.1.swap_suit(Suit::Hearts);
        }
    }

    pub fn canonicalise(&mut self, valid_permutations: &[[Suit; 4]]) {
        let mut candidates: Vec<Hand> = valid_permutations.iter().map(|perm| {
            let mut h = self.clone();
            h.0.swap_suit(perm[h.0.suit() as usize]);
            h.1.swap_suit(perm[h.1.suit() as usize]);
            h = h.sorted();
            h
        }).collect();

        candidates.sort();
        *self = candidates.remove(0);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;
    use crate::card::{Rank, Suit};

    #[test]
    fn test_hand_equality() {
        let hand1 = Hand::from_str("Kh2s").unwrap();
        let hand2 = Hand::from_str("2sKh").unwrap();
        assert_eq!(hand1, hand2);
        
        let hand3 = Hand::from_str("Ah7d").unwrap();
        let hand4 = Hand::from_str("7dAh").unwrap();
        assert_eq!(hand3, hand4);
        
        let hand5 = Hand::from_str("Ah7d").unwrap();
        let hand6 = Hand::from_str("7hAd").unwrap();
        assert_ne!(hand5, hand6);
    }

    #[test]
    fn test_hand_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        fn get_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }

        let hand1 = Hand::from_str("Kh2s").unwrap();
        let hand2 = Hand::from_str("2sKh").unwrap();
        
        assert_eq!(get_hash(&hand1), get_hash(&hand2));
        
        let mut set = HashSet::new();
        set.insert(hand1);
        assert!(set.contains(&hand2));
        assert_eq!(set.len(), 1);
        set.insert(hand2);
        assert_eq!(set.len(), 1);
    }


    #[test]
    fn test_from_str() {
        // With space.
        let hand = Hand::from_str("Ah Kd").unwrap();
        assert_eq!(hand.0, Card::new(Rank::Ace, Suit::Hearts));
        assert_eq!(hand.1, Card::new(Rank::King, Suit::Diamonds));
        // Without space.
        let hand = Hand::from_str("Th4c").unwrap();
        assert_eq!(hand.0, Card::new(Rank::Ten, Suit::Hearts));
        assert_eq!(hand.1, Card::new(Rank::Four, Suit::Clubs));
        // Same rank.
        let hand = Hand::from_str("2h2d").unwrap();
        assert_eq!(hand.0, Card::new(Rank::Two, Suit::Hearts));
        assert_eq!(hand.1, Card::new(Rank::Two, Suit::Diamonds));
        // With separator.
        let hand = Hand::from_str("-Ah//Kd-").unwrap();
        assert_eq!(hand.0, Card::new(Rank::Ace, Suit::Hearts));
        assert_eq!(hand.1, Card::new(Rank::King, Suit::Diamonds));
        // Upper/Lower case.
        let hand = Hand::from_str("aH kD").unwrap();
        assert_eq!(hand.0, Card::new(Rank::Ace, Suit::Hearts));
        assert_eq!(hand.1, Card::new(Rank::King, Suit::Diamonds));
    }

    #[test]
    fn test_chen_score() {
        let hand = Hand::from_str("AhAc").unwrap();
        assert_eq!(hand.chen_score(), 20);

        let hand = Hand::from_str("TcTd").unwrap();
        assert_eq!(hand.chen_score(), 10);

        let hand = Hand::from_str("5h7h").unwrap();
        assert_eq!(hand.chen_score(), 6);

        let hand = Hand::from_str("2c7h").unwrap();
        assert_eq!(hand.chen_score(), -1);

        let hand = Hand::from_str("AsKs").unwrap();
        assert_eq!(hand.chen_score(), 12);
    }

    #[test]
    fn test_hand_range_idx() {
      
        let mut idx = 0;
        for i in 0..52_u8 {
            for j in (i + 1)..52_u8 {
                let hand = Hand(Card(i), Card(j));
                assert_eq!(hand.idx(), idx);
                let hand_inv = Hand(Card(j), Card(i));
                assert_eq!(hand_inv.idx(), idx);
                idx += 1;
            }
        }
    }

    #[test]
    fn test_canonicalise_without_constraints() {
        let mut hand = Hand::from_str("KdQd").unwrap();
        hand.canonicalise_without_constraints();
        assert_eq!(hand, Hand::from_str("KsQs").unwrap());

        let mut hand = Hand::from_str("KdQh").unwrap();
        hand.canonicalise_without_constraints();
        assert_eq!(hand, Hand::from_str("KsQh").unwrap());
    }

    fn test_canonicalise_equivalence_with_board(board_suits: &[Suit]) -> usize {
        use crate::isomorphism::valid_suit_permutations;
        use crate::evaluate::rank_hand_senzee;

        let mut board = [Card::default(); 5];
        let mut used_cards = 0u64;
        for i in 0..5 {
            let suit = board_suits[i % board_suits.len()];
            let mut card;
            loop {
                let rank = fastrand::u8(0..13);
                card = Card::new(rank.into(), suit);
                if used_cards & (1 << card.0) == 0 {
                    used_cards |= 1 << card.0;
                    break;
                }
            }
            board[i] = card;
        }

        let suits_on_board = HashSet::from_iter(board.iter().map(|c| c.suit()));
        let valid_permutations = valid_suit_permutations(&suits_on_board);
        
        let mut hands = Vec::new();
        for i in 0..52_u8 {
            if used_cards & (1 << i) != 0 {
                continue; 
            }
            
            for j in (i + 1)..52_u8 {
                if used_cards & (1 << j) != 0 {
                    continue;
                }
                
                hands.push(Hand(Card(i), Card(j)));
            }
        }

        let mut unique_canonical_hands = HashSet::new();        
        let mut hand = [Card::default(); 7];
        for (i, c) in board.iter().enumerate() {
            hand[i] = *c;
        }
        
        for hole in hands {
            let mut c_hole = hole.clone();
            c_hole.canonicalise(&valid_permutations);
            
            hand[5] = hole.0;
            hand[6] = hole.1;
            let orig_rank = rank_hand_senzee(&hand).unwrap();
            
            hand[5] = c_hole.0;
            hand[6] = c_hole.1;
            let canonical_rank = rank_hand_senzee(&hand).unwrap();
        
            assert_eq!(orig_rank, canonical_rank, "canonical hand doesn't produce the same rank as the original hands");
            unique_canonical_hands.insert(c_hole);
        }

        unique_canonical_hands.len()
    }

    #[test]
    fn test_canonicalise_equivalence_one_suit() {
        let board_suits = [Suit::Spades];
        let canonical = test_canonicalise_equivalence_with_board(&board_suits);
        assert_eq!(canonical, 301);
    }

    #[test]
    fn test_canonicalise_equivalence_two_suits() {
        let board_suits = [Suit::Spades, Suit::Hearts];
        let canonical = test_canonicalise_equivalence_with_board(&board_suits);
        assert_eq!(canonical, 652);
    }

    #[test]
    fn test_canonicalise_equivalence_three_suits() {
        let board_suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds];
        let canonical = test_canonicalise_equivalence_with_board(&board_suits);
        assert_eq!(canonical, 1081);
    }

    #[test]
    fn test_canonicalise_equivalence_four_suits() {
        let board_suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        let canonical = test_canonicalise_equivalence_with_board(&board_suits);
        assert_eq!(canonical, 1081);
    }
    
}