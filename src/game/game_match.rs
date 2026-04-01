//! Management of a complete match between two players
//!
//! A match consists of multiple rounds (default 200, as in Axelrod's tournaments).

use crate::action::Action;
use crate::history::History;
use crate::payoff::PayoffMatrix;
use crate::player::Player;
use rand::{Rng, RngExt};
use serde::{Deserialize, Serialize};

/// Match configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchConfig {
    /// Number of rounds per match
    pub rounds: u32,
    /// Payoff matrix to use
    pub payoff_matrix: PayoffMatrix,
    /// Noise probability (0.0 to 1.0) - chance of action being flipped
    pub noise: f64,
}

impl MatchConfig {
    /// Default configuration (200 rounds, classic matrix, no noise)
    pub fn default() -> Self {
        Self {
            rounds: 200,
            payoff_matrix: PayoffMatrix::classic(),
            noise: 0.0,
        }
    }

    /// Custom configuration
    pub fn new(rounds: u32, payoff_matrix: PayoffMatrix) -> Self {
        Self {
            rounds,
            payoff_matrix,
            noise: 0.0,
        }
    }

    /// Configuration with a custom number of rounds
    pub fn with_rounds(rounds: u32) -> Self {
        Self {
            rounds,
            payoff_matrix: PayoffMatrix::classic(),
            noise: 0.0,
        }
    }

    /// Configuration with rounds and noise
    pub fn with_rounds_and_noise(rounds: u32, noise: f64) -> Self {
        Self {
            rounds,
            payoff_matrix: PayoffMatrix::classic(),
            noise: noise.clamp(0.0, 1.0),
        }
    }

    /// Set noise probability
    pub fn set_noise(&mut self, noise: f64) -> &mut Self {
        self.noise = noise.clamp(0.0, 1.0);
        self
    }
}

/// Result of a round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundResult {
    /// Action of player 1 (after noise)
    pub action1: Action,
    /// Action of player 2 (after noise)
    pub action2: Action,
    /// Points earned by player 1
    pub score1: i32,
    /// Points earned by player 2
    pub score2: i32,
}

/// Complete result of a match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    /// Name of player 1
    pub player1_name: String,
    /// Name of player 2
    pub player2_name: String,
    /// Final score of player 1
    pub score1: i32,
    /// Final score of player 2
    pub score2: i32,
    /// Number of cooperations by player 1
    pub cooperations1: u32,
    /// Number of cooperations by player 2
    pub cooperations2: u32,
    /// History of all rounds
    pub rounds: Vec<RoundResult>,
}

impl MatchResult {
    /// Returns the winner (None if tie)
    pub fn winner(&self) -> Option<&str> {
        if self.score1 > self.score2 {
            Some(&self.player1_name)
        } else if self.score2 > self.score1 {
            Some(&self.player2_name)
        } else {
            None
        }
    }

    /// Returns the cooperation rate of player 1
    pub fn cooperation_rate1(&self) -> f64 {
        if self.rounds.is_empty() {
            0.0
        } else {
            self.cooperations1 as f64 / self.rounds.len() as f64
        }
    }

    /// Returns the cooperation rate of player 2
    pub fn cooperation_rate2(&self) -> f64 {
        if self.rounds.is_empty() {
            0.0
        } else {
            self.cooperations2 as f64 / self.rounds.len() as f64
        }
    }

    /// Returns the mutual cooperation rate
    pub fn mutual_cooperation_rate(&self) -> f64 {
        if self.rounds.is_empty() {
            0.0
        } else {
            let mutual = self
                .rounds
                .iter()
                .filter(|r| r.action1 == Action::Cooperate && r.action2 == Action::Cooperate)
                .count();
            mutual as f64 / self.rounds.len() as f64
        }
    }
}

/// Represents a match between two players
pub struct Match<'a> {
    /// Player 1
    player1: &'a mut Player,
    /// Player 2
    player2: &'a mut Player,
    /// Match configuration
    config: MatchConfig,
}

impl<'a> Match<'a> {
    /// Creates a new match
    pub fn new(player1: &'a mut Player, player2: &'a mut Player, config: MatchConfig) -> Self {
        Self {
            player1,
            player2,
            config,
        }
    }

    /// Creates a match with default configuration
    pub fn with_defaults(player1: &'a mut Player, player2: &'a mut Player) -> Self {
        Self::new(player1, player2, MatchConfig::default())
    }

    /// Apply noise to an action (potentially flip it)
    fn apply_noise(&self, action: Action, rng: &mut impl Rng) -> Action {
        if self.config.noise > 0.0 && rng.random_range(0.0..1.0) < self.config.noise {
            action.opposite()
        } else {
            action
        }
    }

    /// Plays the match and returns the result
    pub fn play(&mut self) -> MatchResult {
        let mut rng = rand::rng();
        self.play_with_rng(&mut rng)
    }

    /// Plays the match with a specific RNG (for reproducibility)
    pub fn play_with_rng(&mut self, rng: &mut impl Rng) -> MatchResult {
        // Reset strategies for a new match
        self.player1.reset_strategy();
        self.player2.reset_strategy();

        // Histories from each player's perspective
        let mut history1 = History::with_capacity(self.config.rounds as usize);
        let mut history2 = History::with_capacity(self.config.rounds as usize);

        let mut total_score1 = 0;
        let mut total_score2 = 0;
        let mut cooperations1 = 0;
        let mut cooperations2 = 0;
        let mut rounds = Vec::with_capacity(self.config.rounds as usize);

        for _ in 0..self.config.rounds {
            // Each player decides their action
            let intended_action1 = self.player1.decide(&history1);
            let intended_action2 = self.player2.decide(&history2);

            // Apply noise (potential execution error)
            let action1 = self.apply_noise(intended_action1, rng);
            let action2 = self.apply_noise(intended_action2, rng);

            // Calculate payoffs
            let (score1, score2) = self.config.payoff_matrix.get_payoffs(action1, action2);

            // Update scores
            total_score1 += score1;
            total_score2 += score2;

            // Count cooperations (actual actions after noise)
            if action1 == Action::Cooperate {
                cooperations1 += 1;
            }
            if action2 == Action::Cooperate {
                cooperations2 += 1;
            }

            // Record player statistics
            self.player1.add_score(score1);
            self.player1.record_round(action1);
            self.player2.add_score(score2);
            self.player2.record_round(action2);

            // Update histories (with actual actions, so players see what actually happened)
            history1.push(action1, action2);
            history2.push(action2, action1);

            // Record the round
            rounds.push(RoundResult {
                action1,
                action2,
                score1,
                score2,
            });
        }

        // Record matches
        self.player1.record_match();
        self.player2.record_match();

        MatchResult {
            player1_name: self.player1.name.clone(),
            player2_name: self.player2.name.clone(),
            score1: total_score1,
            score2: total_score2,
            cooperations1,
            cooperations2,
            rounds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::StrategyType;

    #[test]
    fn test_match_config_default() {
        let config = MatchConfig::default();
        assert_eq!(config.rounds, 200);
        assert_eq!(config.noise, 0.0);
    }

    #[test]
    fn test_always_cooperate_vs_always_cooperate() {
        let mut player1 = Player::new(StrategyType::AlwaysCooperate);
        let mut player2 = Player::new(StrategyType::AlwaysCooperate);

        let config = MatchConfig::with_rounds(10);
        let mut game = Match::new(&mut player1, &mut player2, config);
        let result = game.play();

        // 10 rounds of mutual cooperation: 10 * 3 = 30 each
        assert_eq!(result.score1, 30);
        assert_eq!(result.score2, 30);
        assert_eq!(result.cooperations1, 10);
        assert_eq!(result.cooperations2, 10);
    }

    #[test]
    fn test_always_cooperate_vs_always_defect() {
        let mut player1 = Player::new(StrategyType::AlwaysCooperate);
        let mut player2 = Player::new(StrategyType::AlwaysDefect);

        let config = MatchConfig::with_rounds(10);
        let mut game = Match::new(&mut player1, &mut player2, config);
        let result = game.play();

        // 10 rounds: Coop gets S=0, Defect gets T=5
        assert_eq!(result.score1, 0); // 10 * 0
        assert_eq!(result.score2, 50); // 10 * 5
        assert_eq!(result.winner(), Some("Always Defect"));
    }

    #[test]
    fn test_tit_for_tat_vs_always_cooperate() {
        let mut player1 = Player::new(StrategyType::TitForTat);
        let mut player2 = Player::new(StrategyType::AlwaysCooperate);

        let config = MatchConfig::with_rounds(10);
        let mut game = Match::new(&mut player1, &mut player2, config);
        let result = game.play();

        // TFT always cooperates with a cooperator
        assert_eq!(result.score1, 30);
        assert_eq!(result.score2, 30);
    }

    #[test]
    fn test_tit_for_tat_vs_always_defect() {
        let mut player1 = Player::new(StrategyType::TitForTat);
        let mut player2 = Player::new(StrategyType::AlwaysDefect);

        let config = MatchConfig::with_rounds(10);
        let mut game = Match::new(&mut player1, &mut player2, config);
        let result = game.play();

        // Round 1: TFT cooperates (score S=0), Defect defects (score T=5)
        // Rounds 2-10: TFT defects, Defect defects (P=1 each)
        // TFT: 0 + 9*1 = 9
        // Defect: 5 + 9*1 = 14
        assert_eq!(result.score1, 9);
        assert_eq!(result.score2, 14);
    }

    #[test]
    fn test_match_result_mutual_cooperation_rate() {
        let mut player1 = Player::new(StrategyType::TitForTat);
        let mut player2 = Player::new(StrategyType::TitForTat);

        let config = MatchConfig::with_rounds(100);
        let mut game = Match::new(&mut player1, &mut player2, config);
        let result = game.play();

        // Two TFTs always cooperate together
        assert!((result.mutual_cooperation_rate() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_noise_config() {
        let config = MatchConfig::with_rounds_and_noise(100, 0.1);
        assert_eq!(config.rounds, 100);
        assert_eq!(config.noise, 0.1);

        // Test clamping
        let config2 = MatchConfig::with_rounds_and_noise(100, 1.5);
        assert_eq!(config2.noise, 1.0);

        let config3 = MatchConfig::with_rounds_and_noise(100, -0.5);
        assert_eq!(config3.noise, 0.0);
    }
}
