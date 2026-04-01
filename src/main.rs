use inquire::{Select, Text};

use axelrod_rs::game::MatchConfig;
use axelrod_rs::player::Player;
use axelrod_rs::strategy::StrategyType;
use axelrod_rs::tournament::{EcologicalConfig, EcologicalTournament, RoundRobinTournament};

fn main() {
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
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║  Iterated Prisoner's Dilemma Simulator                                    ║");
    println!("║  Explore the emergence of cooperation between automated strategies        ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝\n");
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

    println!("\nStarting tournament with {} rounds per match...", rounds);
    println!("Each strategy plays against all others (including itself).\n");

    let config = MatchConfig::with_rounds(rounds);
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

    println!(
        "\nSimulating {} generations with {} individuals per strategy...",
        generations, initial_population
    );
    println!("Populations evolve based on each strategy's success.\n");

    let config = EcologicalConfig::new(
        MatchConfig::with_rounds(200),
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

            println!("\n{} vs {} ({} rounds)\n", name1, name2, rounds);

            let mut player1 = Player::new(s1);
            let mut player2 = Player::new(s2);

            let config = MatchConfig::with_rounds(rounds);
            let result = {
                let mut game = axelrod_rs::game::Match::new(&mut player1, &mut player2, config);
                game.play()
            };

            println!("╔═══════════════════════════════════════════════════════════════╗");
            println!("║                       MATCH RESULT                            ║");
            println!("╠═══════════════════════════════════════════════════════════════╣");
            println!(
                "║ {:<25} : {:>5} points ({:>5.1}% coop)    ║",
                truncate_str(&result.player1_name, 25),
                result.score1,
                result.cooperation_rate1() * 100.0
            );
            println!(
                "║ {:<25} : {:>5} points ({:>5.1}% coop)    ║",
                truncate_str(&result.player2_name, 25),
                result.score2,
                result.cooperation_rate2() * 100.0
            );
            println!("╠═══════════════════════════════════════════════════════════════╣");

            match result.winner() {
                Some(winner) => println!("║ Winner: {:<52} ║", winner),
                None => println!("║ Result: TIE                                               ║"),
            }

            println!(
                "║ Mutual cooperation: {:.1}%                                    ║",
                result.mutual_cooperation_rate() * 100.0
            );
            println!("╚═══════════════════════════════════════════════════════════════╝");

            // Display some rounds
            println!("\nFirst 10 rounds:");
            for (i, round) in result.rounds.iter().take(10).enumerate() {
                println!(
                    "  Round {:>3}: {} vs {} -> ({:>2}, {:>2})",
                    i + 1,
                    round.action1,
                    round.action2,
                    round.score1,
                    round.score2
                );
            }
            if result.rounds.len() > 10 {
                println!("  ... ({} rounds total)", result.rounds.len());
            }
        }
    }
}

fn display_strategies() {
    println!("\n╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║                        AVAILABLE STRATEGIES                              ║");
    println!("╠═══════════════════════════════════════════════════════════════════════════╣");
    println!("║ Name                     │ Nice │ Description                             ║");
    println!("╠══════════════════════════╪══════╪═════════════════════════════════════════╣");

    for strategy_type in StrategyType::all() {
        let strategy = strategy_type.create();
        let nice = if strategy.is_nice() { "Yes" } else { "No" };
        println!(
            "║ {:<24} │ {:>4} │ {:<39} ║",
            truncate_str(strategy.name(), 24),
            nice,
            truncate_str(strategy.description(), 39)
        );
    }

    println!("╚═══════════════════════════════════════════════════════════════════════════╝");

    println!("\nLegend:");
    println!("  Nice = Never defects first (winning characteristic according to Axelrod)");
}

fn find_strategy_by_name(name: &str) -> Option<StrategyType> {
    StrategyType::all().into_iter().find(|s| s.name() == name)
}

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
