//! Tournoi Écologique / Évolutionnaire
//!
//! Simule l'évolution des populations de stratégies sur plusieurs générations.
//! La prévalence de chaque stratégie à chaque génération est déterminée par
//! son succès à la génération précédente.
//!
//! C'est ce type de tournoi qui a montré que les stratégies "gentilles"
//! finissent par dominer dans les travaux d'Axelrod.

use crate::game::{Match, MatchConfig};
use crate::player::Player;
use crate::strategy::StrategyType;
use std::collections::HashMap;

/// Configuration du tournoi écologique
#[derive(Debug, Clone)]
pub struct EcologicalConfig {
    /// Configuration des matchs
    pub match_config: MatchConfig,
    /// Nombre de générations à simuler
    pub generations: u32,
    /// Population initiale de chaque stratégie
    pub initial_population: u32,
    /// Seuil minimum de population (en dessous, la stratégie est éliminée)
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

/// État d'une génération
#[derive(Debug, Clone)]
pub struct Generation {
    /// Numéro de la génération
    pub number: u32,
    /// Population de chaque stratégie
    pub populations: HashMap<StrategyType, u32>,
    /// Score moyen de chaque stratégie cette génération
    pub average_scores: HashMap<StrategyType, f64>,
}

impl Generation {
    /// Retourne la population totale
    pub fn total_population(&self) -> u32 {
        self.populations.values().sum()
    }

    /// Retourne les stratégies encore vivantes
    pub fn alive_strategies(&self) -> Vec<StrategyType> {
        self.populations
            .iter()
            .filter(|&(_, &pop)| pop > 0)
            .map(|(&s, _)| s)
            .collect()
    }

    /// Retourne la stratégie dominante
    pub fn dominant_strategy(&self) -> Option<StrategyType> {
        self.populations
            .iter()
            .max_by_key(|&(_, &pop)| pop)
            .map(|(&s, _)| s)
    }

    /// Retourne le pourcentage de population pour chaque stratégie
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

/// Tournoi écologique / évolutionnaire
pub struct EcologicalTournament {
    /// Configuration
    config: EcologicalConfig,
    /// Stratégies participantes
    strategies: Vec<StrategyType>,
}

impl EcologicalTournament {
    /// Crée un nouveau tournoi avec toutes les stratégies
    pub fn new(config: EcologicalConfig) -> Self {
        Self {
            config,
            strategies: StrategyType::all(),
        }
    }

    /// Crée un tournoi avec des stratégies spécifiques
    pub fn with_strategies(strategies: Vec<StrategyType>, config: EcologicalConfig) -> Self {
        Self { config, strategies }
    }

    /// Lance le tournoi et retourne l'historique des générations
    pub fn run(&self) -> Vec<Generation> {
        let mut generations = Vec::with_capacity(self.config.generations as usize);

        // Génération initiale
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

        // Simulation des générations
        for gen_num in 1..=self.config.generations {
            // Jouer les matchs et calculer les scores
            let scores = self.play_generation(&populations);

            // Calculer les scores moyens
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

            // Calculer les nouvelles populations
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

            // Ajuster pour maintenir la population totale
            let current_total: u32 = new_populations.values().sum();
            if current_total > 0 && current_total != new_total_population {
                // Trouver la stratégie avec le meilleur score et ajuster
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

            // Vérifier si une seule stratégie reste
            let alive: Vec<_> = populations.iter().filter(|&(_, &p)| p > 0).collect();
            if alive.len() <= 1 {
                break;
            }
        }

        generations
    }

    /// Joue tous les matchs d'une génération et retourne les scores totaux
    fn play_generation(
        &self,
        populations: &HashMap<StrategyType, u32>,
    ) -> HashMap<StrategyType, i32> {
        let mut scores: HashMap<StrategyType, i32> =
            self.strategies.iter().map(|&s| (s, 0)).collect();

        // Liste des stratégies actives
        let active: Vec<_> = populations
            .iter()
            .filter(|&(_, &pop)| pop > 0)
            .map(|(&s, _)| s)
            .collect();

        // Chaque stratégie joue contre toutes les autres
        for i in 0..active.len() {
            for j in i..active.len() {
                let strategy_i = active[i];
                let strategy_j = active[j];

                let pop_i = *populations.get(&strategy_i).unwrap_or(&0);
                let pop_j = *populations.get(&strategy_j).unwrap_or(&0);

                if pop_i == 0 || pop_j == 0 {
                    continue;
                }

                // Jouer un match représentatif
                let mut player1 = Player::new(strategy_i);
                let mut player2 = Player::new(strategy_j);

                let result = {
                    let mut game =
                        Match::new(&mut player1, &mut player2, self.config.match_config.clone());
                    game.play()
                };

                // Pondérer les scores par la population
                // Nombre d'interactions proportionnel aux populations
                let interactions = if i == j {
                    // Match contre soi-même: n*(n-1)/2 interactions
                    (pop_i * (pop_i - 1)) / 2
                } else {
                    // Match contre autre: n*m interactions
                    pop_i * pop_j
                };

                if i == j {
                    // Auto-match: les deux scores vont à la même stratégie
                    let entry = scores.entry(strategy_i).or_insert(0);
                    *entry += (result.score1 + result.score2) * interactions as i32 / 2;
                } else {
                    let entry_i = scores.entry(strategy_i).or_insert(0);
                    *entry_i += result.score1 * interactions as i32;

                    let entry_j = scores.entry(strategy_j).or_insert(0);
                    *entry_j += result.score2 * interactions as i32;
                }
            }
        }

        scores
    }

    /// Affiche l'évolution des populations
    pub fn display_evolution(generations: &[Generation]) -> String {
        let mut output = String::new();
        output.push_str(
            "\n╔═══════════════════════════════════════════════════════════════════════════╗\n",
        );
        output.push_str(
            "║                    ÉVOLUTION DES POPULATIONS                              ║\n",
        );
        output.push_str(
            "╠═══════════════════════════════════════════════════════════════════════════╣\n",
        );

        // Afficher quelques générations clés
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
            output.push_str(&format!("║ Génération {:>3}: ", generation.number));

            let percentages = generation.population_percentages();
            let mut sorted: Vec<_> = percentages.iter().filter(|&(_, &p)| p > 0.5).collect();
            sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

            let display: Vec<String> = sorted
                .iter()
                .take(4)
                .map(|(s, p)| format!("{}: {:.1}%", s, p))
                .collect();

            output.push_str(&format!("{:<57}║\n", display.join(", ")));
        }

        output.push_str(
            "╠═══════════════════════════════════════════════════════════════════════════╣\n",
        );

        // Afficher le résultat final
        if let Some(last_gen) = generations.last() {
            output.push_str(
                "║ RÉSULTAT FINAL:                                                           ║\n",
            );
            if let Some(dominant) = last_gen.dominant_strategy() {
                let pop_pct = last_gen.population_percentages();
                let pct = pop_pct.get(&dominant).unwrap_or(&0.0);
                output.push_str(&format!(
                    "║   Stratégie dominante: {} ({:.1}% de la population)        \n",
                    dominant, pct
                ));
            }
            output.push_str(&format!(
                "║   Stratégies survivantes: {}                                              \n",
                last_gen.alive_strategies().len()
            ));
        }

        output.push_str(
            "╚═══════════════════════════════════════════════════════════════════════════╝\n",
        );
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

        // Au moins la génération initiale
        assert!(!generations.is_empty());

        // Génération 0 devrait avoir des populations égales
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

        // Le tournoi devrait produire plusieurs générations
        assert!(generations.len() > 1);

        // La population totale devrait rester constante
        let initial_total = generations[0].total_population();
        for generation in &generations {
            assert_eq!(generation.total_population(), initial_total);
        }
    }
}
