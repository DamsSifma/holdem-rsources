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

# Apprendre a faire du Code Haute performance en Rust

## Un Solveur Texas Hold'em (Poker)

Damien Massif
Janvier 2026

---

## Pourquoi Rust ?

**Les problèmes des langages classiques :**

- **C/C++** : Performance maximale, mais bugs mémoire et data races
- **Python** : Simple à écrire, mais trop lent pour le calcul intensif
- **Java/C#** : Garbage collector = pauses imprévisibles

**La promesse de Rust :**

> Performance de C/C++ + Sécurité mémoire garantie à la compilation

**Sans compromis :**

✅ Zero-cost abstractions  
✅ Pas de garbage collector  
✅ Détection des data races à la compilation  
✅ Gestion mémoire sans malloc/free manuel

---

## Rapides règles du Texas Hold'em

**Règles simplifiées :**

1. Chaque joueur reçoit 2 cartes privées (hole cards)
2. 5 cartes communes sont révélées progressivement (3 puis 1 puis 1)
3. Meilleure combinaison de 5 cartes parmi les 7 disponibles gagne

**Exemple de main :**

```
Vos cartes:  A♠ K♠
Table:       Q♠ J♠ T♥ 5♦ 2♣
→ Résultat:  Quinte à l'As (A-K-Q-J-T)
```

**Défi technique :**

- Évaluer des millions de mains par seconde
- Calculer les probabilités de victoire (équité)

---

## Plan de la présentation

1. **Les Bitsets** : Manipulation de bits pour la performance
2. **Cactus Kev** : Algorithme d'évaluation ultra-rapide
3. **Rayon** : Parallélisation sans effort
4. **Benchmarking** : Mesurer la performance avec Criterion et `black_box`
5. **Pourquoi Rust excelle** dans ce contexte

---

# 1. Les Bitsets en Rust

## Représenter 52 cartes en 64 bits

---

## Le problème : Stocker un ensemble de cartes

**Approche naïve (Python/Java) :**

```python
cards = ["As", "Kh", "Qd"]  # Liste de strings
# Tester si une carte est présente : O(n)
# Mémoire : ~50+ octets par carte
```

**Approche Rust avec bitset :**

```rust
pub struct CardSet {
    bits: u64  // Un seul entier de 64 bits !
}
```

- Chaque carte = 1 bit (52 cartes → 52 bits utilisés)
- Test de présence : O(1) avec une seule opération CPU
- Mémoire : 8 octets total, peu importe le nombre de cartes

---

## Implémentation du CardSet

```rust
impl CardSet {
    pub fn new() -> Self {
        CardSet { bits: 0 }
    }

    pub fn insert(&mut self, card: Card) {
        let index = card.index(); // 0..51
        self.bits |= 1u64 << index;  // Insère le bit correspondant
    }

    pub fn contains(&self, card: Card) -> bool {
        let index = card.index();
        (self.bits >> index) & 1 == 1  // Vérifie si le bit d'index est à 1
    }

    pub fn remove(&mut self, card: Card) {
        let index = card.index();
        self.bits &= !(1u64 << index);  // Efface le bit correspondant
    }
}
```

---

## Opérations de bits : Explication visuelle

**Exemple concret : Ajouter le 2♠ (index 5)**

```
État initial (vide) :
bits = 0b...00000000

Étape 1 - Créer un masque (1 << 5) :
masque = 0b...00100000  ← Le bit 5 est à 1
         Position: 543210

Étape 2 - Activer le bit (bits | masque) :
bits   = 0b...00000000  (avant)
masque = 0b...00100000  (OR)
       ---------------
bits   = 0b...00100000  (après) ← Le 2♠ est ajouté !
```

---

## Opérations ensemblistes ultra-rapides

**En Rust avec bitsets :**

```rust
impl CardSet {
    pub fn union(&self, other: &CardSet) -> CardSet {
        CardSet { bits: self.bits | other.bits }
    }

    pub fn intersection(&self, other: &CardSet) -> CardSet {
        CardSet { bits: self.bits & other.bits }
    }

    pub fn difference(&self, other: &CardSet) -> CardSet {
        CardSet { bits: self.bits & !other.bits }
    }

    pub fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }
}
```

---

## Pourquoi c'est important en Rust ?

**Zero-cost abstractions :**

```rust
let mut deck = CardSet::new();
for card in all_cards {
    deck.insert(card);  // Code élégant
}
```

Le compilateur Rust transforme ce code en :

```asm
or rax, rbx  // Une seule instruction assembleur
```

**Pas de surcoût à l'exécution** comparé au code assembleur écrit à la main !

**En Python/Java :** L'abstraction a un coût (appels de fonction, allocation mémoire)

**En Rust :** L'abstraction est gratuite grâce aux optimisations du compilateur

---

# 2. Cactus Kev Algorithm

## Évaluer 7 cartes en ~100ns

---

## Le défi de l'évaluation

**Combinatoire explosive :**

- 7 cartes peuvent former C(7,5) = 21 combinaisons de 5 cartes
- Évaluer chaque combinaison séparément = lent
- Besoin : ~20+ millions d'évaluations par seconde

**Solution : Cactus Kev**

- Tables de lookup précalculées
- Évaluation en temps constant O(1)
- Pas de boucles ni récursion

---

## Architecture de Cactus Kev

**Deux tables de lookup :**

```rust
pub struct LookupEvaluator {
    // Table 1: Pour les flush (couleurs)
    flush_table: [u16; 8192],  // 2^13 = 8192 entrées

    // Table 2: Pas encore implémentée dans notre version,
    // on utilise un calcul optimisé pour les non-flush
}
```

**Pourquoi 8192 entrées ?**

- Chaque carte a une valeur (2..A = 13 valeurs)
- Pour une couleur, on encode 13 bits (présence/absence de chaque valeur)
- 2^13 = 8192 combinaisons possibles

---

## Table de Flush : Génération

```rust
fn generate_flush_table() -> [u16; 8192] {
    let mut table = [0u16; 8192];

    for pattern in 0..8192 {
        // pattern encode quelles valeurs sont présentes
        let rank = evaluate_flush_pattern(pattern);
        table[pattern] = rank;
    }

    table
}
```

**Encodage du pattern :**

```
Pattern = 0b1101000000001  (binaire)
         = A♠ K♠ J♠ 2♠
Position:  AKQJT98765432

→ Meilleure main de 5 cartes : A-K-J-2 (hauteur As)
```

---

## Utilisation de la table de Flush

```rust
fn evaluate_flush(&self, hand: &Hand, suit: Suit) -> Option<u16> {
    let flush_cards = hand.cards_of_suit(suit);

    if flush_cards.len() < 5 {
        return None;  // Pas assez de cartes
    }

    // Convertir les cartes en pattern binaire
    let mut pattern = 0u16;
    for card in flush_cards {
        let value = card.value() as u8;
        pattern |= 1 << value;
    }

    // Lookup O(1) !
    Some(self.flush_table[pattern as usize])
}
```

**Pas de boucles imbriquées, pas de comparaisons → Ultra rapide !**

---

## Évaluation des non-Flush

**Pour les mains sans couleur, on analyse les fréquences :**

```rust
fn evaluate_non_flush(hand: &Hand) -> HandRanking {
    // Compter combien de fois chaque valeur apparaît
    let mut counts = [0u8; 13];
    for card in hand.cards() {
        counts[card.value() as usize] += 1;
    }

    // Trier les valeurs par fréquence
    let pattern = categorize_pattern(&counts);

    match pattern {
        [4, ..] => four_of_a_kind(&counts),
        [3, 2, ..] => full_house(&counts),
        [3, ..] => three_of_a_kind(&counts),
        [2, 2, ..] => two_pair(&counts),
        [2, ..] => one_pair(&counts),
        _ => high_card_or_straight(&counts),
    }
}
```

---

## Avantages de Rust pour Cactus Kev

**1. Tableaux statiques sur la stack :**

```rust
let mut counts = [0u8; 13];  // Stack allocation, pas de malloc
```

**2. Match exhaustif vérifié à la compilation :**

```rust
match pattern {
    [4, ..] => ...,
    [3, 2, ..] => ...,
    // Le compilateur force à couvrir tous les cas
}
```

**3. Types sans surcoût :**

```rust
pub struct HandRanking {
    category: HandCategory,  // enum = 1 octet
    rank: u32,               // score pour départager
}
```

→ Taille totale : 5 octets (vs 24+ octets en Java/C# avec boxing)

---

# 3. Rayon : Parallélisation

## Transformer `iter()` en `par_iter()`

---

## Le problème : Calculs d'équité

**Équité = Probabilité de gagner**

Exemple : `A♠K♠` vs `Q♥Q♦` avec board `J♠T♠5♣`

→ Il faut simuler toutes les Turn/River possibles :

- 47 cartes restantes pour Turn
- 46 cartes pour River
- Total : 47 × 46 = 2,162 simulations

**Pour une range vs range :** Millions de simulations !

---

## Parallélisation naïve (difficile en C/Java)

**Problèmes des threads manuels :**

```c
// En C : gestion manuelle des threads
pthread_t threads[8];
for (int i = 0; i < 8; i++) {
    pthread_create(&threads[i], NULL, worker, &data);
    // Attention aux data races !
    // Attention aux deadlocks !
}
```

**Risques :**

- Data races (accès concurrent à la mémoire)
- Deadlocks (threads qui s'attendent mutuellement)
- Overhead de synchronisation

---

## Rayon : Parallélisation sûre et simple

**Code séquentiel :**

```rust
let results: Vec<f64> = simulations
    .iter()
    .map(|sim| run_simulation(sim))
    .collect();
```

**Code parallèle :**

```rust
use rayon::prelude::*;

let results: Vec<f64> = simulations
    .par_iter()  // ← Seul changement !
    .map(|sim| run_simulation(sim))
    .collect();
```

**Rayon gère automatiquement :**

- Création du thread pool
- Distribution du travail
- Récupération des résultats
- **Sans data races possibles !**

---

## Comment Rayon garantit la sécurité ?

**Le système de types de Rust :**

```rust
pub trait Send {}  // Type transférable entre threads
pub trait Sync {}  // Type partageable entre threads
```

**Rayon refuse de compiler si danger :**

```rust
let mut counter = 0;  // Variable mutable

simulations.par_iter().for_each(|sim| {
    counter += 1;  // ❌ ERREUR DE COMPILATION !
});
```

```
error[E0596]: cannot borrow `counter` as mutable,
as it is a captured variable in a `Fn` closure
```

**Solution correcte : Atomic ou Mutex**

---

## Exemple réel : Range vs Range

```rust
pub fn calculate_equity_range_vs_range(
    range1: &Range,
    range2: &Range,
    board: &CardSet,
) -> (f64, f64) {
    let combos1 = range1.combinations(board);
    let combos2 = range2.combinations(board);

    // Calcul parallèle de toutes les combinaisons
    let results: Vec<_> = combos1
        .par_iter()  // Parallélisation !
        .flat_map(|combo1| {
            combos2.iter().map(|combo2| {
                calculate_equity(combo1, combo2, board)
            })
        })
        .collect();

    // Agréger les résultats
    aggregate_results(&results)
}
```

**Speedup obtenu : ~7x sur un CPU 8-core !**

---

## Work Stealing de Rayon

**Architecture de Rayon :**

```
Thread 1: [Task Task Task ____]
Thread 2: [Task Task Task Task]
Thread 3: [____ ____ ____ ____]  ← Idle !
Thread 4: [Task Task Task Task]
```

**Work Stealing :**

```
Thread 1: [Task Task ____ ____]
Thread 2: [Task Task Task ____]
Thread 3: [Task Task ____ ____]  ← "Vol" de tasks
Thread 4: [Task Task Task ____]
```

→ Équilibrage automatique de charge, maximum d'efficacité !

**Sans effort du développeur :**

- Pas de partitionnement manuel
- Pas de gestion de queue
- Juste `.par_iter()` !

---

# 4. Benchmarking avec Criterion

## Mesurer la performance scientifiquement

---

## Le problème : Optimisations agressives

**Le compilateur est très malin :**

```rust
fn benchmark_simple(b: &mut Bencher) {
    b.iter(|| {
        let hand = parse_hand("As Kh Qd Jc Ts");
        evaluator.evaluate(&hand)
    });
}
```

**Le compilateur peut :**

1. Calculer le résultat à la compilation (constant folding)
2. Éliminer le code mort (dead code elimination)
3. Résultat : Temps mesuré ≈ 0ns (faux !)

---

## La solution : `black_box`

**`std::hint::black_box` dit au compilateur :**

> "Cette valeur vient de l'extérieur, ne l'optimise pas !"

```rust
use std::hint::black_box;

fn benchmark_correct(b: &mut Bencher) {
    b.iter(|| {
        let hand = parse_hand("As Kh Qd Jc Ts");
        evaluator.evaluate(black_box(&hand))
        //                 ^^^^^^^^^^  ← Force le calcul !
    });
}
```

**Résultat :**

- Sans `black_box` : 0.5 ns (faux)
- Avec `black_box` : 95 ns (vrai temps d'évaluation)

---

## Criterion : Framework de benchmarking

```rust
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_hand_evaluation(c: &mut Criterion) {
    let evaluator = LookupEvaluator::new();
    let royal_flush = parse_hand("As Ks Qs Js Ts");

    c.bench_function("royal_flush", |b| {
        b.iter(|| {
            evaluator.evaluate(black_box(&royal_flush))
        })
    });
}

criterion_group!(benches, bench_hand_evaluation);
criterion_main!(benches);
```

**Criterion fournit :**

- Analyse statistique (moyenne, écart-type, percentiles)
- Détection de régressions
- Graphiques HTML
- Comparaison entre versions

---

## Résultats de benchmarks réels

**Évaluation de mains (7 cartes) :**

```
royal_flush         95.2 ns   (±2.1 ns)
straight_flush      98.7 ns   (±1.8 ns)
four_of_a_kind      102.3 ns  (±2.5 ns)
full_house          104.1 ns  (±2.2 ns)
flush               87.5 ns   (±1.9 ns)
straight            110.2 ns  (±3.1 ns)
```

→ ~10 millions d'évaluations par seconde !

**Parsing de ranges :**

```
simple_pairs        1.2 μs    (±45 ns)
medium_range        8.7 μs    (±210 ns)
complex_range       45.3 μs   (±1.2 μs)
```

---

## Exemple complet avec `black_box`

```rust
fn bench_range_vs_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("equity");

    let range1 = Range::from_str("AA, KK, QQ").unwrap();
    let range2 = Range::from_str("22+, A2s+").unwrap();
    let board = CardSet::from_str("As Kh Qd").unwrap();

    group.bench_function("range_vs_range", |b| {
        b.iter(|| {
            calculate_equity_range_vs_range(
                black_box(&range1),
                black_box(&range2),
                black_box(&board),
            )
        })
    });

    group.finish();
}
```

**Sans `black_box` :** Le compilateur pourrait pré-calculer tout !

---

# 5. Pourquoi Rust excelle

## Les avantages démontrés par ce projet

---

## 1. Performance sans compromis

**Vitesses comparables au C/C++ :**

- Évaluation de main : ~95ns (comparable à PokerStove en C++)
- Zero-cost abstractions : le code élégant = code rapide
- Pas de garbage collector → latence prévisible

**Exemple : CardSet**

```rust
// Code Rust élégant
deck.union(&other_deck)

// Compilé en une seule instruction assembleur
or rax, rbx
```

→ Pas de compromis entre lisibilité et performance !

---

## 2. Sécurité mémoire garantie

**Pas de bugs courants en C/C++ :**

- ✅ Pas de use-after-free
- ✅ Pas de double-free
- ✅ Pas de null pointer dereference

**Exemple : Ownership**

```rust
let hand = create_hand();
process_hand(hand);  // hand est "moved"
println!("{:?}", hand);  // ❌ ERREUR DE COMPILATION !
```

```
error[E0382]: borrow of moved value: `hand`
```

→ Les bugs mémoire sont _impossibles_ !

---

## 3. Concurrence sans data races

**Le système de types empêche les bugs :**

```rust
// ✅ Accepté : chaque thread a ses propres données
simulations.par_iter().map(|sim| process(sim))

// ❌ Refusé : partage mutable concurrent
let mut total = 0;
simulations.par_iter().for_each(|_| total += 1);
```

**Comparaison avec d'autres langages :**

- **C/C++** : Data races possibles → bugs aléatoires horribles
- **Java/C#** : Protection via locks → risque de deadlocks
- **Python** : GIL → pas de vrai parallélisme
- **Rust** : Vérification à la compilation → bugs impossibles !

---

## 4. Ecosystem moderne et productif

**Cargo : Gestionnaire de paquets intégré**

```bash
cargo new holdem-rsources    # Créer projet
cargo add rayon              # Ajouter dépendance
cargo test                   # Lancer tests
cargo bench                  # Lancer benchmarks
cargo doc --open             # Générer documentation
```

**Crates (bibliothèques) de qualité :**

- `rayon` : Parallélisation facile
- `criterion` : Benchmarks scientifiques
- `serde` : Serialization/deserialization
- `ndarray` : Calcul numérique

---

## 5. Documentation et Tooling excellents

**Documentation intégrée :**

````rust
/// Évalue une main de poker.
///
/// # Arguments
/// * `hand` - Main de 5 à 7 cartes
///
/// # Examples
/// ```
/// let hand = parse_hand("As Kh Qd Jc Ts");
/// let rank = evaluator.evaluate(&hand);
/// ```
pub fn evaluate(&self, hand: &Hand) -> HandRanking {
    // ...
}
````

**Outils puissants :**

- `clippy` : Linter avancé
- `rustfmt` : Formattage automatique
- `rust-analyzer` : IDE support excellent

---

## 6. Gestion d'erreurs explicite

**Pas d'exceptions cachées :**

```rust
// Parsing de carte peut échouer
pub fn parse_card(s: &str) -> Result<Card, ParseError> {
    if s.len() != 2 {
        return Err(ParseError::InvalidLength);
    }
    // ...
}

// L'appelant DOIT gérer l'erreur
let card = parse_card(input)?;  // Propage l'erreur
// ou
let card = parse_card(input).unwrap_or_default();  // Valeur par défaut
```

**Avantages :**

- Toutes les erreurs sont visibles dans la signature
- Pas de crashes surprises à l'exécution
- Code plus robuste

---

## Résultats concrets de ce projet

**Performance mesurée :**

- 10M+ évaluations de mains/seconde
- Speedup 7x avec parallélisation (8 cores)
- Latence constante (pas de GC pauses)

**Qualité du code :**

- 0 bugs de mémoire (impossible par construction)
- 0 data races (vérifiés à la compilation)
- Tests et benchmarks intégrés

---

## Quand utiliser Rust ?

**✅ Rust est excellent pour :**

- Systèmes de calcul haute performance
- Applications où la latence est critique
- Services web haute concurrence
- Outils CLI et système
- WebAssembly
- Embedded systems

**❌ Rust peut être overkill pour :**

- Prototypes rapides
- Scripts simples
- Applications CRUD basiques
- Projets avec beaucoup de dépendances tierces instables

---

## Fonctionnalités clés du projet

**1. Évaluation de mains**

- Algorithme Cactus Kev avec tables de lookup
- Performance : ~95ns par évaluation (7 cartes)
- Support de toutes les combinaisons poker

**2. Analyse de ranges**

- Parsing de notation poker (AA, AKs, QQ+, etc.)
- Calcul de combinaisons avec dead cards
- Statistiques détaillées (pairs, suited, offsuit)

**3. Calculs d'équité**

- Monte Carlo pour approximations rapides
- Énumération exacte pour précision maximale
- Range vs Range avec parallélisation Rayon

---

## Ressources pour apprendre Rust

**Livres officiels (gratuits) :**

- [The Rust Programming Language](https://doc.rust-lang.org/book/) ("The Book")
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) (unsafe Rust)

**Exercices pratiques :**

- [Rustlings](https://github.com/rust-lang/rustlings) : Petits exercices
- [Exercism Rust Track](https://exercism.org/tracks/rust) : Défis progressifs

---

## Conclusion

**Ce qu'on a vu :**

1. **Bitsets** → Performance maximale avec abstractions zéro-coût
2. **Cactus Kev** → Tables de lookup pour évaluation O(1)
3. **Rayon** → Parallélisation triviale et sûre
4. **Benchmarking** → `black_box` pour mesures précises

**Les super-pouvoirs de Rust :**

- Performance de C/C++ avec sécurité de haut niveau
- Parallélisme sans data races
- Écosystème moderne et productif
- Courbe d'apprentissage raide, mais ça vaut le coup !

---

## Questions ?

**Projet sur GitHub :**
https://github.com/DamsSifma/holdem-rsources

**Merci !**
