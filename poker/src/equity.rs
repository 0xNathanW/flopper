
// Results of player i at index i.
#[derive(Debug, Clone)]
pub struct EquityResults {
    pub wins:   Vec<f32>,
    pub ties:   Vec<f32>,
    pub total:  f32,
}

impl EquityResults {

    pub fn new(num_players: usize) -> Self {
        Self {
            wins:   vec![0.0; num_players],
            ties:   vec![0.0; num_players],
            total:  0.0,
        }
    }

    pub fn equities(&self) -> Vec<f32> {
        let mut equities = vec![0.0; self.wins.len()];
        for i in 0..self.wins.len() {
            equities[i] = ((self.wins[i] + self.ties[i] / 2.0) / self.total) * 100.0;
        }
        equities
    }

    pub fn win_pct(&self) -> Vec<f32> {
        let mut win_pct = vec![0.0; self.wins.len()];
        for i in 0..self.wins.len() {
            win_pct[i] = self.wins[i] / self.total * 100.0;
        }
        win_pct
    }

    pub fn tie_pct(&self) -> Vec<f32> {
        let mut tie_pct = vec![0.0; self.wins.len()];
        for i in 0..self.wins.len() {
            tie_pct[i] = self.ties[i] / self.total * 100.0;
        }
        tie_pct
    }
}


