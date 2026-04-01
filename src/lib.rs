//! axelrod-rs - Simulateur de théorie des jeux évolutionnaire
//!
//! Ce projet explore le Dilemme du Prisonnier Itéré en faisant s'affronter
//! différentes stratégies automatisées pour observer l'émergence de la
//! coopération, de la confiance et de la trahison dans un environnement compétitif.
//!
//! Inspiré par les travaux de Robert Axelrod dans "The Evolution of Cooperation" (1984).

pub mod action;
pub mod game;
pub mod history;
pub mod payoff;
pub mod player;
pub mod strategy;
pub mod tournament;

pub use action::Action;
pub use history::History;
pub use payoff::PayoffMatrix;
pub use player::Player;
pub use strategy::{Strategy, StrategyType};
