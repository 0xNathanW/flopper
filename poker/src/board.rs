use std::collections::HashSet;
use thiserror::Error;
use crate::card::{Card, CardParseError};

#[derive(Debug, Default)]
pub struct Board {
    pub flop:  [Card; 3],
    pub turn:  Card,
    pub river: Card,
}

#[derive(Debug, Error)]
pub enum BoardError {
    #[error("Invalid board size: {0} cards.  Expected 0, 3, 4 or 5.")]
    InvalidBoardSize(usize),

    #[error("Error parsing card: {0}")]
    CardError(#[from] CardParseError),

    #[error("Duplicate card in board.")]
    DuplicateCard,
}

impl Board {

    pub fn from_vec(cards: Vec<Card>) -> Result<Board, BoardError> {

        let b =match cards.len() {
            0 => Board::default(),
            3 => Board {
                flop: [cards[0], cards[1], cards[2]],
                ..Board::default()
            },
            4 => Board {
                flop: [cards[0], cards[1], cards[2]],
                turn: cards[3],
                ..Board::default()
            },
            5 => Board {
                flop: [cards[0], cards[1], cards[2]],
                turn: cards[3],
                river: cards[4],
            },
            _ => return Err(BoardError::InvalidBoardSize(cards.len())),
        };

        b.check_duplicates()?;
        Ok(b)
    }

    pub fn from_arr(cards: [Card; 5]) -> Result<Board, BoardError> {
        let b = Board {
            flop: [cards[0], cards[1], cards[2]],
            turn: cards[3],
            river: cards[4],
        };
        b.check_duplicates()?;
        Ok(b)
    }

    pub fn from_str(s: &str) -> Result<Board, BoardError> {
        let vec = Card::vec_from_str(s)?;
        Board::from_vec(vec)
    }

    pub fn as_vec(&self) -> Vec<Card> {
        let mut cards = Vec::new();
        
        if self.is_flop_dealt() {
            cards.extend_from_slice(&self.flop);
        } 
        if self.is_turn_dealt() {
            cards.push(self.turn);
        }
        if self.is_river_dealt() {
            cards.push(self.river);
        }

        cards
    }

    // Can verify any of the flop cards.
    pub fn is_flop_dealt(&self) -> bool {
        self.flop[2].is_dealt()
    }

    pub fn is_turn_dealt(&self) -> bool {
        self.turn.is_dealt()
    }

    pub fn is_river_dealt(&self) -> bool {
        self.river.is_dealt()
    }

    pub fn mask(&self) -> u64 {
        let mut mask: u64 = 0;
        self.as_vec().iter().for_each(|c| mask |= 1 << c.0);
        mask
    }

    pub fn dead_mask(&self) -> u64 {
        let mut mask: u64 = 0;
        self.as_vec().iter().for_each(|c| mask |= 1 << c.0);
        mask
    }

    fn check_duplicates(&self) -> Result<(), BoardError> {
        let unique = HashSet::<Card>::from_iter(self.as_vec());
        if unique.len() != self.as_vec().len() {
            return Err(BoardError::DuplicateCard);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {

        let b = Board::from_vec(vec![]).unwrap();
        assert!(!b.is_flop_dealt());
        assert!(!b.is_turn_dealt());
        assert!(!b.is_river_dealt());

        let mut v = vec![
            Card::from_str("Ks").unwrap(),
            Card::from_str("3h").unwrap(),
            Card::from_str("Qd").unwrap(),
        ];

        let b = Board::from_vec(v.clone()).unwrap();
        assert!(b.is_flop_dealt());
        assert!(!b.is_turn_dealt());
        assert!(!b.is_river_dealt());

        v.push(Card::from_str("2c").unwrap());
        let a: [Card; 5] = [v[0], v[1], v[2], v[3], Card(0xFF)];
        
        let b = Board::from_arr(a).unwrap();
        assert!(b.is_flop_dealt());
        assert!(b.is_turn_dealt());
        assert!(!b.is_river_dealt());
    
        let b = Board::from_str("Ks3hQd2c5d").unwrap();
        assert!(b.is_flop_dealt());
        assert!(b.is_turn_dealt());
        assert!(b.is_river_dealt());
    }
}