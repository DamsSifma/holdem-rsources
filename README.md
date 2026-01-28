# holdem-rsources

A high-performance Texas Hold'em poker library written in Rust.

## Features

- **Hand Evaluation**: Fast 7-card hand evaluation with complete hand ranking
- **Equity Calculation**: 
  - Heads-up (2 players): Monte Carlo and exact equity calculation
  - **Multi-way (3-9 players)**: Monte Carlo simulation for multi-way pots
- **Range Parsing**: Support for poker range notation (e.g., `AA, KK+, AKs, JTs+`)
- **Parallel Processing**: Multi-threaded equity calculations using Rayon
- **Optimized Performance**: Bitset-based card representation for efficient operations

## Usage

```bash
cargo build --release
cargo test
cargo bench
```

Bench comparison per commit is available via GitHub Actions [here](https://damssifma.github.io/holdem-rsources/dev/bench/)