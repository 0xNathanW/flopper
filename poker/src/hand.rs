use thiserror::Error;
use regex::Regex;
use crate::card::{Card, Rank, CardParseError};

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
    pub fn idx(&self) -> usize {
        let (mut high, mut low) = (self.0.0, self.1.0);
        if high < low {
            std::mem::swap(&mut high, &mut low);
        }

        low as usize * (101 - low as usize) / 2 + high as usize - 1
    }

    pub fn from_idx(idx: usize) -> Hand {
        let card1 = (103 - (103.0 * 103.0 - 8.0 * idx as f64).sqrt().ceil() as u16) / 2;
        let card2 = idx as u16 - card1 * (101 - card1) / 2 + 1;
        Hand(Card(card1 as u8), Card(card2 as u8))
    }

    // Returns a bit mask of the two cards.
    pub fn mask(&self) -> u64 {
        (1 << self.0.0) | (1 << self.1.0)
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
}