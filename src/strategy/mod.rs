//! Module des stratégies pour le Dilemme du Prisonnier Itéré
//!
//! Ce module contient le trait `Strategy` et toutes les implémentations
//! des stratégies classiques des tournois d'Axelrod (1980-1984).

mod always_cooperate;
mod always_defect;
mod generous_tft;
mod go_by_majority;
mod grudger;
mod pavlov;
mod prober;
mod random;
mod suspicious_tft;
mod tit_for_tat;
mod tit_for_two_tats;

pub use always_cooperate::AlwaysCooperate;
pub use always_defect::AlwaysDefect;
pub use generous_tft::GenerousTitForTat;
pub use go_by_majority::{HardGoByMajority, SoftGoByMajority};
pub use grudger::Grudger;
pub use pavlov::Pavlov;
pub use prober::Prober;
pub use random::Random;
pub use suspicious_tft::SuspiciousTitForTat;
pub use tit_for_tat::TitForTat;
pub use tit_for_two_tats::TitForTwoTats;

use crate::action::Action;
use crate::history::History;
use std::fmt::Debug;

/// Trait définissant une stratégie pour le Dilemme du Prisonnier
pub trait Strategy: Debug + Send + Sync {
    /// Retourne le nom de la stratégie
    fn name(&self) -> &'static str;

    /// Retourne une description courte de la stratégie
    fn description(&self) -> &'static str;

    /// Décide de l'action à jouer en fonction de l'historique
    fn decide(&mut self, history: &History) -> Action;

    /// Réinitialise l'état interne de la stratégie (pour un nouveau match)
    fn reset(&mut self) {}

    /// Indique si la stratégie est "gentille" (ne trahit jamais en premier)
    fn is_nice(&self) -> bool {
        false
    }

    /// Crée un clone boxé de la stratégie
    fn clone_box(&self) -> Box<dyn Strategy>;
}

impl Clone for Box<dyn Strategy> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Énumération de toutes les stratégies disponibles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrategyType {
    AlwaysCooperate,
    AlwaysDefect,
    TitForTat,
    SuspiciousTitForTat,
    TitForTwoTats,
    GenerousTitForTat,
    Grudger,
    Random,
    Pavlov,
    Prober,
    HardGoByMajority,
    SoftGoByMajority,
}

impl StrategyType {
    /// Retourne toutes les stratégies disponibles
    pub fn all() -> Vec<StrategyType> {
        vec![
            StrategyType::AlwaysCooperate,
            StrategyType::AlwaysDefect,
            StrategyType::TitForTat,
            StrategyType::SuspiciousTitForTat,
            StrategyType::TitForTwoTats,
            StrategyType::GenerousTitForTat,
            StrategyType::Grudger,
            StrategyType::Random,
            StrategyType::Pavlov,
            StrategyType::Prober,
            StrategyType::HardGoByMajority,
            StrategyType::SoftGoByMajority,
        ]
    }

    /// Crée une instance de la stratégie
    pub fn create(&self) -> Box<dyn Strategy> {
        match self {
            StrategyType::AlwaysCooperate => Box::new(AlwaysCooperate),
            StrategyType::AlwaysDefect => Box::new(AlwaysDefect),
            StrategyType::TitForTat => Box::new(TitForTat),
            StrategyType::SuspiciousTitForTat => Box::new(SuspiciousTitForTat),
            StrategyType::TitForTwoTats => Box::new(TitForTwoTats),
            StrategyType::GenerousTitForTat => Box::new(GenerousTitForTat::default()),
            StrategyType::Grudger => Box::new(Grudger::new()),
            StrategyType::Random => Box::new(Random::new()),
            StrategyType::Pavlov => Box::new(Pavlov),
            StrategyType::Prober => Box::new(Prober::new()),
            StrategyType::HardGoByMajority => Box::new(HardGoByMajority),
            StrategyType::SoftGoByMajority => Box::new(SoftGoByMajority),
        }
    }

    /// Retourne le nom de la stratégie
    pub fn name(&self) -> &'static str {
        match self {
            StrategyType::AlwaysCooperate => "Always Cooperate",
            StrategyType::AlwaysDefect => "Always Defect",
            StrategyType::TitForTat => "Tit for Tat",
            StrategyType::SuspiciousTitForTat => "Suspicious Tit for Tat",
            StrategyType::TitForTwoTats => "Tit for Two Tats",
            StrategyType::GenerousTitForTat => "Generous Tit for Tat",
            StrategyType::Grudger => "Grudger",
            StrategyType::Random => "Random",
            StrategyType::Pavlov => "Pavlov",
            StrategyType::Prober => "Prober",
            StrategyType::HardGoByMajority => "Hard Go By Majority",
            StrategyType::SoftGoByMajority => "Soft Go By Majority",
        }
    }
}

impl std::fmt::Display for StrategyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
