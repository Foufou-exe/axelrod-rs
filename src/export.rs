//! Export module for tournament results in CSV and JSON formats

use crate::game::MatchResult;
use crate::tournament::{Generation, PlayerScore, TournamentResult};
use serde::Serialize;
use std::fs::File;
use std::io;
use std::path::Path;

/// Export format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Json,
    Csv,
}

impl ExportFormat {
    /// Detect format from file extension
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| match ext.to_str()?.to_lowercase().as_str() {
                "json" => Some(ExportFormat::Json),
                "csv" => Some(ExportFormat::Csv),
                _ => None,
            })
    }
}

/// Flattened player score for CSV export
#[derive(Debug, Serialize)]
pub struct CsvPlayerScore {
    pub rank: usize,
    pub name: String,
    pub strategy_type: String,
    pub total_score: i32,
    pub matches_played: u32,
    pub average_score: f64,
    pub cooperation_rate: f64,
    pub is_nice: bool,
}

impl CsvPlayerScore {
    pub fn from_player_score(rank: usize, score: &PlayerScore) -> Self {
        Self {
            rank,
            name: score.name.clone(),
            strategy_type: format!("{:?}", score.strategy_type),
            total_score: score.total_score,
            matches_played: score.matches_played,
            average_score: score.average_score,
            cooperation_rate: score.cooperation_rate,
            is_nice: score.is_nice,
        }
    }
}

/// Flattened generation data for CSV export
#[derive(Debug, Serialize)]
pub struct CsvGeneration {
    pub generation: u32,
    pub strategy: String,
    pub population: u32,
    pub population_percentage: f64,
    pub average_score: f64,
}

/// Export round-robin tournament results
pub fn export_round_robin(result: &TournamentResult, path: &Path) -> io::Result<()> {
    let format = ExportFormat::from_path(path).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Unsupported file format. Use .json or .csv",
        )
    })?;

    match format {
        ExportFormat::Json => export_round_robin_json(result, path),
        ExportFormat::Csv => export_round_robin_csv(result, path),
    }
}

/// Export round-robin results to JSON
fn export_round_robin_json(result: &TournamentResult, path: &Path) -> io::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, result).map_err(io::Error::other)
}

/// Export round-robin results to CSV
fn export_round_robin_csv(result: &TournamentResult, path: &Path) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = csv::Writer::from_writer(file);

    for (rank, score) in result.rankings.iter().enumerate() {
        let csv_score = CsvPlayerScore::from_player_score(rank + 1, score);
        writer.serialize(csv_score).map_err(io::Error::other)?;
    }

    writer.flush()?;
    Ok(())
}

/// Export ecological tournament results
pub fn export_ecological(generations: &[Generation], path: &Path) -> io::Result<()> {
    let format = ExportFormat::from_path(path).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Unsupported file format. Use .json or .csv",
        )
    })?;

    match format {
        ExportFormat::Json => export_ecological_json(generations, path),
        ExportFormat::Csv => export_ecological_csv(generations, path),
    }
}

/// Export ecological results to JSON
fn export_ecological_json(generations: &[Generation], path: &Path) -> io::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, generations).map_err(io::Error::other)
}

/// Export ecological results to CSV
fn export_ecological_csv(generations: &[Generation], path: &Path) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = csv::Writer::from_writer(file);

    for generation in generations {
        let percentages = generation.population_percentages();

        for (&strategy, &population) in &generation.populations {
            let avg_score = generation
                .average_scores
                .get(&strategy)
                .copied()
                .unwrap_or(0.0);

            let pop_pct = percentages.get(&strategy).copied().unwrap_or(0.0);

            let csv_gen = CsvGeneration {
                generation: generation.number,
                strategy: format!("{}", strategy),
                population,
                population_percentage: pop_pct,
                average_score: avg_score,
            };

            writer.serialize(csv_gen).map_err(io::Error::other)?;
        }
    }

    writer.flush()?;
    Ok(())
}

/// Export a single match result
pub fn export_match(result: &MatchResult, path: &Path) -> io::Result<()> {
    let format = ExportFormat::from_path(path).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Unsupported file format. Use .json or .csv",
        )
    })?;

    match format {
        ExportFormat::Json => export_match_json(result, path),
        ExportFormat::Csv => export_match_csv(result, path),
    }
}

/// Export match result to JSON
fn export_match_json(result: &MatchResult, path: &Path) -> io::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, result).map_err(io::Error::other)
}

/// Flattened round data for CSV export
#[derive(Debug, Serialize)]
pub struct CsvRound {
    pub round: usize,
    pub player1: String,
    pub player2: String,
    pub action1: String,
    pub action2: String,
    pub score1: i32,
    pub score2: i32,
}

/// Export match result to CSV
fn export_match_csv(result: &MatchResult, path: &Path) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = csv::Writer::from_writer(file);

    for (i, round) in result.rounds.iter().enumerate() {
        let csv_round = CsvRound {
            round: i + 1,
            player1: result.player1_name.clone(),
            player2: result.player2_name.clone(),
            action1: format!("{}", round.action1),
            action2: format!("{}", round.action2),
            score1: round.score1,
            score2: round.score2,
        };

        writer.serialize(csv_round).map_err(io::Error::other)?;
    }

    writer.flush()?;
    Ok(())
}

/// Write to stdout in specified format
pub fn write_round_robin_stdout(result: &TournamentResult, format: ExportFormat) -> io::Result<()> {
    match format {
        ExportFormat::Json => {
            let json = serde_json::to_string_pretty(result).map_err(io::Error::other)?;
            println!("{}", json);
            Ok(())
        }
        ExportFormat::Csv => {
            let mut writer = csv::Writer::from_writer(io::stdout());
            for (rank, score) in result.rankings.iter().enumerate() {
                let csv_score = CsvPlayerScore::from_player_score(rank + 1, score);
                writer.serialize(csv_score).map_err(io::Error::other)?;
            }
            writer.flush()?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_format_detection() {
        assert_eq!(
            ExportFormat::from_path(&PathBuf::from("test.json")),
            Some(ExportFormat::Json)
        );
        assert_eq!(
            ExportFormat::from_path(&PathBuf::from("test.JSON")),
            Some(ExportFormat::Json)
        );
        assert_eq!(
            ExportFormat::from_path(&PathBuf::from("test.csv")),
            Some(ExportFormat::Csv)
        );
        assert_eq!(ExportFormat::from_path(&PathBuf::from("test.txt")), None);
    }
}
