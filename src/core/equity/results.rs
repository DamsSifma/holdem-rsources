#[derive(Debug, Clone, Copy)]
pub struct EquityResult {
    pub player1_equity: f64,
    pub player2_equity: f64,
    pub tie_equity: f64,
    pub simulations: usize,
}

impl EquityResult {
    pub fn player1_percent(&self) -> f64 {
        self.player1_equity * 100.0
    }

    pub fn player2_percent(&self) -> f64 {
        self.player2_equity * 100.0
    }

    pub fn tie_percent(&self) -> f64 {
        self.tie_equity * 100.0
    }
}

/// Result structure for multi-player (3-9 players) equity calculations
#[derive(Debug, Clone)]
pub struct MultiPlayerEquityResult {
    /// Equity for each player (win equity + share of tie equity)
    pub player_equities: Vec<f64>,
    /// Raw win count for each player
    pub wins: Vec<usize>,
    /// Number of ties
    pub ties: usize,
    /// Total simulations run
    pub simulations: usize,
}

impl MultiPlayerEquityResult {
    pub fn player_percent(&self, player_idx: usize) -> f64 {
        self.player_equities.get(player_idx).copied().unwrap_or(0.0) * 100.0
    }

    pub fn tie_percent(&self) -> f64 {
        (self.ties as f64 / self.simulations as f64) * 100.0
    }

    pub fn num_players(&self) -> usize {
        self.player_equities.len()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RangeEquityResult {
    pub range_equity: f64,
    pub opponent_equity: f64,
    pub tie_equity: f64,
    pub combos_evaluated: usize,
    pub total_simulations: usize,
}

impl RangeEquityResult {
    pub fn range_percent(&self) -> f64 {
        self.range_equity * 100.0
    }

    pub fn opponent_percent(&self) -> f64 {
        self.opponent_equity * 100.0
    }

    pub fn tie_percent(&self) -> f64 {
        self.tie_equity * 100.0
    }
}
