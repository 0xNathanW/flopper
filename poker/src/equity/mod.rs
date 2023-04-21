use thiserror::Error;
use crate::{deck::Deck, card::Card};

mod enumerate;
mod monte_carlo;

pub use enumerate::equity_enumerate;

#[derive(Debug, Error)]
pub enum EquityError {    
    #[error("Duplicate card: {0:?}")]
    DuplicateCard(Card),

    #[error("Invalid board size: {0}. Must be either 0, 3, 4 or 5.")]
    InvalidBoardSize(usize),

    #[error("Error loading lookup table: {0}")]
    LookupTableError(#[from] std::io::Error),

    #[error("Dead card on board: {0:?}")]
    DeadBoardCard(Card),
}

// Results of player i at index i.
#[derive(Debug, Clone)]
pub struct EquityResults {
    pub wins:   Vec<usize>,
    pub ties:   Vec<usize>,
    pub total:  usize,
}

impl EquityResults {

    pub fn new(num_players: usize) -> Self {
        Self {
            wins:   vec![0; num_players],
            ties:   vec![0; num_players],
            total:  0,
        }
    }

    pub fn equities(&self) -> Vec<f64> {
        let mut equities = vec![0.0; self.wins.len()];
        for i in 0..self.wins.len() {
            equities[i] = ((self.wins[i] as f64 + self.ties[i] as f64 / 2.0) / self.total as f64) * 100.0;
        }
        equities
    }

    pub fn win_pct(&self) -> Vec<f64> {
        let mut win_pct = vec![0.0; self.wins.len()];
        for i in 0..self.wins.len() {
            win_pct[i] = self.wins[i] as f64 / self.total as f64 * 100.0;
        }
        win_pct
    }

    pub fn tie_pct(&self) -> Vec<f64> {
        let mut tie_pct = vec![0.0; self.wins.len()];
        for i in 0..self.wins.len() {
            tie_pct[i] = self.ties[i] as f64 / self.total as f64 * 100.0;
        }
        tie_pct
    }
}

// Remove combos conflicting with board and dead cards.
fn setup_cards(ranges: &mut Vec<Vec<(usize, usize)>>, board: Vec<Card>) -> Result<(Vec<usize>, Deck<usize>), EquityError> {
    
    let mut deck = Deck::<usize>::new();
    let mut removed = 0_u64;

    let board = board
        .iter()
        .map(|card| {
            let c = card.idx();
            deck.remove_dead(c);
            removed |= 1 << c;
            c
        }).collect();

    for range in ranges.iter_mut() {
        range.retain(|(i, j)| {
            if removed & (1 << i) == 0 && removed & (1 << j) == 0 {
                true
            } else {
                false
            }            
        }) 
    }

    Ok((board, deck))
}

#[inline]
fn check_board(board: &Vec<Card>) -> Result<(), EquityError> {
    if board.len() != 0 && board.len() != 3 && board.len() != 4 && board.len() != 5 {
        return Err(EquityError::InvalidBoardSize(board.len()));
    }

    Ok(())
}

#[cfg(test)]
pub fn assert_results_eq(results: &EquityResults, equities: Vec<u32>) {
    for (i, pct) in equities.iter().enumerate() {
        assert_eq!((results.equities()[i]).round() as u32, *pct);
    }
}
