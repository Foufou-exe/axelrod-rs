//! Matrice de gains (Payoff Matrix) pour le Dilemme du Prisonnier
//!
//! Utilise la matrice classique d'Axelrod:
//! - R (Reward) = 3 : Récompense mutuelle pour coopération
//! - T (Temptation) = 5 : Tentation de trahir un coopérateur
//! - S (Sucker) = 0 : Pénalité du "pigeon" (coopérer face à une trahison)
//! - P (Punishment) = 1 : Punition mutuelle pour trahison
//!
//! Conditions du Dilemme du Prisonnier:
//! - T > R > P > S (5 > 3 > 1 > 0)
//! - 2R > T + S (6 > 5) pour éviter l'alternance coopération/trahison

use crate::action::Action;

/// Points pour la récompense mutuelle (les deux coopèrent)
pub const REWARD: i32 = 3;

/// Points pour la tentation (trahir un coopérateur)
pub const TEMPTATION: i32 = 5;

/// Points du "pigeon" (coopérer face à une trahison)
pub const SUCKER: i32 = 0;

/// Points pour la punition mutuelle (les deux trahissent)
pub const PUNISHMENT: i32 = 1;

/// Matrice de gains pour le Dilemme du Prisonnier
#[derive(Debug, Clone)]
pub struct PayoffMatrix {
    /// Récompense mutuelle (R)
    pub reward: i32,
    /// Tentation de trahir (T)
    pub temptation: i32,
    /// Pénalité du pigeon (S)
    pub sucker: i32,
    /// Punition mutuelle (P)
    pub punishment: i32,
}

impl PayoffMatrix {
    /// Crée une nouvelle matrice de gains personnalisée
    pub fn new(reward: i32, temptation: i32, sucker: i32, punishment: i32) -> Self {
        Self {
            reward,
            temptation,
            sucker,
            punishment,
        }
    }

    /// Retourne la matrice classique d'Axelrod (R=3, T=5, S=0, P=1)
    pub fn classic() -> Self {
        Self {
            reward: REWARD,
            temptation: TEMPTATION,
            sucker: SUCKER,
            punishment: PUNISHMENT,
        }
    }

    /// Vérifie si la matrice respecte les conditions du Dilemme du Prisonnier
    /// T > R > P > S et 2R > T + S
    pub fn is_valid(&self) -> bool {
        self.temptation > self.reward
            && self.reward > self.punishment
            && self.punishment > self.sucker
            && 2 * self.reward > self.temptation + self.sucker
    }

    /// Calcule les gains pour un round donné
    /// Retourne (gain_joueur1, gain_joueur2)
    pub fn get_payoffs(&self, action1: Action, action2: Action) -> (i32, i32) {
        match (action1, action2) {
            (Action::Cooperate, Action::Cooperate) => (self.reward, self.reward),
            (Action::Cooperate, Action::Defect) => (self.sucker, self.temptation),
            (Action::Defect, Action::Cooperate) => (self.temptation, self.sucker),
            (Action::Defect, Action::Defect) => (self.punishment, self.punishment),
        }
    }

    /// Retourne le gain maximum possible en un round
    pub fn max_per_round(&self) -> i32 {
        self.temptation
    }

    /// Retourne le gain pour une coopération mutuelle parfaite sur n rounds
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
        // T < R viole la condition
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

        // Joueur 1 coopère, Joueur 2 trahit
        let (p1, p2) = matrix.get_payoffs(Action::Cooperate, Action::Defect);
        assert_eq!(p1, 0); // S (sucker)
        assert_eq!(p2, 5); // T (temptation)

        // Joueur 1 trahit, Joueur 2 coopère
        let (p1, p2) = matrix.get_payoffs(Action::Defect, Action::Cooperate);
        assert_eq!(p1, 5); // T
        assert_eq!(p2, 0); // S
    }

    #[test]
    fn test_perfect_cooperation_score() {
        let matrix = PayoffMatrix::classic();
        // 200 rounds de coopération mutuelle = 200 * 3 = 600
        assert_eq!(matrix.perfect_cooperation_score(200), 600);
    }
}
