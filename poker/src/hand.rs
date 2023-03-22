use std::collections::{BTreeSet, HashSet};

use crate::card::{Card, Deck, Rank, CardParseError};
use thiserror::Error;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HandRank {
    HighCard(u32),
    Pair(u32),
    TwoPair(u32),
    ThreeOfAKind(u32),
    Straight(u32),
    Flush(u32),
    FullHouse(u32),
    FourOfAKind(u32),
    StraightFlush(u32),
}

impl From<u16> for HandRank {
    fn from(value: u16) -> Self {
        match value {
            0..=1276 => HandRank::HighCard(value as u32),
            1277..=4136 => HandRank::Pair(value as u32 - 1277),
            4137..=4994 => HandRank::TwoPair(value as u32 - 4137),
            4995..=5852 => HandRank::ThreeOfAKind(value as u32 - 4995),
            5853..=5862 => HandRank::Straight(value as u32 - 5853),
            5863..=7139 => HandRank::Flush(value as u32 - 5863),
            7140..=7295 => HandRank::FullHouse(value as u32 - 7140),
            7296..=7451 => HandRank::FourOfAKind(value as u32 - 7296),
            7452..=7461 => HandRank::StraightFlush(value as u32 - 7452),
            _ => panic!("Unexpected hand rank value! '{}'", value)
        }
    }
}

#[derive(Error, Debug)]
pub enum HandParseError {
    #[error("Invalid number of cards in hand. expected 2")]
    InvalidNumberOfCards,
    #[error("Card error: {0}")]
    CardError(#[from] CardParseError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Hand(pub Card, pub Card);

impl Hand {
    pub fn deal(deck: &mut Deck) -> Hand {
        Hand(deck.draw().unwrap(), deck.draw().unwrap())
    }

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

    pub fn all_hands() -> Vec<Hand> {
        
        let deck_1 = Deck::new();
        let deck_2 = Deck::new();
        let mut hands = Vec::new();

        for i in 0..deck_1.len() {
            for j in i+1..deck_2.len() {
                hands.push(Hand(deck_1[i], deck_2[j]));
            }
        }

        hands
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

    // Returns index of hand in the range array.
    pub fn range_idx(&self) -> usize {
        let (high, low): (usize, usize) = if self.0.rank() >= self.1.rank() {
            (self.0.rank() as u8 as usize , self.1.rank() as u8 as usize)
        } else {
            (self.1.rank() as u8 as usize, self.0.rank() as u8 as usize)
        };
        
        high * (101 - high) / 2 + low - 1
    }

    pub fn chen_score(&self) -> i32 {
        
        let mut base = self.0.max(self.1).chen_score();
        let gap = ((self.0.rank() as i8 - self.1.rank() as i8).abs() as u8).saturating_sub(1);

        if self.pocket_pair() {
            base = 5.0_f32.max(base * 2.0);
        }
        if self.suited() {
            base += 2.0;
        }

        // Subtract points if their is a gap between the two cards.
        // Add 1 point if there is a 0 or 1 card gap and both cards are lower than a Q. (e.g. JT, 75, 32 etc, this bonus point does not apply to pocket pairs).
        base -= match gap {
            0 => 0.0,
            1 => {
                if self.0.rank().max(self.1.rank()) < Rank::Queen {
                    0.0
                } else {
                    1.0
                }
            },
            2 => {
                if self.0.rank().max(self.1.rank()) < Rank::Queen {
                    1.0
                } else {
                    2.0
                }
            },
            3 => 4.0,
            _ => 5.0,
        };

        // Round up to the nearest integer.
        base.ceil() as i32
    }
}

impl HandRank {
    pub fn rank_variant(value: HandRank) -> HandRank {
        match value {
            HandRank::HighCard(_) => HandRank::HighCard(0),
            HandRank::Pair(_) => HandRank::Pair(0),
            HandRank::TwoPair(_) => HandRank::TwoPair(0),
            HandRank::ThreeOfAKind(_) => HandRank::ThreeOfAKind(0),
            HandRank::Straight(_) => HandRank::Straight(0),
            HandRank::Flush(_) => HandRank::Flush(0),
            HandRank::FullHouse(_) => HandRank::FullHouse(0),
            HandRank::FourOfAKind(_) => HandRank::FourOfAKind(0),
            HandRank::StraightFlush(_) => HandRank::StraightFlush(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

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
    fn test_hand_rank_ord() {
        assert!(HandRank::HighCard(1) > HandRank::HighCard(0));
        assert!(HandRank::StraightFlush(5) > HandRank::HighCard(7));
        assert!(HandRank::Pair(2) == HandRank::Pair(2));
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
    fn test_range_idx() {
        let hand = Hand::from_str("AsAc").unwrap();
        println!("{}", hand.range_idx());
    }

    #[test]
    fn test_all_hands() {
        let hands = Hand::all_hands();
        assert_eq!(hands.len(), 1326);
    }
}