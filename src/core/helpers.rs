use super::card::{Card, SUITS, VALUES};
use super::hand::{Hand, HoleCards};

pub fn all_cards() -> Vec<Card> {
    let mut cards = Vec::with_capacity(52);
    for &value in &VALUES {
        for &suit in &SUITS {
            cards.push(Card::new(value, suit));
        }
    }
    cards
}

pub fn build_hand(hole: &HoleCards, board: &[Card]) -> Hand {
    let mut hand = Hand::new();
    hand.add(hole.high());
    hand.add(hole.low());
    for card in board {
        hand.add(*card);
    }
    hand
}
