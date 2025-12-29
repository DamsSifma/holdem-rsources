use holdem_rsources::core::{Card, HoleCards, Hand, Value, Suit};

#[test]
fn test_hole_cards() {
    let card1 = Card::new(Value::Ace, Suit::Spades);
    let card2 = Card::new(Value::King, Suit::Spades);
    let hole = HoleCards::new(card1, card2);

    assert!(hole.is_suited());
    assert!(!hole.is_pair());
    assert_eq!(hole.gap(), 1);
    assert_eq!(hole.high().value, Value::Ace);
}

#[test]
fn test_hole_cards_pair() {
    let card1 = Card::new(Value::Queen, Suit::Hearts);
    let card2 = Card::new(Value::Queen, Suit::Diamonds);
    let hole = HoleCards::new(card1, card2);

    assert!(!hole.is_suited());
    assert!(hole.is_pair());
    assert_eq!(hole.gap(), 0);
}

#[test]
fn test_hole_cards_parse() {
    let hole = HoleCards::from_str("AhKs").unwrap();
    assert_eq!(hole.high().value, Value::Ace);
    assert_eq!(hole.low().value, Value::King);
}

#[test]
fn test_hand_basic() {
    let mut hand = Hand::new();
    assert!(hand.is_empty());

    let card = Card::new(Value::Ace, Suit::Spades);
    hand.add(card);
    assert_eq!(hand.len(), 1);
    assert!(hand.contains(card));
}

#[test]
fn test_hand_with_board() {
    let hole = HoleCards::new(
        Card::new(Value::Ace, Suit::Spades),
        Card::new(Value::King, Suit::Spades),
    );
    let hand = Hand::from_card_set(hole.to_card_set());

    let board = vec![
        Card::new(Value::Queen, Suit::Spades),
        Card::new(Value::Jack, Suit::Spades),
        Card::new(Value::Ten, Suit::Spades),
    ];

    let full_hand = hand.with_board(&board);
    assert_eq!(full_hand.len(), 5);
}

