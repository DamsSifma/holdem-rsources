use super::card::{Card, Value, Suit};
use super::hand::HoleCards;
use super::card_set::CardSet;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    hands: HashSet<HandPattern>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum HandPattern {
    Pair(Value),
    /// Mains paires >= à un certain pattern (ex: "KK+")
    PairPlus(Value),
    Suited(Value, Value),
    SuitedPlus(Value, Value),
    Offsuit(Value, Value),
    OffsuitPlus(Value, Value),
    Any(Value, Value),
    AnyPlus(Value, Value),
}

impl Range {
    pub fn new() -> Self {
        Self {
            hands: HashSet::new(),
        }
    }

    pub fn from_str(s: &str) -> Result<Self, RangeParseError> {
        let mut range = Range::new();

        for part in s.split(',') {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }

            let pattern = HandPattern::parse(trimmed)?;
            range.hands.insert(pattern);
        }

        Ok(range)
    }

    /// Convertit la range en une liste de HoleCards, en tenant compte des cartes mortes
    pub fn to_hole_cards(&self, dead_cards: Option<CardSet>) -> Vec<HoleCards> {
        let mut result = Vec::new();
        let dead = dead_cards.unwrap_or(CardSet::new());

        for pattern in &self.hands {
            result.extend(pattern.to_hole_cards(&dead));
        }

        result.sort_by_key(|h| (h.high().index(), h.low().index()));
        result.dedup();

        result
    }

    pub fn combo_count(&self, dead_cards: Option<CardSet>) -> usize {
        self.to_hole_cards(dead_cards).len()
    }

    pub fn combo_breakdown(&self, dead_cards: Option<CardSet>) -> ComboBreakdown {
        let hole_cards = self.to_hole_cards(dead_cards);
        let mut pairs = 0;
        let mut suited = 0;
        let mut offsuit = 0;

        for hc in hole_cards {
            if hc.is_pair() {
                pairs += 1;
            } else if hc.is_suited() {
                suited += 1;
            } else {
                offsuit += 1;
            }
        }

        ComboBreakdown {
            pairs,
            suited,
            offsuit,
            total: pairs + suited + offsuit,
        }
    }

    pub fn add_pattern(&mut self, pattern: &str) -> Result<(), RangeParseError> {
        let p = HandPattern::parse(pattern)?;
        self.hands.insert(p);
        Ok(())
    }

    pub fn contains(&self, hole_cards: &HoleCards) -> bool {
        self.to_hole_cards(None).contains(hole_cards)
    }
}

impl Default for Range {
    fn default() -> Self {
        Self::new()
    }
}

/// Breakdown des combos par type
#[derive(Clone, Debug, PartialEq)]
pub struct ComboBreakdown {
    pub pairs: usize,
    pub suited: usize,
    pub offsuit: usize,
    pub total: usize,
}

impl fmt::Display for ComboBreakdown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Total: {} (Pairs: {}, Suited: {}, Offsuit: {})",
            self.total, self.pairs, self.suited, self.offsuit
        )
    }
}

impl HandPattern {
    fn parse(s: &str) -> Result<Self, RangeParseError> {
        let s = s.trim();

        let (base, is_plus) = if s.ends_with('+') {
            (&s[..s.len() - 1], true)
        } else {
            (s, false)
        };

        if base.len() < 2 || base.len() > 3 {
            return Err(RangeParseError::InvalidFormat(s.to_string()));
        }

        // Collect characters once to avoid multiple iterations
        let chars: Vec<char> = base.chars().collect();

        let v1 = Value::from_char(chars[0])
            .ok_or_else(|| RangeParseError::InvalidValue(chars[0]))?;
        let v2 = Value::from_char(chars[1])
            .ok_or_else(|| RangeParseError::InvalidValue(chars[1]))?;

        let (high, low) = if v1 >= v2 {
            (v1, v2)
        } else {
            (v2, v1)
        };

        if high == low {
            return Ok(if is_plus {
                HandPattern::PairPlus(high)
            } else {
                HandPattern::Pair(high)
            });
        }

        let suited_marker = chars.len() == 3;
        if suited_marker {
            let marker = chars[2];
            match marker {
                's' => Ok(if is_plus {
                    HandPattern::SuitedPlus(high, low)
                } else {
                    HandPattern::Suited(high, low)
                }),
                'o' => Ok(if is_plus {
                    HandPattern::OffsuitPlus(high, low)
                } else {
                    HandPattern::Offsuit(high, low)
                }),
                _ => Err(RangeParseError::InvalidSuitMarker(marker)),
            }
        } else {
            // Pas de marqueur s/o = les deux
            Ok(if is_plus {
                HandPattern::AnyPlus(high, low)
            } else {
                HandPattern::Any(high, low)
            })
        }
    }

    fn to_hole_cards(&self, dead_cards: &CardSet) -> Vec<HoleCards> {
        match self {
            HandPattern::Pair(v) => generate_pair_combos(*v, dead_cards),
            HandPattern::PairPlus(v) => {
                let mut result = Vec::new();
                for &val in Value::all_values() {
                    if val >= *v {
                        result.extend(generate_pair_combos(val, dead_cards));
                    }
                }
                result
            }
            HandPattern::Suited(h, l) => generate_suited_combos(*h, *l, dead_cards),
            HandPattern::SuitedPlus(h, l) => {
                let mut result = Vec::new();
                // Générer toutes les combos suited avec high fixe et low >= l
                for &low_val in Value::all_values() {
                    if low_val >= *l && low_val < *h {
                        result.extend(generate_suited_combos(*h, low_val, dead_cards));
                    }
                }
                result
            }
            HandPattern::Offsuit(h, l) => generate_offsuit_combos(*h, *l, dead_cards),
            HandPattern::OffsuitPlus(h, l) => {
                let mut result = Vec::new();
                for &low_val in Value::all_values() {
                    if low_val >= *l && low_val < *h {
                        result.extend(generate_offsuit_combos(*h, low_val, dead_cards));
                    }
                }
                result
            }
            HandPattern::Any(h, l) => {
                let mut result = generate_suited_combos(*h, *l, dead_cards);
                result.extend(generate_offsuit_combos(*h, *l, dead_cards));
                result
            }
            HandPattern::AnyPlus(h, l) => {
                let mut result = Vec::new();
                for &low_val in Value::all_values() {
                    if low_val >= *l && low_val < *h {
                        result.extend(generate_suited_combos(*h, low_val, dead_cards));
                        result.extend(generate_offsuit_combos(*h, low_val, dead_cards));
                    }
                }
                result
            }
        }
    }
}

fn generate_pair_combos(value: Value, dead_cards: &CardSet) -> Vec<HoleCards> {
    let mut result = Vec::new();
    let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

    for (i, &s1) in suits.iter().enumerate() {
        for &s2 in suits.iter().skip(i + 1) {
            let c1 = Card::new(value, s1);
            let c2 = Card::new(value, s2);

            if !dead_cards.contains(c1) && !dead_cards.contains(c2) {
                result.push(HoleCards::new(c1, c2));
            }
        }
    }

    result
}

fn generate_suited_combos(high: Value, low: Value, dead_cards: &CardSet) -> Vec<HoleCards> {
    let mut result = Vec::new();
    let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

    for &suit in &suits {
        let c1 = Card::new(high, suit);
        let c2 = Card::new(low, suit);

        if !dead_cards.contains(c1) && !dead_cards.contains(c2) {
            result.push(HoleCards::new(c1, c2));
        }
    }

    result
}

fn generate_offsuit_combos(high: Value, low: Value, dead_cards: &CardSet) -> Vec<HoleCards> {
    let mut result = Vec::new();
    let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

    for &s1 in &suits {
        for &s2 in &suits {
            if s1 == s2 {
                continue; // Skip suited combos
            }

            let c1 = Card::new(high, s1);
            let c2 = Card::new(low, s2);

            if !dead_cards.contains(c1) && !dead_cards.contains(c2) {
                result.push(HoleCards::new(c1, c2));
            }
        }
    }

    result
}

#[derive(Debug, Clone, PartialEq)]
pub enum RangeParseError {
    InvalidFormat(String),
    InvalidValue(char),
    InvalidSuitMarker(char),
}

impl fmt::Display for RangeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RangeParseError::InvalidFormat(s) => write!(f, "Invalid range format: {}", s),
            RangeParseError::InvalidValue(c) => write!(f, "Invalid card value: {}", c),
            RangeParseError::InvalidSuitMarker(c) => write!(f, "Invalid suit marker: {}", c),
        }
    }
}

impl std::error::Error for RangeParseError {}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let patterns: Vec<String> = self.hands.iter()
            .map(|p| p.to_string())
            .collect();
        write!(f, "{}", patterns.join(", "))
    }
}

impl fmt::Display for HandPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandPattern::Pair(v) => write!(f, "{}{}", v.to_char(), v.to_char()),
            HandPattern::PairPlus(v) => write!(f, "{}{}+", v.to_char(), v.to_char()),
            HandPattern::Suited(h, l) => write!(f, "{}{}s", h.to_char(), l.to_char()),
            HandPattern::SuitedPlus(h, l) => write!(f, "{}{}s+", h.to_char(), l.to_char()),
            HandPattern::Offsuit(h, l) => write!(f, "{}{}o", h.to_char(), l.to_char()),
            HandPattern::OffsuitPlus(h, l) => write!(f, "{}{}o+", h.to_char(), l.to_char()),
            HandPattern::Any(h, l) => write!(f, "{}{}", h.to_char(), l.to_char()),
            HandPattern::AnyPlus(h, l) => write!(f, "{}{}+", h.to_char(), l.to_char()),
        }
    }
}

