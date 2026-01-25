window.BENCHMARK_DATA = {
  "lastUpdate": 1769343891275,
  "repoUrl": "https://github.com/DamsSifma/holdem-rsources",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "dmassif@centrale-marseille.fr",
            "name": "dmassif",
            "username": "DamsSifma"
          },
          "committer": {
            "email": "dmassif@centrale-marseille.fr",
            "name": "dmassif",
            "username": "DamsSifma"
          },
          "distinct": true,
          "id": "063a97d6e5b068b7c8ebd149e1e1a9d910ece3f7",
          "message": "Fix formatting",
          "timestamp": "2026-01-25T13:17:01+01:00",
          "tree_id": "694fa89c30a315f47d0131596b2f52c982fa77c3",
          "url": "https://github.com/DamsSifma/holdem-rsources/commit/063a97d6e5b068b7c8ebd149e1e1a9d910ece3f7"
        },
        "date": 1769343891048,
        "tool": "cargo",
        "benches": [
          {
            "name": "hand_evaluation/royal_flush",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/straight_flush",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/four_of_a_kind",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/full_house",
            "value": 69,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/flush",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/straight",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/three_of_a_kind",
            "value": 125,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/two_pair",
            "value": 68,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/one_pair",
            "value": 123,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/high_card",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/seven_card_hand",
            "value": 130,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/simple_pairs",
            "value": 194,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/medium_range",
            "value": 335,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/complex_range",
            "value": 861,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/typical_3bet_range",
            "value": 1655,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/small",
            "value": 299,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/medium",
            "value": 1648,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/large",
            "value": 2942,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/very_large",
            "value": 7096,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/preflop_AA_vs_KK",
            "value": 5390905,
            "range": "± 21035",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/preflop_AK_vs_QQ",
            "value": 5307571,
            "range": "± 21794",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/flop_equity",
            "value": 4738819,
            "range": "± 13747",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/turn_equity",
            "value": 4650939,
            "range": "± 15629",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/river_exact",
            "value": 435,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/1000",
            "value": 538759,
            "range": "± 1263",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/5000",
            "value": 2691582,
            "range": "± 7777",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/10000",
            "value": 5382664,
            "range": "± 12961",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/50000",
            "value": 26947296,
            "range": "± 64665",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/100000",
            "value": 53836522,
            "range": "± 554072",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_equity/small_ranges_1000",
            "value": 77890388,
            "range": "± 356080",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_equity/medium_ranges_1000",
            "value": 206072743,
            "range": "± 1045732",
            "unit": "ns/iter"
          },
          {
            "name": "card_operations/card_creation",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "card_operations/card_parsing",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "card_operations/cardset_insert",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "card_operations/cardset_contains",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "card_operations/holecards_parsing",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}