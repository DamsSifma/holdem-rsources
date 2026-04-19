window.BENCHMARK_DATA = {
  "lastUpdate": 1776631462687,
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
      },
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
          "id": "e5070c245838cde764f6f3068c1c82a7792067e0",
          "message": "Add multi-threaded equity calculations",
          "timestamp": "2026-01-27T11:41:30+01:00",
          "tree_id": "914aea08be56368604678471984f41c38de3ad92",
          "url": "https://github.com/DamsSifma/holdem-rsources/commit/e5070c245838cde764f6f3068c1c82a7792067e0"
        },
        "date": 1769510967575,
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
            "range": "± 1",
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/straight",
            "value": 30,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/three_of_a_kind",
            "value": 130,
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
            "value": 122,
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
            "value": 126,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/simple_pairs",
            "value": 195,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/medium_range",
            "value": 344,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/complex_range",
            "value": 860,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/typical_3bet_range",
            "value": 1681,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/small",
            "value": 299,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/medium",
            "value": 1620,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/large",
            "value": 2812,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/very_large",
            "value": 7057,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/preflop_AA_vs_KK",
            "value": 5348558,
            "range": "± 13856",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/preflop_AK_vs_QQ",
            "value": 5278447,
            "range": "± 85082",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/flop_equity",
            "value": 4797778,
            "range": "± 55296",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/turn_equity",
            "value": 4689013,
            "range": "± 28125",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/river_exact",
            "value": 434,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/1000",
            "value": 533851,
            "range": "± 2103",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/5000",
            "value": 2666809,
            "range": "± 5172",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/10000",
            "value": 5343969,
            "range": "± 21286",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/50000",
            "value": 26683549,
            "range": "± 100808",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/100000",
            "value": 53384960,
            "range": "± 166637",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_equity/small_ranges_1000",
            "value": 31011512,
            "range": "± 184505",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_equity/medium_ranges_1000",
            "value": 82211630,
            "range": "± 1693985",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_parallel_vs_sequential/parallel",
            "value": 69373322,
            "range": "± 541153",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_parallel_vs_sequential/sequential",
            "value": 176206150,
            "range": "± 464902",
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
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "card_operations/cardset_insert",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "card_operations/cardset_contains",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "card_operations/holecards_parsing",
            "value": 19,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dmassif@centrale-marseille.fr",
            "name": "dmassif",
            "username": "DamsSifma"
          },
          "committer": {
            "email": "119428147+DamsSifma@users.noreply.github.com",
            "name": "Damien Massif",
            "username": "DamsSifma"
          },
          "distinct": true,
          "id": "bfa3a90ab707e145c8f498ef9438bd90c628eea4",
          "message": "Fix bug and add tie test cases",
          "timestamp": "2026-01-28T12:12:55+01:00",
          "tree_id": "c4d6396b8d0008a84851a3990f01e850af28ae4f",
          "url": "https://github.com/DamsSifma/holdem-rsources/commit/bfa3a90ab707e145c8f498ef9438bd90c628eea4"
        },
        "date": 1769599342912,
        "tool": "cargo",
        "benches": [
          {
            "name": "hand_evaluation/royal_flush",
            "value": 23,
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
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/full_house",
            "value": 61,
            "range": "± 0",
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
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/three_of_a_kind",
            "value": 120,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/two_pair",
            "value": 61,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/one_pair",
            "value": 115,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/high_card",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/seven_card_hand",
            "value": 120,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/simple_pairs",
            "value": 196,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/medium_range",
            "value": 340,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/complex_range",
            "value": 890,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/typical_3bet_range",
            "value": 1716,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/small",
            "value": 350,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/medium",
            "value": 1791,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/large",
            "value": 2846,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/very_large",
            "value": 7197,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/preflop_AA_vs_KK",
            "value": 5148639,
            "range": "± 20018",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/preflop_AK_vs_QQ",
            "value": 5070986,
            "range": "± 13885",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/flop_equity",
            "value": 4314528,
            "range": "± 39925",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/turn_equity",
            "value": 4209347,
            "range": "± 16688",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/river_exact",
            "value": 444,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/1000",
            "value": 515345,
            "range": "± 4814",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/5000",
            "value": 2575531,
            "range": "± 11342",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/10000",
            "value": 5147366,
            "range": "± 9647",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/50000",
            "value": 25732939,
            "range": "± 96277",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/100000",
            "value": 51487094,
            "range": "± 201891",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_equity/small_ranges_1000",
            "value": 29083404,
            "range": "± 179053",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_equity/medium_ranges_1000",
            "value": 77956492,
            "range": "± 891338",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_parallel_vs_sequential/parallel",
            "value": 65354821,
            "range": "± 587031",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_parallel_vs_sequential/sequential",
            "value": 169171642,
            "range": "± 551480",
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
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "card_operations/cardset_insert",
            "value": 2,
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
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/3_players_1000_sims",
            "value": 398370,
            "range": "± 29130",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/4_players_1000_sims",
            "value": 476054,
            "range": "± 36946",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/6_players_1000_sims",
            "value": 611254,
            "range": "± 34213",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/9_players_1000_sims",
            "value": 877728,
            "range": "± 41871",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/3_players_river_exact",
            "value": 528,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_parallel_vs_sequential/3_players_parallel",
            "value": 3308438,
            "range": "± 62620",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_parallel_vs_sequential/3_players_sequential",
            "value": 7514320,
            "range": "± 27621",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_parallel_vs_sequential/6_players_parallel",
            "value": 5306113,
            "range": "± 56180",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_parallel_vs_sequential/6_players_sequential",
            "value": 13041157,
            "range": "± 334767",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "119428147+DamsSifma@users.noreply.github.com",
            "name": "Damien Massif",
            "username": "DamsSifma"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5f2aa32e0dfe84d12a988e756bad1e4074c3b5f5",
          "message": "Tweaks pour front (#2)",
          "timestamp": "2026-03-04T10:56:30+01:00",
          "tree_id": "65956e152f818d270ff0e1a9d4fe6dfa67dda3c8",
          "url": "https://github.com/DamsSifma/holdem-rsources/commit/5f2aa32e0dfe84d12a988e756bad1e4074c3b5f5"
        },
        "date": 1772618798538,
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
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/full_house",
            "value": 62,
            "range": "± 0",
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
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/three_of_a_kind",
            "value": 121,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/two_pair",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/one_pair",
            "value": 125,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/high_card",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/seven_card_hand",
            "value": 139,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/simple_pairs",
            "value": 198,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/medium_range",
            "value": 351,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/complex_range",
            "value": 893,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/typical_3bet_range",
            "value": 1728,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/small",
            "value": 302,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/medium",
            "value": 1702,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/large",
            "value": 2829,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/very_large",
            "value": 7175,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/preflop_AA_vs_KK",
            "value": 5468108,
            "range": "± 21328",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/preflop_AK_vs_QQ",
            "value": 5388677,
            "range": "± 87831",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/flop_equity",
            "value": 4830291,
            "range": "± 34721",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/turn_equity",
            "value": 4664187,
            "range": "± 25701",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/river_exact",
            "value": 449,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/1000",
            "value": 546784,
            "range": "± 2861",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/5000",
            "value": 2737790,
            "range": "± 8434",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/10000",
            "value": 5464949,
            "range": "± 25582",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/50000",
            "value": 27409990,
            "range": "± 389507",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/100000",
            "value": 54620286,
            "range": "± 162998",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_equity/small_ranges_1000",
            "value": 31721329,
            "range": "± 134522",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_equity/medium_ranges_1000",
            "value": 83851800,
            "range": "± 500936",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_parallel_vs_sequential/parallel",
            "value": 71914017,
            "range": "± 514616",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_parallel_vs_sequential/sequential",
            "value": 178567325,
            "range": "± 835193",
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
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "card_operations/cardset_insert",
            "value": 2,
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
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/3_players_1000_sims",
            "value": 382362,
            "range": "± 16103",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/4_players_1000_sims",
            "value": 456742,
            "range": "± 15847",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/6_players_1000_sims",
            "value": 607449,
            "range": "± 14919",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/9_players_1000_sims",
            "value": 859657,
            "range": "± 27215",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/3_players_river_exact",
            "value": 541,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_parallel_vs_sequential/3_players_parallel",
            "value": 3595363,
            "range": "± 66012",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_parallel_vs_sequential/3_players_sequential",
            "value": 7924562,
            "range": "± 43624",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_parallel_vs_sequential/6_players_parallel",
            "value": 5682004,
            "range": "± 76551",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_parallel_vs_sequential/6_players_sequential",
            "value": 13505044,
            "range": "± 69616",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "119428147+DamsSifma@users.noreply.github.com",
            "name": "Damien Massif",
            "username": "DamsSifma"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "88e67a73e4da4bd4ee563a4ce17c30c57015a3fd",
          "message": "Add weighted range support (#3)",
          "timestamp": "2026-04-19T22:34:33+02:00",
          "tree_id": "c7790b7c67e4ab1359ef3519a1c447acd2070f99",
          "url": "https://github.com/DamsSifma/holdem-rsources/commit/88e67a73e4da4bd4ee563a4ce17c30c57015a3fd"
        },
        "date": 1776631462394,
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
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/four_of_a_kind",
            "value": 73,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/full_house",
            "value": 74,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/flush",
            "value": 25,
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
            "value": 143,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/two_pair",
            "value": 72,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/one_pair",
            "value": 124,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/high_card",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hand_evaluation/seven_card_hand",
            "value": 130,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/simple_pairs",
            "value": 198,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/medium_range",
            "value": 340,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/complex_range",
            "value": 863,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "range_parsing/typical_3bet_range",
            "value": 1711,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/small",
            "value": 302,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/medium",
            "value": 1709,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/large",
            "value": 2898,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "range_expansion/very_large",
            "value": 7115,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/preflop_AA_vs_KK",
            "value": 5099719,
            "range": "± 93969",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/preflop_AK_vs_QQ",
            "value": 5148780,
            "range": "± 93917",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/flop_equity",
            "value": 4436797,
            "range": "± 38016",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/turn_equity",
            "value": 4329999,
            "range": "± 61428",
            "unit": "ns/iter"
          },
          {
            "name": "equity_calculation/river_exact",
            "value": 442,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/1000",
            "value": 519397,
            "range": "± 11119",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/5000",
            "value": 2559030,
            "range": "± 52440",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/10000",
            "value": 5282024,
            "range": "± 170812",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/50000",
            "value": 26423128,
            "range": "± 1150237",
            "unit": "ns/iter"
          },
          {
            "name": "equity_simulation_sizes/100000",
            "value": 50774829,
            "range": "± 793667",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_equity/small_ranges_1000",
            "value": 30258742,
            "range": "± 372013",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_equity/medium_ranges_1000",
            "value": 78420855,
            "range": "± 1769412",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_parallel_vs_sequential/parallel",
            "value": 68362055,
            "range": "± 602528",
            "unit": "ns/iter"
          },
          {
            "name": "range_vs_range_parallel_vs_sequential/sequential",
            "value": 169550908,
            "range": "± 4512007",
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
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "card_operations/cardset_insert",
            "value": 2,
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
          },
          {
            "name": "multiway_equity/3_players_1000_sims",
            "value": 383121,
            "range": "± 6602",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/4_players_1000_sims",
            "value": 448442,
            "range": "± 3933",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/6_players_1000_sims",
            "value": 586621,
            "range": "± 10761",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/9_players_1000_sims",
            "value": 817822,
            "range": "± 20751",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_equity/3_players_river_exact",
            "value": 523,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_parallel_vs_sequential/3_players_parallel",
            "value": 3413658,
            "range": "± 52249",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_parallel_vs_sequential/3_players_sequential",
            "value": 7316598,
            "range": "± 147872",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_parallel_vs_sequential/6_players_parallel",
            "value": 5434274,
            "range": "± 55642",
            "unit": "ns/iter"
          },
          {
            "name": "multiway_parallel_vs_sequential/6_players_sequential",
            "value": 13016682,
            "range": "± 262724",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}