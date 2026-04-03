//! Command-line interface definitions using clap

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Iterated Prisoner's Dilemma Simulator
///
/// Based on Robert Axelrod's "The Evolution of Cooperation" (1984)
#[derive(Parser, Debug)]
#[command(name = "axelrod-rs")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Random seed for reproducibility
    #[arg(short, long, global = true)]
    pub seed: Option<u64>,

    /// Output file for results (supports .json and .csv)
    #[arg(short, long, global = true)]
    pub output: Option<PathBuf>,

    /// Quiet mode (minimal output)
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a round-robin tournament (each strategy vs all others)
    RoundRobin {
        /// Number of rounds per match
        #[arg(short, long, default_value = "200")]
        rounds: u32,

        /// Noise probability (0.0 to 1.0) - chance of action being flipped
        #[arg(short, long, default_value = "0.0")]
        noise: f64,
    },

    /// Run an ecological/evolutionary tournament
    Ecological {
        /// Number of rounds per match
        #[arg(short, long, default_value = "200")]
        rounds: u32,

        /// Number of generations to simulate
        #[arg(short, long, default_value = "100")]
        generations: u32,

        /// Initial population per strategy
        #[arg(short, long, default_value = "100")]
        population: u32,

        /// Noise probability (0.0 to 1.0)
        #[arg(short, long, default_value = "0.0")]
        noise: f64,
    },

    /// Run a 1v1 match between two strategies
    Match {
        /// First strategy name
        #[arg(short = '1', long)]
        player1: String,

        /// Second strategy name
        #[arg(short = '2', long)]
        player2: String,

        /// Number of rounds
        #[arg(short, long, default_value = "200")]
        rounds: u32,

        /// Noise probability (0.0 to 1.0)
        #[arg(short, long, default_value = "0.0")]
        noise: f64,
    },

    /// List all available strategies
    Strategies,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }

    /// Check if running in interactive mode (no subcommand)
    pub fn is_interactive(&self) -> bool {
        self.command.is_none()
    }
}
