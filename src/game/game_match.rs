//! Gestion d'un match complet entre deux joueurs
//!
//! Un match consiste en plusieurs rounds (par défaut 200, comme les tournois d'Axelrod).

use crate::action::Action;
use crate::history::History;
use crate::payoff::PayoffMatrix;
use crate::player::Player;

/// Configuration d'un match
#[derive(Debug, Clone)]
pub struct MatchConfig {
    /// Nombre de rounds par match
    pub rounds: u32,
    /// Matrice de gains à utiliser
    pub payoff_matrix: PayoffMatrix,
}

impl MatchConfig {
    /// Configuration par défaut (200 rounds, matrice classique)
    pub fn default() -> Self {
        Self {
            rounds: 200,
            payoff_matrix: PayoffMatrix::classic(),
        }
    }

    /// Configuration personnalisée
    pub fn new(rounds: u32, payoff_matrix: PayoffMatrix) -> Self {
        Self {
            rounds,
            payoff_matrix,
        }
    }

    /// Configuration avec un nombre de rounds personnalisé
    pub fn with_rounds(rounds: u32) -> Self {
        Self {
            rounds,
            payoff_matrix: PayoffMatrix::classic(),
        }
    }
}

/// Résultat d'un round
#[derive(Debug, Clone)]
pub struct RoundResult {
    /// Action du joueur 1
    pub action1: Action,
    /// Action du joueur 2
    pub action2: Action,
    /// Points gagnés par le joueur 1
    pub score1: i32,
    /// Points gagnés par le joueur 2
    pub score2: i32,
}

/// Résultat complet d'un match
#[derive(Debug, Clone)]
pub struct MatchResult {
    /// Nom du joueur 1
    pub player1_name: String,
    /// Nom du joueur 2
    pub player2_name: String,
    /// Score final du joueur 1
    pub score1: i32,
    /// Score final du joueur 2
    pub score2: i32,
    /// Nombre de coopérations du joueur 1
    pub cooperations1: u32,
    /// Nombre de coopérations du joueur 2
    pub cooperations2: u32,
    /// Historique de tous les rounds
    pub rounds: Vec<RoundResult>,
}

impl MatchResult {
    /// Retourne le gagnant (None si égalité)
    pub fn winner(&self) -> Option<&str> {
        if self.score1 > self.score2 {
            Some(&self.player1_name)
        } else if self.score2 > self.score1 {
            Some(&self.player2_name)
        } else {
            None
        }
    }

    /// Retourne le taux de coopération du joueur 1
    pub fn cooperation_rate1(&self) -> f64 {
        if self.rounds.is_empty() {
            0.0
        } else {
            self.cooperations1 as f64 / self.rounds.len() as f64
        }
    }

    /// Retourne le taux de coopération du joueur 2
    pub fn cooperation_rate2(&self) -> f64 {
        if self.rounds.is_empty() {
            0.0
        } else {
            self.cooperations2 as f64 / self.rounds.len() as f64
        }
    }

    /// Retourne le taux de coopération mutuelle
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

/// Représente un match entre deux joueurs
pub struct Match<'a> {
    /// Joueur 1
    player1: &'a mut Player,
    /// Joueur 2
    player2: &'a mut Player,
    /// Configuration du match
    config: MatchConfig,
}

impl<'a> Match<'a> {
    /// Crée un nouveau match
    pub fn new(player1: &'a mut Player, player2: &'a mut Player, config: MatchConfig) -> Self {
        Self {
            player1,
            player2,
            config,
        }
    }

    /// Crée un match avec la configuration par défaut
    pub fn with_defaults(player1: &'a mut Player, player2: &'a mut Player) -> Self {
        Self::new(player1, player2, MatchConfig::default())
    }

    /// Joue le match et retourne le résultat
    pub fn play(&mut self) -> MatchResult {
        // Réinitialise les stratégies pour un nouveau match
        self.player1.reset_strategy();
        self.player2.reset_strategy();

        // Historiques du point de vue de chaque joueur
        let mut history1 = History::with_capacity(self.config.rounds as usize);
        let mut history2 = History::with_capacity(self.config.rounds as usize);

        let mut total_score1 = 0;
        let mut total_score2 = 0;
        let mut cooperations1 = 0;
        let mut cooperations2 = 0;
        let mut rounds = Vec::with_capacity(self.config.rounds as usize);

        for _ in 0..self.config.rounds {
            // Chaque joueur décide de son action
            let action1 = self.player1.decide(&history1);
            let action2 = self.player2.decide(&history2);

            // Calcul des gains
            let (score1, score2) = self.config.payoff_matrix.get_payoffs(action1, action2);

            // Mise à jour des scores
            total_score1 += score1;
            total_score2 += score2;

            // Comptage des coopérations
            if action1 == Action::Cooperate {
                cooperations1 += 1;
            }
            if action2 == Action::Cooperate {
                cooperations2 += 1;
            }

            // Enregistrement des statistiques des joueurs
            self.player1.add_score(score1);
            self.player1.record_round(action1);
            self.player2.add_score(score2);
            self.player2.record_round(action2);

            // Mise à jour des historiques
            history1.push(action1, action2);
            history2.push(action2, action1);

            // Enregistrement du round
            rounds.push(RoundResult {
                action1,
                action2,
                score1,
                score2,
            });
        }

        // Enregistrement des matchs
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
    }

    #[test]
    fn test_always_cooperate_vs_always_cooperate() {
        let mut player1 = Player::new(StrategyType::AlwaysCooperate);
        let mut player2 = Player::new(StrategyType::AlwaysCooperate);

        let config = MatchConfig::with_rounds(10);
        let mut game = Match::new(&mut player1, &mut player2, config);
        let result = game.play();

        // 10 rounds de coopération mutuelle: 10 * 3 = 30 chacun
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

        // 10 rounds: Coop gagne S=0, Defect gagne T=5
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

        // TFT coopère toujours avec un coopérateur
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

        // Round 1: TFT coopère (score S=0), Defect trahit (score T=5)
        // Rounds 2-10: TFT trahit, Defect trahit (P=1 chacun)
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

        // Deux TFT coopèrent toujours ensemble
        assert!((result.mutual_cooperation_rate() - 1.0).abs() < 0.001);
    }
}
