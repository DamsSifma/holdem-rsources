#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum HandCategory {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfAKind = 7,
    StraightFlush = 8,
}

impl HandCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::HighCard => "High Card",
            Self::OnePair => "One Pair",
            Self::TwoPair => "Two Pair",
            Self::ThreeOfAKind => "Three of a Kind",
            Self::Straight => "Straight",
            Self::Flush => "Flush",
            Self::FullHouse => "Full House",
            Self::FourOfAKind => "Four of a Kind",
            Self::StraightFlush => "Straight Flush",
        }
    }
}

impl std::fmt::Display for HandCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// - Bits 24-27: Catégorie de main (0-8)
/// - Bits 0-23: Kickers et rangs pour départager
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HandRanking {
    score: u32,
}

impl HandRanking {
    pub const MIN: HandRanking = HandRanking { score: 0 };

    pub const MAX: HandRanking = HandRanking { score: u32::MAX };

    pub fn from_score(score: u32) -> Self {
        Self { score }
    }

    pub fn new(category: HandCategory, kickers: u32) -> Self {
        debug_assert!(kickers < (1 << 24), "Kickers overflow");
        let score = ((category as u32) << 24) | kickers;
        Self { score }
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn category(&self) -> HandCategory {
        match self.score >> 24 {
            0 => HandCategory::HighCard,
            1 => HandCategory::OnePair,
            2 => HandCategory::TwoPair,
            3 => HandCategory::ThreeOfAKind,
            4 => HandCategory::Straight,
            5 => HandCategory::Flush,
            6 => HandCategory::FullHouse,
            7 => HandCategory::FourOfAKind,
            8 => HandCategory::StraightFlush,
            _ => HandCategory::HighCard,
        }
    }

    pub fn is_royal_flush(&self) -> bool {
        // Le kicker le plus haut pour une quinte flush est 0xC (12 = Ace high straight)
        // donc on prend les 4 bits de poids faible et on vérifie qu'ils sont égaux à 12
        self.category() == HandCategory::StraightFlush && (self.score & 0xF) == 12
    }

    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl Default for HandRanking {
    fn default() -> Self {
        Self::MIN
    }
}

impl std::fmt::Display for HandRanking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_royal_flush() {
            write!(f, "Royal Flush")
        } else {
            write!(f, "{}", self.category())
        }
    }
}

pub fn encode_kickers(ranks: &[u8]) -> u32 {
    let mut result = 0u32;
    for (i, &rank) in ranks.iter().take(5).enumerate() {
        result |= (rank as u32) << (16 - i * 4);
    }
    result
}

impl HandRanking {
    pub fn high_card(kickers: &[u8]) -> Self {
        Self::new(HandCategory::HighCard, encode_kickers(kickers))
    }
    pub fn one_pair(pair_rank: u8, kickers: &[u8]) -> Self {
        let mut ranks = vec![pair_rank];
        ranks.extend(kickers.iter().take(3));
        Self::new(HandCategory::OnePair, encode_kickers(&ranks))
    }
    pub fn two_pair(high_pair: u8, low_pair: u8, kicker: u8) -> Self {
        let ranks = [high_pair, low_pair, kicker];
        Self::new(HandCategory::TwoPair, encode_kickers(&ranks))
    }
    pub fn three_of_a_kind(trips_rank: u8, kickers: &[u8]) -> Self {
        let mut ranks = vec![trips_rank];
        ranks.extend(kickers.iter().take(2));
        Self::new(HandCategory::ThreeOfAKind, encode_kickers(&ranks))
    }
    pub fn straight(high_card: u8) -> Self {
        Self::new(HandCategory::Straight, high_card as u32)
    }
    pub fn flush(kickers: &[u8]) -> Self {
        Self::new(HandCategory::Flush, encode_kickers(kickers))
    }
    pub fn full_house(trips_rank: u8, pair_rank: u8) -> Self {
        let ranks = [trips_rank, pair_rank];
        Self::new(HandCategory::FullHouse, encode_kickers(&ranks))
    }
    pub fn four_of_a_kind(quads_rank: u8, kicker: u8) -> Self {
        let ranks = [quads_rank, kicker];
        Self::new(HandCategory::FourOfAKind, encode_kickers(&ranks))
    }

    pub fn straight_flush(high_card: u8) -> Self {
        Self::new(HandCategory::StraightFlush, high_card as u32)
    }

    pub fn royal_flush() -> Self {
        Self::straight_flush(12)
    }
}

