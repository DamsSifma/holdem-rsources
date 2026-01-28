use holdem_rsources::core::*;
use std::str::FromStr;

fn main() {
    let calc = EquityCalculator::new();

    println!("=== Multi-way Equity Calculator ===\n");

    // Example 1: 3-way pot preflop
    println!("Example 1: Classic 3-way pot (AA vs KK vs QQ)");
    println!("{}", "-".repeat(50));

    let hole_cards = vec![
        HoleCards::from_str("AsAh").unwrap(),
        HoleCards::from_str("KcKd").unwrap(),
        HoleCards::from_str("QhQs").unwrap(),
    ];

    let result = calc.calculate_multiway_monte_carlo(&hole_cards, &[], 50000);

    println!("AA equity: {:.2}%", result.player_percent(0));
    println!("KK equity: {:.2}%", result.player_percent(1));
    println!("QQ equity: {:.2}%", result.player_percent(2));
    println!("Simulations: {}\n", result.simulations);

    // Example 2: 4-way pot
    println!("Example 2: 4-way pot (AA vs AKs vs QQ vs 76s)");
    println!("{}", "-".repeat(50));

    let hole_cards_4 = vec![
        HoleCards::from_str("AsAh").unwrap(),
        HoleCards::from_str("AdKd").unwrap(),
        HoleCards::from_str("QcQs").unwrap(),
        HoleCards::from_str("7h6h").unwrap(),
    ];

    let result_4 = calc.calculate_multiway_monte_carlo(&hole_cards_4, &[], 50000);

    for i in 0..4 {
        println!(
            "Player {} equity: {:.2}%",
            i + 1,
            result_4.player_percent(i)
        );
    }
    println!();

    // Example 3: Full table (9 players)
    println!("Example 3: Full table - 9 players preflop");
    println!("{}", "-".repeat(50));

    let hole_cards_9 = vec![
        HoleCards::from_str("AsAh").unwrap(),
        HoleCards::from_str("KcKd").unwrap(),
        HoleCards::from_str("QhQs").unwrap(),
        HoleCards::from_str("JdJc").unwrap(),
        HoleCards::from_str("ThTd").unwrap(),
        HoleCards::from_str("9s9h").unwrap(),
        HoleCards::from_str("AcKh").unwrap(),
        HoleCards::from_str("AdQd").unwrap(),
        HoleCards::from_str("7c7d").unwrap(),
    ];

    let result_9 = calc.calculate_multiway_monte_carlo(&hole_cards_9, &[], 10000);

    for (i, equity) in result_9.player_equities.iter().enumerate() {
        println!("Player {}: {:.2}%", i + 1, equity * 100.0);
    }
}
