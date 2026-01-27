use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use holdem_rsources::core::*;
use std::hint::black_box;
use std::str::FromStr;

fn bench_hand_evaluation(c: &mut Criterion) {
    let evaluator = LookupEvaluator::new();

    let mut group = c.benchmark_group("hand_evaluation");

    // Helper function to create hands from string
    fn parse_hand(s: &str) -> Hand {
        let cards: Vec<Card> = s
            .split_whitespace()
            .filter_map(|c| Card::try_from(c).ok())
            .collect();
        Hand::from_cards(&cards)
    }

    // Royal flush
    let royal_flush = parse_hand("As Ks Qs Js Ts");
    group.bench_function("royal_flush", |b| {
        b.iter(|| evaluator.evaluate(black_box(&royal_flush)))
    });

    // Straight flush
    let straight_flush = parse_hand("9h 8h 7h 6h 5h");
    group.bench_function("straight_flush", |b| {
        b.iter(|| evaluator.evaluate(black_box(&straight_flush)))
    });

    // Four of a kind
    let four_kind = parse_hand("Kc Kd Kh Ks 2c");
    group.bench_function("four_of_a_kind", |b| {
        b.iter(|| evaluator.evaluate(black_box(&four_kind)))
    });

    // Full house
    let full_house = parse_hand("Qh Qd Qs 9c 9h");
    group.bench_function("full_house", |b| {
        b.iter(|| evaluator.evaluate(black_box(&full_house)))
    });

    // Flush
    let flush = parse_hand("Kd Jd 9d 6d 3d");
    group.bench_function("flush", |b| {
        b.iter(|| evaluator.evaluate(black_box(&flush)))
    });

    // Straight
    let straight = parse_hand("Tc 9d 8h 7s 6c");
    group.bench_function("straight", |b| {
        b.iter(|| evaluator.evaluate(black_box(&straight)))
    });

    // Three of a kind
    let three_kind = parse_hand("7h 7d 7c Ah Kd");
    group.bench_function("three_of_a_kind", |b| {
        b.iter(|| evaluator.evaluate(black_box(&three_kind)))
    });

    // Two pair
    let two_pair = parse_hand("Jh Jd 4c 4s 2h");
    group.bench_function("two_pair", |b| {
        b.iter(|| evaluator.evaluate(black_box(&two_pair)))
    });

    // One pair
    let one_pair = parse_hand("As Ah Kd Qc Jh");
    group.bench_function("one_pair", |b| {
        b.iter(|| evaluator.evaluate(black_box(&one_pair)))
    });

    // High card
    let high_card = parse_hand("Ah Kd Qc Jh 9s");
    group.bench_function("high_card", |b| {
        b.iter(|| evaluator.evaluate(black_box(&high_card)))
    });

    // 7-card hand (typical Texas Hold'em scenario)
    let seven_card = parse_hand("As Ad Kh Qc Jd Ts 9h");
    group.bench_function("seven_card_hand", |b| {
        b.iter(|| evaluator.evaluate(black_box(&seven_card)))
    });

    group.finish();
}

fn bench_range_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("range_parsing");

    // Simple range
    group.bench_function("simple_pairs", |b| {
        b.iter(|| Range::from_str(black_box("AA, KK, QQ")).unwrap())
    });

    // Medium complexity
    group.bench_function("medium_range", |b| {
        b.iter(|| Range::from_str(black_box("AA, KK+, AKs, AQs, KQs")).unwrap())
    });

    // Complex range
    group.bench_function("complex_range", |b| {
        b.iter(|| {
            Range::from_str(black_box(
                "AA, KK, QQ+, AKs, AKo, AQs+, KQs+, JTs+, T9s+, 98s+",
            ))
            .unwrap()
        })
    });

    // Very complex typical 3-bet range
    group.bench_function("typical_3bet_range", |b| {
        b.iter(|| {
            Range::from_str(black_box(
                "AA, KK, QQ, JJ, TT, 99, AKs, AKo, AQs, AQo, AJs, ATs, KQs, KJs, QJs, JTs",
            ))
            .unwrap()
        })
    });

    group.finish();
}

fn bench_range_expansion(c: &mut Criterion) {
    let mut group = c.benchmark_group("range_expansion");

    let ranges = vec![
        ("small", "AA, KK"),
        ("medium", "AA, KK, QQ+, AKs, AKo"),
        (
            "large",
            "AA, KK, QQ+, AKs, AKo, AQs+, KQs+, JTs+, T9s+, 98s+, 87s+, 76s+",
        ),
        (
            "very_large",
            "AA, KK, QQ, JJ, TT, 99, 88, 77, 66, 55, 44, 33, 22, AKs, AKo, AQs, AQo, AJs, AJo, ATs, ATo, KQs, KQo, KJs, KJo, KTs, QJs, QJo, QTs, JTs",
        ),
    ];

    for (name, range_str) in ranges {
        let range = Range::from_str(range_str).unwrap();
        group.bench_with_input(BenchmarkId::from_parameter(name), &range, |b, r| {
            b.iter(|| r.to_hole_cards(black_box(None)))
        });
    }

    group.finish();
}

fn bench_equity_calculation(c: &mut Criterion) {
    let calc = EquityCalculator::new();

    let mut group = c.benchmark_group("equity_calculation");

    // Classic preflop scenarios
    let aa = HoleCards::from_str("AsAh").unwrap();
    let kk = HoleCards::from_str("KcKd").unwrap();
    let ak = HoleCards::from_str("AhKh").unwrap();
    let qq = HoleCards::from_str("QsQd").unwrap();
    let _72o = HoleCards::from_str("7h2c").unwrap();

    // AA vs KK (classic cooler)
    group.bench_function("preflop_AA_vs_KK", |b| {
        b.iter(|| {
            calc.calculate_monte_carlo(
                black_box(&aa),
                black_box(&kk),
                black_box(&[]),
                black_box(10000),
            )
        })
    });

    // AK vs QQ (coin flip)
    group.bench_function("preflop_AK_vs_QQ", |b| {
        b.iter(|| {
            calc.calculate_monte_carlo(
                black_box(&ak),
                black_box(&qq),
                black_box(&[]),
                black_box(10000),
            )
        })
    });

    // With board (flop)
    let board_flop = vec![
        Card::try_from("Ah").unwrap(),
        Card::try_from("Kd").unwrap(),
        Card::try_from("Qc").unwrap(),
    ];

    group.bench_function("flop_equity", |b| {
        b.iter(|| {
            calc.calculate_monte_carlo(
                black_box(&aa),
                black_box(&kk),
                black_box(&board_flop),
                black_box(10000),
            )
        })
    });

    // With board (turn)
    let board_turn = vec![
        Card::try_from("Ah").unwrap(),
        Card::try_from("Kd").unwrap(),
        Card::try_from("Qc").unwrap(),
        Card::try_from("Jh").unwrap(),
    ];

    group.bench_function("turn_equity", |b| {
        b.iter(|| {
            calc.calculate_monte_carlo(
                black_box(&aa),
                black_box(&kk),
                black_box(&board_turn),
                black_box(10000),
            )
        })
    });

    // Exact calculation on river (only 1 possibility)
    let board_river = vec![
        Card::try_from("Ah").unwrap(),
        Card::try_from("Kd").unwrap(),
        Card::try_from("Qc").unwrap(),
        Card::try_from("Jh").unwrap(),
        Card::try_from("Ts").unwrap(),
    ];

    group.bench_function("river_exact", |b| {
        b.iter(|| calc.calculate_exact(black_box(&aa), black_box(&kk), black_box(&board_river)))
    });

    group.finish();
}

fn bench_equity_simulation_sizes(c: &mut Criterion) {
    let calc = EquityCalculator::new();
    let aa = HoleCards::from_str("AsAh").unwrap();
    let kk = HoleCards::from_str("KcKd").unwrap();

    let mut group = c.benchmark_group("equity_simulation_sizes");

    for simulations in [1000, 5000, 10000, 50000, 100000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(simulations),
            simulations,
            |b, &sims| {
                b.iter(|| {
                    calc.calculate_monte_carlo(
                        black_box(&aa),
                        black_box(&kk),
                        black_box(&[]),
                        black_box(sims),
                    )
                })
            },
        );
    }

    group.finish();
}

fn bench_range_vs_range_equity(c: &mut Criterion) {
    let calc = EquityCalculator::new();

    let mut group = c.benchmark_group("range_vs_range_equity");

    // Small ranges
    let r1 = Range::from_str("AA, KK").unwrap();
    let r2 = Range::from_str("QQ, JJ").unwrap();

    group.bench_function("small_ranges_1000", |b| {
        b.iter(|| {
            calc.calculate_range_vs_range(
                black_box(&r1),
                black_box(&r2),
                black_box(&[]),
                black_box(1000),
            )
        })
    });

    // Medium ranges
    let r3 = Range::from_str("AA, KK, QQ, AKs").unwrap();
    let r4 = Range::from_str("JJ, TT, AQs, KQs").unwrap();

    group.bench_function("medium_ranges_1000", |b| {
        b.iter(|| {
            calc.calculate_range_vs_range(
                black_box(&r3),
                black_box(&r4),
                black_box(&[]),
                black_box(1000),
            )
        })
    });

    group.finish();
}

fn bench_range_vs_range_parallel_comparison(c: &mut Criterion) {
    let calc = EquityCalculator::new();

    let mut group = c.benchmark_group("range_vs_range_parallel_vs_sequential");

    let r1 = Range::from_str("AA, KK, QQ").unwrap();
    let r2 = Range::from_str("JJ, TT, 99").unwrap();

    group.bench_function("parallel", |b| {
        b.iter(|| {
            calc.calculate_range_vs_range(
                black_box(&r1),
                black_box(&r2),
                black_box(&[]),
                black_box(1000),
            )
        })
    });

    group.bench_function("sequential", |b| {
        b.iter(|| {
            calc.calculate_range_vs_range_sequential(
                black_box(&r1),
                black_box(&r2),
                black_box(&[]),
                black_box(1000),
            )
        })
    });

    group.finish();
}

fn bench_card_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("card_operations");

    // Card creation
    group.bench_function("card_creation", |b| {
        b.iter(|| Card::new(black_box(Value::Ace), black_box(Suit::Spades)))
    });

    // Card parsing
    group.bench_function("card_parsing", |b| {
        b.iter(|| Card::try_from(black_box("Ah")).unwrap())
    });

    // CardSet operations
    let mut card_set = CardSet::new();
    let card = Card::try_from("Ah").unwrap();

    group.bench_function("cardset_insert", |b| {
        b.iter(|| {
            let mut cs = CardSet::new();
            cs.insert(black_box(card));
            cs
        })
    });

    card_set.insert(card);
    group.bench_function("cardset_contains", |b| {
        b.iter(|| card_set.contains(black_box(card)))
    });

    // HoleCards creation
    group.bench_function("holecards_parsing", |b| {
        b.iter(|| HoleCards::from_str(black_box("AsKh")).unwrap())
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_hand_evaluation,
    bench_range_parsing,
    bench_range_expansion,
    bench_equity_calculation,
    bench_equity_simulation_sizes,
    bench_range_vs_range_equity,
    bench_range_vs_range_parallel_comparison,
    bench_card_operations,
);

criterion_main!(benches);
