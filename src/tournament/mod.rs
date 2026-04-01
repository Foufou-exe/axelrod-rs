//! Tournament module - Round-robin and Ecological/Evolutionary

mod ecological;
mod round_robin;

pub use ecological::{EcologicalConfig, EcologicalTournament, Generation};
pub use round_robin::{PlayerScore, RoundRobinTournament, TournamentResult};
