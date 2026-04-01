//! Module représentant un joueur dans le tournoi

use crate::history::History;
use crate::strategy::{Strategy, StrategyType};

/// Représente un joueur avec sa stratégie et son score
#[derive(Debug)]
pub struct Player {
    /// Nom du joueur (généralement le nom de la stratégie)
    pub name: String,
    /// La stratégie utilisée par ce joueur
    pub strategy: Box<dyn Strategy>,
    /// Type de stratégie (pour clonage et affichage)
    pub strategy_type: StrategyType,
    /// Score total accumulé
    pub score: i32,
    /// Nombre de matchs joués
    pub matches_played: u32,
    /// Nombre de rounds joués
    pub rounds_played: u32,
    /// Nombre de coopérations effectuées
    pub cooperations: u32,
    /// Nombre de trahisons effectuées
    pub defections: u32,
}

impl Player {
    /// Crée un nouveau joueur avec une stratégie donnée
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

    /// Crée un joueur avec un nom personnalisé
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

    /// Décide de l'action à jouer
    pub fn decide(&mut self, history: &History) -> crate::action::Action {
        self.strategy.decide(history)
    }

    /// Ajoute des points au score
    pub fn add_score(&mut self, points: i32) {
        self.score += points;
    }

    /// Enregistre un round joué
    pub fn record_round(&mut self, action: crate::action::Action) {
        self.rounds_played += 1;
        match action {
            crate::action::Action::Cooperate => self.cooperations += 1,
            crate::action::Action::Defect => self.defections += 1,
        }
    }

    /// Enregistre un match terminé
    pub fn record_match(&mut self) {
        self.matches_played += 1;
    }

    /// Réinitialise la stratégie pour un nouveau match
    pub fn reset_strategy(&mut self) {
        self.strategy.reset();
    }

    /// Réinitialise complètement le joueur (score, statistiques, stratégie)
    pub fn reset(&mut self) {
        self.score = 0;
        self.matches_played = 0;
        self.rounds_played = 0;
        self.cooperations = 0;
        self.defections = 0;
        self.strategy.reset();
    }

    /// Retourne le taux de coopération (0.0 à 1.0)
    pub fn cooperation_rate(&self) -> f64 {
        if self.rounds_played == 0 {
            0.0
        } else {
            self.cooperations as f64 / self.rounds_played as f64
        }
    }

    /// Retourne le score moyen par match
    pub fn average_score_per_match(&self) -> f64 {
        if self.matches_played == 0 {
            0.0
        } else {
            self.score as f64 / self.matches_played as f64
        }
    }

    /// Retourne le score moyen par round
    pub fn average_score_per_round(&self) -> f64 {
        if self.rounds_played == 0 {
            0.0
        } else {
            self.score as f64 / self.rounds_played as f64
        }
    }

    /// Indique si la stratégie est "gentille"
    pub fn is_nice(&self) -> bool {
        self.strategy.is_nice()
    }

    /// Clone le joueur avec un nouvel état
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
