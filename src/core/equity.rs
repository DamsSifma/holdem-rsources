use super::card::Card;
use super::card_set::CardSet;
use super::evaluator::{HandEvaluator, LookupEvaluator};
use super::hand::{Hand, HoleCards};
use super::range::Range;
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

struct EnumerationContext<'a> {
    board: &'a [Card],
    hole1: &'a HoleCards,
    hole2: &'a HoleCards,
    p1_wins: &'a mut usize,
    p2_wins: &'a mut usize,
    ties: &'a mut usize,
    total: &'a mut usize,
}

impl EquityCalculator {
    pub fn new() -> Self {
        Self {
            evaluator: LookupEvaluator::new(),
        }
    }

    fn all_cards() -> Vec<Card> {
        use crate::core::{Suit, Value};
        let values = [
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
            Value::Ace,
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

        let mut indices = vec![0; cards_needed];
        let mut ctx = EnumerationContext {
            board,
            hole1,
            hole2,
            p1_wins: &mut p1_wins,
            p2_wins: &mut p2_wins,
            ties: &mut ties,
            total: &mut total,
        };
        self.enumerate_helper(&available_cards, &mut indices, 0, 0, &mut ctx);

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

    fn enumerate_helper(
        &self,
        available: &[Card],
        indices: &mut [usize],
        depth: usize,
        start: usize,
        ctx: &mut EnumerationContext,
    ) {
        if depth == indices.len() {
            let mut full_board = ctx.board.to_vec();
            for &idx in indices.iter() {
                full_board.push(available[idx]);
            }

            let hand1 = self.build_hand(ctx.hole1, &full_board);
            let hand2 = self.build_hand(ctx.hole2, &full_board);

            let rank1 = self.evaluator.evaluate(&hand1);
            let rank2 = self.evaluator.evaluate(&hand2);

            match rank1.cmp(&rank2) {
                std::cmp::Ordering::Greater => *ctx.p1_wins += 1,
                std::cmp::Ordering::Less => *ctx.p2_wins += 1,
                std::cmp::Ordering::Equal => *ctx.ties += 1,
            }
            *ctx.total += 1;
            return;
        }

        for i in start..available.len() {
            indices[depth] = i;
            self.enumerate_helper(available, indices, depth + 1, i + 1, ctx);
        }
    }

    /// Calcule l'équité d'une range contre une main spécifique
    /// # Arguments
    /// * `range` - Range du joueur 1
    /// * `hole2` - Main du joueur 2
    /// * `board` - Cards already on the board
    /// * `iterations` - Number of Monte Carlo simulations per combo
    pub fn calculate_range_vs_hand(
        &self,
        range: &Range,
        hole2: &HoleCards,
        board: &[Card],
        iterations_per_combo: usize,
    ) -> RangeEquityResult {
        let mut dead_cards = CardSet::from_cards(&[hole2.high(), hole2.low()]);
        for card in board {
            dead_cards.insert(*card);
        }

        let combos = range.to_hole_cards(Some(dead_cards));

        if combos.is_empty() {
            return RangeEquityResult {
                range_equity: 0.0,
                opponent_equity: 1.0,
                tie_equity: 0.0,
                combos_evaluated: 0,
                total_simulations: 0,
            };
        }

        let mut total_range_wins = 0.0;
        let mut total_opponent_wins = 0.0;
        let mut total_ties = 0.0;
        let total_simulations = combos.len() * iterations_per_combo;

        for hole1 in &combos {
            let result = self.calculate_monte_carlo(hole1, hole2, board, iterations_per_combo);
            total_range_wins += result.player1_equity;
            total_opponent_wins += result.player2_equity;
            total_ties += result.tie_equity;
        }

        let num_combos = combos.len() as f64;
        RangeEquityResult {
            range_equity: total_range_wins / num_combos,
            opponent_equity: total_opponent_wins / num_combos,
            tie_equity: total_ties / num_combos,
            combos_evaluated: combos.len(),
            total_simulations,
        }
    }

    /// Calcule l'équité range vs range
    /// # Arguments
    /// * `range1` - Range du joueur 1
    /// * `range2` - Range du joueur 2
    /// * `board` - Cards already on the board
    /// * `iterations` - Number of Monte Carlo simulations per matchup
    pub fn calculate_range_vs_range(
        &self,
        range1: &Range,
        range2: &Range,
        board: &[Card],
        iterations_per_matchup: usize,
    ) -> RangeEquityResult {
        let mut board_cards = CardSet::new();
        for card in board {
            board_cards.insert(*card);
        }

        let combos1 = range1.to_hole_cards(Some(board_cards));
        let combos2 = range2.to_hole_cards(Some(board_cards));

        if combos1.is_empty() || combos2.is_empty() {
            return RangeEquityResult {
                range_equity: 0.0,
                opponent_equity: 0.0,
                tie_equity: 0.0,
                combos_evaluated: 0,
                total_simulations: 0,
            };
        }

        let mut total_range1_wins = 0.0;
        let mut total_range2_wins = 0.0;
        let mut total_ties = 0.0;
        let mut matchups = 0usize;

        for hole1 in &combos1 {
            for hole2 in &combos2 {
                // Vérifier qu'il n'y a pas de cartes en commun
                if hole1.high() == hole2.high()
                    || hole1.high() == hole2.low()
                    || hole1.low() == hole2.high()
                    || hole1.low() == hole2.low()
                {
                    continue;
                }

                let result =
                    self.calculate_monte_carlo(hole1, hole2, board, iterations_per_matchup);
                total_range1_wins += result.player1_equity;
                total_range2_wins += result.player2_equity;
                total_ties += result.tie_equity;
                matchups += 1;
            }
        }

        if matchups == 0 {
            return RangeEquityResult {
                range_equity: 0.0,
                opponent_equity: 0.0,
                tie_equity: 0.0,
                combos_evaluated: 0,
                total_simulations: 0,
            };
        }

        let num_matchups = matchups as f64;
        RangeEquityResult {
            range_equity: total_range1_wins / num_matchups,
            opponent_equity: total_range2_wins / num_matchups,
            tie_equity: total_ties / num_matchups,
            combos_evaluated: matchups,
            total_simulations: matchups * iterations_per_matchup,
        }
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

impl Default for EquityCalculator {
    fn default() -> Self {
        Self::new()
    }
}
