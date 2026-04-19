use holdem_rsources::core::{Card, EquityCalculator, HoleCards, Range, WeightedRange};
use std::str::FromStr;

fn main() {
    println!("=== Weighted Range Examples ===\n");

    example_from_parser();
    println!("\n{}\n", "-".repeat(60));

    example_solver_style_mutation();
    println!("\n{}\n", "-".repeat(60));

    example_weighted_range_vs_range();
}

fn example_from_parser() {
    println!("Example 1: Parse weighted notation");

    let weighted = WeightedRange::from_str("QQ+:1.0, AKo:0.7, AKs:1.0, A5s:0.35").unwrap();

    println!("Active combos: {}", weighted.num_combos());
    println!("Total raw weight: {:.3}", weighted.total_weight());

    let villain = HoleCards::from_str("QdQc").unwrap();
    let result = calc_equity_vs_hand(&weighted, &villain, &[]);

    println!(
        "Vs QdQc preflop -> Hero: {:.2}% | Villain: {:.2}%",
        result.range_percent(),
        result.opponent_percent()
    );
}

fn example_solver_style_mutation() {
    println!("Example 2: Solver-style in-place weight updates");

    let mut strategy = WeightedRange::empty();

    // Start from a binary opening range.
    let open_range = Range::from_str("77+, AJs+, AQo+, KQs").unwrap();
    for combo in open_range.to_hole_cards(None) {
        strategy.set_weight_for_hole_cards(combo, 1.0);
    }

    // Simulate one policy update: reduce AKo frequency, boost A5s bluff frequency.
    for combo in Range::from_str("AKo").unwrap().to_hole_cards(None) {
        strategy.set_weight_for_hole_cards(combo, 0.55);
    }
    for combo in Range::from_str("A5s").unwrap().to_hole_cards(None) {
        strategy.set_weight_for_hole_cards(combo, 0.80);
    }

    // CFR-style normalization for a distribution-like view.
    strategy.normalize();

    println!("Active combos after update: {}", strategy.num_combos());
    println!(
        "Total weight after normalize: {:.3}",
        strategy.total_weight()
    );

    let villain = HoleCards::from_str("KhKc").unwrap();
    let result = calc_equity_vs_hand(&strategy, &villain, &[]);

    println!(
        "Updated strategy vs KhKc -> Hero: {:.2}% | Villain: {:.2}%",
        result.range_percent(),
        result.opponent_percent()
    );
}

fn example_weighted_range_vs_range() {
    println!("Example 3: Weighted range vs weighted range");

    let hero = WeightedRange::from_str("AA:1.0, KK:1.0, AKs:1.0, AKo:0.50, A5s:0.40").unwrap();
    let villain = WeightedRange::from_str("QQ+:1.0, AKs:1.0, AKo:0.75, AQs:0.60").unwrap();

    let flop = vec![
        Card::try_from("Ah").unwrap(),
        Card::try_from("7d").unwrap(),
        Card::try_from("2c").unwrap(),
    ];

    let result =
        EquityCalculator::new().calculate_weighted_range_vs_range(&hero, &villain, &flop, 500);

    println!(
        "On Ah7d2c -> Hero: {:.2}% | Villain: {:.2}% | Tie: {:.2}%",
        result.range_percent(),
        result.opponent_percent(),
        result.tie_percent()
    );
    println!("Matchups evaluated: {}", result.combos_evaluated);
}

fn calc_equity_vs_hand(
    weighted: &WeightedRange,
    villain: &HoleCards,
    board: &[Card],
) -> holdem_rsources::core::RangeEquityResult {
    EquityCalculator::new().calculate_weighted_range_vs_hand(weighted, villain, board, 500)
}
