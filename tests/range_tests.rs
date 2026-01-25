use holdem_rsources::core::{Card, CardSet, EquityCalculator, HoleCards, Range};
use std::str::FromStr;

#[macro_use]
mod test_utils;
use test_utils::TOLERANCE;
#[test]
fn test_parse_single_pair() {
    let range = Range::from_str("AA").unwrap();
    assert_eq!(range.combo_count(None), 6); // C(4,2) = 6 combos pour une paire
}

#[test]
fn test_parse_pair_plus() {
    let range = Range::from_str("QQ+").unwrap();
    // QQ (6) + KK (6) + AA (6) = 18
    assert_eq!(range.combo_count(None), 18);
}

#[test]
fn test_parse_suited() {
    let range = Range::from_str("AKs").unwrap();
    assert_eq!(range.combo_count(None), 4); // 4 combos suited
}

#[test]
fn test_parse_suited_plus() {
    let range = Range::from_str("ATs+").unwrap();
    // ATs, AJs, AQs, AKs = 4 * 4 = 16
    assert_eq!(range.combo_count(None), 16);
}

#[test]
fn test_parse_offsuit() {
    let range = Range::from_str("AKo").unwrap();
    assert_eq!(range.combo_count(None), 12); // 4*3 = 12 combos offsuit
}

#[test]
fn test_parse_offsuit_plus() {
    let range = Range::from_str("AQo+").unwrap();
    // AQo, AKo = 12 + 12 = 24
    assert_eq!(range.combo_count(None), 24);
}

#[test]
fn test_parse_any() {
    let range = Range::from_str("AK").unwrap();
    assert_eq!(range.combo_count(None), 16); // 4 suited + 12 offsuit
}

#[test]
fn test_parse_any_plus() {
    let range = Range::from_str("AQ+").unwrap();
    // AQ (16) + AK (16) = 32
    assert_eq!(range.combo_count(None), 32);
}

#[test]
fn test_parse_complex_range() {
    let range = Range::from_str("AA, KK, AKs, AQs, AKo").unwrap();
    // AA (6) + KK (6) + AKs (4) + AQs (4) + AKo (12) = 32
    assert_eq!(range.combo_count(None), 32);
}

#[test]
fn test_parse_premium_range() {
    let range = Range::from_str("QQ+, AK").unwrap();
    // QQ (6) + KK (6) + AA (6) + AK (16) = 34
    assert_eq!(range.combo_count(None), 34);
}

#[test]
fn test_combo_breakdown() {
    let range = Range::from_str("AA, AKs, AKo").unwrap();
    let breakdown = range.combo_breakdown(None);

    assert_eq!(breakdown.pairs, 6);
    assert_eq!(breakdown.suited, 4);
    assert_eq!(breakdown.offsuit, 12);
    assert_eq!(breakdown.total, 22);
}

#[test]
fn test_range_with_dead_cards() {
    let range = Range::from_str("AA").unwrap();

    // Sans dead cards: 6 combos
    assert_eq!(range.combo_count(None), 6);

    // Avec un As mort: seulement 3 combos (C(3,2))
    let mut dead = CardSet::new();
    dead.insert(Card::try_from("Ah").unwrap());
    assert_eq!(range.combo_count(Some(dead)), 3);

    // Avec deux As morts: seulement 1 combo (C(2,2))
    dead.insert(Card::try_from("As").unwrap());
    assert_eq!(range.combo_count(Some(dead)), 1);
}

#[test]
fn test_to_hole_cards() {
    let range = Range::from_str("KK").unwrap();
    let hole_cards = range.to_hole_cards(None);

    assert_eq!(hole_cards.len(), 6);

    // Vérifier que toutes les combos sont des paires de rois
    for hc in hole_cards {
        assert!(hc.is_pair());
        assert_eq!(hc.high().value, holdem_rsources::core::Value::King);
    }
}

#[test]
fn test_suited_combos() {
    let range = Range::from_str("AKs").unwrap();
    let hole_cards = range.to_hole_cards(None);

    assert_eq!(hole_cards.len(), 4);

    // Vérifier que toutes les combos sont suited
    for hc in hole_cards {
        assert!(hc.is_suited());
    }
}

#[test]
fn test_offsuit_combos() {
    let range = Range::from_str("AKo").unwrap();
    let hole_cards = range.to_hole_cards(None);

    assert_eq!(hole_cards.len(), 12);

    // Vérifier que toutes les combos sont offsuit
    for hc in hole_cards {
        assert!(!hc.is_suited());
    }
}

#[test]
fn test_contains() {
    let range = Range::from_str("AA, KK, AKs").unwrap();

    let aa = HoleCards::from_str("AhAs").unwrap();
    assert!(range.contains(&aa));

    let kk = HoleCards::from_str("KdKc").unwrap();
    assert!(range.contains(&kk));

    let aks = HoleCards::from_str("AhKh").unwrap();
    assert!(range.contains(&aks));

    let ako = HoleCards::from_str("AhKd").unwrap();
    assert!(!range.contains(&ako));

    let qq = HoleCards::from_str("QhQd").unwrap();
    assert!(!range.contains(&qq));
}

#[test]
fn test_broadway_range() {
    let range = Range::from_str("TT+, AK, AQ, AJ, KQ").unwrap();
    let breakdown = range.combo_breakdown(None);

    // TT, JJ, QQ, KK, AA = 5 * 6 = 30 pairs
    // AK, AQ, AJ, KQ = 4 * 16 = 64 autres
    // Total = 94
    assert_eq!(breakdown.total, 94);
}

#[test]
fn test_suited_connectors() {
    let range = Range::from_str("JTs, T9s, 98s, 87s").unwrap();
    let breakdown = range.combo_breakdown(None);

    // 4 mains * 4 combos chacune = 16
    assert_eq!(breakdown.suited, 16);
    assert_eq!(breakdown.offsuit, 0);
    assert_eq!(breakdown.pairs, 0);
}

#[test]
fn test_range_vs_hand_equity() {
    let range = Range::from_str("AA, KK").unwrap();
    let villain_hand = HoleCards::from_str("QdQc").unwrap();
    let calculator = EquityCalculator::new();

    let result = calculator.calculate_range_vs_hand(&range, &villain_hand, &[], 1000);

    assert_within_tolerance!(result.range_percent(), 81.7, TOLERANCE);
    assert_eq!(result.combos_evaluated, 12); // 6 combos AA + 6 combos KK
}

#[test]
fn test_range_vs_range_equity() {
    let range1 = Range::from_str("AA").unwrap();
    let range2 = Range::from_str("KK").unwrap();
    let calculator = EquityCalculator::new();

    let result = calculator.calculate_range_vs_range(&range1, &range2, &[], 1000);

    // AA devrait avoir environ 80% contre KK
    assert_within_tolerance!(result.range_percent(), 82.0, TOLERANCE);
}

#[test]
fn test_range_display() {
    let range = Range::from_str("AA, KK+").unwrap();
    let display = format!("{}", range);

    // Le display devrait contenir les patterns
    assert!(display.contains("AA") || display.contains("KK"));
}
