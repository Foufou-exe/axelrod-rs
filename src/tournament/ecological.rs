//! Ecological / Evolutionary Tournament
//!
//! Simulates the evolution of strategy populations over multiple generations.
//! The prevalence of each strategy at each generation is determined by
//! its success in the previous generation.
//!
//! This type of tournament showed that "nice" strategies
//! eventually dominate in Axelrod's work.

use crate::game::{Match, MatchConfig};
use crate::player::Player;
use crate::strategy::StrategyType;
use comfy_table::{presets::UTF8_FULL, Cell, ContentArrangement, Table};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ecological tournament configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcologicalConfig {
    /// Match configuration
    pub match_config: MatchConfig,
    /// Number of generations to simulate
    pub generations: u32,
    /// Initial population of each strategy
    pub initial_population: u32,
    /// Minimum population threshold (below this, the strategy is eliminated)
    pub extinction_threshold: u32,
}

impl EcologicalConfig {
    pub fn default() -> Self {
        Self {
            match_config: MatchConfig::default(),
            generations: 100,
            initial_population: 100,
            extinction_threshold: 1,
        }
    }

    pub fn new(
        match_config: MatchConfig,
        generations: u32,
        initial_population: u32,
        extinction_threshold: u32,
    ) -> Self {
        Self {
            match_config,
            generations,
            initial_population,
            extinction_threshold,
        }
    }
}

/// State of a generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generation {
    /// Generation number
    pub number: u32,
    /// Population of each strategy
    pub populations: HashMap<StrategyType, u32>,
    /// Average score of each strategy this generation
    pub average_scores: HashMap<StrategyType, f64>,
}

impl Generation {
    /// Returns the total population
    pub fn total_population(&self) -> u32 {
        self.populations.values().sum()
    }

    /// Returns the strategies still alive
    pub fn alive_strategies(&self) -> Vec<StrategyType> {
        self.populations
            .iter()
            .filter(|&(_, &pop)| pop > 0)
            .map(|(&s, _)| s)
            .collect()
    }

    /// Returns the dominant strategy
    pub fn dominant_strategy(&self) -> Option<StrategyType> {
        self.populations
            .iter()
            .max_by_key(|&(_, &pop)| pop)
            .map(|(&s, _)| s)
    }

    /// Returns the population percentage for each strategy
    pub fn population_percentages(&self) -> HashMap<StrategyType, f64> {
        let total = self.total_population() as f64;
        if total == 0.0 {
            return HashMap::new();
        }
        self.populations
            .iter()
            .map(|(&s, &pop)| (s, pop as f64 / total * 100.0))
            .collect()
    }
}

/// Ecological / Evolutionary tournament
pub struct EcologicalTournament {
    /// Configuration
    config: EcologicalConfig,
    /// Participating strategies
    strategies: Vec<StrategyType>,
}

impl EcologicalTournament {
    /// Creates a new tournament with all strategies
    pub fn new(config: EcologicalConfig) -> Self {
        Self {
            config,
            strategies: StrategyType::all(),
        }
    }

    /// Creates a tournament with specific strategies
    pub fn with_strategies(strategies: Vec<StrategyType>, config: EcologicalConfig) -> Self {
        Self { config, strategies }
    }

    /// Runs the tournament and returns the generation history
    pub fn run(&self) -> Vec<Generation> {
        let mut generations = Vec::with_capacity(self.config.generations as usize);

        // Initial generation
        let mut populations: HashMap<StrategyType, u32> = self
            .strategies
            .iter()
            .map(|&s| (s, self.config.initial_population))
            .collect();

        let initial_gen = Generation {
            number: 0,
            populations: populations.clone(),
            average_scores: HashMap::new(),
        };
        generations.push(initial_gen);

        // Simulation of generations
        for gen_num in 1..=self.config.generations {
            // Play matches and calculate scores
            let scores = self.play_generation(&populations);

            // Calculate average scores
            let total_population: u32 = populations.values().sum();
            if total_population == 0 {
                break;
            }

            let mut average_scores: HashMap<StrategyType, f64> = HashMap::new();
            for (&strategy, &score) in &scores {
                if let Some(&pop) = populations.get(&strategy) {
                    if pop > 0 {
                        average_scores.insert(strategy, score as f64 / pop as f64);
                    }
                }
            }

            // Calculate new populations
            let total_score: i64 = scores.values().map(|&s| s as i64).sum();
            if total_score <= 0 {
                break;
            }

            let new_total_population = total_population;
            let mut new_populations: HashMap<StrategyType, u32> = HashMap::new();

            for &strategy in &self.strategies {
                let score = *scores.get(&strategy).unwrap_or(&0) as i64;
                let new_pop = ((score * new_total_population as i64) / total_score) as u32;

                if new_pop >= self.config.extinction_threshold {
                    new_populations.insert(strategy, new_pop);
                } else {
                    new_populations.insert(strategy, 0);
                }
            }

            // Adjust to maintain total population
            let current_total: u32 = new_populations.values().sum();
            if current_total > 0 && current_total != new_total_population {
                // Find the strategy with the best score and adjust
                if let Some(&best_strategy) = scores
                    .iter()
                    .filter(|&(&s, _)| *new_populations.get(&s).unwrap_or(&0) > 0)
                    .max_by_key(|&(_, &score)| score)
                    .map(|(s, _)| s)
                {
                    let diff = new_total_population as i32 - current_total as i32;
                    if let Some(pop) = new_populations.get_mut(&best_strategy) {
                        *pop = (*pop as i32 + diff).max(0) as u32;
                    }
                }
            }

            populations = new_populations;

            let generation = Generation {
                number: gen_num,
                populations: populations.clone(),
                average_scores,
            };
            generations.push(generation);

            // Check if only one strategy remains
            let alive: Vec<_> = populations.iter().filter(|&(_, &p)| p > 0).collect();
            if alive.len() <= 1 {
                break;
            }
        }

        generations
    }

    /// Plays all matches of a generation and returns total scores (parallelized)
    fn play_generation(
        &self,
        populations: &HashMap<StrategyType, u32>,
    ) -> HashMap<StrategyType, i32> {
        // List of active strategies
        let active: Vec<_> = populations
            .iter()
            .filter(|&(_, &pop)| pop > 0)
            .map(|(&s, _)| s)
            .collect();

        // Generate all match pairs
        let mut match_pairs: Vec<(usize, usize)> = Vec::new();
        for i in 0..active.len() {
            for j in i..active.len() {
                match_pairs.push((i, j));
            }
        }

        // Run matches in parallel
        let match_results: Vec<(usize, usize, i32, i32)> = match_pairs
            .par_iter()
            .map(|&(i, j)| {
                let strategy_i = active[i];
                let strategy_j = active[j];

                let mut player1 = Player::new(strategy_i);
                let mut player2 = Player::new(strategy_j);

                let result = {
                    let mut game =
                        Match::new(&mut player1, &mut player2, self.config.match_config.clone());
                    game.play()
                };

                (i, j, result.score1, result.score2)
            })
            .collect();

        // Aggregate scores
        let mut scores: HashMap<StrategyType, i32> =
            self.strategies.iter().map(|&s| (s, 0)).collect();

        for (i, j, score1, score2) in match_results {
            let strategy_i = active[i];
            let strategy_j = active[j];

            let pop_i = *populations.get(&strategy_i).unwrap_or(&0);
            let pop_j = *populations.get(&strategy_j).unwrap_or(&0);

            if pop_i == 0 || pop_j == 0 {
                continue;
            }

            // Weight scores by population
            let interactions = if i == j {
                // Self-match: n*(n-1)/2 interactions
                (pop_i * (pop_i - 1)) / 2
            } else {
                // Match against other: n*m interactions
                pop_i * pop_j
            };

            if i == j {
                // Self-match: both scores go to the same strategy
                let entry = scores.entry(strategy_i).or_insert(0);
                *entry += (score1 + score2) * interactions as i32 / 2;
            } else {
                let entry_i = scores.entry(strategy_i).or_insert(0);
                *entry_i += score1 * interactions as i32;

                let entry_j = scores.entry(strategy_j).or_insert(0);
                *entry_j += score2 * interactions as i32;
            }
        }

        scores
    }

    /// Displays population evolution
    pub fn display_evolution(generations: &[Generation]) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Gen"),
                Cell::new("Top Strategies (by population %)"),
            ]);

        // Display some key generations
        let key_gens: Vec<usize> = if generations.len() <= 10 {
            (0..generations.len()).collect()
        } else {
            let step = generations.len() / 10;
            (0..10)
                .map(|i| i * step)
                .chain(std::iter::once(generations.len() - 1))
                .collect()
        };

        for &gen_idx in &key_gens {
            if gen_idx >= generations.len() {
                continue;
            }
            let generation = &generations[gen_idx];

            let percentages = generation.population_percentages();
            let mut sorted: Vec<_> = percentages.iter().filter(|&(_, &p)| p > 0.5).collect();
            sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

            let display: Vec<String> = sorted
                .iter()
                .take(4)
                .map(|(s, p)| format!("{}: {:.1}%", s, p))
                .collect();

            table.add_row(vec![
                Cell::new(generation.number),
                Cell::new(display.join(", ")),
            ]);
        }

        let mut output = format!("\n      POPULATION EVOLUTION\n\n{}\n", table);

        // Display the final result
        if let Some(last_gen) = generations.last() {
            let mut result_table = Table::new();
            result_table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![Cell::new("FINAL RESULT")]);

            if let Some(dominant) = last_gen.dominant_strategy() {
                let pop_pct = last_gen.population_percentages();
                let pct = pop_pct.get(&dominant).unwrap_or(&0.0);
                result_table.add_row(vec![Cell::new(format!(
                    "Dominant: {} ({:.1}%)",
                    dominant, pct
                ))]);
            }
            result_table.add_row(vec![Cell::new(format!(
                "Surviving strategies: {}",
                last_gen.alive_strategies().len()
            ))]);

            output.push_str(&format!("\n{}", result_table));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_generation() {
        let config = EcologicalConfig {
            match_config: MatchConfig::with_rounds(10),
            generations: 5,
            initial_population: 10,
            extinction_threshold: 1,
        };

        let tournament = EcologicalTournament::with_strategies(
            vec![StrategyType::TitForTat, StrategyType::AlwaysDefect],
            config,
        );

        let generations = tournament.run();

        // At least the initial generation
        assert!(!generations.is_empty());

        // Generation 0 should have equal populations
        let gen0 = &generations[0];
        assert_eq!(gen0.populations.get(&StrategyType::TitForTat), Some(&10));
        assert_eq!(gen0.populations.get(&StrategyType::AlwaysDefect), Some(&10));
    }

    #[test]
    fn test_ecological_evolution() {
        let config = EcologicalConfig {
            match_config: MatchConfig::with_rounds(50),
            generations: 20,
            initial_population: 100,
            extinction_threshold: 1,
        };

        let tournament = EcologicalTournament::with_strategies(
            vec![
                StrategyType::TitForTat,
                StrategyType::AlwaysCooperate,
                StrategyType::AlwaysDefect,
            ],
            config,
        );

        let generations = tournament.run();

        // The tournament should produce multiple generations
        assert!(generations.len() > 1);

        // Total population should remain constant
        let initial_total = generations[0].total_population();
        for generation in &generations {
            assert_eq!(generation.total_population(), initial_total);
        }
    }
}
