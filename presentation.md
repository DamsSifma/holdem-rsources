---
marp: true
theme: default
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
style: |
  section {
    font-size: 28px;
  }
  h1 {
    color: #d73a4a;
  }
  h2 {
    color: #0969da;
  }
  code {
    background-color: #f6f8fa;
  }
---

# Faire du code haute performance en Rust

## Un Solveur Texas Hold'em Haute Performance en Rust

Damien Massif
Janvier 2026

---

## Plan

1. Vue d'ensemble du projet
2. Architecture centrale
3. Système d'évaluation des mains
4. Analyse des ranges
5. Calculs d'équité
6. Implémentation technique
7. Optimisations de performance
8. Exemples de code
9. Tests et benchmarks
10. Améliorations futures

---

## Vue d'ensemble du projet

**holdem-rsources** est une bibliothèque complète de poker Texas Hold'em écrite en Rust.

**Fonctionnalités clés :**

- Évaluation rapide des mains avec tables de lookup
- Calculs d'équité Monte Carlo
- Parsing et analyse avancés des ranges
- Traitement parallèle avec Rayon
- Calculs d'équité exacts et approximatifs

**Stack technologique :**

- Rust
- ndarray, rayon, serde
- Criterion pour les benchmarks

---

## Architecture centrale

```
src/core/
├── card.rs           // Représentation des cartes (Value, Suit)
├── card_set.rs       // Stockage efficace des cartes (bitset)
├── hand.rs           // Types Hand & HoleCards
├── hand_rank.rs      // Système de classement des mains
├── evaluator.rs      // Moteur d'évaluation des mains
├── range.rs          // Parsing et manipulation des ranges
└── equity.rs         // Calculs d'équité
```

**Principes de conception :**

- Abstractions à coût zéro
- Manipulation de bits pour la performance
- Sécurité des types pour la logique de jeu

---

## Représentation des cartes

```rust
pub enum Value {
    Two = 0, Three, Four, Five, Six, Seven,
    Eight, Nine, Ten, Jack, Queen, King, Ace
}

pub enum Suit {
    Clubs, Diamonds, Hearts, Spades
}

pub struct Card {
    // Encodée en u8: rang * 4 + couleur
}
```

**CardSet :** Utilise un bitset 64 bits pour un stockage efficace

- Chaque carte = 1 bit
- Opérations rapides d'union, intersection, contenance
- Efficace en mémoire (8 octets pour toute combinaison)

---

## Système d'évaluation des mains

**Hiérarchie HandCategory :**

```
8. Quinte Flush    ♠️
7. Carré           A-A-A-A
6. Full House      A-A-A-2-2
5. Couleur         ♠♠♠♠♠
4. Quinte          5-4-3-2-A
3. Brelan          A-A-A
2. Double Paire    K-K-Q-Q
1. Paire           A-A
0. Carte Haute     A-K-Q
```

Chaque catégorie possède un classement unique pour départager les égalités.

---

## Design du LookupEvaluator

**Inspiré de l'algorithme Cactus Kev :**

1. **Table de Flush** (8192 entrées = 2^13)
   - Classements pré-calculés pour toutes les combinaisons de couleur
   - Détecte automatiquement les quintes flush

2. **Calcul dynamique** pour les non-couleurs
   - Analyse les fréquences de rangs
   - Identifie les paires, brelans, carrés
   - Encode efficacement les kickers

**Résultat :** HandRanking avec score 32 bits

- 8 bits supérieurs : catégorie (0-8)
- 24 bits inférieurs : kickers pour départager

---

## Analyse des ranges

**Exemples de syntaxe de range :**

```
"AA"          // Paire d'as (6 combos)
"AKs"         // As-Roi assorti (4 combos)
"AKo"         // As-Roi dépareillé (12 combos)
"QQ+"         // Dames ou mieux
"A9s+"        // A9s, ATs, AJs, AQs, AKs
"77+, ATs+, KQs"  // Ranges combinées
```

**Fonctionnalités :**

- Parser la notation poker
- Suivre assorti/dépareillé/paires séparément
- Tenir compte des cartes mortes
- Calculer les combinaisons totales

---

## Statistiques des ranges

```rust
pub struct ComboBreakdown {
    pairs: usize,
    suited: usize,
    offsuit: usize,
    total: usize,
}
```

**Exemple réel :**

```
Range d'ouverture BTN: "22+, A2s+, A5o+, K6s+..."
├── Paires: 78 combos
├── Assorties: 142 combos
├── Dépareillées: 184 combos
└── Total: 404 combos (30.5% VPIP)
```

---

## Calculs d'équité

**Deux méthodes de calcul :**

### 1. Exacte (Énumération)

- Évalue TOUTES les combinaisons possibles du board
- Précision à 100%
- Meilleure pour : scénarios preflop, flop

### 2. Monte Carlo (Simulation)

- Échantillonnage aléatoire des runouts
- Itérations configurables (10 000+)
- Meilleure pour : turn, river, grandes ranges
- Parallélisable avec rayon

---

## Code du calculateur d'équité

```rust
pub struct EquityCalculator {
    evaluator: LookupEvaluator,
}

impl EquityCalculator {
    pub fn calculate_exact(
        &self,
        hole1: &HoleCards,
        hole2: &HoleCards,
        board: &[Card],
    ) -> EquityResult {
        // Énumère tous les runouts possibles
    }

    pub fn calculate_monte_carlo(
        &self, hole1, hole2, board,
        iterations: usize
    ) -> EquityResult {
        // Échantillonnage aléatoire
    }
}
```

---

## Structure du résultat d'équité

```rust
pub struct EquityResult {
    pub player1_equity: f64,    // 0.0 à 1.0
    pub player2_equity: f64,
    pub tie_equity: f64,
    pub simulations: usize,
}
```

**Exemple de sortie :**

```
AA vs KK preflop:
├── Joueur 1 (AA): 82.4%
├── Joueur 2 (KK): 17.2%
└── Égalité: 0.4%
Simulations: 1 070 190
```

---

## Optimisations de performance

**Manipulation de bits :**

- Cartes encodées en entiers 64 bits
- Opérations d'ensemble rapides (union, intersection)
- Structures de données cache-friendly

**Traitement parallèle :**

```rust
use rayon::prelude::*;

combos.par_iter()
    .map(|combo| calculate_equity(combo))
    .collect()
```

**Tables de lookup :**

- Classements de flush pré-calculés (8KB)
- Lookup en O(1) pour les scénarios courants

---

## Exemple de code : Ranges par position

```rust
let utg = Range::from_str("77+, ATs+, AJo+, KQs")?;
let btn = Range::from_str(
    "22+, A2s+, A5o+, K6s+, Q8s+, J8s+, T8s+"
)?;

println!("Combos UTG: {}", utg.combo_count(None));
println!("Combos BTN: {}", btn.combo_count(None));

// Calculer le VPIP (% de mains jouées)
let vpip = (btn.combo_count(None) as f64 / 1326.0) * 100.0;
println!("BTN VPIP: {:.1}%", vpip);
```

**Sortie :**

```
Combos UTG: 116 (8.7% VPIP)
Combos BTN: 404 (30.5% VPIP)
```

---

## Exemple de code : Analyse de texture du board

```rust
let range = Range::from_str("88+, ATs+, AJo+, KQs")?;
let board = vec![
    Card::from_str("Ah")?,
    Card::from_str("Kd")?,
    Card::from_str("Qc")?,
];

let dead = CardSet::from_cards(&board);
let remaining = range.combo_count(Some(dead));

println!("Combos qui touchent: {}/{}",
    initial - remaining, initial);
```

---

## Stratégie de test

**Suite de tests complète :**

```
tests/
├── card_set_tests.rs
├── equity_tests.rs
├── evaluator_tests.rs
├── hand_rank_tests.rs
├── hand_tests.rs
└── range_tests.rs
```

**Couverture de test :**

- Tests unitaires pour chaque module
- Tests d'intégration pour les calculs d'équité
- Cas limites (quinte blanche, as-bas)
- Validation du parsing des ranges

---

## Dépendances

```toml
[dependencies]
criterion = "0.8.1"      # Benchmarking
serde = "1.0"            # Sérialisation
ndarray = "0.17.1"       # Opérations matricielles
rand = "0.9.2"           # RNG pour Monte Carlo
rayon = "1.11.0"         # Traitement parallèle
thiserror = "2.0.17"     # Gestion d'erreurs
anyhow = "1.0.100"       # Contextes d'erreurs
```

**Pourquoi ces choix ?**

- Rayon : Parallélisme facile
- Serde : Export/import de données
- Criterion : Benchmarking statistique

---

## Cas d'usage

**1. Logiciel d'entraînement au poker**

- Calculer les cotes du pot
- Analyser les ranges de mains
- Étudier l'équité vs ranges

**2. Analyse stratégique**

- Ranges par position
- Stratégies de 3-bet/4-bet

**3. Recherche & simulation**

- Jeu optimal de théorie des jeux (GTO)
- Distributions d'équité

---

## Améliorations futures

**Fonctionnalités prévues :**

- [ ] Équité range vs range
- [ ] Calculs de pots multi-joueurs
- [ ] Intégration des cotes du pot
- [ ] Intégration de solveur GTO
- [ ] Compilation WASM pour le web
- [ ] Outil CLI pour calculs rapides
- [ ] API REST pour outils externes

**Objectifs de performance :**

- Évaluation de main en moins d'une microseconde
- Plus d'1M de simulations par seconde
- Accélération GPU pour grandes ranges

---

## Merci de votre attention

Des questions ?
