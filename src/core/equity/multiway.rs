use super::results::MultiPlayerEquityResult;
use crate::core::card::Card;
use crate::core::card_set::CardSet;
use crate::core::evaluator::{HandEvaluator, LookupEvaluator};
use crate::core::hand::HoleCards;
use crate::core::helpers;
use rand::seq::SliceRandom;
use rayon::prelude::*;

pub trait MultiwayEquityCalculator {
    /// Calculate equity for multi-way pots (2-9 players) using parallel Monte Carlo simulation
    ///
    /// # Arguments
    /// * `hole_cards` - Slice of hole cards for each player (2-9 players)
    /// * `board` - Cards already on the board (0-5 cards)
    /// * `iterations` - Number of Monte Carlo simulations to run
    ///
    /// # Panics
    /// Panics if number of players is < 2 or > 9
    fn calculate_multiway_monte_carlo(
        &self,
        hole_cards: &[HoleCards],
        board: &[Card],
        iterations: usize,
    ) -> MultiPlayerEquityResult;

    /// Calculate equity for multi-way pots (sequential version for benchmarking)
    ///
    /// This is a non-parallel version of `calculate_multiway_monte_carlo` for performance
    /// comparison and benchmarking purposes. It uses the same algorithm but runs
    /// simulations sequentially in a single thread.
    ///
    /// # Arguments
    /// * `hole_cards` - Slice of hole cards for each player (2-9 players)
    /// * `board` - Cards already on the board (0-5 cards)
    /// * `iterations` - Number of Monte Carlo simulations to run
    ///
    /// # Panics
    /// Panics if number of players is < 2 or > 9
    ///
    /// # Usage
    /// This method should primarily be used for benchmarking to measure the speedup
    /// gained from parallel execution. For production use, prefer `calculate_multiway_monte_carlo`.
    fn calculate_multiway_monte_carlo_sequential(
        &self,
        hole_cards: &[HoleCards],
        board: &[Card],
        iterations: usize,
    ) -> MultiPlayerEquityResult;

    /// Calculate exact equity for multi-way pots on the river
    ///
    /// # Arguments
    /// * `hole_cards` - Slice of hole cards for each player (2-9 players)
    /// * `board` - Exactly 5 board cards
    ///
    /// # Panics
    /// Panics if board doesn't have exactly 5 cards or number of players is invalid
    fn calculate_multiway_exact(
        &self,
        hole_cards: &[HoleCards],
        board: &[Card],
    ) -> MultiPlayerEquityResult;
}

/// Internal helper for building dead cards set
fn build_dead_cards(hole_cards: &[HoleCards], board: &[Card]) -> CardSet {
    let mut dead_cards = CardSet::new();
    for hole in hole_cards {
        dead_cards.insert(hole.high());
        dead_cards.insert(hole.low());
    }
    for card in board {
        dead_cards.insert(*card);
    }
    dead_cards
}

pub struct MultiwayCalculator<'a> {
    evaluator: &'a LookupEvaluator,
}

impl<'a> MultiwayCalculator<'a> {
    pub fn new(evaluator: &'a LookupEvaluator) -> Self {
        Self { evaluator }
    }

    pub fn calculate_parallel(
        &self,
        hole_cards: &[HoleCards],
        board: &[Card],
        iterations: usize,
    ) -> MultiPlayerEquityResult {
        let num_players = hole_cards.len();
        assert!(
            (2..=9).contains(&num_players),
            "Number of players must be between 2 and 9"
        );

        let dead_cards = build_dead_cards(hole_cards, board);
        let available_cards: Vec<Card> = helpers::all_cards()
            .into_iter()
            .filter(|c| !dead_cards.contains(*c))
            .collect();

        let cards_needed = 5 - board.len();

        // Parallel Monte Carlo simulation
        let results: Vec<_> = (0..iterations)
            .into_par_iter()
            .map(|_| {
                let mut rng = rand::rng();
                let mut shuffled = available_cards.clone();
                shuffled.shuffle(&mut rng);

                let mut full_board = board.to_vec();
                full_board.extend_from_slice(&shuffled[..cards_needed]);

                // Evaluate all hands
                let rankings: Vec<_> = hole_cards
                    .iter()
                    .map(|hole| {
                        let hand = helpers::build_hand(hole, &full_board);
                        self.evaluator.evaluate(&hand)
                    })
                    .collect();

                let best_rank = rankings.iter().max().copied().unwrap();

                let winners: Vec<usize> = rankings
                    .iter()
                    .enumerate()
                    .filter_map(
                        |(idx, &rank)| {
                            if rank == best_rank { Some(idx) } else { None }
                        },
                    )
                    .collect();

                winners
            })
            .collect();

        Self::aggregate_results(results, num_players, iterations)
    }

    pub fn calculate_sequential(
        &self,
        hole_cards: &[HoleCards],
        board: &[Card],
        iterations: usize,
    ) -> MultiPlayerEquityResult {
        let num_players = hole_cards.len();
        assert!(
            (2..=9).contains(&num_players),
            "Number of players must be between 2 and 9"
        );

        let dead_cards = build_dead_cards(hole_cards, board);
        let mut available_cards: Vec<Card> = helpers::all_cards()
            .into_iter()
            .filter(|c| !dead_cards.contains(*c))
            .collect();

        let cards_needed = 5 - board.len();
        let mut rng = rand::rng();

        let mut wins = vec![0usize; num_players];
        let mut ties = 0usize;

        for _ in 0..iterations {
            available_cards.shuffle(&mut rng);
            let mut full_board = board.to_vec();
            full_board.extend_from_slice(&available_cards[..cards_needed]);

            let rankings: Vec<_> = hole_cards
                .iter()
                .map(|hole| {
                    let hand = helpers::build_hand(hole, &full_board);
                    self.evaluator.evaluate(&hand)
                })
                .collect();

            let best_rank = rankings.iter().max().copied().unwrap();

            let winners: Vec<usize> = rankings
                .iter()
                .enumerate()
                .filter_map(
                    |(idx, &rank)| {
                        if rank == best_rank { Some(idx) } else { None }
                    },
                )
                .collect();

            if winners.len() == 1 {
                wins[winners[0]] += 1;
            } else {
                ties += 1;
            }
        }

        Self::build_result(wins, ties, num_players, iterations)
    }

    pub fn calculate_exact(
        &self,
        hole_cards: &[HoleCards],
        board: &[Card],
    ) -> MultiPlayerEquityResult {
        let num_players = hole_cards.len();
        assert!(
            (2..=9).contains(&num_players),
            "Number of players must be between 2 and 9"
        );
        assert_eq!(board.len(), 5, "Board must have exactly 5 cards");

        let rankings: Vec<_> = hole_cards
            .iter()
            .map(|hole| {
                let hand = helpers::build_hand(hole, board);
                self.evaluator.evaluate(&hand)
            })
            .collect();

        let best_rank = rankings.iter().max().copied().unwrap();

        let winners: Vec<usize> = rankings
            .iter()
            .enumerate()
            .filter_map(
                |(idx, &rank)| {
                    if rank == best_rank { Some(idx) } else { None }
                },
            )
            .collect();

        let mut wins = vec![0usize; num_players];
        let ties = if winners.len() > 1 { 1 } else { 0 };

        if winners.len() == 1 {
            wins[winners[0]] = 1;
        }

        let player_equities: Vec<f64> = wins
            .iter()
            .enumerate()
            .map(|(idx, &w)| {
                if w > 0 {
                    1.0
                } else if winners.contains(&idx) {
                    1.0 / winners.len() as f64
                } else {
                    0.0
                }
            })
            .collect();

        MultiPlayerEquityResult {
            player_equities,
            wins,
            ties,
            simulations: 1,
        }
    }

    fn aggregate_results(
        results: Vec<Vec<usize>>,
        num_players: usize,
        iterations: usize,
    ) -> MultiPlayerEquityResult {
        let mut wins = vec![0usize; num_players];
        let mut ties = 0usize;

        for winners in results {
            if winners.len() == 1 {
                wins[winners[0]] += 1;
            } else {
                ties += 1;
            }
        }

        Self::build_result(wins, ties, num_players, iterations)
    }

    fn build_result(
        wins: Vec<usize>,
        ties: usize,
        num_players: usize,
        iterations: usize,
    ) -> MultiPlayerEquityResult {
        let tie_equity_per_player = if ties > 0 {
            ties as f64 / iterations as f64 / num_players as f64
        } else {
            0.0
        };

        let player_equities: Vec<f64> = wins
            .iter()
            .map(|&w| (w as f64 / iterations as f64) + tie_equity_per_player)
            .collect();

        MultiPlayerEquityResult {
            player_equities,
            wins,
            ties,
            simulations: iterations,
        }
    }
}
