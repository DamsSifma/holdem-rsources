use holdem_rsources::core::*;
use std::str::FromStr;

#[test]
fn test_multiway_exact_three_players() {
    let calc = EquityCalculator::new();

    // Board: As Ks Qs Js Ts (royal flush on board)
    let board = vec![
        Card::try_from("As").unwrap(),
        Card::try_from("Ks").unwrap(),
        Card::try_from("Qs").unwrap(),
        Card::try_from("Js").unwrap(),
        Card::try_from("Ts").unwrap(),
    ];

    // All players tie with royal flush
    let hole_cards = vec![
        HoleCards::from_str("2h3h").unwrap(),
        HoleCards::from_str("4c5c").unwrap(),
        HoleCards::from_str("6d7d").unwrap(),
    ];

    let result = calc.calculate_multiway_exact(&hole_cards, &board);

    assert_eq!(result.num_players(), 3);
    assert_eq!(result.ties, 1);

    // Each player should have ~33.33% equity
    for i in 0..3 {
        assert!(
            (result.player_percent(i) - 33.33).abs() < 0.1,
            "Player {} equity: {}%",
            i,
            result.player_percent(i)
        );
    }
}

#[test]
fn test_multiway_exact_one_winner() {
    let calc = EquityCalculator::new();

    // Board: Ah Kh Qh 2c 3d
    let board = vec![
        Card::try_from("Ah").unwrap(),
        Card::try_from("Kh").unwrap(),
        Card::try_from("Qh").unwrap(),
        Card::try_from("2c").unwrap(),
        Card::try_from("3d").unwrap(),
    ];

    let hole_cards = vec![
        HoleCards::from_str("JhTh").unwrap(), // Royal flush
        HoleCards::from_str("AsKs").unwrap(), // Two pair
        HoleCards::from_str("QdQc").unwrap(), // Three of a kind
    ];

    let result = calc.calculate_multiway_exact(&hole_cards, &board);

    assert_eq!(result.num_players(), 3);
    assert_eq!(result.ties, 0);
    assert_eq!(result.wins[0], 1);
    assert_eq!(result.wins[1], 0);
    assert_eq!(result.wins[2], 0);

    assert!((result.player_percent(0) - 100.0).abs() < 0.01);
    assert!((result.player_percent(1) - 0.0).abs() < 0.01);
    assert!((result.player_percent(2) - 0.0).abs() < 0.01);
}

#[test]
fn test_multiway_monte_carlo_three_players() {
    let calc = EquityCalculator::new();

    let hole_cards = vec![
        HoleCards::from_str("AsAh").unwrap(), // AA
        HoleCards::from_str("KcKd").unwrap(), // KK
        HoleCards::from_str("QhQs").unwrap(), // QQ
    ];

    let result = calc.calculate_multiway_monte_carlo(&hole_cards, &[], 10000);

    assert_eq!(result.num_players(), 3);
    assert_eq!(result.simulations, 10000);

    // AA should have highest equity (~50-55%)
    // KK should have ~23-28%
    // QQ should have ~18-23%
    println!("AA equity: {}%", result.player_percent(0));
    println!("KK equity: {}%", result.player_percent(1));
    println!("QQ equity: {}%", result.player_percent(2));

    assert!(result.player_percent(0) > 45.0);
    assert!(result.player_percent(1) > 18.0);
    assert!(result.player_percent(2) > 13.0);

    // AA should be ahead
    assert!(result.player_equities[0] > result.player_equities[1]);
    assert!(result.player_equities[1] > result.player_equities[2]);

    // Total equity should sum to ~100%
    let total: f64 = result.player_equities.iter().sum();
    assert!((total - 1.0).abs() < 0.01);
}

#[test]
fn test_multiway_monte_carlo_four_players() {
    let calc = EquityCalculator::new();

    let hole_cards = vec![
        HoleCards::from_str("AsAh").unwrap(), // AA
        HoleCards::from_str("KcKd").unwrap(), // KK
        HoleCards::from_str("QhQs").unwrap(), // QQ
        HoleCards::from_str("JdJc").unwrap(), // JJ
    ];

    let result = calc.calculate_multiway_monte_carlo(&hole_cards, &[], 5000);

    assert_eq!(result.num_players(), 4);

    println!("AA equity: {}%", result.player_percent(0));
    println!("KK equity: {}%", result.player_percent(1));
    println!("QQ equity: {}%", result.player_percent(2));
    println!("JJ equity: {}%", result.player_percent(3));

    // AA should still be ahead
    assert!(result.player_equities[0] > result.player_equities[1]);

    // Total equity should sum to ~100%
    let total: f64 = result.player_equities.iter().sum();
    assert!((total - 1.0).abs() < 0.01);
}

#[test]
fn test_multiway_with_board() {
    let calc = EquityCalculator::new();

    let board = vec![
        Card::try_from("Ah").unwrap(),
        Card::try_from("Kh").unwrap(),
        Card::try_from("Qh").unwrap(),
    ];

    let hole_cards = vec![
        HoleCards::from_str("JhTh").unwrap(), // Royal flush draw
        HoleCards::from_str("AsKs").unwrap(), // Two pair
        HoleCards::from_str("2c3c").unwrap(), // Nothing
    ];

    let result = calc.calculate_multiway_monte_carlo(&hole_cards, &board, 5000);

    assert_eq!(result.num_players(), 3);

    println!("JhTh equity: {}%", result.player_percent(0));
    println!("AsKs equity: {}%", result.player_percent(1));
    println!("2c3c equity: {}%", result.player_percent(2));

    // JhTh has flush draw and straight draw, should have good equity
    assert!(result.player_percent(0) > 30.0);

    // Total equity should sum to ~100%
    let total: f64 = result.player_equities.iter().sum();
    assert!((total - 1.0).abs() < 0.01);
}

#[test]
#[should_panic(expected = "Number of players must be between 2 and 9")]
fn test_multiway_too_few_players() {
    let calc = EquityCalculator::new();
    let hole_cards = vec![HoleCards::from_str("AsAh").unwrap()];
    calc.calculate_multiway_monte_carlo(&hole_cards, &[], 1000);
}

#[test]
#[should_panic(expected = "Number of players must be between 2 and 9")]
fn test_multiway_too_many_players() {
    let calc = EquityCalculator::new();
    let hole_cards = vec![
        HoleCards::from_str("AsAh").unwrap(),
        HoleCards::from_str("KcKd").unwrap(),
        HoleCards::from_str("QhQs").unwrap(),
        HoleCards::from_str("JdJc").unwrap(),
        HoleCards::from_str("ThTd").unwrap(),
        HoleCards::from_str("9s9h").unwrap(),
        HoleCards::from_str("8c8d").unwrap(),
        HoleCards::from_str("7h7s").unwrap(),
        HoleCards::from_str("6d6c").unwrap(),
        HoleCards::from_str("5h5s").unwrap(), // 10 players - too many
    ];
    calc.calculate_multiway_monte_carlo(&hole_cards, &[], 1000);
}

#[test]
fn test_multiway_nine_players_max() {
    let calc = EquityCalculator::new();
    let hole_cards = vec![
        HoleCards::from_str("AsAh").unwrap(),
        HoleCards::from_str("KcKd").unwrap(),
        HoleCards::from_str("QhQs").unwrap(),
        HoleCards::from_str("JdJc").unwrap(),
        HoleCards::from_str("ThTd").unwrap(),
        HoleCards::from_str("9s9h").unwrap(),
        HoleCards::from_str("8c8d").unwrap(),
        HoleCards::from_str("7h7s").unwrap(),
        HoleCards::from_str("6d6c").unwrap(), // 9 players - max allowed
    ];

    let result = calc.calculate_multiway_monte_carlo(&hole_cards, &[], 1000);

    assert_eq!(result.num_players(), 9);

    // AA should still have highest equity
    for i in 1..9 {
        assert!(result.player_equities[0] > result.player_equities[i]);
    }

    // Total equity should sum to ~100%
    let total: f64 = result.player_equities.iter().sum();
    assert!((total - 1.0).abs() < 0.01);
}
