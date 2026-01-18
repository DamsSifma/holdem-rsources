use holdem_rsources::core::{Card, HoleCards, Value, Suit, EquityCalculator};

#[macro_use]
mod test_utils;
use test_utils::{TOLERANCE, STRICT_TOLERANCE};

#[test]
fn test_equity_aa_vs_kk_preflop() {
    let calc = EquityCalculator::new();

    let aa = HoleCards::new(
        Card::new(Value::Ace, Suit::Spades),
        Card::new(Value::Ace, Suit::Hearts),
    );
    let kk = HoleCards::new(
        Card::new(Value::King, Suit::Clubs),
        Card::new(Value::King, Suit::Diamonds),
    );

    let result = calc.calculate_monte_carlo(&aa, &kk, &[], 50000);

    println!("AA vs KK preflop:");
    println!("  AA: {:.2}%", result.player1_percent());
    println!("  KK: {:.2}%", result.player2_percent());
    println!("  Tie: {:.2}%", result.tie_percent());

    // Assert based on known poker odds
    assert_within_tolerance!(result.player1_percent(), 82.4, TOLERANCE);
    assert_within_tolerance!(result.player2_percent(), 17.6, TOLERANCE);
}

#[test]
fn test_equity_ako_vs_ajs_preflop() {
    let calc = EquityCalculator::new();

    let ako = HoleCards::new(
        Card::new(Value::Ace, Suit::Spades),
        Card::new(Value::King, Suit::Hearts),
    );

    let ajs = HoleCards::new(
        Card::new(Value::Ace, Suit::Clubs),
        Card::new(Value::Jack, Suit::Clubs),
    );

    let result = calc.calculate_monte_carlo(&ako, &ajs, &[], 50000);

    println!("\nAKo (AsKh) vs AJs (AcJc) preflop:");
    println!("  AKo: {:.2}%", result.player1_percent());
    println!("  AJs: {:.2}%", result.player2_percent());
    println!("  Tie: {:.2}%", result.tie_percent());

    assert_within_tolerance!(result.player1_percent(), 69.8, TOLERANCE);
    assert_within_tolerance!(result.player2_percent(), 30.2, TOLERANCE);
}

#[test]
fn test_equity_ako_vs_ajs_different_suits() {
    let calc = EquityCalculator::new();

    let ako = HoleCards::new(
        Card::new(Value::Ace, Suit::Hearts),
        Card::new(Value::King, Suit::Diamonds),
    );

    let ajs = HoleCards::new(
        Card::new(Value::Ace, Suit::Spades),
        Card::new(Value::Jack, Suit::Spades),
    );

    let result = calc.calculate_monte_carlo(&ako, &ajs, &[], 50000);

    println!("\nAKo (AhKd) vs AJs (AsJs) preflop:");
    println!("  AKo: {:.2}%", result.player1_percent());
    println!("  AJs: {:.2}%", result.player2_percent());
    println!("  Tie: {:.2}%", result.tie_percent());

    assert_within_tolerance!(result.player1_percent(), 69.8, TOLERANCE);
    assert_within_tolerance!(result.player2_percent(), 30.2, TOLERANCE);
}

#[test]
fn test_equity_with_flop() {
    let calc = EquityCalculator::new();

    let hand1 = HoleCards::new(
        Card::new(Value::Ace, Suit::Spades),
        Card::new(Value::King, Suit::Spades),
    );
    let hand2 = HoleCards::new(
        Card::new(Value::Queen, Suit::Hearts),
        Card::new(Value::Queen, Suit::Diamonds),
    );

    let flop = vec![
        Card::new(Value::Queen, Suit::Clubs),
        Card::new(Value::Seven, Suit::Spades),
        Card::new(Value::Two, Suit::Spades),
    ];

    let result = calc.calculate_monte_carlo(&hand1, &hand2, &flop, 50000);

    println!("\nAKs vs QQ avec flop Qc7s2s:");
    println!("  AKs: {:.2}%", result.player1_percent());
    println!("  QQ: {:.2}%", result.player2_percent());
    println!("  Tie: {:.2}%", result.tie_percent());

    assert_within_tolerance!(result.player1_percent(), 25.5, TOLERANCE);
    assert_within_tolerance!(result.player2_percent(), 74.5, TOLERANCE);
}

#[test]
fn test_exact_equity_postflop() {
    let calc = EquityCalculator::new();

    let hand1 = HoleCards::new(
        Card::new(Value::Ace, Suit::Spades),
        Card::new(Value::King, Suit::Spades),
    );
    let hand2 = HoleCards::new(
        Card::new(Value::Queen, Suit::Hearts),
        Card::new(Value::Queen, Suit::Diamonds),
    );

    let board = vec![
        Card::new(Value::Queen, Suit::Clubs),
        Card::new(Value::Seven, Suit::Spades),
        Card::new(Value::Two, Suit::Spades),
        Card::new(Value::Three, Suit::Hearts),
    ];

    let result = calc.calculate_exact(&hand1, &hand2, &board);

    println!("\nCalcul exact - AKs vs QQ avec turn Qc7s2s3h:");
    println!("  AKs: {:.2}%", result.player1_percent());
    println!("  QQ: {:.2}%", result.player2_percent());
    println!("  Simulations: {}", result.simulations);

    assert_eq!(result.simulations, 44);

    assert_within_tolerance!(result.player1_percent(), 15.91, STRICT_TOLERANCE);
    assert_within_tolerance!(result.player2_percent(), 84.09, STRICT_TOLERANCE);
}

