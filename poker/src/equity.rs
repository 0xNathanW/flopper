
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
            tie_pct[i] = self.ties[i] as f64 / self.total as f64 * 100.0 / 2.0;
        }
        tie_pct
    }
}


