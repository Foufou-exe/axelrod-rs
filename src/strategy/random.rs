//! Stratégie Random (Aléatoire)
//!
//! Coopère ou trahit de manière aléatoire avec une probabilité de 50%.
//! Utile comme baseline et pour tester la robustesse des autres stratégies.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;
use rand::Rng;

/// Stratégie aléatoire
#[derive(Debug, Clone)]
pub struct Random {
    /// Probabilité de coopérer (par défaut 0.5)
    cooperation_probability: f64,
}

impl Random {
    /// Crée une nouvelle instance avec probabilité de coopération personnalisée
    pub fn with_probability(cooperation_probability: f64) -> Self {
        Self {
            cooperation_probability: cooperation_probability.clamp(0.0, 1.0),
        }
    }

    /// Crée une instance avec probabilité 50/50
    pub fn new() -> Self {
        Self::with_probability(0.5)
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for Random {
    fn name(&self) -> &'static str {
        "Random"
    }

    fn description(&self) -> &'static str {
        "Coopère ou trahit aléatoirement (50/50)"
    }

    fn decide(&mut self, _history: &History) -> Action {
        let mut rng = rand::thread_rng();
        if Rng::r#gen::<f64>(&mut rng) < self.cooperation_probability {
            Action::Cooperate
        } else {
            Action::Defect
        }
    }

    fn is_nice(&self) -> bool {
        false // Peut trahir en premier
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_cooperates_with_probability_1() {
        let mut strategy = Random::with_probability(1.0);
        let history = History::new();

        for _ in 0..100 {
            assert_eq!(strategy.decide(&history), Action::Cooperate);
        }
    }

    #[test]
    fn test_always_defects_with_probability_0() {
        let mut strategy = Random::with_probability(0.0);
        let history = History::new();

        for _ in 0..100 {
            assert_eq!(strategy.decide(&history), Action::Defect);
        }
    }

    #[test]
    fn test_produces_both_actions() {
        let mut strategy = Random::new();
        let history = History::new();

        let mut cooperations = 0;
        let mut defections = 0;

        for _ in 0..1000 {
            match strategy.decide(&history) {
                Action::Cooperate => cooperations += 1,
                Action::Defect => defections += 1,
            }
        }

        // Avec 1000 essais et p=0.5, on devrait avoir les deux
        assert!(cooperations > 0);
        assert!(defections > 0);
        // Et environ 50% chacun (avec une marge)
        assert!(cooperations > 400 && cooperations < 600);
    }

    #[test]
    fn test_is_not_nice() {
        assert!(!Random::new().is_nice());
    }
}
