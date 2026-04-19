use super::card::Card;
use super::card_set::CardSet;
use std::str::FromStr;
use std::sync::LazyLock;

pub const COMBO_COUNT: usize = 1326;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HoleCards {
    cards: [Card; 2],
}

impl HoleCards {
    pub fn new(card1: Card, card2: Card) -> Self {
        let cards = if card1 >= card2 {
            [card1, card2]
        } else {
            [card2, card1]
        };
        Self { cards }
    }

    pub fn high(&self) -> Card {
        self.cards[0]
    }

    pub fn low(&self) -> Card {
        self.cards[1]
    }

    pub fn is_suited(&self) -> bool {
        self.cards[0].suit == self.cards[1].suit
    }

    pub fn is_pair(&self) -> bool {
        self.cards[0].value == self.cards[1].value
    }

    pub fn gap(&self) -> u8 {
        let high = u8::from(self.cards[0].value);
        let low = u8::from(self.cards[1].value);
        high - low
    }

    pub fn to_card_set(&self) -> CardSet {
        CardSet::from_cards(&self.cards)
    }

    pub fn cards(&self) -> &[Card; 2] {
        &self.cards
    }

    /// Returns a canonical combo index in [0, 1325].
    ///
    /// Mapping is based on card indices i < j:
    /// index = j * (j - 1) / 2 + i
    pub fn combo_index(&self) -> u16 {
        let a = self.cards[0].index();
        let b = self.cards[1].index();
        let (i, j) = if a < b { (a, b) } else { (b, a) };

        let i = u16::from(i);
        let j = u16::from(j);
        j * (j - 1) / 2 + i
    }

    /// Builds hole cards from a canonical combo index in [0, 1325].
    pub fn from_combo_index(index: u16) -> Option<Self> {
        if usize::from(index) >= COMBO_COUNT {
            return None;
        }

        let mut remaining = index;
        let mut j = 1u16;
        while j < 52 && remaining >= j {
            remaining -= j;
            j += 1;
        }

        if j >= 52 {
            return None;
        }

        let i = remaining;
        let high = Card::from_index(j as u8)?;
        let low = Card::from_index(i as u8)?;
        Some(Self::new(high, low))
    }

    /// Returns all 1326 unique two-card combinations.
    /// The position in this array matches `combo_index()`.
    pub fn all_combos() -> &'static [HoleCards; COMBO_COUNT] {
        static ALL: LazyLock<Box<[HoleCards; COMBO_COUNT]>> = LazyLock::new(|| {
            let mut combos = Vec::with_capacity(COMBO_COUNT);

            for j in 1u8..52 {
                for i in 0u8..j {
                    let high = Card::from_index(j).expect("valid card index");
                    let low = Card::from_index(i).expect("valid card index");
                    combos.push(HoleCards::new(high, low));
                }
            }

            Box::new(
                combos
                    .try_into()
                    .expect("exactly 1326 hole card combinations"),
            )
        });

        ALL.as_ref()
    }

    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.len() == 4 {
            let card1 = Card::try_from(&s[0..2]).ok()?;
            let card2 = Card::try_from(&s[2..4]).ok()?;
            Some(Self::new(card1, card2))
        } else {
            None
        }
    }
}

impl std::fmt::Display for HoleCards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.cards[0], self.cards[1])
    }
}

impl FromStr for HoleCards {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or_else(|| format!("Invalid hole cards format: '{}'", s))
    }
}

impl TryFrom<&str> for HoleCards {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::parse(s).ok_or(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Hand {
    cards: CardSet,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: CardSet::new(),
        }
    }

    pub fn from_card_set(cards: CardSet) -> Self {
        Self { cards }
    }

    pub fn from_cards(cards: &[Card]) -> Self {
        Self {
            cards: CardSet::from_cards(cards),
        }
    }

    pub fn add(&mut self, card: Card) {
        self.cards.insert(card);
    }

    pub fn len(&self) -> usize {
        self.cards.count() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn contains(&self, card: Card) -> bool {
        self.cards.contains(card)
    }

    pub fn card_set(&self) -> CardSet {
        self.cards
    }

    pub fn as_u64(&self) -> u64 {
        self.cards.as_u64()
    }

    /// Combine la main avec le board
    pub fn with_board(&self, board: &[Card]) -> Self {
        let mut combined = self.cards;
        for card in board {
            combined.insert(*card);
        }
        Self { cards: combined }
    }

    pub fn iter(&self) -> impl Iterator<Item = Card> {
        self.cards.iter()
    }

    pub fn parse(s: &str) -> Option<Self> {
        let cards: Vec<Card> = s
            .split_whitespace()
            .filter_map(|card_str| Card::try_from(card_str).ok())
            .collect();

        if cards.is_empty() {
            None
        } else {
            Some(Self::from_cards(&cards))
        }
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or_else(|| format!("Invalid hand format: '{}'", s))
    }
}

impl FromIterator<Card> for Hand {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        Self {
            cards: CardSet::from_iter(iter),
        }
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards: Vec<String> = self.cards.iter().map(|c| c.to_string()).collect();
        write!(f, "{}", cards.join(" "))
    }
}
