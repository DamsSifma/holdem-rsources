#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Debug, Copy, Hash)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Value {
    pub fn from_char(c: char) -> Option<Self> {
        Self::try_from(c).ok()
    }

    pub fn to_char(self) -> char {
        char::from(self)
    }

    pub fn all_values() -> &'static [Value] {
        &VALUES
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

const VALUES: [Value; 13] = [
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

const SUITS: [Suit; 4] = [
    Suit::Clubs,
    Suit::Diamonds,
    Suit::Hearts,
    Suit::Spades,
];
impl TryFrom<char> for Value {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(()),
        }
    }
}

impl From<Value> for u8 {
    fn from(value: Value) -> Self {
        match value {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            Value::Ace => 14,
        }
    }
}

impl From<Value> for char {
    fn from(value: Value) -> Self {
        match value {
            Value::Two => '2',
            Value::Three => '3',
            Value::Four => '4',
            Value::Five => '5',
            Value::Six => '6',
            Value::Seven => '7',
            Value::Eight => '8',
            Value::Nine => '9',
            Value::Ten => 'T',
            Value::Jack => 'J',
            Value::Queen => 'Q',
            Value::King => 'K',
            Value::Ace => 'A',
        }
    }
}

impl TryFrom<char> for Suit {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'h' => Ok(Self::Hearts),
            'd' => Ok(Self::Diamonds),
            'c' => Ok(Self::Clubs),
            's' => Ok(Self::Spades),
            _ => Err(()),
        }
    }
}

impl From<Suit> for char {
    fn from(suit: Suit) -> Self {
        match suit {
            Suit::Hearts => 'h',
            Suit::Diamonds => 'd',
            Suit::Clubs => 'c',
            Suit::Spades => 's',
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Copy, Hash)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Self { value, suit }
    }


    pub fn index(&self) -> u8 {
        let suit_idx = match self.suit {
            Suit::Clubs => 0,
            Suit::Diamonds => 1,
            Suit::Hearts => 2,
            Suit::Spades => 3,
        };
        let rank_idx = u8::from(self.value) - 2; // 0-12
        rank_idx * 4 + suit_idx
    }

    pub fn from_index(index: u8) -> Option<Self> {
        if index >= 52 {
            return None;
        }
        let value = VALUES.get((index / 4) as usize).copied()?;
        let suit = SUITS[(index % 4) as usize];
        Some(Self { value, suit })
    }
}

impl TryFrom<&str> for Card {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut chars = s.chars();
        let value = Value::try_from(chars.next().ok_or(())?)?;
        let suit = Suit::try_from(chars.next().ok_or(())?)?;
        Ok(Self { value, suit })
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", char::from(self.value), char::from(self.suit))
    }
}
