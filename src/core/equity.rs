use super::card::Card;
use super::card_set::CardSet;
use super::hand::{Hand, HoleCards};
use super::evaluator::{HandEvaluator, LookupEvaluator};
use rand::seq::SliceRandom;

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

pub struct EquityCalculator {
    evaluator: LookupEvaluator,
}

impl EquityCalculator {
    pub fn new() -> Self {
        Self {
            evaluator: LookupEvaluator::new(),
        }
    }

    fn all_cards() -> Vec<Card> {
        use crate::core::{Value, Suit};
        let values = [
            Value::Two, Value::Three, Value::Four, Value::Five,
            Value::Six, Value::Seven, Value::Eight, Value::Nine,
            Value::Ten, Value::Jack, Value::Queen, Value::King, Value::Ace,
        ];
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

        let mut cards = Vec::with_capacity(52);
        for &value in &values {
            for &suit in &suits {
                cards.push(Card::new(value, suit));
            }
        }
        cards
    }

    /// # Arguments
    /// * `hole1` - Hole cards of player 1
    /// * `hole2` - Hole cards of player 2
    /// * `board` - Cards already on the board
    pub fn calculate_exact(
        &self,
        hole1: &HoleCards,
        hole2: &HoleCards,
        board: &[Card],
    ) -> EquityResult {
        let mut dead_cards = CardSet::new();
        dead_cards.insert(hole1.high());
        dead_cards.insert(hole1.low());
        dead_cards.insert(hole2.high());
        dead_cards.insert(hole2.low());
        for card in board {
            dead_cards.insert(*card);
        }

        let available_cards: Vec<Card> = Self::all_cards()
            .into_iter()
            .filter(|c| !dead_cards.contains(*c))
            .collect();

        let cards_needed = 5 - board.len();

        let mut p1_wins = 0usize;
        let mut p2_wins = 0usize;
        let mut ties = 0usize;
        let mut total = 0usize;

        self.enumerate_combinations(
            &available_cards,
            cards_needed,
            board,
            hole1,
            hole2,
            &mut p1_wins,
            &mut p2_wins,
            &mut ties,
            &mut total,
        );

        EquityResult {
            player1_equity: (p1_wins as f64 + ties as f64 / 2.0) / total as f64,
            player2_equity: (p2_wins as f64 + ties as f64 / 2.0) / total as f64,
            tie_equity: ties as f64 / total as f64,
            simulations: total,
        }
    }

    /// # Arguments
    /// * `hole1` - Hole cards of player 1
    /// * `hole2` - Hole cards of player 2
    /// * `board` - Cards already on the board
    /// * `iterations` - Number of Monte Carlo simulations to run (10000+ recommended)
    pub fn calculate_monte_carlo(
        &self,
        hole1: &HoleCards,
        hole2: &HoleCards,
        board: &[Card],
        iterations: usize,
    ) -> EquityResult {
        let mut dead_cards = CardSet::new();
        dead_cards.insert(hole1.high());
        dead_cards.insert(hole1.low());
        dead_cards.insert(hole2.high());
        dead_cards.insert(hole2.low());
        for card in board {
            dead_cards.insert(*card);
        }

        let mut available_cards: Vec<Card> = Self::all_cards()
            .into_iter()
            .filter(|c| !dead_cards.contains(*c))
            .collect();

        let cards_needed = 5 - board.len();
        let mut rng = rand::rng();

        let mut p1_wins = 0usize;
        let mut p2_wins = 0usize;
        let mut ties = 0usize;

        for _ in 0..iterations {
            available_cards.shuffle(&mut rng);
            let mut full_board = board.to_vec();
            full_board.extend_from_slice(&available_cards[..cards_needed]);

            let hand1 = self.build_hand(hole1, &full_board);
            let hand2 = self.build_hand(hole2, &full_board);

            let rank1 = self.evaluator.evaluate(&hand1);
            let rank2 = self.evaluator.evaluate(&hand2);

            match rank1.cmp(&rank2) {
                std::cmp::Ordering::Greater => p1_wins += 1,
                std::cmp::Ordering::Less => p2_wins += 1,
                std::cmp::Ordering::Equal => ties += 1,
            }
        }

        EquityResult {
            player1_equity: (p1_wins as f64 + ties as f64 / 2.0) / iterations as f64,
            player2_equity: (p2_wins as f64 + ties as f64 / 2.0) / iterations as f64,
            tie_equity: ties as f64 / iterations as f64,
            simulations: iterations,
        }
    }

    fn build_hand(&self, hole: &HoleCards, board: &[Card]) -> Hand {
        let mut hand = Hand::new();
        hand.add(hole.high());
        hand.add(hole.low());
        for card in board {
            hand.add(*card);
        }
        hand
    }

    fn enumerate_combinations(
        &self,
        available: &[Card],
        needed: usize,
        board: &[Card],
        hole1: &HoleCards,
        hole2: &HoleCards,
        p1_wins: &mut usize,
        p2_wins: &mut usize,
        ties: &mut usize,
        total: &mut usize,
    ) {
        let mut indices = vec![0; needed];
        self.enumerate_helper(
            available,
            &mut indices,
            0,
            0,
            board,
            hole1,
            hole2,
            p1_wins,
            p2_wins,
            ties,
            total,
        );
    }

    fn enumerate_helper(
        &self,
        available: &[Card],
        indices: &mut [usize],
        depth: usize,
        start: usize,
        board: &[Card],
        hole1: &HoleCards,
        hole2: &HoleCards,
        p1_wins: &mut usize,
        p2_wins: &mut usize,
        ties: &mut usize,
        total: &mut usize,
    ) {
        if depth == indices.len() {
            let mut full_board = board.to_vec();
            for &idx in indices.iter() {
                full_board.push(available[idx]);
            }

            let hand1 = self.build_hand(hole1, &full_board);
            let hand2 = self.build_hand(hole2, &full_board);

            let rank1 = self.evaluator.evaluate(&hand1);
            let rank2 = self.evaluator.evaluate(&hand2);

            match rank1.cmp(&rank2) {
                std::cmp::Ordering::Greater => *p1_wins += 1,
                std::cmp::Ordering::Less => *p2_wins += 1,
                std::cmp::Ordering::Equal => *ties += 1,
            }
            *total += 1;
            return;
        }

        for i in start..available.len() {
            indices[depth] = i;
            self.enumerate_helper(
                available,
                indices,
                depth + 1,
                i + 1,
                board,
                hole1,
                hole2,
                p1_wins,
                p2_wins,
                ties,
                total,
            );
        }
    }
}

impl Default for EquityCalculator {
    fn default() -> Self {
        Self::new()
    }
}



