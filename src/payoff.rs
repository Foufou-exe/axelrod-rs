//! Payoff Matrix for the Prisoner's Dilemma
//!
//! Uses Axelrod's classic matrix:
//! - R (Reward) = 3: Mutual reward for cooperation
//! - T (Temptation) = 5: Temptation to defect against a cooperator
//! - S (Sucker) = 0: Sucker's payoff (cooperating against a defector)
//! - P (Punishment) = 1: Mutual punishment for defection
//!
//! Prisoner's Dilemma conditions:
//! - T > R > P > S (5 > 3 > 1 > 0)
//! - 2R > T + S (6 > 5) to prevent alternating cooperation/defection

use crate::action::Action;
use serde::{Deserialize, Serialize};

/// Points for mutual reward (both cooperate)
pub const REWARD: i32 = 3;

/// Points for temptation (defect against a cooperator)
pub const TEMPTATION: i32 = 5;

/// Sucker's points (cooperate against a defector)
pub const SUCKER: i32 = 0;

/// Points for mutual punishment (both defect)
pub const PUNISHMENT: i32 = 1;

/// Payoff matrix for the Prisoner's Dilemma
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoffMatrix {
    /// Mutual reward (R)
    pub reward: i32,
    /// Temptation to defect (T)
    pub temptation: i32,
    /// Sucker's payoff (S)
    pub sucker: i32,
    /// Mutual punishment (P)
    pub punishment: i32,
}

impl PayoffMatrix {
    /// Creates a new custom payoff matrix
    pub fn new(reward: i32, temptation: i32, sucker: i32, punishment: i32) -> Self {
        Self {
            reward,
            temptation,
            sucker,
            punishment,
        }
    }

    /// Returns Axelrod's classic matrix (R=3, T=5, S=0, P=1)
    pub fn classic() -> Self {
        Self {
            reward: REWARD,
            temptation: TEMPTATION,
            sucker: SUCKER,
            punishment: PUNISHMENT,
        }
    }

    /// Checks if the matrix satisfies Prisoner's Dilemma conditions
    /// T > R > P > S and 2R > T + S
    pub fn is_valid(&self) -> bool {
        self.temptation > self.reward
            && self.reward > self.punishment
            && self.punishment > self.sucker
            && 2 * self.reward > self.temptation + self.sucker
    }

    /// Calculates payoffs for a given round
    /// Returns (player1_payoff, player2_payoff)
    pub fn get_payoffs(&self, action1: Action, action2: Action) -> (i32, i32) {
        match (action1, action2) {
            (Action::Cooperate, Action::Cooperate) => (self.reward, self.reward),
            (Action::Cooperate, Action::Defect) => (self.sucker, self.temptation),
            (Action::Defect, Action::Cooperate) => (self.temptation, self.sucker),
            (Action::Defect, Action::Defect) => (self.punishment, self.punishment),
        }
    }

    /// Returns the maximum possible payoff in a single round
    pub fn max_per_round(&self) -> i32 {
        self.temptation
    }

    /// Returns the payoff for perfect mutual cooperation over n rounds
    pub fn perfect_cooperation_score(&self, rounds: u32) -> i32 {
        self.reward * rounds as i32
    }
}

impl Default for PayoffMatrix {
    fn default() -> Self {
        Self::classic()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_matrix() {
        let matrix = PayoffMatrix::classic();
        assert_eq!(matrix.reward, 3);
        assert_eq!(matrix.temptation, 5);
        assert_eq!(matrix.sucker, 0);
        assert_eq!(matrix.punishment, 1);
    }

    #[test]
    fn test_classic_is_valid() {
        let matrix = PayoffMatrix::classic();
        assert!(matrix.is_valid());
    }

    #[test]
    fn test_invalid_matrix() {
        // T < R violates the condition
        let invalid = PayoffMatrix::new(5, 3, 0, 1);
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_payoffs_both_cooperate() {
        let matrix = PayoffMatrix::classic();
        let (p1, p2) = matrix.get_payoffs(Action::Cooperate, Action::Cooperate);
        assert_eq!(p1, 3); // R
        assert_eq!(p2, 3); // R
    }

    #[test]
    fn test_payoffs_both_defect() {
        let matrix = PayoffMatrix::classic();
        let (p1, p2) = matrix.get_payoffs(Action::Defect, Action::Defect);
        assert_eq!(p1, 1); // P
        assert_eq!(p2, 1); // P
    }

    #[test]
    fn test_payoffs_mixed() {
        let matrix = PayoffMatrix::classic();

        // Player 1 cooperates, Player 2 defects
        let (p1, p2) = matrix.get_payoffs(Action::Cooperate, Action::Defect);
        assert_eq!(p1, 0); // S (sucker)
        assert_eq!(p2, 5); // T (temptation)

        // Player 1 defects, Player 2 cooperates
        let (p1, p2) = matrix.get_payoffs(Action::Defect, Action::Cooperate);
        assert_eq!(p1, 5); // T
        assert_eq!(p2, 0); // S
    }

    #[test]
    fn test_perfect_cooperation_score() {
        let matrix = PayoffMatrix::classic();
        // 200 rounds of mutual cooperation = 200 * 3 = 600
        assert_eq!(matrix.perfect_cooperation_score(200), 600);
    }
}
