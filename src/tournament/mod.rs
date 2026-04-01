//! Module de tournoi - Round-robin et évolutionnaire

mod ecological;
mod round_robin;

pub use ecological::{EcologicalTournament, EcologicalConfig, Generation};
pub use round_robin::{RoundRobinTournament, TournamentResult, PlayerScore};
