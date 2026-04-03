//! Strategies module for the Iterated Prisoner's Dilemma
//!
//! This module contains the `Strategy` trait and all implementations
//! of classic strategies from Axelrod's tournaments (1980-1984).

mod always_cooperate;
mod always_defect;
mod davis;
mod feld;
mod generous_tft;
mod go_by_majority;
mod graaskamp;
mod grofman;
mod grudger;
mod joss;
mod nydegger;
mod pavlov;
mod prober;
mod random;
mod shubik;
mod suspicious_tft;
mod tit_for_tat;
mod tit_for_two_tats;
mod tullock;

pub use always_cooperate::AlwaysCooperate;
pub use always_defect::AlwaysDefect;
pub use davis::Davis;
pub use feld::Feld;
pub use generous_tft::GenerousTitForTat;
pub use go_by_majority::{HardGoByMajority, SoftGoByMajority};
pub use graaskamp::Graaskamp;
pub use grofman::Grofman;
pub use grudger::Grudger;
pub use joss::Joss;
pub use nydegger::Nydegger;
pub use pavlov::Pavlov;
pub use prober::Prober;
pub use random::Random;
pub use shubik::Shubik;
pub use suspicious_tft::SuspiciousTitForTat;
pub use tit_for_tat::TitForTat;
pub use tit_for_two_tats::TitForTwoTats;
pub use tullock::Tullock;

use crate::action::Action;
use crate::history::History;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Trait defining a strategy for the Prisoner's Dilemma
pub trait Strategy: Debug + Send + Sync {
    /// Returns the strategy name
    fn name(&self) -> &'static str;

    /// Returns a short description of the strategy
    fn description(&self) -> &'static str;

    /// Decides which action to play based on the history
    fn decide(&mut self, history: &History) -> Action;

    /// Resets the strategy's internal state (for a new match)
    fn reset(&mut self) {}

    /// Indicates if the strategy is "nice" (never defects first)
    fn is_nice(&self) -> bool {
        false
    }

    /// Creates a boxed clone of the strategy
    fn clone_box(&self) -> Box<dyn Strategy>;
}

impl Clone for Box<dyn Strategy> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Enumeration of all available strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    // Axelrod's Tournament Strategies (1980-1984)
    Joss,
    Graaskamp,
    Tullock,
    Feld,
    Nydegger,
    Grofman,
    Shubik,
    Davis,
}

impl StrategyType {
    /// Returns all available strategies
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
            // Axelrod's Tournament Strategies (1980-1984)
            StrategyType::Joss,
            StrategyType::Graaskamp,
            StrategyType::Tullock,
            StrategyType::Feld,
            StrategyType::Nydegger,
            StrategyType::Grofman,
            StrategyType::Shubik,
            StrategyType::Davis,
        ]
    }

    /// Creates an instance of the strategy
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
            // Axelrod's Tournament Strategies (1980-1984)
            StrategyType::Joss => Box::new(Joss::new()),
            StrategyType::Graaskamp => Box::new(Graaskamp::new()),
            StrategyType::Tullock => Box::new(Tullock::new()),
            StrategyType::Feld => Box::new(Feld::new()),
            StrategyType::Nydegger => Box::new(Nydegger::new()),
            StrategyType::Grofman => Box::new(Grofman),
            StrategyType::Shubik => Box::new(Shubik::new()),
            StrategyType::Davis => Box::new(Davis::new()),
        }
    }

    /// Returns the strategy name
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
            // Axelrod's Tournament Strategies (1980-1984)
            StrategyType::Joss => "Joss",
            StrategyType::Graaskamp => "Graaskamp",
            StrategyType::Tullock => "Tullock",
            StrategyType::Feld => "Feld",
            StrategyType::Nydegger => "Nydegger",
            StrategyType::Grofman => "Grofman",
            StrategyType::Shubik => "Shubik",
            StrategyType::Davis => "Davis",
        }
    }

    /// Find a strategy by name (case-insensitive)
    pub fn from_name(name: &str) -> Option<StrategyType> {
        let name_lower = name.to_lowercase();
        StrategyType::all()
            .into_iter()
            .find(|s| s.name().to_lowercase() == name_lower)
    }
}

impl std::fmt::Display for StrategyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
