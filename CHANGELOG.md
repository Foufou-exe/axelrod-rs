# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Replaced manual ASCII table rendering with `comfy-table` library for better display
- Tables now automatically adapt to terminal width with dynamic content arrangement
- Improved readability of tournament results, population evolution, and match displays

## [0.1.0] - 2026-04-01

### Added

- Initial release of the Iterated Prisoner's Dilemma simulator
- **11 classic strategies** from Axelrod's original tournaments (1980-1984):
  - Always Cooperate
  - Always Defect
  - Tit for Tat (tournament winner)
  - Suspicious Tit for Tat
  - Tit for Two Tats
  - Generous Tit for Tat
  - Grudger (Grim Trigger)
  - Pavlov (Win-Stay, Lose-Shift)
  - Prober
  - Go By Majority (Soft Majority)
  - Random
- **Round-Robin Tournament**: Each strategy plays against all others (including itself)
- **Ecological/Evolutionary Tournament**: Population-based simulation over multiple generations
- **1v1 Match mode**: Direct confrontation between two selected strategies
- Classic payoff matrix: R=3 (Reward), T=5 (Temptation), S=0 (Sucker), P=1 (Punishment)
- Configurable rounds per match (default: 200, as in Axelrod's tournaments)
- Interactive CLI interface using `inquire`
- Cooperation rate tracking and analysis
- "Nice" strategy classification (never defects first)

### Technical

- Built with Rust 2024 edition
- Uses `rand 0.10.0` with updated API (`rand::rng()`, `RngExt::random_range()`)
- Handles Rust 2024 reserved keyword `gen` appropriately

[Unreleased]: https://github.com/yourusername/axelrod-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/axelrod-rs/releases/tag/v0.1.0
