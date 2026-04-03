//! axelrod-rs - Evolutionary Game Theory Simulator
//!
//! This project explores the Iterated Prisoner's Dilemma by pitting
//! different automated strategies against each other to observe the emergence
//! of cooperation, trust, and betrayal in a competitive environment.
//!
//! Inspired by Robert Axelrod's work in "The Evolution of Cooperation" (1984).

pub mod action;
pub mod cli;
pub mod export;
pub mod game;
pub mod history;
pub mod payoff;
pub mod player;
pub mod strategy;
pub mod tournament;

pub use action::Action;
pub use cli::{Cli, Commands};
pub use history::History;
pub use payoff::PayoffMatrix;
pub use player::Player;
pub use strategy::{Strategy, StrategyType};
