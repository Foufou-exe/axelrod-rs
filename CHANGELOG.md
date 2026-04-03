# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **8 new historical strategies** from Axelrod's first tournament (1980):
  - **Joss**: Sneaky Tit for Tat - cooperates like TFT but randomly defects ~10% of the time
  - **Graaskamp**: TFT with a probe - defects on round 50 to test opponent's response
  - **Tullock**: Cooperates for 11 rounds, then mostly defects (10% cooperation)
  - **Feld**: TFT with decreasing cooperation probability (100% → 50% over 200 rounds)
  - **Nydegger**: Uses a lookup table based on the last 3 rounds of play
  - **Grofman**: Cooperates if both players made the same action last round
  - **Shubik**: Escalating retaliation - retaliates N times after Nth betrayal
  - **Davis**: 10 rounds grace period, then permanent Grudger behavior

### Changed

- Total strategies increased from 12 to 20

## [0.2.0] - 2026-04-01

### Added

- **CLI with clap**: Full command-line interface with subcommands
  - `round-robin` - Run round-robin tournaments from CLI
  - `ecological` - Run ecological/evolutionary tournaments from CLI
  - `match` - Run 1v1 matches between specific strategies
  - `strategies` - List all available strategies
- **Export to CSV/JSON**: Export tournament and match results
  - `--output results.json` or `--output results.csv`
  - Automatic format detection based on file extension
- **Seed support**: Reproducible simulations with `--seed <u64>`
- **Noise/error probability**: Simulate execution errors with `--noise 0.05`
  - Actions may be randomly flipped with the specified probability
  - Useful for studying robustness of strategies
- **Parallel execution**: Tournament matches run in parallel using rayon
  - Significant speedup for round-robin and ecological tournaments
- **Quiet mode**: `--quiet` flag for minimal output (useful for scripting)

### Changed

- Interactive mode is now the fallback when no subcommand is provided
- Replaced manual ASCII table rendering with `comfy-table` library for better display
- Tables now automatically adapt to terminal width with dynamic content arrangement
- Improved readability of tournament results, population evolution, and match displays
- Updated noise configuration in interactive mode prompts

### Technical

- Added dependencies: `clap`, `serde`, `serde_json`, `csv`, `rayon`, `rand_chacha`
- New modules: `cli.rs` (CLI definitions), `export.rs` (CSV/JSON export)
- All result structs now derive `Serialize, Deserialize` for export support
- `MatchConfig::with_rounds_and_noise()` constructor for noise configuration
- `Match::play_with_rng()` method for seeded/reproducible matches
- `StrategyType::from_name()` for CLI strategy lookup

### CI/CD

- Added GitHub Actions CI workflow (`.github/workflows/ci.yml`)
  - Multi-platform builds (Linux, Windows, macOS)
  - Automated tests, clippy lints, and format checks
- Added GitHub Actions release workflow (`.github/workflows/release.yml`)
  - Automatic binary releases on version tags
  - Cross-compilation for Linux, Windows, macOS (x86_64 + ARM64)

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

[Unreleased]: https://github.com/thibautmaurras/axelrod-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/thibautmaurras/axelrod-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/thibautmaurras/axelrod-rs/releases/tag/v0.1.0
