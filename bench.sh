#!/bin/bash
# Benchmark runner helper script for holdem-rsources

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== holdem-rsources Benchmark Suite ===${NC}\n"

# Function to run a specific benchmark group
run_bench() {
    local group=$1
    local sample_size=${2:-100}
    echo -e "${GREEN}Running benchmark: ${group}${NC}"
    cargo bench --bench holdem_benchmarks -- "$group" --sample-size "$sample_size"
}

# Function to run quick benchmarks
quick_bench() {
    echo -e "${YELLOW}Running quick benchmarks (20 samples)...${NC}\n"
    run_bench "hand_evaluation" 20
    run_bench "range_parsing" 20
    run_bench "card_operations" 20
}

# Function to run full benchmarks
full_bench() {
    echo -e "${YELLOW}Running full benchmarks (100 samples)...${NC}\n"
    cargo bench --bench holdem_benchmarks
}

# Function to run equity benchmarks (slower)
equity_bench() {
    echo -e "${YELLOW}Running equity benchmarks (may take several minutes)...${NC}\n"
    run_bench "equity_calculation" 10
    run_bench "equity_simulation_sizes" 10
    run_bench "range_vs_range_equity" 10
}

# Function to save baseline
save_baseline() {
    local name=${1:-main}
    echo -e "${YELLOW}Saving baseline as '${name}'...${NC}\n"
    cargo bench --bench holdem_benchmarks -- --save-baseline "$name"
}

# Function to compare with baseline
compare_baseline() {
    local name=${1:-main}
    echo -e "${YELLOW}Comparing with baseline '${name}'...${NC}\n"
    cargo bench --bench holdem_benchmarks -- --baseline "$name"
}

# Parse command line arguments
case "${1:-}" in
    quick)
        quick_bench
        ;;
    full)
        full_bench
        ;;
    equity)
        equity_bench
        ;;
    hand|eval|evaluation)
        run_bench "hand_evaluation" "${2:-100}"
        ;;
    range)
        run_bench "range_parsing" "${2:-100}"
        run_bench "range_expansion" "${2:-100}"
        ;;
    card|cards)
        run_bench "card_operations" "${2:-100}"
        ;;
    save)
        save_baseline "${2:-main}"
        ;;
    compare)
        compare_baseline "${2:-main}"
        ;;
    report)
        echo -e "${GREEN}Opening benchmark report in browser...${NC}"
        open target/criterion/report/index.html 2>/dev/null || \
        xdg-open target/criterion/report/index.html 2>/dev/null || \
        echo "Could not open browser. Please open target/criterion/report/index.html manually"
        ;;
    help|--help|-h)
        echo "Usage: $0 [command] [options]"
        echo ""
        echo "Commands:"
        echo "  quick              Run quick benchmarks (20 samples) for basic groups"
        echo "  full               Run all benchmarks with default settings"
        echo "  equity             Run equity calculation benchmarks (slower)"
        echo "  hand [samples]     Run hand evaluation benchmarks"
        echo "  range [samples]    Run range parsing and expansion benchmarks"
        echo "  card [samples]     Run card operations benchmarks"
        echo "  save [name]        Save current results as baseline (default: 'main')"
        echo "  compare [name]     Compare with saved baseline (default: 'main')"
        echo "  report             Open HTML benchmark report in browser"
        echo "  help               Show this help message"
        echo ""
        echo "Examples:"
        echo "  $0 quick                    # Quick benchmark run"
        echo "  $0 hand 50                  # Benchmark hand evaluation with 50 samples"
        echo "  $0 save before_opt          # Save baseline before optimization"
        echo "  $0 compare before_opt       # Compare current with 'before_opt' baseline"
        ;;
    *)
        echo -e "${YELLOW}No command specified. Running quick benchmarks...${NC}\n"
        echo -e "Use '${GREEN}$0 help${NC}' for more options.\n"
        quick_bench
        ;;
esac

echo -e "\n${BLUE}Benchmark complete!${NC}"
echo -e "View detailed results: ${GREEN}$0 report${NC}"
