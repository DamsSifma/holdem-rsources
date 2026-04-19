use holdem_rsources::core::{
    COMBO_COUNT, Card, CardSet, EquityCalculator, HoleCards, Range, WeightedRange,
    WeightedRangeParseError,
};
use std::collections::HashSet;
use std::str::FromStr;

#[macro_use]
mod test_utils;
use test_utils::TOLERANCE;

#[test]
fn test_combo_index_roundtrip() {
    for i in 0..COMBO_COUNT {
        let hc = HoleCards::from_combo_index(i as u16).unwrap();
        assert_eq!(hc.combo_index(), i as u16);
    }
}

#[test]
fn test_combo_index_uniqueness() {
    let mut seen = HashSet::new();

    for hc in HoleCards::all_combos() {
        assert!(seen.insert(hc.combo_index()));
    }

    assert_eq!(seen.len(), COMBO_COUNT);
}

#[test]
fn test_all_combos_count() {
    assert_eq!(HoleCards::all_combos().len(), COMBO_COUNT);
}

#[test]
fn test_weighted_from_range() {
    let range = Range::from_str("AA").unwrap();
    let weighted = WeightedRange::from_range(&range);

    assert_eq!(weighted.num_combos(), 6);
    for (idx, w) in weighted.iter_nonzero() {
        let hc = HoleCards::from_combo_index(idx).unwrap();
        assert!(hc.is_pair());
        assert_eq!(w, 1.0);
    }
}

#[test]
fn test_weighted_parse_simple() {
    let weighted = WeightedRange::from_str("AKs:0.75, QQ").unwrap();

    let aks = Range::from_str("AKs").unwrap();
    for hc in aks.to_hole_cards(None) {
        assert_eq!(weighted.weight_for_hole_cards(hc), 0.75);
    }

    let qq = Range::from_str("QQ").unwrap();
    for hc in qq.to_hole_cards(None) {
        assert_eq!(weighted.weight_for_hole_cards(hc), 1.0);
    }
}

#[test]
fn test_weighted_parse_invalid_weight() {
    let result = WeightedRange::from_str("AKs:1.2");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        WeightedRangeParseError::InvalidWeight(_)
    ));
}

#[test]
fn test_weighted_parse_invalid_pattern() {
    let result = WeightedRange::from_str("ZZ:0.5");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        WeightedRangeParseError::Range(_)
    ));
}

#[test]
fn test_dead_cards_filtering() {
    let mut weighted = WeightedRange::uniform(1.0);
    let ah = Card::try_from("Ah").unwrap();

    let mut dead = CardSet::new();
    dead.insert(ah);

    weighted.apply_dead_cards(dead);

    for hc in HoleCards::all_combos() {
        if hc.high() == ah || hc.low() == ah {
            assert_eq!(weighted.weight_for_hole_cards(*hc), 0.0);
        }
    }
}

#[test]
fn test_weighted_normalize() {
    let mut weighted = WeightedRange::empty();
    let aa = Range::from_str("AA").unwrap();

    for hc in aa.to_hole_cards(None) {
        weighted.set_weight_for_hole_cards(hc, 2.0);
    }

    weighted.normalize();
    let total = weighted.total_weight();
    assert!((total - 1.0).abs() < 1e-6);
}

#[test]
fn test_weighted_range_vs_hand_equity_matches_binary_when_uniform() {
    let binary = Range::from_str("AA, KK").unwrap();
    let weighted = WeightedRange::from_range(&binary);
    let villain = HoleCards::from_str("QdQc").unwrap();
    let calculator = EquityCalculator::new();

    let unweighted_result = calculator.calculate_range_vs_hand(&binary, &villain, &[], 500);
    let weighted_result =
        calculator.calculate_weighted_range_vs_hand(&weighted, &villain, &[], 500);

    assert_within_tolerance!(
        weighted_result.range_percent(),
        unweighted_result.range_percent(),
        TOLERANCE
    );
}

#[test]
fn test_weighted_range_vs_range_equity_matches_binary_when_uniform() {
    let binary1 = Range::from_str("AA").unwrap();
    let binary2 = Range::from_str("KK").unwrap();

    let weighted1 = WeightedRange::from_range(&binary1);
    let weighted2 = WeightedRange::from_range(&binary2);

    let calculator = EquityCalculator::new();

    let unweighted_result = calculator.calculate_range_vs_range(&binary1, &binary2, &[], 500);
    let weighted_result =
        calculator.calculate_weighted_range_vs_range(&weighted1, &weighted2, &[], 500);

    assert_within_tolerance!(
        weighted_result.range_percent(),
        unweighted_result.range_percent(),
        TOLERANCE
    );
}

#[test]
fn test_partial_weights_change_equity() {
    let calculator = EquityCalculator::new();
    let villain = HoleCards::from_str("KcKd").unwrap();

    let mut weighted = WeightedRange::empty();

    let aa = Range::from_str("AA").unwrap();
    for hc in aa.to_hole_cards(None) {
        weighted.set_weight_for_hole_cards(hc, 1.0);
    }

    let result_aa_only = calculator.calculate_weighted_range_vs_hand(&weighted, &villain, &[], 300);

    let seven_two_o = Range::from_str("72o").unwrap();
    for hc in seven_two_o.to_hole_cards(None) {
        weighted.set_weight_for_hole_cards(hc, 1.0);
    }
    weighted.normalize();

    let result_mixed = calculator.calculate_weighted_range_vs_hand(&weighted, &villain, &[], 300);

    assert!(result_mixed.range_equity < result_aa_only.range_equity);
}
