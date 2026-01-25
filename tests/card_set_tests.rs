use holdem_rsources::core::{Card, CardSet, Suit, Value};

#[test]
fn test_card_set_basic() {
    let mut set = CardSet::new();
    assert!(set.is_empty());
    assert_eq!(set.count(), 0);

    let card = Card::new(Value::Ace, Suit::Spades);
    set.insert(card);
    assert!(set.contains(card));
    assert_eq!(set.count(), 1);

    set.remove(card);
    assert!(!set.contains(card));
    assert!(set.is_empty());
}

#[test]
fn test_card_set_operations() {
    let card1 = Card::new(Value::Ace, Suit::Spades);
    let card2 = Card::new(Value::King, Suit::Hearts);
    let card3 = Card::new(Value::Queen, Suit::Diamonds);

    let set1 = CardSet::from_cards(&[card1, card2]);
    let set2 = CardSet::from_cards(&[card2, card3]);

    assert_eq!(set1.union(set2).count(), 3);
    assert_eq!(set1.intersection(set2).count(), 1);
    assert!(set1.intersection(set2).contains(card2));
    assert_eq!(set1.difference(set2).count(), 1);
    assert!(set1.overlaps(set2));
}

#[test]
fn test_full_deck() {
    assert_eq!(CardSet::FULL_DECK.count(), 52);
}

#[test]
fn test_iterator() {
    let cards = vec![
        Card::new(Value::Two, Suit::Clubs),
        Card::new(Value::Ace, Suit::Spades),
    ];
    let set = CardSet::from_cards(&cards);
    let collected: Vec<Card> = set.iter().collect();
    assert_eq!(collected.len(), 2);
}
