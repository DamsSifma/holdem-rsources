use super::card::Card;
use super::hand::Hand;
use super::hand_rank::HandRanking;

pub trait HandEvaluator {
    /// Évalue une main de 5-7 cartes et retourne son rang
    fn evaluate(&self, hand: &Hand) -> HandRanking;

    /// Évalue depuis le bitset u64 directement
    fn evaluate_u64(&self, cards: u64) -> HandRanking;
}

/// Évaluateur utilisant des lookup tables précalculées
///
/// Utilise une approche inspirée de Cactus Kev avec qqs différences:
/// - Table de flush pour les couleurs
/// - Table de rangs uniques pour les quintes et high cards
/// - Calcul dynamique pour les paires/brelans/carrés (plus rapide avec peu de mémoire)
pub struct LookupEvaluator {
    /// Table pour évaluer les flush (8192 entrées = 2^13 combinaisons de rangs)
    flush_table: Box<[u16; 8192]>,
}

impl LookupEvaluator {
    pub fn new() -> Self {
        let flush_table = Self::generate_flush_table();

        Self { flush_table }
    }

    /// Génère la table des flush (5 cartes de même couleur)
    fn generate_flush_table() -> Box<[u16; 8192]> {
        let mut table = Box::new([0u16; 8192]);

        // Pour chaque combinaison de 5 rangs parmi 13
        for c1 in 0..13u8 {
            for c2 in (c1 + 1)..13 {
                for c3 in (c2 + 1)..13 {
                    for c4 in (c3 + 1)..13 {
                        for c5 in (c4 + 1)..13 {
                            let key = (1 << c1) | (1 << c2) | (1 << c3) | (1 << c4) | (1 << c5);
                            let ranks = [c5, c4, c3, c2, c1]; // Du plus haut au plus bas

                            // Vérifie si c'est une quinte flush
                            if let Some(high) = Self::is_straight_ranks(&ranks) {
                                // Straight flush: catégorie 8, high card comme kicker
                                table[key as usize] = 8000 + high as u16;
                            } else {
                                // Flush simple: catégorie 5
                                let score = Self::encode_high_card_score(&ranks);
                                table[key as usize] = 5000 + score;
                            }
                        }
                    }
                }
            }
        }

        table
    }

    fn is_straight_ranks(ranks: &[u8; 5]) -> Option<u8> {
        let high = ranks[0];
        let low = ranks[4];

        if high - low == 4 {
            return Some(high);
        }

        // ranks = [12, 3, 2, 1, 0] => Ace, 5, 4, 3, 2
        if ranks == &[12, 3, 2, 1, 0] {
            return Some(3); // = 5-high straight
        }

        None
    }

    /// Encode un score de high card sur 10 bits
    fn encode_high_card_score(ranks: &[u8; 5]) -> u16 {
        // Encode 5 rangs avec poids décroissant
        let mut score = 0u16;
        for (i, &r) in ranks.iter().enumerate() {
            score += (r as u16) * [371, 28, 2, 1, 1][i]; // Poids approximatifs
        }
        score.min(999) // Cap à 999 pour rester dans la catégorie
    }

    /// Extrait les rangs depuis un bitset de 5 cartes (sans couleur)
    fn extract_rank_bits(cards: u64) -> u16 {
        let mut rank_bits = 0u16;
        let mut remaining = cards;

        while remaining != 0 {
            let idx = remaining.trailing_zeros() as u8;
            let rank = idx / 4; // 0-12
            rank_bits |= 1 << rank;
            remaining &= remaining - 1;
        }

        rank_bits
    }

    /// Compte les cartes par rang
    fn count_ranks(cards: u64) -> [u8; 13] {
        let mut counts = [0u8; 13];
        let mut remaining = cards;

        while remaining != 0 {
            let idx = remaining.trailing_zeros() as u8;
            let rank = (idx / 4) as usize;
            counts[rank] += 1;
            remaining &= remaining - 1;
        }

        counts
    }

    fn check_flush(cards: u64) -> Option<u16> {
        let mut suit_ranks = [0u16; 4];
        let mut remaining = cards;

        while remaining != 0 {
            let idx = remaining.trailing_zeros();
            let suit = (idx % 4) as usize;
            let rank = idx / 4; // 0-12
            suit_ranks[suit] |= 1 << rank;
            remaining &= remaining - 1;
        }

        for &rank_bits in &suit_ranks {
            if rank_bits.count_ones() >= 5 {
                // Prend les 5 meilleurs rangs
                let mut bits = rank_bits;
                let mut count = 0;
                let mut result = 0u16;

                while bits != 0 && count < 5 {
                    let high = 15 - bits.leading_zeros();
                    result |= 1 << high;
                    bits &= !(1 << high);
                    count += 1;
                }

                return Some(result);
            }
        }

        None
    }

    /// Évalue les mains avec paires/brelans/carrés
    fn evaluate_paired(&self, counts: &[u8; 13]) -> HandRanking {
        let mut quads = Vec::new();
        let mut trips = Vec::new();
        let mut pairs = Vec::new();
        let mut singles = Vec::new();

        // Parcourt du plus haut (As=12) au plus bas (2=0)
        for rank in (0..13).rev() {
            match counts[rank] {
                4 => quads.push(rank as u8),
                3 => trips.push(rank as u8),
                2 => pairs.push(rank as u8),
                1 => singles.push(rank as u8),
                _ => {}
            }
        }

        // Carré
        if !quads.is_empty() {
            let kicker = trips
                .first()
                .or(pairs.first())
                .or(singles.first())
                .copied()
                .unwrap_or(0);
            return HandRanking::four_of_a_kind(quads[0], kicker);
        }

        // Full house
        if !trips.is_empty() && (!pairs.is_empty() || trips.len() >= 2) {
            let trip_rank = trips[0];
            let pair_rank = if trips.len() >= 2 { trips[1] } else { pairs[0] };
            return HandRanking::full_house(trip_rank, pair_rank);
        }

        // Brelan
        if !trips.is_empty() {
            let kickers: Vec<u8> = pairs
                .iter()
                .chain(singles.iter())
                .take(2)
                .copied()
                .collect();
            return HandRanking::three_of_a_kind(trips[0], &kickers);
        }

        // Double paire
        if pairs.len() >= 2 {
            let kicker = pairs.get(2).or(singles.first()).copied().unwrap_or(0);
            return HandRanking::two_pair(pairs[0], pairs[1], kicker);
        }

        // Paire
        if !pairs.is_empty() {
            let kickers: Vec<u8> = singles.iter().take(3).copied().collect();
            return HandRanking::one_pair(pairs[0], &kickers);
        }

        // High card (ne devrait pas arriver ici si appelé correctement)
        let kickers: Vec<u8> = singles.iter().take(5).copied().collect();
        HandRanking::high_card(&kickers)
    }

    /// Vérifie si c'est une quinte et retourne la carte haute
    fn check_straight(&self, rank_bits: u16) -> Option<u8> {
        // Patterns de quintes (du plus haut au plus bas)
        const STRAIGHT_PATTERNS: [(u16, u8); 10] = [
            (0b1111100000000, 12), // A-K-Q-J-T
            (0b0111110000000, 11), // K-Q-J-T-9
            (0b0011111000000, 10), // Q-J-T-9-8
            (0b0001111100000, 9),  // J-T-9-8-7
            (0b0000111110000, 8),  // T-9-8-7-6
            (0b0000011111000, 7),  // 9-8-7-6-5
            (0b0000001111100, 6),  // 8-7-6-5-4
            (0b0000000111110, 5),  // 7-6-5-4-3
            (0b0000000011111, 4),  // 6-5-4-3-2
            (0b1000000001111, 3),  // A-5-4-3-2 (wheel)
        ];

        for (pattern, high) in STRAIGHT_PATTERNS {
            if (rank_bits & pattern) == pattern {
                return Some(high);
            }
        }

        None
    }
}

impl Default for LookupEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl HandEvaluator for LookupEvaluator {
    fn evaluate(&self, hand: &Hand) -> HandRanking {
        self.evaluate_u64(hand.as_u64())
    }

    fn evaluate_u64(&self, cards: u64) -> HandRanking {
        let card_count = cards.count_ones();

        if card_count < 5 {
            return HandRanking::MIN;
        }

        // Vérifie d'abord la couleur
        if let Some(flush_ranks) = Self::check_flush(cards) {
            let table_score = self.flush_table[flush_ranks as usize];
            // table_score: 8000+ = straight flush, 5000-5999 = flush
            // Convertir vers le format HandRanking (category << 24 | kickers)
            if table_score >= 8000 {
                // Straight flush: high card est table_score - 8000
                let high = (table_score - 8000) as u8;
                return HandRanking::straight_flush(high);
            } else {
                // Flush: le score relatif est table_score - 5000
                let kicker_score = (table_score - 5000) as u32;
                return HandRanking::from_score((5u32 << 24) | kicker_score);
            }
        }

        let counts = Self::count_ranks(cards);
        let has_pairs = counts.iter().any(|&c| c >= 2);

        if has_pairs {
            return self.evaluate_paired(&counts);
        }

        // Pas de paires - vérifie quinte ou high card
        let rank_bits = Self::extract_rank_bits(cards);

        if let Some(high) = self.check_straight(rank_bits) {
            return HandRanking::straight(high);
        }

        // High card - prend les 5 meilleurs rangs
        let mut kickers = Vec::new();
        for rank in (0..13).rev() {
            if counts[rank] > 0 {
                kickers.push(rank as u8);
                if kickers.len() == 5 {
                    break;
                }
            }
        }

        HandRanking::high_card(&kickers)
    }
}

/// Évalue les 7 cartes (2 hole + 5 board) et trouve la meilleure main de 5
pub fn evaluate_7_cards(evaluator: &impl HandEvaluator, cards: &[Card; 7]) -> HandRanking {
    let mut best = HandRanking::MIN;

    // Génère toutes les combinaisons de 5 cartes parmi 7 (21 combinaisons)
    for i in 0..7 {
        for j in (i + 1)..7 {
            // Exclut les cartes i et j
            let mut hand_bits = 0u64;
            for (k, card) in cards.iter().enumerate() {
                if k != i && k != j {
                    hand_bits |= 1u64 << card.index();
                }
            }

            let ranking = evaluator.evaluate_u64(hand_bits);
            if ranking > best {
                best = ranking;
            }
        }
    }

    best
}

/// Évalue rapidement avec le bitset complet (optimisé pour 7 cartes)
pub fn evaluate_7_cards_fast(evaluator: &impl HandEvaluator, cards_bits: u64) -> HandRanking {
    // Pour 7 cartes, on peut directement évaluer le bitset
    // L'évaluateur choisira les 5 meilleures cartes
    evaluator.evaluate_u64(cards_bits)
}
