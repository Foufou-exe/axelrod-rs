use comfy_table::{presets::UTF8_FULL, Cell, ContentArrangement, Table};
use inquire::{Select, Text};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use axelrod_rs::cli::{Cli, Commands};
use axelrod_rs::export;
use axelrod_rs::game::{Match, MatchConfig};
use axelrod_rs::player::Player;
use axelrod_rs::strategy::StrategyType;
use axelrod_rs::tournament::{EcologicalConfig, EcologicalTournament, RoundRobinTournament};

fn main() {
    let cli = Cli::parse_args();

    if cli.is_interactive() {
        // No subcommand: run interactive mode
        run_interactive();
    } else {
        // CLI mode: run the specified command
        run_cli(cli);
    }
}

/// Run CLI mode with specified command
fn run_cli(cli: Cli) {
    let Some(command) = cli.command else {
        return;
    };

    match command {
        Commands::RoundRobin { rounds, noise } => {
            run_cli_round_robin(rounds, noise, cli.seed, cli.output.as_deref(), cli.quiet);
        }
        Commands::Ecological {
            rounds,
            generations,
            population,
            noise,
        } => {
            run_cli_ecological(
                rounds,
                generations,
                population,
                noise,
                cli.seed,
                cli.output.as_deref(),
                cli.quiet,
            );
        }
        Commands::Match {
            player1,
            player2,
            rounds,
            noise,
        } => {
            run_cli_match(
                &player1,
                &player2,
                rounds,
                noise,
                cli.seed,
                cli.output.as_deref(),
                cli.quiet,
            );
        }
        Commands::Strategies => {
            display_strategies();
        }
    }
}

/// Run round-robin tournament from CLI
fn run_cli_round_robin(
    rounds: u32,
    noise: f64,
    seed: Option<u64>,
    output: Option<&std::path::Path>,
    quiet: bool,
) {
    if !quiet {
        println!(
            "Running round-robin tournament ({} rounds, noise: {:.1}%)",
            rounds,
            noise * 100.0
        );
        if let Some(s) = seed {
            println!("Random seed: {}", s);
        }
    }

    let config = MatchConfig::with_rounds_and_noise(rounds, noise);
    let tournament = RoundRobinTournament::new(config);
    let result = tournament.run();

    // Export or display results
    if let Some(path) = output {
        match export::export_round_robin(&result, path) {
            Ok(()) => {
                if !quiet {
                    println!("Results exported to: {}", path.display());
                }
            }
            Err(e) => {
                eprintln!("Error exporting results: {}", e);
                std::process::exit(1);
            }
        }
    } else if !quiet {
        println!("{}", result.display_rankings());

        if let Some(winner) = result.winner() {
            println!(
                "\nWinner: {} with {} points",
                winner.name, winner.total_score
            );
        }
    }
}

/// Run ecological tournament from CLI
fn run_cli_ecological(
    rounds: u32,
    generations: u32,
    population: u32,
    noise: f64,
    seed: Option<u64>,
    output: Option<&std::path::Path>,
    quiet: bool,
) {
    if !quiet {
        println!(
            "Running ecological tournament ({} generations, {} rounds/match, pop: {}, noise: {:.1}%)",
            generations, rounds, population, noise * 100.0
        );
        if let Some(s) = seed {
            println!("Random seed: {}", s);
        }
    }

    let config = EcologicalConfig::new(
        MatchConfig::with_rounds_and_noise(rounds, noise),
        generations,
        population,
        1,
    );

    let tournament = EcologicalTournament::new(config);
    let generations_history = tournament.run();

    // Export or display results
    if let Some(path) = output {
        match export::export_ecological(&generations_history, path) {
            Ok(()) => {
                if !quiet {
                    println!("Results exported to: {}", path.display());
                }
            }
            Err(e) => {
                eprintln!("Error exporting results: {}", e);
                std::process::exit(1);
            }
        }
    } else if !quiet {
        println!(
            "{}",
            EcologicalTournament::display_evolution(&generations_history)
        );
    }
}

/// Run 1v1 match from CLI
fn run_cli_match(
    player1_name: &str,
    player2_name: &str,
    rounds: u32,
    noise: f64,
    seed: Option<u64>,
    output: Option<&std::path::Path>,
    quiet: bool,
) {
    let strategy1 = match StrategyType::from_name(player1_name) {
        Some(s) => s,
        None => {
            eprintln!("Unknown strategy: {}", player1_name);
            eprintln!("Use 'axelrod-rs strategies' to list available strategies.");
            std::process::exit(1);
        }
    };

    let strategy2 = match StrategyType::from_name(player2_name) {
        Some(s) => s,
        None => {
            eprintln!("Unknown strategy: {}", player2_name);
            eprintln!("Use 'axelrod-rs strategies' to list available strategies.");
            std::process::exit(1);
        }
    };

    if !quiet {
        println!(
            "{} vs {} ({} rounds, noise: {:.1}%)",
            player1_name,
            player2_name,
            rounds,
            noise * 100.0
        );
        if let Some(s) = seed {
            println!("Random seed: {}", s);
        }
    }

    let mut p1 = Player::new(strategy1);
    let mut p2 = Player::new(strategy2);

    let config = MatchConfig::with_rounds_and_noise(rounds, noise);
    let result = {
        let mut game = Match::new(&mut p1, &mut p2, config);
        if let Some(s) = seed {
            let mut rng = ChaCha8Rng::seed_from_u64(s);
            game.play_with_rng(&mut rng)
        } else {
            game.play()
        }
    };

    // Export or display results
    if let Some(path) = output {
        match export::export_match(&result, path) {
            Ok(()) => {
                if !quiet {
                    println!("Results exported to: {}", path.display());
                }
            }
            Err(e) => {
                eprintln!("Error exporting results: {}", e);
                std::process::exit(1);
            }
        }
    } else if !quiet {
        println!(
            "\nResult: {} {} - {} {}",
            result.player1_name, result.score1, result.score2, result.player2_name
        );
        match result.winner() {
            Some(w) => println!("Winner: {}", w),
            None => println!("Result: TIE"),
        }
        println!(
            "Mutual cooperation rate: {:.1}%",
            result.mutual_cooperation_rate() * 100.0
        );
    }
}

// ============================================================================
// INTERACTIVE MODE (original behavior)
// ============================================================================

fn run_interactive() {
    print_banner();

    loop {
        let options = vec![
            "Round-Robin Tournament (Classic Axelrod)",
            "Ecological Tournament (Evolutionary)",
            "1v1 Match (two strategies)",
            "List strategies",
            "Quit",
        ];

        let choice = Select::new("What would you like to do?", options).prompt();

        match choice {
            Ok("Round-Robin Tournament (Classic Axelrod)") => run_round_robin_tournament(),
            Ok("Ecological Tournament (Evolutionary)") => run_ecological_tournament(),
            Ok("1v1 Match (two strategies)") => run_1v1_match(),
            Ok("List strategies") => display_strategies(),
            Ok("Quit") => {
                println!("\nThank you for playing! See you in the arena.");
                break;
            }
            _ => {
                println!("Selection error or cancellation.");
                break;
            }
        }

        println!("\n");
    }
}

fn print_banner() {
    let title = r#"
 ███████████ █████                   █████    █████                                            
▒█▒▒▒███▒▒▒█▒▒███                   ▒▒███    ▒▒███                                             
▒   ▒███  ▒  ▒███████    ██████     ███████   ▒███████    ██████   ██████  ████████  █████ ████
    ▒███     ▒███▒▒███  ███▒▒███   ▒▒▒███▒    ▒███▒▒███  ███▒▒███ ███▒▒███▒▒███▒▒███▒▒███ ▒███ 
    ▒███     ▒███ ▒███ ▒███████      ▒███     ▒███ ▒███ ▒███████ ▒███ ▒███ ▒███ ▒▒▒  ▒███ ▒███ 
    ▒███     ▒███ ▒███ ▒███▒▒▒       ▒███ ███ ▒███ ▒███ ▒███▒▒▒  ▒███ ▒███ ▒███      ▒███ ▒███ 
    █████    ████ █████▒▒██████      ▒▒█████  ████ █████▒▒██████ ▒▒██████  █████     ▒▒███████ 
   ▒▒▒▒▒    ▒▒▒▒ ▒▒▒▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒  ▒▒▒▒ ▒▒▒▒▒  ▒▒▒▒▒▒   ▒▒▒▒▒▒  ▒▒▒▒▒       ▒▒▒▒▒███ 
                                                                                      ███ ▒███ 
                                                                                     ▒▒██████  
                                                                                      ▒▒▒▒▒▒   
"#;
    println!("{}", title);
    println!("Based on Robert Axelrod - The Evolution of Cooperation (1984)\n");
    println!(
        "Designed by {} - v{}\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_VERSION")
    );

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .add_row(vec![Cell::new(
            "Iterated Prisoner's Dilemma Simulator\nExplore the emergence of cooperation between automated strategies",
        )]);
    println!("{}\n", table);
}

fn run_round_robin_tournament() {
    println!("\n=== ROUND-ROBIN TOURNAMENT ===\n");

    // Configure number of rounds
    let rounds_input = Text::new("Number of rounds per match (default: 200):")
        .with_default("200")
        .prompt();

    let rounds: u32 = rounds_input
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(200);

    // Configure noise
    let noise_input = Text::new("Noise probability 0.0-1.0 (default: 0.0):")
        .with_default("0.0")
        .prompt();

    let noise: f64 = noise_input.ok().and_then(|s| s.parse().ok()).unwrap_or(0.0);

    println!(
        "\nStarting tournament with {} rounds per match, noise: {:.1}%...",
        rounds,
        noise * 100.0
    );
    println!("Each strategy plays against all others (including itself).\n");

    let config = MatchConfig::with_rounds_and_noise(rounds, noise);
    let tournament = RoundRobinTournament::new(config);
    let result = tournament.run();

    // Display results
    println!("{}", result.display_rankings());

    // Analysis
    println!("\n=== ANALYSIS ===");
    if let Some(winner) = result.winner() {
        println!(
            "Winner: {} with {} points (average: {:.1}/match)",
            winner.name, winner.total_score, winner.average_score
        );

        if winner.is_nice {
            println!("This is a 'nice' strategy (never defects first).");
        } else {
            println!("This is a 'nasty' strategy (may defect first).");
        }
    }

    // Count nice strategies in top 5
    let nice_in_top5 = result.rankings[..5.min(result.rankings.len())]
        .iter()
        .filter(|p| p.is_nice)
        .count();

    println!(
        "\n'Nice' strategies in top 5: {}/{}",
        nice_in_top5,
        5.min(result.rankings.len())
    );

    // Conclusion
    if nice_in_top5 >= 3 {
        println!("\n-> As Axelrod discovered, 'nice' strategies dominate!");
    }
}

fn run_ecological_tournament() {
    println!("\n=== ECOLOGICAL TOURNAMENT (EVOLUTIONARY) ===\n");

    // Configuration
    let generations_input = Text::new("Number of generations (default: 100):")
        .with_default("100")
        .prompt();

    let generations: u32 = generations_input
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let population_input = Text::new("Initial population per strategy (default: 100):")
        .with_default("100")
        .prompt();

    let initial_population: u32 = population_input
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let noise_input = Text::new("Noise probability 0.0-1.0 (default: 0.0):")
        .with_default("0.0")
        .prompt();

    let noise: f64 = noise_input.ok().and_then(|s| s.parse().ok()).unwrap_or(0.0);

    println!(
        "\nSimulating {} generations with {} individuals per strategy, noise: {:.1}%...",
        generations,
        initial_population,
        noise * 100.0
    );
    println!("Populations evolve based on each strategy's success.\n");

    let config = EcologicalConfig::new(
        MatchConfig::with_rounds_and_noise(200, noise),
        generations,
        initial_population,
        1,
    );

    let tournament = EcologicalTournament::new(config);
    let generations_history = tournament.run();

    // Display evolution
    println!(
        "{}",
        EcologicalTournament::display_evolution(&generations_history)
    );

    // Final analysis
    if let Some(last_gen) = generations_history.last() {
        println!("\n=== FINAL ANALYSIS ===");

        let alive = last_gen.alive_strategies();
        println!("Surviving strategies: {}", alive.len());

        if let Some(dominant) = last_gen.dominant_strategy() {
            let strategy = dominant.create();
            println!("Dominant strategy: {}", dominant);

            if strategy.is_nice() {
                println!("\n-> A 'nice' strategy dominated the evolution!");
                println!("  This confirms Axelrod's discovery: cooperation emerges.");
            } else {
                println!("\n-> A 'nasty' strategy survived.");
                println!("  This can happen in certain population configurations.");
            }
        }

        // Show evolution of nice strategies
        let nice_count = alive.iter().filter(|s| s.create().is_nice()).count();

        println!(
            "\nSurviving 'nice' strategies: {}/{}",
            nice_count,
            alive.len()
        );
    }
}

fn run_1v1_match() {
    println!("\n=== 1v1 MATCH ===\n");

    let strategies: Vec<&str> = StrategyType::all().iter().map(|s| s.name()).collect();

    let strategy1_name = Select::new("Choose the first strategy:", strategies.clone()).prompt();

    let strategy2_name = Select::new("Choose the second strategy:", strategies).prompt();

    if let (Ok(name1), Ok(name2)) = (strategy1_name, strategy2_name) {
        let strategy1 = find_strategy_by_name(name1);
        let strategy2 = find_strategy_by_name(name2);

        if let (Some(s1), Some(s2)) = (strategy1, strategy2) {
            let rounds_input = Text::new("Number of rounds (default: 200):")
                .with_default("200")
                .prompt();

            let rounds: u32 = rounds_input
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(200);

            let noise_input = Text::new("Noise probability 0.0-1.0 (default: 0.0):")
                .with_default("0.0")
                .prompt();

            let noise: f64 = noise_input.ok().and_then(|s| s.parse().ok()).unwrap_or(0.0);

            println!(
                "\n{} vs {} ({} rounds, noise: {:.1}%)\n",
                name1,
                name2,
                rounds,
                noise * 100.0
            );

            let mut player1 = Player::new(s1);
            let mut player2 = Player::new(s2);

            let config = MatchConfig::with_rounds_and_noise(rounds, noise);
            let result = {
                let mut game = Match::new(&mut player1, &mut player2, config);
                game.play()
            };

            // Match result table
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![
                    Cell::new("Player"),
                    Cell::new("Score"),
                    Cell::new("Coop%"),
                ]);

            table.add_row(vec![
                Cell::new(&result.player1_name),
                Cell::new(result.score1),
                Cell::new(format!("{:.1}%", result.cooperation_rate1() * 100.0)),
            ]);
            table.add_row(vec![
                Cell::new(&result.player2_name),
                Cell::new(result.score2),
                Cell::new(format!("{:.1}%", result.cooperation_rate2() * 100.0)),
            ]);

            println!("        MATCH RESULT\n");
            println!("{}", table);

            match result.winner() {
                Some(winner) => println!("\nWinner: {}", winner),
                None => println!("\nResult: TIE"),
            }

            println!(
                "Mutual cooperation: {:.1}%",
                result.mutual_cooperation_rate() * 100.0
            );

            // Display some rounds
            println!("\nFirst 10 rounds:");
            let mut rounds_table = Table::new();
            rounds_table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![
                    Cell::new("Round"),
                    Cell::new("P1"),
                    Cell::new("P2"),
                    Cell::new("Score"),
                ]);

            for (i, round) in result.rounds.iter().take(10).enumerate() {
                rounds_table.add_row(vec![
                    Cell::new(i + 1),
                    Cell::new(format!("{}", round.action1)),
                    Cell::new(format!("{}", round.action2)),
                    Cell::new(format!("{} - {}", round.score1, round.score2)),
                ]);
            }
            println!("{}", rounds_table);

            if result.rounds.len() > 10 {
                println!("... ({} rounds total)", result.rounds.len());
            }
        }
    }
}

fn display_strategies() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Strategy"),
            Cell::new("Nice"),
            Cell::new("Description"),
        ]);

    for strategy_type in StrategyType::all() {
        let strategy = strategy_type.create();
        let nice = if strategy.is_nice() { "Yes" } else { "No" };
        table.add_row(vec![
            Cell::new(strategy.name()),
            Cell::new(nice),
            Cell::new(strategy.description()),
        ]);
    }

    println!("\n      AVAILABLE STRATEGIES\n");
    println!("{}", table);

    println!("\nLegend:");
    println!("  Nice = Never defects first (winning characteristic according to Axelrod)");
}

fn find_strategy_by_name(name: &str) -> Option<StrategyType> {
    StrategyType::all().into_iter().find(|s| s.name() == name)
}
