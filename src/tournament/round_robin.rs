//! Round-Robin Tournament
//!
//! Each strategy plays against all others (and against itself).
//! This is the format used by Axelrod in his original tournaments.

use crate::game::{Match, MatchConfig, MatchResult};
use crate::player::Player;
use crate::strategy::StrategyType;
use comfy_table::{Cell, ContentArrangement, Table, presets::UTF8_FULL};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Player score in the tournament
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerScore {
    /// Strategy name
    pub name: String,
    /// Strategy type
    pub strategy_type: StrategyType,
    /// Total score
    pub total_score: i32,
    /// Number of matches played
    pub matches_played: u32,
    /// Average score per match
    pub average_score: f64,
    /// Cooperation rate
    pub cooperation_rate: f64,
    /// Is this a "nice" strategy?
    pub is_nice: bool,
}

/// Complete tournament result
#[derive(Debug, Serialize, Deserialize)]
pub struct TournamentResult {
    /// Player rankings (from best to worst)
    pub rankings: Vec<PlayerScore>,
    /// All match results
    pub match_results: Vec<MatchResult>,
    /// Score matrix (strategy i vs strategy j) - serialized as a list
    #[serde(skip)]
    pub score_matrix: HashMap<(StrategyType, StrategyType), (i32, i32)>,
}

impl TournamentResult {
    /// Returns the tournament winner
    pub fn winner(&self) -> Option<&PlayerScore> {
        self.rankings.first()
    }

    /// Displays the formatted rankings
    pub fn display_rankings(&self) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Rank"),
                Cell::new("Strategy"),
                Cell::new("Score"),
                Cell::new("Avg/Match"),
                Cell::new("Coop%"),
                Cell::new("Nice"),
            ]);

        for (i, player) in self.rankings.iter().enumerate() {
            let nice_str = if player.is_nice { "Yes" } else { "No" };
            table.add_row(vec![
                Cell::new(i + 1),
                Cell::new(&player.name),
                Cell::new(player.total_score),
                Cell::new(format!("{:.1}", player.average_score)),
                Cell::new(format!("{:.1}%", player.cooperation_rate * 100.0)),
                Cell::new(nice_str),
            ]);
        }

        format!("\n        TOURNAMENT RESULTS\n\n{}", table)
    }
}

/// Round-Robin Tournament
pub struct RoundRobinTournament {
    /// Match configuration
    config: MatchConfig,
    /// Strategies participating in the tournament
    strategies: Vec<StrategyType>,
}

impl RoundRobinTournament {
    /// Creates a new tournament with all strategies
    pub fn new(config: MatchConfig) -> Self {
        Self {
            config,
            strategies: StrategyType::all(),
        }
    }

    /// Creates a tournament with specific strategies
    pub fn with_strategies(strategies: Vec<StrategyType>, config: MatchConfig) -> Self {
        Self { config, strategies }
    }
}

impl Default for RoundRobinTournament {
    /// Creates a tournament with default configuration
    fn default() -> Self {
        Self::new(MatchConfig::default())
    }
}

impl RoundRobinTournament {
    /// Runs the tournament and returns the results
    pub fn run(&self) -> TournamentResult {
        // Generate all match pairs
        let n = self.strategies.len();
        let mut match_pairs: Vec<(usize, usize)> = Vec::new();
        for i in 0..n {
            for j in i..n {
                match_pairs.push((i, j));
            }
        }

        // Run matches in parallel
        let match_results: Vec<(usize, usize, MatchResult)> = match_pairs
            .par_iter()
            .map(|&(i, j)| {
                let mut player1 = Player::new(self.strategies[i]);
                let mut player2 = Player::new(self.strategies[j]);

                let result = {
                    let mut game = Match::new(&mut player1, &mut player2, self.config.clone());
                    game.play()
                };

                (i, j, result)
            })
            .collect();

        // Aggregate results
        let mut players: Vec<Player> = self.strategies.iter().map(|s| Player::new(*s)).collect();
        let mut all_match_results = Vec::new();
        let mut score_matrix: HashMap<(StrategyType, StrategyType), (i32, i32)> = HashMap::new();

        for (i, j, result) in match_results {
            // Update player i
            players[i].add_score(result.score1);
            players[i].cooperations += result.cooperations1;
            players[i].rounds_played += result.rounds.len() as u32;
            players[i].matches_played += 1;

            if i != j {
                // Match against a different opponent
                players[j].add_score(result.score2);
                players[j].cooperations += result.cooperations2;
                players[j].rounds_played += result.rounds.len() as u32;
                players[j].matches_played += 1;

                // Also record the reverse match in the matrix
                score_matrix.insert(
                    (self.strategies[j], self.strategies[i]),
                    (result.score2, result.score1),
                );
            }

            // Record in the score matrix
            score_matrix.insert(
                (self.strategies[i], self.strategies[j]),
                (result.score1, result.score2),
            );

            all_match_results.push(result);
        }

        // Create rankings
        let mut rankings: Vec<PlayerScore> = players
            .iter()
            .map(|p| PlayerScore {
                name: p.name.clone(),
                strategy_type: p.strategy_type,
                total_score: p.score,
                matches_played: p.matches_played,
                average_score: p.average_score_per_match(),
                cooperation_rate: p.cooperation_rate(),
                is_nice: p.is_nice(),
            })
            .collect();

        // Sort by descending score
        rankings.sort_by(|a, b| b.total_score.cmp(&a.total_score));

        TournamentResult {
            rankings,
            match_results: all_match_results,
            score_matrix,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tournament_with_two_strategies() {
        let tournament = RoundRobinTournament::with_strategies(
            vec![StrategyType::AlwaysCooperate, StrategyType::AlwaysDefect],
            MatchConfig::with_rounds(10),
        );

        let result = tournament.run();

        // Always Defect should win against Always Cooperate
        assert_eq!(result.rankings.len(), 2);
        assert_eq!(result.rankings[0].strategy_type, StrategyType::AlwaysDefect);
    }

    #[test]
    fn test_tournament_all_strategies() {
        let tournament = RoundRobinTournament::new(MatchConfig::with_rounds(10));
        let result = tournament.run();

        // All strategies should be present
        assert_eq!(result.rankings.len(), StrategyType::all().len());
    }

    #[test]
    fn test_nice_strategies_tend_to_do_well() {
        let tournament = RoundRobinTournament::with_strategies(
            vec![
                StrategyType::TitForTat,
                StrategyType::AlwaysCooperate,
                StrategyType::AlwaysDefect,
                StrategyType::Grudger,
            ],
            MatchConfig::with_rounds(100),
        );

        let result = tournament.run();

        // "Nice" strategies (TFT, AC, Grudger) should generally
        // have a good average score because they cooperate with each other
        // Let's verify that TFT or Grudger is in the top 2
        let top_2_types: Vec<_> = result.rankings[..2]
            .iter()
            .map(|r| r.strategy_type)
            .collect();

        assert!(
            top_2_types.contains(&StrategyType::TitForTat)
                || top_2_types.contains(&StrategyType::Grudger)
        );
    }
}
