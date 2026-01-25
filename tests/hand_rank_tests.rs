use holdem_rsources::core::hand_rank::{HandCategory, HandRanking};

#[test]
fn test_hand_category_ordering() {
    assert!(HandCategory::OnePair > HandCategory::HighCard);
    assert!(HandCategory::TwoPair > HandCategory::OnePair);
    assert!(HandCategory::StraightFlush > HandCategory::FourOfAKind);
}

#[test]
fn test_hand_ranking_comparison() {
    let high_card = HandRanking::high_card(&[12, 10, 8, 6, 4]);
    let pair = HandRanking::one_pair(10, &[12, 8, 6]);
    let two_pair = HandRanking::two_pair(10, 8, 12);
    let trips = HandRanking::three_of_a_kind(10, &[12, 8]);
    let straight = HandRanking::straight(10);
    let flush = HandRanking::flush(&[12, 10, 8, 6, 4]);
    let full_house = HandRanking::full_house(10, 8);
    let quads = HandRanking::four_of_a_kind(10, 12);
    let straight_flush = HandRanking::straight_flush(10);
    let royal_flush = HandRanking::royal_flush();

    assert!(pair > high_card);
    assert!(two_pair > pair);
    assert!(trips > two_pair);
    assert!(straight > trips);
    assert!(flush > straight);
    assert!(full_house > flush);
    assert!(quads > full_house);
    assert!(straight_flush > quads);
    assert!(royal_flush > straight_flush);
}

#[test]
fn test_same_category_comparison() {
    let pair_aces = HandRanking::one_pair(12, &[10, 8, 6]);
    let pair_kings = HandRanking::one_pair(11, &[12, 10, 8]);
    assert!(pair_aces > pair_kings);

    let pair_aces_good_kicker = HandRanking::one_pair(12, &[11, 10, 9]);
    let pair_aces_bad_kicker = HandRanking::one_pair(12, &[10, 9, 8]);
    assert!(pair_aces_good_kicker > pair_aces_bad_kicker);
}

#[test]
fn test_royal_flush_detection() {
    let royal = HandRanking::royal_flush();
    let sf_king = HandRanking::straight_flush(11);

    assert!(royal.is_royal_flush());
    assert!(!sf_king.is_royal_flush());
}

#[test]
fn test_category_extraction() {
    assert_eq!(
        HandRanking::high_card(&[12]).category(),
        HandCategory::HighCard
    );
    assert_eq!(
        HandRanking::one_pair(10, &[]).category(),
        HandCategory::OnePair
    );
    assert_eq!(
        HandRanking::royal_flush().category(),
        HandCategory::StraightFlush
    );
}
