use super::card_set::CardSet;
use super::hand::{COMBO_COUNT, HoleCards};
use super::range::{Range, RangeParseError};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct WeightedRange {
    weights: [f32; COMBO_COUNT],
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeightedRangeParseError {
    Range(RangeParseError),
    InvalidWeight(String),
}

impl WeightedRange {
    pub fn empty() -> Self {
        Self {
            weights: [0.0; COMBO_COUNT],
        }
    }

    pub fn uniform(weight: f32) -> Self {
        Self {
            weights: [weight.clamp(0.0, 1.0); COMBO_COUNT],
        }
    }

    pub fn from_range(range: &Range) -> Self {
        let mut weighted = Self::empty();

        for hole_cards in range.to_hole_cards(None) {
            weighted.set_weight_for_hole_cards(hole_cards, 1.0);
        }

        weighted
    }

    pub fn parse(s: &str) -> Result<Self, WeightedRangeParseError> {
        let mut weighted = Self::empty();

        for part in s.split(',') {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }

            let (pattern_str, weight) = match trimmed.rsplit_once(':') {
                Some((pattern, weight_str)) => {
                    let weight = weight_str.trim().parse::<f32>().map_err(|_| {
                        WeightedRangeParseError::InvalidWeight(weight_str.trim().to_string())
                    })?;

                    if !(0.0..=1.0).contains(&weight) {
                        return Err(WeightedRangeParseError::InvalidWeight(
                            weight_str.trim().to_string(),
                        ));
                    }

                    (pattern.trim(), weight)
                }
                None => (trimmed, 1.0),
            };

            let range = Range::parse(pattern_str).map_err(WeightedRangeParseError::Range)?;
            for hole_cards in range.to_hole_cards(None) {
                weighted.set_weight_for_hole_cards(hole_cards, weight);
            }
        }

        Ok(weighted)
    }

    pub fn weight(&self, combo_index: u16) -> f32 {
        self.weights[usize::from(combo_index)]
    }

    pub fn set_weight(&mut self, combo_index: u16, weight: f32) {
        self.weights[usize::from(combo_index)] = weight.clamp(0.0, 1.0);
    }

    pub fn weight_for_hole_cards(&self, hole_cards: HoleCards) -> f32 {
        self.weight(hole_cards.combo_index())
    }

    pub fn set_weight_for_hole_cards(&mut self, hole_cards: HoleCards, weight: f32) {
        self.set_weight(hole_cards.combo_index(), weight);
    }

    pub fn weights(&self) -> &[f32; COMBO_COUNT] {
        &self.weights
    }

    pub fn weights_mut(&mut self) -> &mut [f32; COMBO_COUNT] {
        &mut self.weights
    }

    pub fn iter(&self) -> impl Iterator<Item = (u16, f32)> + '_ {
        self.weights
            .iter()
            .enumerate()
            .map(|(i, &w)| (u16::try_from(i).expect("combo index always fits in u16"), w))
    }

    pub fn iter_nonzero(&self) -> impl Iterator<Item = (u16, f32)> + '_ {
        self.iter().filter(|(_, w)| *w > 0.0)
    }

    pub fn with_dead_cards(&self, dead_cards: CardSet) -> Self {
        let mut result = self.clone();
        result.apply_dead_cards(dead_cards);
        result
    }

    pub fn apply_dead_cards(&mut self, dead_cards: CardSet) {
        if dead_cards.is_empty() {
            return;
        }

        for hole_cards in HoleCards::all_combos().iter() {
            if dead_cards.contains(hole_cards.high()) || dead_cards.contains(hole_cards.low()) {
                self.set_weight_for_hole_cards(*hole_cards, 0.0);
            }
        }
    }

    pub fn total_weight(&self) -> f32 {
        self.weights.iter().sum()
    }

    pub fn num_combos(&self) -> usize {
        self.weights.iter().filter(|w| **w > 0.0).count()
    }

    pub fn normalize(&mut self) {
        let total = self.total_weight();
        if total == 0.0 {
            return;
        }

        for weight in self.weights.iter_mut() {
            *weight /= total;
        }
    }

    pub fn to_hole_cards(&self, dead_cards: Option<CardSet>) -> Vec<(HoleCards, f32)> {
        let mut result = Vec::new();
        let dead = dead_cards.unwrap_or_default();

        for (combo_idx, weight) in self.iter_nonzero() {
            let hole_cards =
                HoleCards::from_combo_index(combo_idx).expect("combo index from iterator is valid");

            if dead.contains(hole_cards.high()) || dead.contains(hole_cards.low()) {
                continue;
            }

            result.push((hole_cards, weight));
        }

        result
    }
}

impl Default for WeightedRange {
    fn default() -> Self {
        Self::empty()
    }
}

impl FromStr for WeightedRange {
    type Err = WeightedRangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl fmt::Display for WeightedRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for (idx, weight) in self.iter_nonzero() {
            let hole_cards = HoleCards::from_combo_index(idx).expect("valid combo index");
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}:{:.3}", hole_cards, weight)?;
            first = false;
        }
        Ok(())
    }
}

impl fmt::Display for WeightedRangeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeightedRangeParseError::Range(err) => write!(f, "{}", err),
            WeightedRangeParseError::InvalidWeight(weight) => {
                write!(f, "Invalid weight: {}", weight)
            }
        }
    }
}

impl std::error::Error for WeightedRangeParseError {}
