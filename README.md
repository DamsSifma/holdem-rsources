# holdem-rsources

A high-performance Texas Hold'em poker library written in Rust.

## Features

- **Hand Evaluation**: Fast 7-card hand evaluation with complete hand ranking
- **Equity Calculation**: Monte Carlo simulation for equity analysis (hand vs hand, range vs range)
- **Range Parsing**: Support for poker range notation (e.g., `AA, KK+, AKs, JTs+`)
- **Parallel Processing**: Multi-threaded equity calculations using Rayon
- **Optimized Performance**: Bitset-based card representation for efficient operations

## Usage

```bash
cargo build --release
cargo test
cargo bench
```
