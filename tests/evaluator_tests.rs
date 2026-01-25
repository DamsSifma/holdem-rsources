use holdem_rsources::core::hand_rank::HandCategory;
use holdem_rsources::core::{Card, Hand, HandEvaluator, LookupEvaluator, Suit, Value};

fn make_card(value: Value, suit: Suit) -> Card {
    Card::new(value, suit)
}

fn cards_to_hand(cards: &[Card]) -> Hand {
    Hand::from_cards(cards)
}

#[test]
fn test_high_card() {
    let evaluator = LookupEvaluator::new();
    let hand = cards_to_hand(&[
        make_card(Value::Ace, Suit::Spades),
        make_card(Value::King, Suit::Hearts),
        make_card(Value::Queen, Suit::Diamonds),
        make_card(Value::Jack, Suit::Clubs),
        make_card(Value::Nine, Suit::Spades),
    ]);

    let ranking = evaluator.evaluate(&hand);
    assert_eq!(ranking.category(), HandCategory::HighCard);
}

#[test]
fn test_pair() {
    let evaluator = LookupEvaluator::new();
    let hand = cards_to_hand(&[
        make_card(Value::Ace, Suit::Spades),
        make_card(Value::Ace, Suit::Hearts),
        make_card(Value::King, Suit::Diamonds),
        make_card(Value::Queen, Suit::Clubs),
        make_card(Value::Jack, Suit::Spades),
    ]);

    let ranking = evaluator.evaluate(&hand);
    assert_eq!(ranking.category(), HandCategory::OnePair);
}

#[test]
fn test_two_pair() {
    let evaluator = LookupEvaluator::new();
    let hand = cards_to_hand(&[
        make_card(Value::Ace, Suit::Spades),
        make_card(Value::Ace, Suit::Hearts),
        make_card(Value::King, Suit::Diamonds),
        make_card(Value::King, Suit::Clubs),
        make_card(Value::Queen, Suit::Spades),
    ]);

    let ranking = evaluator.evaluate(&hand);
    assert_eq!(ranking.category(), HandCategory::TwoPair);
}

#[test]
fn test_three_of_a_kind() {
    let evaluator = LookupEvaluator::new();
    let hand = cards_to_hand(&[
        make_card(Value::Ace, Suit::Spades),
        make_card(Value::Ace, Suit::Hearts),
        make_card(Value::Ace, Suit::Diamonds),
        make_card(Value::King, Suit::Clubs),
        make_card(Value::Queen, Suit::Spades),
    ]);

    let ranking = evaluator.evaluate(&hand);
    assert_eq!(ranking.category(), HandCategory::ThreeOfAKind);
}

#[test]
fn test_straight() {
    let evaluator = LookupEvaluator::new();
    let hand = cards_to_hand(&[
        make_card(Value::Ten, Suit::Spades),
        make_card(Value::Jack, Suit::Hearts),
        make_card(Value::Queen, Suit::Diamonds),
        make_card(Value::King, Suit::Clubs),
        make_card(Value::Ace, Suit::Spades),
    ]);

    let ranking = evaluator.evaluate(&hand);
    assert_eq!(ranking.category(), HandCategory::Straight);
}

#[test]
fn test_wheel_straight() {
    let evaluator = LookupEvaluator::new();
    let hand = cards_to_hand(&[
        make_card(Value::Ace, Suit::Spades),
        make_card(Value::Two, Suit::Hearts),
        make_card(Value::Three, Suit::Diamonds),
        make_card(Value::Four, Suit::Clubs),
        make_card(Value::Five, Suit::Spades),
    ]);

    let ranking = evaluator.evaluate(&hand);
    assert_eq!(ranking.category(), HandCategory::Straight);
}

#[test]
fn test_flush() {
    let evaluator = LookupEvaluator::new();
    let hand = cards_to_hand(&[
        make_card(Value::Ace, Suit::Spades),
        make_card(Value::King, Suit::Spades),
        make_card(Value::Queen, Suit::Spades),
        make_card(Value::Jack, Suit::Spades),
        make_card(Value::Nine, Suit::Spades),
    ]);

    let ranking = evaluator.evaluate(&hand);
    assert_eq!(ranking.category(), HandCategory::Flush);
}

#[test]
fn test_full_house() {
    let evaluator = LookupEvaluator::new();
    let hand = cards_to_hand(&[
        make_card(Value::Ace, Suit::Spades),
        make_card(Value::Ace, Suit::Hearts),
        make_card(Value::Ace, Suit::Diamonds),
        make_card(Value::King, Suit::Clubs),
        make_card(Value::King, Suit::Spades),
    ]);

    let ranking = evaluator.evaluate(&hand);
    assert_eq!(ranking.category(), HandCategory::FullHouse);
}

#[test]
fn test_four_of_a_kind() {
    let evaluator = LookupEvaluator::new();
    let hand = cards_to_hand(&[
        make_card(Value::Ace, Suit::Spades),
        make_card(Value::Ace, Suit::Hearts),
        make_card(Value::Ace, Suit::Diamonds),
        make_card(Value::Ace, Suit::Clubs),
        make_card(Value::King, Suit::Spades),
    ]);

    let ranking = evaluator.evaluate(&hand);
    assert_eq!(ranking.category(), HandCategory::FourOfAKind);
}

#[test]
fn test_straight_flush() {
    let evaluator = LookupEvaluator::new();
    let hand = cards_to_hand(&[
        make_card(Value::Nine, Suit::Spades),
        make_card(Value::Ten, Suit::Spades),
        make_card(Value::Jack, Suit::Spades),
        make_card(Value::Queen, Suit::Spades),
        make_card(Value::King, Suit::Spades),
    ]);

    let ranking = evaluator.evaluate(&hand);
    assert_eq!(ranking.category(), HandCategory::StraightFlush);
}

#[test]
fn test_royal_flush() {
    let evaluator = LookupEvaluator::new();
    let hand = cards_to_hand(&[
        make_card(Value::Ten, Suit::Spades),
        make_card(Value::Jack, Suit::Spades),
        make_card(Value::Queen, Suit::Spades),
        make_card(Value::King, Suit::Spades),
        make_card(Value::Ace, Suit::Spades),
    ]);

    let ranking = evaluator.evaluate(&hand);
    assert_eq!(ranking.category(), HandCategory::StraightFlush);
}

#[test]
fn test_hand_comparison() {
    let evaluator = LookupEvaluator::new();

    let pair = cards_to_hand(&[
        make_card(Value::Ace, Suit::Spades),
        make_card(Value::Ace, Suit::Hearts),
        make_card(Value::King, Suit::Diamonds),
        make_card(Value::Queen, Suit::Clubs),
        make_card(Value::Jack, Suit::Spades),
    ]);

    let two_pair = cards_to_hand(&[
        make_card(Value::King, Suit::Spades),
        make_card(Value::King, Suit::Hearts),
        make_card(Value::Queen, Suit::Diamonds),
        make_card(Value::Queen, Suit::Clubs),
        make_card(Value::Jack, Suit::Spades),
    ]);

    let pair_rank = evaluator.evaluate(&pair);
    let two_pair_rank = evaluator.evaluate(&two_pair);

    assert!(two_pair_rank > pair_rank);
}
