use prettytable::{Table, Row, Cell};
use crate::prelude::*;

mod enumerate;
mod monte_carlo;
#[cfg(test)]
mod tests;

pub use enumerate::equity_enumerate;
pub use monte_carlo::equity_monte_carlo;

pub trait ProgressReporter: Send + Sync {
    fn board_complete(&self);
}

pub struct EquityParams<'a> {
    pub ranges:   Vec<Range>,
    pub board:    Board,
    pub lookup:   &'a [i32],
    pub reporter: Option<&'a dyn ProgressReporter>,
}

pub enum EquityMethod {
    Enumerate,
    MonteCarlo(Option<usize>),
}

#[derive(Debug, Clone)]
pub struct EquityResults {
    pub wins:   Vec<f64>,
    pub ties:   Vec<f64>,
    pub total:  f64,
}

impl EquityResults {

    pub fn new(num_players: usize) -> Self {
        Self {
            wins:   vec![0.0; num_players],
            ties:   vec![0.0; num_players],
            total:  0.0,
        }
    }

    pub fn combine(results: Vec<EquityResults>) -> Self {
        let mut total = Self::new(results[0].wins.len());
        for result in results {
            total.wins.iter_mut().zip(result.wins.iter()).for_each(|(a, b)| *a += b);
            total.ties.iter_mut().zip(result.ties.iter()).for_each(|(a, b)| *a += b);
            total.total += result.total;
        }
        total
    }

    pub fn equities(&self) -> Vec<f64> {
        let mut equities = vec![0.0; self.wins.len()];
        for i in 0..self.wins.len() {
            equities[i] = ((self.wins[i] + self.ties[i] / 2.0) / self.total) * 100.0;
        }
        equities
    }

    pub fn win_pct(&self) -> Vec<f64> {
        let mut win_pct = vec![0.0; self.wins.len()];
        for i in 0..self.wins.len() {
            win_pct[i] = self.wins[i] / self.total * 100.0;
        }
        win_pct
    }

    pub fn tie_pct(&self) -> Vec<f64> {
        let mut tie_pct = vec![0.0; self.wins.len()];
        for i in 0..self.wins.len() {
            tie_pct[i] = self.ties[i] / self.total * 100.0;
        }
        tie_pct
    }

    pub fn print(&self, range_str: &[String]) {
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Range"),
            Cell::new("Equity"),
            Cell::new("Win %"),
            Cell::new("Tie %"),
        ]));

        let equities = self.equities();
        let win_pct = self.wins.iter().map(|w| *w / self.total * 100.0).collect::<Vec<f64>>();
        let tie_pct = self.ties.iter().map(|t| *t / self.total * 100.0).collect::<Vec<f64>>();

        for i in 0..range_str.len() {
            table.add_row(Row::new(vec![
                Cell::new(&range_str[i]),
                Cell::new(&format!("{:.2}%", equities[i])),
                Cell::new(&format!("{:.2}%", win_pct[i])),
                Cell::new(&format!("{:.2}%", tie_pct[i])),
            ]));
        }

        table.printstd();
    }
}

pub fn remove_dead(ranges: Vec<Range>, board: &[Card]) -> Result<(Vec<Vec<Hand>>, Deck)> {
    
    let mut deck = Deck::new();
    let mut removed = 0_u64;

    board.iter().for_each(|card| {
        deck.remove(card);
        removed |= 1 << card.0;
    });
        
    let hands = ranges
        .iter()
        .map(|range| {
            let hands = range.hand_combos(removed);
            hands
        }).collect();

    Ok((hands, deck))
}