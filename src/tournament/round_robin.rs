//! Tournoi Round-Robin
//!
//! Chaque stratégie joue contre toutes les autres (et contre elle-même).
//! C'est le format utilisé par Axelrod dans ses tournois originaux.

use crate::game::{Match, MatchConfig, MatchResult};
use crate::player::Player;
use crate::strategy::StrategyType;
use std::collections::HashMap;

/// Score d'un joueur dans le tournoi
#[derive(Debug, Clone)]
pub struct PlayerScore {
    /// Nom de la stratégie
    pub name: String,
    /// Type de stratégie
    pub strategy_type: StrategyType,
    /// Score total
    pub total_score: i32,
    /// Nombre de matchs joués
    pub matches_played: u32,
    /// Score moyen par match
    pub average_score: f64,
    /// Taux de coopération
    pub cooperation_rate: f64,
    /// Est-ce une stratégie "gentille"?
    pub is_nice: bool,
}

/// Résultat complet du tournoi
#[derive(Debug)]
pub struct TournamentResult {
    /// Classement des joueurs (du meilleur au moins bon)
    pub rankings: Vec<PlayerScore>,
    /// Tous les résultats de matchs
    pub match_results: Vec<MatchResult>,
    /// Matrice des scores (stratégie i vs stratégie j)
    pub score_matrix: HashMap<(StrategyType, StrategyType), (i32, i32)>,
}

impl TournamentResult {
    /// Retourne le gagnant du tournoi
    pub fn winner(&self) -> Option<&PlayerScore> {
        self.rankings.first()
    }

    /// Affiche le classement formaté
    pub fn display_rankings(&self) -> String {
        let mut output = String::new();
        output.push_str(
            "\n╔═══════════════════════════════════════════════════════════════════════╗\n",
        );
        output.push_str(
            "║                      RÉSULTATS DU TOURNOI                             ║\n",
        );
        output.push_str(
            "╠═══════════════════════════════════════════════════════════════════════╣\n",
        );
        output
            .push_str("║ Rang │ Stratégie                │ Score  │ Moy/Match │ Coop%  │ Nice ║\n");
        output
            .push_str("╠══════╪══════════════════════════╪════════╪═══════════╪════════╪══════╣\n");

        for (i, player) in self.rankings.iter().enumerate() {
            let nice_str = if player.is_nice { "Oui" } else { "Non" };
            output.push_str(&format!(
                "║ {:>4} │ {:<24} │ {:>6} │ {:>9.1} │ {:>5.1}% │ {:>4} ║\n",
                i + 1,
                truncate_str(&player.name, 24),
                player.total_score,
                player.average_score,
                player.cooperation_rate * 100.0,
                nice_str
            ));
        }

        output.push_str(
            "╚═══════════════════════════════════════════════════════════════════════╝\n",
        );
        output
    }
}

/// Tronque une chaîne à une longueur maximale
fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Tournoi Round-Robin
pub struct RoundRobinTournament {
    /// Configuration des matchs
    config: MatchConfig,
    /// Stratégies participant au tournoi
    strategies: Vec<StrategyType>,
}

impl RoundRobinTournament {
    /// Crée un nouveau tournoi avec toutes les stratégies
    pub fn new(config: MatchConfig) -> Self {
        Self {
            config,
            strategies: StrategyType::all(),
        }
    }

    /// Crée un tournoi avec des stratégies spécifiques
    pub fn with_strategies(strategies: Vec<StrategyType>, config: MatchConfig) -> Self {
        Self { config, strategies }
    }

    /// Crée un tournoi avec la configuration par défaut
    pub fn default() -> Self {
        Self::new(MatchConfig::default())
    }

    /// Lance le tournoi et retourne les résultats
    pub fn run(&self) -> TournamentResult {
        let n = self.strategies.len();
        let mut players: Vec<Player> = self.strategies.iter().map(|s| Player::new(*s)).collect();

        let mut match_results = Vec::new();
        let mut score_matrix: HashMap<(StrategyType, StrategyType), (i32, i32)> = HashMap::new();

        // Chaque stratégie joue contre toutes les autres (y compris elle-même)
        for i in 0..n {
            for j in i..n {
                // Clone les joueurs pour ce match
                let mut player1 = players[i].clone_fresh();
                let mut player2 = players[j].clone_fresh();

                let result = {
                    let mut game = Match::new(&mut player1, &mut player2, self.config.clone());
                    game.play()
                };

                // Mise à jour des scores totaux
                players[i].add_score(result.score1);
                players[i].cooperations += result.cooperations1;
                players[i].rounds_played += result.rounds.len() as u32;
                players[i].matches_played += 1;

                if i != j {
                    // Match contre un adversaire différent
                    players[j].add_score(result.score2);
                    players[j].cooperations += result.cooperations2;
                    players[j].rounds_played += result.rounds.len() as u32;
                    players[j].matches_played += 1;

                    // Enregistrer aussi le match inverse dans la matrice
                    score_matrix.insert(
                        (self.strategies[j], self.strategies[i]),
                        (result.score2, result.score1),
                    );
                }

                // Enregistrer dans la matrice des scores
                score_matrix.insert(
                    (self.strategies[i], self.strategies[j]),
                    (result.score1, result.score2),
                );

                match_results.push(result);
            }
        }

        // Création du classement
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

        // Tri par score décroissant
        rankings.sort_by(|a, b| b.total_score.cmp(&a.total_score));

        TournamentResult {
            rankings,
            match_results,
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

        // Always Defect devrait gagner contre Always Cooperate
        assert_eq!(result.rankings.len(), 2);
        assert_eq!(result.rankings[0].strategy_type, StrategyType::AlwaysDefect);
    }

    #[test]
    fn test_tournament_all_strategies() {
        let tournament = RoundRobinTournament::new(MatchConfig::with_rounds(10));
        let result = tournament.run();

        // Toutes les stratégies devraient être présentes
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

        // Les stratégies "nice" (TFT, AC, Grudger) devraient généralement
        // avoir un bon score moyen car elles coopèrent entre elles
        // Vérifions que TFT ou Grudger est dans le top 2
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
