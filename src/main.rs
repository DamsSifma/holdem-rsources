use holdem_rsources::core::{Card, Hand, HoleCards, LookupEvaluator, HandEvaluator};

fn main() {
    println!("=== Holdem RSources - Hand Evaluator Demo ===\n");

    // Crée l'évaluateur
    let evaluator = LookupEvaluator::new();

    // Exemple: AKs vs QQ sur un board
    let hero = HoleCards::from_str("AhKh").unwrap();
    let villain = HoleCards::from_str("QsQd").unwrap();

    println!("Hero: {} (suited: {})", hero, hero.is_suited());
    println!("Villain: {} (pair: {})", villain, villain.is_pair());

    // Board: Ah Qc Jh Th 2s
    let board = [
        Card::try_from("Ac").unwrap(),
        Card::try_from("Qc").unwrap(),
        Card::try_from("Jh").unwrap(),
        Card::try_from("Th").unwrap(),
        Card::try_from("2H").unwrap(),
    ];

    println!("\nBoard: {}", board.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));

    // Évalue les mains
    let hero_hand = Hand::from_card_set(hero.to_card_set()).with_board(&board);
    let villain_hand = Hand::from_card_set(villain.to_card_set()).with_board(&board);

    let hero_rank = evaluator.evaluate(&hero_hand);
    let villain_rank = evaluator.evaluate(&villain_hand);

    println!("\nHero: {} (score: {})", hero_rank, hero_rank.score());
    println!("Villain: {} (score: {})", villain_rank, villain_rank.score());

    match hero_rank.cmp(&villain_rank) {
        std::cmp::Ordering::Greater => println!("\n🏆 Hero wins!"),
        std::cmp::Ordering::Less => println!("\n🏆 Villain wins!"),
        std::cmp::Ordering::Equal => println!("\n🤝 Split pot!"),
    }
}
