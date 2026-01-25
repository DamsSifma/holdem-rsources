use holdem_rsources::core::{
    Card, CardSet, EquityCalculator, Hand, HandEvaluator, HoleCards, LookupEvaluator, Range,
};
use std::time::Instant;

fn main() {
    // Code généré par Copilot pour une démo complète des fonctionnalités
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║      HOLDEM-RSOURCES - Poker Hand Evaluation Engine       ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    demo_hand_evaluation();
    println!("\n{}\n", "─".repeat(60));
    demo_hand_comparisons();
    println!("\n{}\n", "─".repeat(60));
    demo_equity_preflop();
    println!("\n{}\n", "─".repeat(60));
    demo_equity_postflop();
    println!("\n{}\n", "─".repeat(60));
    demo_ranges();
    println!("\n{}\n", "─".repeat(60));
    demo_cardset_operations();
}

/// Démo 1: Évaluation de différentes mains de poker
fn demo_hand_evaluation() {
    println!("📊 DEMO 1: Hand Evaluation");
    println!("{}", "─".repeat(60));

    let evaluator = LookupEvaluator::new();

    let hands = vec![
        ("Royal Flush", vec!["Ah", "Kh", "Qh", "Jh", "Th"]),
        ("Straight Flush", vec!["9s", "8s", "7s", "6s", "5s"]),
        ("Four of a Kind", vec!["Ac", "Ad", "Ah", "As", "Kh"]),
        ("Full House", vec!["Qd", "Qh", "Qs", "Jc", "Jd"]),
        ("Flush", vec!["Kd", "Jd", "9d", "6d", "3d"]),
        ("Straight", vec!["Tc", "9h", "8s", "7d", "6c"]),
        ("Three of a Kind", vec!["7h", "7s", "7d", "Ac", "Kh"]),
        ("Two Pair", vec!["Jc", "Jd", "8h", "8s", "Ah"]),
        ("One Pair", vec!["9h", "9c", "Ah", "Kd", "Qc"]),
        ("High Card", vec!["Ah", "Ks", "Qd", "Jc", "9h"]),
    ];

    for (name, cards_str) in hands {
        let cards: Vec<Card> = cards_str
            .iter()
            .map(|s| Card::try_from(*s).unwrap())
            .collect();

        let card_set = CardSet::from_cards(&cards);
        let hand = Hand::from_card_set(card_set);
        let ranking = evaluator.evaluate(&hand);

        println!(
            "  {:<18} → {:>16} (score: {:4})",
            name,
            ranking.to_string(),
            ranking.score()
        );
    }
}

/// Démo 2: Comparaisons de mains
fn demo_hand_comparisons() {
    println!("⚔️  DEMO 2: Hand Comparisons");
    println!("{}", "─".repeat(60));

    let evaluator = LookupEvaluator::new();

    let matchups = vec![
        ("AhAs", "KdKs", "Pocket Aces vs Pocket Kings"),
        ("AhKh", "QsQd", "AKs vs QQ"),
        ("JcTc", "AhQs", "JTs vs AQo"),
        ("7h7s", "AhKd", "77 vs AKo"),
    ];

    for (hand1_str, hand2_str, description) in matchups {
        println!("  {}", description);

        let hand1 = HoleCards::from_str(hand1_str).unwrap();
        let hand2 = HoleCards::from_str(hand2_str).unwrap();

        // Créer un board pour la comparaison
        let board = [
            Card::try_from("9h").unwrap(),
            Card::try_from("5d").unwrap(),
            Card::try_from("2c").unwrap(),
            Card::try_from("Ks").unwrap(),
            Card::try_from("3h").unwrap(),
        ];

        let h1 = Hand::from_card_set(hand1.to_card_set()).with_board(&board);
        let h2 = Hand::from_card_set(hand2.to_card_set()).with_board(&board);

        let rank1 = evaluator.evaluate(&h1);
        let rank2 = evaluator.evaluate(&h2);

        println!(
            "    {} → {:<16} ({})",
            hand1_str,
            rank1.to_string(),
            rank1.score()
        );
        println!(
            "    {} → {:<16} ({})",
            hand2_str,
            rank2.to_string(),
            rank2.score()
        );

        match rank1.cmp(&rank2) {
            std::cmp::Ordering::Greater => println!("    ✅ {} wins!\n", hand1_str),
            std::cmp::Ordering::Less => println!("    ✅ {} wins!\n", hand2_str),
            std::cmp::Ordering::Equal => println!("    🤝 Split pot!\n"),
        }
    }
}

/// Démo 3: Calculs d'équité preflop
fn demo_equity_preflop() {
    println!("📈 DEMO 3: Preflop Equity Calculations");
    println!("{}", "─".repeat(60));

    let matchups = vec![
        ("AhAs", "KdKs", "AA vs KK"),
        ("AhKh", "QdQc", "AKs vs QQ"),
        ("AhKd", "JcTs", "AKo vs JTs"),
        ("7h7s", "AhKd", "77 vs AKo"),
        ("AhQd", "KhJs", "AQo vs KJs"),
    ];

    let calculator = EquityCalculator::new();

    for (hand1_str, hand2_str, description) in matchups {
        let hand1 = HoleCards::from_str(hand1_str).unwrap();
        let hand2 = HoleCards::from_str(hand2_str).unwrap();

        print!("  Computing equity for {}... ", description);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let start = Instant::now();
        let result = calculator.calculate_monte_carlo(&hand1, &hand2, &[], 10000);
        let duration = start.elapsed();

        println!();
        println!(
            "    {} → {:.1}% equity",
            hand1_str,
            result.player1_percent()
        );
        println!(
            "    {} → {:.1}% equity",
            hand2_str,
            result.player2_percent()
        );
        println!(
            "    Tie: {:.1}% | Simulations: {} ({:.0} sim/ms)",
            result.tie_percent(),
            result.simulations,
            result.simulations as f64 / duration.as_millis().max(1) as f64
        );
        println!();
    }
}

/// Démo 4: Calculs d'équité postflop
fn demo_equity_postflop() {
    println!("🎯 DEMO 4: Postflop Equity (with Board)");
    println!("{}", "─".repeat(60));

    let scenarios = vec![
        (
            "AhKh",
            "QdQc",
            vec!["As", "Jh", "Th"],
            "Top pair + flush draw vs overpair",
        ),
        (
            "7h7s",
            "AhKd",
            vec!["Ac", "9d", "2c"],
            "Underpair vs top pair",
        ),
        ("JcTc", "AhQs", vec!["Kh", "Qd", "9s"], "OESD vs top pair"),
    ];

    let calculator = EquityCalculator::new();

    for (hand1_str, hand2_str, board_str, description) in scenarios {
        let hand1 = HoleCards::from_str(hand1_str).unwrap();
        let hand2 = HoleCards::from_str(hand2_str).unwrap();

        let board: Vec<Card> = board_str
            .iter()
            .map(|s| Card::try_from(*s).unwrap())
            .collect();

        let board_display = board_str.join(" ");

        println!("  Scenario: {}", description);
        println!("  Board: {}", board_display);

        let start = Instant::now();
        let result = calculator.calculate_monte_carlo(&hand1, &hand2, &board, 5000);
        let duration = start.elapsed();

        println!(
            "    {} → {:.1}% equity",
            hand1_str,
            result.player1_percent()
        );
        println!(
            "    {} → {:.1}% equity",
            hand2_str,
            result.player2_percent()
        );
        println!("    ({} simulations in {:?})", result.simulations, duration);
        println!();
    }
}

/// Démo 5: Utilisation des Ranges
fn demo_ranges() {
    println!("🎲 DEMO 5: Poker Ranges");
    println!("{}", "─".repeat(60));

    // Parsing de ranges
    println!("  📝 Parsing de ranges:");
    let ranges = vec![
        "AA",
        "QQ+",
        "AKs",
        "ATs+",
        "AKo",
        "AQo+",
        "JTs, T9s, 98s",
        "TT+, AK, AQ",
    ];

    for range_str in ranges {
        let range = Range::from_str(range_str).unwrap();
        let breakdown = range.combo_breakdown(None);
        println!("    {:20} → {}", range_str, breakdown);
    }

    // Équité range vs main
    println!("\n  ⚔️  Range vs Hand Equity:");
    let opening_range = Range::from_str("QQ+, AK").unwrap();
    let villain_hand = HoleCards::from_str("JdJc").unwrap();

    println!("    Range: QQ+, AK");
    println!("    vs Hand: {}", villain_hand);

    let calculator = EquityCalculator::new();
    let start = Instant::now();
    let result = calculator.calculate_range_vs_hand(&opening_range, &villain_hand, &[], 1000);
    let duration = start.elapsed();

    println!("    Range equity: {:.1}%", result.range_percent());
    println!("    Hand equity:  {:.1}%", result.opponent_percent());
    println!(
        "    ({} combos, {} sims in {:?})",
        result.combos_evaluated, result.total_simulations, duration
    );

    // Équité range vs range
    println!("\n  🎯 Range vs Range Equity:");
    let hero_range = Range::from_str("TT+, AK, AQ").unwrap();
    let villain_range = Range::from_str("77+, AJ+, KQ").unwrap();

    println!("    Hero:    TT+, AK, AQ");
    println!("    Villain: 77+, AJ+, KQ");

    let start = Instant::now();
    let result = calculator.calculate_range_vs_range(&hero_range, &villain_range, &[], 500);
    let duration = start.elapsed();

    println!("    Hero equity:    {:.1}%", result.range_percent());
    println!("    Villain equity: {:.1}%", result.opponent_percent());
    println!(
        "    ({} matchups, {} sims in {:?})",
        result.combos_evaluated, result.total_simulations, duration
    );

    // Range avec dead cards
    println!("\n  🎴 Range avec dead cards:");
    let range = Range::from_str("AA").unwrap();
    println!("    Range: AA");
    println!("    Sans dead cards: {} combos", range.combo_count(None));

    let mut dead = CardSet::new();
    dead.insert(Card::try_from("Ah").unwrap());
    println!(
        "    Avec Ah mort:    {} combos",
        range.combo_count(Some(dead))
    );

    dead.insert(Card::try_from("As").unwrap());
    println!(
        "    Avec Ah,As morts: {} combo",
        range.combo_count(Some(dead))
    );

    // Conversion en hole cards
    println!("\n  🃏 Génération de combos:");
    let range = Range::from_str("KK").unwrap();
    let combos = range.to_hole_cards(None);
    print!("    KK → ");
    for (i, combo) in combos.iter().enumerate() {
        print!("{}", combo);
        if i < combos.len() - 1 {
            print!(", ");
        }
    }
    println!();
}

/// Démo 6: Opérations sur les CardSet
fn demo_cardset_operations() {
    println!("🃏 DEMO 6: CardSet Operations");
    println!("{}", "─".repeat(60));

    let hand1 = HoleCards::from_str("AhKh").unwrap();
    let hand2 = HoleCards::from_str("QsQd").unwrap();

    let set1 = hand1.to_card_set();
    let set2 = hand2.to_card_set();

    println!("  Hand 1: {} ({} cards)", hand1, set1.count());
    println!("  Hand 2: {} ({} cards)", hand2, set2.count());

    let union = set1.union(set2);
    println!("\n  Union: {} cards", union.count());
    print!("    Cards: ");
    for card in union {
        print!("{} ", card);
    }
    println!();

    let deck = CardSet::FULL_DECK;
    println!("\n  Full deck: {} cards", deck.count());

    let remaining = deck.difference(union);
    println!("  Remaining cards: {} cards", remaining.count());

    // Sélectionne 5 cartes aléatoires pour un board
    println!("\n  Random board (5 cards):");
    let mut board_cards = Vec::new();
    let mut temp_remaining = remaining;

    for _ in 0..5 {
        if let Some(card) = temp_remaining.iter().next() {
            board_cards.push(card);
            temp_remaining.remove(card);
        }
    }

    print!("    ");
    for card in board_cards {
        print!("{} ", card);
    }
    println!();

    println!("\n  Testing contains:");
    let ace_hearts = Card::try_from("Ah").unwrap();
    let deuce_clubs = Card::try_from("2c").unwrap();
    println!("    {} in hand1? {}", ace_hearts, set1.contains(ace_hearts));
    println!(
        "    {} in hand1? {}",
        deuce_clubs,
        set1.contains(deuce_clubs)
    );
}
