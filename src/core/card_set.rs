use super::card::Card;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct CardSet(pub u64);

impl CardSet {
    pub const EMPTY: CardSet = CardSet(0);
    pub const FULL_DECK: CardSet = CardSet((1u64 << 52) - 1);

    pub const fn new() -> Self {
        Self(0)
    }

    pub fn from_card(card: Card) -> Self {
        Self(1u64 << card.index())
    }

    pub fn from_cards(cards: &[Card]) -> Self {
        let mut set = Self::new();
        for card in cards {
            set.insert(*card);
        }
        set
    }

    pub fn insert(&mut self, card: Card) {
        self.0 |= 1u64 << card.index();
    }

    pub fn remove(&mut self, card: Card) {
        self.0 &= !(1u64 << card.index());
    }

    pub fn contains(&self, card: Card) -> bool {
        (self.0 & (1u64 << card.index())) != 0
    }

    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    pub fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    pub fn overlaps(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    pub fn iter(&self) -> CardSetIter {
        CardSetIter(self.0)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl FromIterator<Card> for CardSet {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut set = CardSet::new();
        for card in iter {
            set.insert(card);
        }
        set
    }
}

impl IntoIterator for CardSet {
    type Item = Card;
    type IntoIter = CardSetIter;

    fn into_iter(self) -> Self::IntoIter {
        CardSetIter(self.0)
    }
}

pub struct CardSetIter(u64);

impl Iterator for CardSetIter {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let idx = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1; // Efface le bit le plus bas
        Card::from_index(idx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.0.count_ones() as usize;
        (count, Some(count))
    }
}

impl ExactSizeIterator for CardSetIter {}

impl std::fmt::Display for CardSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards: Vec<String> = self.iter().map(|c| c.to_string()).collect();
        write!(f, "[{}]", cards.join(" "))
    }
}
