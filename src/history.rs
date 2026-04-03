//! History of interactions between two players
//!
//! Allows strategies to consult past moves to make their decisions.

use crate::action::Action;

/// Represents a played round (both actions)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Round {
    /// Action of the player holding this history
    pub my_action: Action,
    /// Opponent's action
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

/// History of interactions from one player's perspective
#[derive(Debug, Clone, Default)]
pub struct History {
    /// List of played rounds
    rounds: Vec<Round>,
}

impl History {
    /// Creates a new empty history
    pub fn new() -> Self {
        Self { rounds: Vec::new() }
    }

    /// Creates a history with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            rounds: Vec::with_capacity(capacity),
        }
    }

    /// Adds a round to the history
    pub fn push(&mut self, my_action: Action, opponent_action: Action) {
        self.rounds.push(Round::new(my_action, opponent_action));
    }

    /// Returns the number of played rounds
    pub fn len(&self) -> usize {
        self.rounds.len()
    }

    /// Checks if the history is empty
    pub fn is_empty(&self) -> bool {
        self.rounds.is_empty()
    }

    /// Returns the last played round
    pub fn last(&self) -> Option<&Round> {
        self.rounds.last()
    }

    /// Returns the opponent's last action
    pub fn last_opponent_action(&self) -> Option<Action> {
        self.rounds.last().map(|r| r.opponent_action)
    }

    /// Returns the player's last action
    pub fn last_my_action(&self) -> Option<Action> {
        self.rounds.last().map(|r| r.my_action)
    }

    /// Returns the last n opponent actions
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

    /// Counts the number of opponent cooperations
    pub fn count_opponent_cooperations(&self) -> usize {
        self.rounds
            .iter()
            .filter(|r| r.opponent_action == Action::Cooperate)
            .count()
    }

    /// Counts the number of opponent defections
    pub fn count_opponent_defections(&self) -> usize {
        self.rounds
            .iter()
            .filter(|r| r.opponent_action == Action::Defect)
            .count()
    }

    /// Checks if the opponent has ever defected
    pub fn opponent_has_defected(&self) -> bool {
        self.rounds
            .iter()
            .any(|r| r.opponent_action == Action::Defect)
    }

    /// Returns the round at the given index (0-indexed)
    pub fn get(&self, index: usize) -> Option<&Round> {
        self.rounds.get(index)
    }

    /// Iterates over all rounds
    pub fn iter(&self) -> impl Iterator<Item = &Round> {
        self.rounds.iter()
    }

    /// Returns all opponent actions
    pub fn opponent_actions(&self) -> Vec<Action> {
        self.rounds.iter().map(|r| r.opponent_action).collect()
    }

    /// Returns all player actions
    pub fn my_actions(&self) -> Vec<Action> {
        self.rounds.iter().map(|r| r.my_action).collect()
    }

    /// Clears the history
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
