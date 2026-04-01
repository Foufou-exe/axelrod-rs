//! Module representing a player in the tournament

use crate::history::History;
use crate::strategy::{Strategy, StrategyType};

/// Represents a player with their strategy and score
#[derive(Debug)]
pub struct Player {
    /// Player name (usually the strategy name)
    pub name: String,
    /// The strategy used by this player
    pub strategy: Box<dyn Strategy>,
    /// Strategy type (for cloning and display)
    pub strategy_type: StrategyType,
    /// Total accumulated score
    pub score: i32,
    /// Number of matches played
    pub matches_played: u32,
    /// Number of rounds played
    pub rounds_played: u32,
    /// Number of cooperations made
    pub cooperations: u32,
    /// Number of defections made
    pub defections: u32,
}

impl Player {
    /// Creates a new player with a given strategy
    pub fn new(strategy_type: StrategyType) -> Self {
        let strategy = strategy_type.create();
        Self {
            name: strategy.name().to_string(),
            strategy,
            strategy_type,
            score: 0,
            matches_played: 0,
            rounds_played: 0,
            cooperations: 0,
            defections: 0,
        }
    }

    /// Creates a player with a custom name
    pub fn with_name(name: String, strategy_type: StrategyType) -> Self {
        let strategy = strategy_type.create();
        Self {
            name,
            strategy,
            strategy_type,
            score: 0,
            matches_played: 0,
            rounds_played: 0,
            cooperations: 0,
            defections: 0,
        }
    }

    /// Decides which action to play
    pub fn decide(&mut self, history: &History) -> crate::action::Action {
        self.strategy.decide(history)
    }

    /// Adds points to the score
    pub fn add_score(&mut self, points: i32) {
        self.score += points;
    }

    /// Records a played round
    pub fn record_round(&mut self, action: crate::action::Action) {
        self.rounds_played += 1;
        match action {
            crate::action::Action::Cooperate => self.cooperations += 1,
            crate::action::Action::Defect => self.defections += 1,
        }
    }

    /// Records a completed match
    pub fn record_match(&mut self) {
        self.matches_played += 1;
    }

    /// Resets the strategy for a new match
    pub fn reset_strategy(&mut self) {
        self.strategy.reset();
    }

    /// Fully resets the player (score, statistics, strategy)
    pub fn reset(&mut self) {
        self.score = 0;
        self.matches_played = 0;
        self.rounds_played = 0;
        self.cooperations = 0;
        self.defections = 0;
        self.strategy.reset();
    }

    /// Returns the cooperation rate (0.0 to 1.0)
    pub fn cooperation_rate(&self) -> f64 {
        if self.rounds_played == 0 {
            0.0
        } else {
            self.cooperations as f64 / self.rounds_played as f64
        }
    }

    /// Returns the average score per match
    pub fn average_score_per_match(&self) -> f64 {
        if self.matches_played == 0 {
            0.0
        } else {
            self.score as f64 / self.matches_played as f64
        }
    }

    /// Returns the average score per round
    pub fn average_score_per_round(&self) -> f64 {
        if self.rounds_played == 0 {
            0.0
        } else {
            self.score as f64 / self.rounds_played as f64
        }
    }

    /// Indicates if the strategy is "nice"
    pub fn is_nice(&self) -> bool {
        self.strategy.is_nice()
    }

    /// Clones the player with a fresh state
    pub fn clone_fresh(&self) -> Self {
        Self::new(self.strategy_type)
    }
}

impl Clone for Player {
    fn clone(&self) -> Self {
        let mut player = Self::new(self.strategy_type);
        player.name = self.name.clone();
        player.score = self.score;
        player.matches_played = self.matches_played;
        player.rounds_played = self.rounds_played;
        player.cooperations = self.cooperations;
        player.defections = self.defections;
        player
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Action;
    use crate::history::History;

    #[test]
    fn test_new_player() {
        let player = Player::new(StrategyType::TitForTat);
        assert_eq!(player.name, "Tit for Tat");
        assert_eq!(player.score, 0);
        assert_eq!(player.matches_played, 0);
    }

    #[test]
    fn test_decide() {
        let mut player = Player::new(StrategyType::AlwaysCooperate);
        let history = History::new();
        assert_eq!(player.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_add_score() {
        let mut player = Player::new(StrategyType::TitForTat);
        player.add_score(10);
        assert_eq!(player.score, 10);
        player.add_score(5);
        assert_eq!(player.score, 15);
    }

    #[test]
    fn test_record_round() {
        let mut player = Player::new(StrategyType::TitForTat);
        player.record_round(Action::Cooperate);
        player.record_round(Action::Cooperate);
        player.record_round(Action::Defect);

        assert_eq!(player.rounds_played, 3);
        assert_eq!(player.cooperations, 2);
        assert_eq!(player.defections, 1);
    }

    #[test]
    fn test_cooperation_rate() {
        let mut player = Player::new(StrategyType::TitForTat);
        player.record_round(Action::Cooperate);
        player.record_round(Action::Cooperate);
        player.record_round(Action::Defect);
        player.record_round(Action::Defect);

        assert!((player.cooperation_rate() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_reset() {
        let mut player = Player::new(StrategyType::TitForTat);
        player.add_score(100);
        player.record_match();
        player.record_round(Action::Cooperate);

        player.reset();

        assert_eq!(player.score, 0);
        assert_eq!(player.matches_played, 0);
        assert_eq!(player.rounds_played, 0);
    }
}
