# axelrod-rs

[![CI](https://github.com/foufou-exe/axelrod-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/foufou-exe/axelrod-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)

*[Version francaise](README.FR.md)*

A high-performance evolutionary game theory simulator written in Rust. This project explores the **Iterated Prisoner's Dilemma** by pitting various automated strategies against each other to observe the emergence of cooperation, trust, and betrayal in a competitive environment.

Inspired by Robert Axelrod's groundbreaking work *"The Evolution of Cooperation"* (1984) and his computer tournaments (1980-1984).

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Strategies](#strategies)
- [Tournament Modes](#tournament-modes)
- [CLI Reference](#cli-reference)
- [The Prisoner's Dilemma](#the-prisoners-dilemma)
- [Contributing](#contributing)
- [License](#license)

## Features

- **20 classic strategies** from Axelrod's original tournaments (1980-1984)
- **Round-Robin tournaments** - Every strategy plays against all others
- **Ecological/Evolutionary tournaments** - Population dynamics over generations
- **1v1 Match mode** - Direct confrontation between two strategies
- **Parallel execution** - Multi-threaded tournament processing with Rayon
- **Noise simulation** - Introduce random errors to test strategy robustness
- **Reproducible results** - Seed support for deterministic simulations
- **Export results** - CSV and JSON output formats
- **Interactive mode** - User-friendly CLI with `inquire`

## Installation

### From Source

Requires [Rust](https://rustup.rs/) (1.85+ recommended for Rust 2024 edition).

```bash
git clone https://github.com/foufou-exe/axelrod-rs.git
cd axelrod-rs
cargo build --release
```

The binary will be at `target/release/axelrod-rs`.

### From Releases

Download pre-built binaries from the [Releases](https://github.com/foufou-exe/axelrod-rs/releases) page.

## Quick Start

### Interactive Mode

Simply run without arguments for guided interaction:

```bash
./axelrod-rs
```

### Run a Round-Robin Tournament

```bash
./axelrod-rs round-robin --rounds 200
```

### Run an Ecological Tournament

```bash
./axelrod-rs ecological --rounds 200 --generations 100
```

### Run a 1v1 Match

```bash
./axelrod-rs match --strategy1 tit-for-tat --strategy2 random --rounds 200
```

### List All Strategies

```bash
./axelrod-rs strategies
```

## Strategies

All 20 strategies implemented from Axelrod's tournaments:

| Strategy | Description | Nice* |
|----------|-------------|-------|
| **Always Cooperate** | Always cooperates | Yes |
| **Always Defect** | Always defects | No |
| **Tit for Tat** | Cooperates first, then copies opponent's last move | Yes |
| **Suspicious Tit for Tat** | Like TFT but defects first | No |
| **Tit for Two Tats** | Defects only after two consecutive defections | Yes |
| **Generous Tit for Tat** | TFT with 10% chance to forgive defection | Yes |
| **Grudger** | Cooperates until betrayed, then always defects | Yes |
| **Random** | 50/50 cooperation/defection | No |
| **Pavlov** | Win-Stay, Lose-Shift strategy | Yes |
| **Prober** | Tests opponent with D-C-C, then exploits or plays TFT | No |
| **Hard Go By Majority** | Defects if opponent defected >= 50% | No |
| **Soft Go By Majority** | Cooperates if opponent cooperated >= 50% | Yes |
| **Joss** | Sneaky TFT - randomly defects ~10% of the time | No |
| **Graaskamp** | TFT with a probe defection at round 50 | No |
| **Tullock** | Cooperates 11 rounds, then mostly defects | No |
| **Feld** | TFT with decreasing cooperation (100% to 50%) | No |
| **Nydegger** | Lookup table based on last 3 rounds | No |
| **Grofman** | Cooperates if both players matched last round | Yes |
| **Shubik** | Escalating retaliation - N retaliations after Nth betrayal | Yes |
| **Davis** | 10 rounds grace period, then Grudger | Yes |

*\*Nice: Never defects first*

## Tournament Modes

### Round-Robin Tournament

Each strategy plays against every other strategy (including itself). Results are ranked by total score.

```bash
./axelrod-rs round-robin --rounds 200 --output results.csv
```

### Ecological Tournament

Simulates evolution: strategies with higher scores reproduce, lower scores die out. Watch populations evolve over generations.

```bash
./axelrod-rs ecological --rounds 200 --generations 100 --initial-population 100
```

### 1v1 Match

Direct confrontation between two specific strategies with detailed move-by-move analysis.

```bash
./axelrod-rs match -1 grudger -2 prober --rounds 50
```

## CLI Reference

```
axelrod-rs [COMMAND]

Commands:
  round-robin  Run a round-robin tournament
  ecological   Run an ecological/evolutionary tournament
  match        Run a 1v1 match between two strategies
  strategies   List all available strategies
  help         Print help

Global Options:
  --rounds <N>       Number of rounds per match (default: 200)
  --noise <PROB>     Error probability 0.0-1.0 (default: 0.0)
  --seed <N>         Random seed for reproducibility
  --output <FILE>    Export results to CSV or JSON
  --quiet            Minimal output
  -h, --help         Print help
  -V, --version      Print version
```

### Examples

```bash
# Tournament with noise (5% error rate)
./axelrod-rs round-robin --noise 0.05 --rounds 200

# Reproducible ecological simulation
./axelrod-rs ecological --seed 42 --generations 500

# Export match results to JSON
./axelrod-rs match -1 tit-for-tat -2 pavlov --output match.json
```

## The Prisoner's Dilemma

The Prisoner's Dilemma is a fundamental game in game theory. Two players simultaneously choose to **Cooperate (C)** or **Defect (D)**.

### Payoff Matrix

|  | Opponent Cooperates | Opponent Defects |
|--|---------------------|------------------|
| **You Cooperate** | R=3, R=3 | S=0, T=5 |
| **You Defect** | T=5, S=0 | P=1, P=1 |

- **R (Reward)**: 3 points - Mutual cooperation
- **T (Temptation)**: 5 points - You defect, opponent cooperates
- **S (Sucker)**: 0 points - You cooperate, opponent defects
- **P (Punishment)**: 1 point - Mutual defection

The dilemma: Defection is individually rational, but mutual cooperation yields better collective outcomes.

### Axelrod's Discovery

In his 1980s tournaments, Axelrod found that **"nice" strategies** (those that never defect first) consistently outperformed aggressive ones. **Tit for Tat** - the simplest nice strategy - won both tournaments.

Key properties of successful strategies:
1. **Nice** - Don't defect first
2. **Retaliatory** - Punish defection
3. **Forgiving** - Return to cooperation after punishment
4. **Clear** - Be predictable so opponents can adapt

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

```bash
# Run tests
cargo test

# Run with all checks
cargo fmt && cargo clippy && cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
