/// Exemples avancés d'utilisation des ranges de poker
///
/// Ce fichier montre différents cas d'usage pratiques des ranges
use holdem_rsources::core::{Card, CardSet, EquityCalculator, Range};
use std::str::FromStr;

fn main() {
    println!("=== Exemples Avancés de Ranges ===\n");

    example_position_ranges();
    println!("\n{}\n", "─".repeat(60));

    example_3bet_ranges();
    println!("\n{}\n", "─".repeat(60));

    example_board_texture_analysis();
    println!("\n{}\n", "─".repeat(60));

    example_range_manipulation();
}

/// Exemple 1: Ranges par position
fn example_position_ranges() {
    println!("📍 Ranges par Position (Cash Game 6-max)");
    println!("{}", "─".repeat(60));

    let positions = vec![
        ("UTG", "77+, ATs+, AJo+, KQs"),
        ("MP", "66+, A9s+, ATo+, KJs+, KQo, QJs"),
        ("CO", "55+, A7s+, A9o+, K9s+, KTo+, QTs+, JTs"),
        (
            "BTN",
            "22+, A2s+, A5o+, K6s+, K9o+, Q8s+, QTo+, J8s+, T8s+, 98s",
        ),
        (
            "SB",
            "22+, A2s+, A7o+, K2s+, K9o+, Q5s+, Q9o+, J7s+, T7s+, 97s+, 87s",
        ),
    ];

    for (position, range_str) in positions {
        let range = Range::from_str(range_str).unwrap();
        let breakdown = range.combo_breakdown(None);

        println!("\n  {}: {}", position, range_str);
        println!("    {}", breakdown);

        // Calculer le VPIP (% de mains jouées)
        let total_combos = 1326.0; // C(52,2)
        let vpip = (breakdown.total as f64 / total_combos) * 100.0;
        println!("    VPIP: {:.1}%", vpip);
    }
}

/// Exemple 2: Ranges de 3bet
fn example_3bet_ranges() {
    println!("🎯 Ranges de 3-bet");
    println!("{}", "─".repeat(60));

    // Range d'ouverture CO
    let co_open = Range::from_str("55+, A7s+, A9o+, K9s+, KTo+, QTs+, JTs").unwrap();

    // Range de 3bet value du BTN
    let btn_3bet_value = Range::from_str("QQ+, AKs, AKo").unwrap();

    // Range de 3bet bluff du BTN
    let btn_3bet_bluff = Range::from_str("A5s, A4s, A3s, A2s").unwrap();

    println!("\n  CO Opening Range:");
    println!("    {}", co_open.combo_breakdown(None));

    println!("\n  BTN 3-bet Value Range: QQ+, AK");
    println!("    {}", btn_3bet_value.combo_breakdown(None));

    println!("\n  BTN 3-bet Bluff Range: A5s-A2s");
    println!("    {}", btn_3bet_bluff.combo_breakdown(None));

    let total_3bet = btn_3bet_value.combo_count(None) + btn_3bet_bluff.combo_count(None);
    let value_ratio = (btn_3bet_value.combo_count(None) as f64 / total_3bet as f64) * 100.0;

    println!(
        "\n  Ratio Value/Bluff: {:.0}% value / {:.0}% bluff",
        value_ratio,
        100.0 - value_ratio
    );
}

/// Exemple 3: Analyse par texture de board
fn example_board_texture_analysis() {
    println!("🎴 Analyse de Range sur Différentes Textures");
    println!("{}", "─".repeat(60));

    // Range d'ouverture preflop
    let preflop_range = Range::from_str("88+, ATs+, AJo+, KQs").unwrap();

    let scenarios = vec![
        ("A♥ K♦ Q♣ (Broadway connecté)", vec!["Ah", "Kd", "Qc"]),
        ("7♥ 7♦ 2♣ (Paire de 7)", vec!["7h", "7d", "2c"]),
        ("9♠ 5♠ 2♠ (Monotone pique)", vec!["9s", "5s", "2s"]),
    ];

    for (description, board_str) in scenarios {
        println!("\n  Board: {}", description);

        let board: Vec<Card> = board_str
            .iter()
            .map(|s| Card::try_from(*s).unwrap())
            .collect();

        let board_cardset = CardSet::from_cards(&board);

        // Combien de combos de la range sont encore possibles
        let possible_combos = preflop_range.to_hole_cards(Some(board_cardset));

        println!(
            "    Combos possibles: {} / {}",
            possible_combos.len(),
            preflop_range.combo_count(None)
        );

        // Analyser les top combos
        let breakdown = Range::from_str("88+, ATs+, AJo+, KQs")
            .unwrap()
            .combo_breakdown(Some(board_cardset));
        println!("    {}", breakdown);
    }
}

/// Exemple 4: Manipulation de ranges
fn example_range_manipulation() {
    println!("🔧 Manipulation et Filtrage de Ranges");
    println!("{}", "─".repeat(60));

    // Range large
    let wide_range =
        Range::from_str("22+, A2s+, A5o+, K2s+, K9o+, Q8s+, QTo+, J8s+, T8s+, 98s, 87s").unwrap();

    println!("\n  Range initiale (Large):");
    println!("    {}", wide_range.combo_breakdown(None));

    // Filtrer seulement les paires
    let pairs_only = Range::from_str("22+").unwrap();
    println!("\n  Paires uniquement:");
    println!("    {}", pairs_only.combo_breakdown(None));

    // Filtrer seulement les suited
    let suited_only = Range::from_str("A2s+, K2s+, Q8s+, J8s+, T8s+, 98s, 87s").unwrap();
    println!("\n  Suited uniquement:");
    println!("    {}", suited_only.combo_breakdown(None));

    // Range après avoir vu des cartes
    println!("\n  Impact des cartes visibles:");
    let range = Range::from_str("AA, KK, QQ").unwrap();
    println!("    Range: AA, KK, QQ");
    println!("    Sans dead cards: {} combos", range.combo_count(None));

    // Si on voit A♥ au flop
    let mut dead = CardSet::new();
    dead.insert(Card::try_from("Ah").unwrap());
    println!(
        "    Avec A♥ au flop: {} combos",
        range.combo_count(Some(dead))
    );

    // Si on voit A♥ K♦ au flop
    dead.insert(Card::try_from("Kd").unwrap());
    println!(
        "    Avec A♥ K♦ au flop: {} combos",
        range.combo_count(Some(dead))
    );

    // Calcul d'équité range vs range avec board
    println!("\n  Équité avec board:");
    let hero = Range::from_str("AK, AQ").unwrap();
    let villain = Range::from_str("JJ, TT").unwrap();

    let board_preflop: Vec<Card> = vec![];
    let board_flop = vec![
        Card::try_from("Ah").unwrap(),
        Card::try_from("Kd").unwrap(),
        Card::try_from("2c").unwrap(),
    ];

    let calculator = EquityCalculator::new();

    let result_preflop = calculator.calculate_range_vs_range(&hero, &villain, &board_preflop, 500);
    println!("    Preflop - Hero (AK, AQ) vs Villain (JJ, TT):");
    println!(
        "      Hero: {:.1}% | Villain: {:.1}%",
        result_preflop.range_percent(),
        result_preflop.opponent_percent()
    );

    let result_flop = calculator.calculate_range_vs_range(&hero, &villain, &board_flop, 500);
    println!("    Flop A♥K♦2♣ - Hero (AK, AQ) vs Villain (JJ, TT):");
    println!(
        "      Hero: {:.1}% | Villain: {:.1}%",
        result_flop.range_percent(),
        result_flop.opponent_percent()
    );
}
