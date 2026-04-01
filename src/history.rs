//! Historique des interactions entre deux joueurs
//!
//! Permet aux stratégies de consulter les coups passés pour prendre leurs décisions.

use crate::action::Action;

/// Représente un round joué (les deux actions)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Round {
    /// Action du joueur dont on tient l'historique
    pub my_action: Action,
    /// Action de l'adversaire
    pub opponent_action: Action,
}

impl Round {
    pub fn new(my_action: Action, opponent_action: Action) -> Self {
        Self {
            my_action,
            opponent_action,
        }
    }
}

/// Historique des interactions du point de vue d'un joueur
#[derive(Debug, Clone, Default)]
pub struct History {
    /// Liste des rounds joués
    rounds: Vec<Round>,
}

impl History {
    /// Crée un nouvel historique vide
    pub fn new() -> Self {
        Self { rounds: Vec::new() }
    }

    /// Crée un historique avec une capacité pré-allouée
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            rounds: Vec::with_capacity(capacity),
        }
    }

    /// Ajoute un round à l'historique
    pub fn push(&mut self, my_action: Action, opponent_action: Action) {
        self.rounds.push(Round::new(my_action, opponent_action));
    }

    /// Retourne le nombre de rounds joués
    pub fn len(&self) -> usize {
        self.rounds.len()
    }

    /// Vérifie si l'historique est vide
    pub fn is_empty(&self) -> bool {
        self.rounds.is_empty()
    }

    /// Retourne le dernier round joué
    pub fn last(&self) -> Option<&Round> {
        self.rounds.last()
    }

    /// Retourne la dernière action de l'adversaire
    pub fn last_opponent_action(&self) -> Option<Action> {
        self.rounds.last().map(|r| r.opponent_action)
    }

    /// Retourne la dernière action du joueur
    pub fn last_my_action(&self) -> Option<Action> {
        self.rounds.last().map(|r| r.my_action)
    }

    /// Retourne les n dernières actions de l'adversaire
    pub fn last_n_opponent_actions(&self, n: usize) -> Vec<Action> {
        self.rounds
            .iter()
            .rev()
            .take(n)
            .map(|r| r.opponent_action)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    /// Compte le nombre de coopérations de l'adversaire
    pub fn count_opponent_cooperations(&self) -> usize {
        self.rounds
            .iter()
            .filter(|r| r.opponent_action == Action::Cooperate)
            .count()
    }

    /// Compte le nombre de trahisons de l'adversaire
    pub fn count_opponent_defections(&self) -> usize {
        self.rounds
            .iter()
            .filter(|r| r.opponent_action == Action::Defect)
            .count()
    }

    /// Vérifie si l'adversaire a déjà trahi
    pub fn opponent_has_defected(&self) -> bool {
        self.rounds
            .iter()
            .any(|r| r.opponent_action == Action::Defect)
    }

    /// Retourne le round à l'index donné (0-indexed)
    pub fn get(&self, index: usize) -> Option<&Round> {
        self.rounds.get(index)
    }

    /// Itère sur tous les rounds
    pub fn iter(&self) -> impl Iterator<Item = &Round> {
        self.rounds.iter()
    }

    /// Retourne toutes les actions de l'adversaire
    pub fn opponent_actions(&self) -> Vec<Action> {
        self.rounds.iter().map(|r| r.opponent_action).collect()
    }

    /// Retourne toutes les actions du joueur
    pub fn my_actions(&self) -> Vec<Action> {
        self.rounds.iter().map(|r| r.my_action).collect()
    }

    /// Efface l'historique
    pub fn clear(&mut self) {
        self.rounds.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_history_is_empty() {
        let history = History::new();
        assert!(history.is_empty());
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_push_and_len() {
        let mut history = History::new();
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(history.len(), 1);

        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn test_last_actions() {
        let mut history = History::new();
        history.push(Action::Cooperate, Action::Defect);
        history.push(Action::Defect, Action::Cooperate);

        assert_eq!(history.last_my_action(), Some(Action::Defect));
        assert_eq!(history.last_opponent_action(), Some(Action::Cooperate));
    }

    #[test]
    fn test_last_n_opponent_actions() {
        let mut history = History::new();
        history.push(Action::Cooperate, Action::Cooperate);
        history.push(Action::Cooperate, Action::Defect);
        history.push(Action::Defect, Action::Defect);

        let last_2 = history.last_n_opponent_actions(2);
        assert_eq!(last_2, vec![Action::Defect, Action::Defect]);

        let last_5 = history.last_n_opponent_actions(5);
        assert_eq!(
            last_5,
            vec![Action::Cooperate, Action::Defect, Action::Defect]
        );
    }

    #[test]
    fn test_count_opponent_actions() {
        let mut history = History::new();
        history.push(Action::Cooperate, Action::Cooperate);
        history.push(Action::Cooperate, Action::Defect);
        history.push(Action::Defect, Action::Defect);

        assert_eq!(history.count_opponent_cooperations(), 1);
        assert_eq!(history.count_opponent_defections(), 2);
    }

    #[test]
    fn test_opponent_has_defected() {
        let mut history = History::new();
        assert!(!history.opponent_has_defected());

        history.push(Action::Cooperate, Action::Cooperate);
        assert!(!history.opponent_has_defected());

        history.push(Action::Cooperate, Action::Defect);
        assert!(history.opponent_has_defected());
    }
}
