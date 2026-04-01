use inquire::{Select, Text};

use axelrod_rs::game::MatchConfig;
use axelrod_rs::player::Player;
use axelrod_rs::strategy::StrategyType;
use axelrod_rs::tournament::{EcologicalConfig, EcologicalTournament, RoundRobinTournament};

fn main() {
    print_banner();

    loop {
        let options = vec![
            "Tournoi Round-Robin (Axelrod classique)",
            "Tournoi Écologique (Évolutionnaire)",
            "Match 1v1 (deux stratégies)",
            "Liste des stratégies",
            "Quitter",
        ];

        let choice = Select::new("Que souhaitez-vous faire ?", options).prompt();

        match choice {
            Ok("Tournoi Round-Robin (Axelrod classique)") => run_round_robin_tournament(),
            Ok("Tournoi Écologique (Évolutionnaire)") => run_ecological_tournament(),
            Ok("Match 1v1 (deux stratégies)") => run_1v1_match(),
            Ok("Liste des stratégies") => display_strategies(),
            Ok("Quitter") => {
                println!("\nMerci d'avoir joué ! À bientôt dans l'arène.");
                break;
            }
            _ => {
                println!("Erreur lors de la sélection ou annulation.");
                break;
            }
        }

        println!("\n");
    }
}

fn print_banner() {
    let titre = r#"
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
    println!("{}", titre);
    println!("D'après Robert Axelrod - The Evolution of Cooperation (1984)\n");
    println!(
        "Conception de {} - v{}\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_VERSION")
    );
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║  Simulateur du Dilemme du Prisonnier Itéré                                ║");
    println!("║  Explorez l'émergence de la coopération entre stratégies automatisées     ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝\n");
}

fn run_round_robin_tournament() {
    println!("\n═══ TOURNOI ROUND-ROBIN ═══\n");

    // Configuration du nombre de rounds
    let rounds_input = Text::new("Nombre de rounds par match (défaut: 200):")
        .with_default("200")
        .prompt();

    let rounds: u32 = rounds_input
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(200);

    println!("\nLancement du tournoi avec {} rounds par match...", rounds);
    println!("Chaque stratégie joue contre toutes les autres (y compris elle-même).\n");

    let config = MatchConfig::with_rounds(rounds);
    let tournament = RoundRobinTournament::new(config);
    let result = tournament.run();

    // Affichage des résultats
    println!("{}", result.display_rankings());

    // Analyse
    println!("\n═══ ANALYSE ═══");
    if let Some(winner) = result.winner() {
        println!(
            "Vainqueur: {} avec {} points (moyenne: {:.1}/match)",
            winner.name, winner.total_score, winner.average_score
        );

        if winner.is_nice {
            println!("C'est une stratégie 'gentille' (ne trahit jamais en premier).");
        } else {
            println!("C'est une stratégie 'méchante' (peut trahir en premier).");
        }
    }

    // Compter les stratégies nice dans le top 5
    let nice_in_top5 = result.rankings[..5.min(result.rankings.len())]
        .iter()
        .filter(|p| p.is_nice)
        .count();

    println!(
        "\nStratégies 'gentilles' dans le top 5: {}/{}",
        nice_in_top5,
        5.min(result.rankings.len())
    );

    // Conclusion
    if nice_in_top5 >= 3 {
        println!("\n→ Comme Axelrod l'a découvert, les stratégies 'gentilles' dominent !");
    }
}

fn run_ecological_tournament() {
    println!("\n═══ TOURNOI ÉCOLOGIQUE (ÉVOLUTIONNAIRE) ═══\n");

    // Configuration
    let generations_input = Text::new("Nombre de générations (défaut: 100):")
        .with_default("100")
        .prompt();

    let generations: u32 = generations_input
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let population_input = Text::new("Population initiale par stratégie (défaut: 100):")
        .with_default("100")
        .prompt();

    let initial_population: u32 = population_input
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    println!(
        "\nSimulation de {} générations avec {} individus par stratégie...",
        generations, initial_population
    );
    println!("Les populations évoluent selon le succès de chaque stratégie.\n");

    let config = EcologicalConfig::new(
        MatchConfig::with_rounds(200),
        generations,
        initial_population,
        1,
    );

    let tournament = EcologicalTournament::new(config);
    let generations_history = tournament.run();

    // Affichage de l'évolution
    println!(
        "{}",
        EcologicalTournament::display_evolution(&generations_history)
    );

    // Analyse finale
    if let Some(last_gen) = generations_history.last() {
        println!("\n═══ ANALYSE FINALE ═══");

        let alive = last_gen.alive_strategies();
        println!("Stratégies survivantes: {}", alive.len());

        if let Some(dominant) = last_gen.dominant_strategy() {
            let strategy = dominant.create();
            println!("Stratégie dominante: {}", dominant);

            if strategy.is_nice() {
                println!("\n→ Une stratégie 'gentille' a dominé l'évolution !");
                println!("  Cela confirme la découverte d'Axelrod: la coopération émerge.");
            } else {
                println!("\n→ Une stratégie 'méchante' a survécu.");
                println!("  Cela peut arriver dans certaines configurations de population.");
            }
        }

        // Montrer l'évolution des stratégies nice
        let nice_count = alive.iter().filter(|s| s.create().is_nice()).count();

        println!(
            "\nStratégies 'gentilles' survivantes: {}/{}",
            nice_count,
            alive.len()
        );
    }
}

fn run_1v1_match() {
    println!("\n═══ MATCH 1v1 ═══\n");

    let strategies: Vec<&str> = StrategyType::all().iter().map(|s| s.name()).collect();

    let strategy1_name =
        Select::new("Choisissez la première stratégie:", strategies.clone()).prompt();

    let strategy2_name = Select::new("Choisissez la deuxième stratégie:", strategies).prompt();

    if let (Ok(name1), Ok(name2)) = (strategy1_name, strategy2_name) {
        let strategy1 = find_strategy_by_name(name1);
        let strategy2 = find_strategy_by_name(name2);

        if let (Some(s1), Some(s2)) = (strategy1, strategy2) {
            let rounds_input = Text::new("Nombre de rounds (défaut: 200):")
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
            println!("║                    RÉSULTAT DU MATCH                          ║");
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
                Some(winner) => println!("║ Vainqueur: {:<50} ║", winner),
                None => println!("║ Résultat: ÉGALITÉ                                         ║"),
            }

            println!(
                "║ Coopération mutuelle: {:.1}%                                  ║",
                result.mutual_cooperation_rate() * 100.0
            );
            println!("╚═══════════════════════════════════════════════════════════════╝");

            // Afficher quelques rounds
            println!("\n10 premiers rounds:");
            for (i, round) in result.rounds.iter().take(10).enumerate() {
                println!(
                    "  Round {:>3}: {} vs {} → ({:>2}, {:>2})",
                    i + 1,
                    round.action1,
                    round.action2,
                    round.score1,
                    round.score2
                );
            }
            if result.rounds.len() > 10 {
                println!("  ... ({} rounds au total)", result.rounds.len());
            }
        }
    }
}

fn display_strategies() {
    println!("\n╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║                        STRATÉGIES DISPONIBLES                             ║");
    println!("╠═══════════════════════════════════════════════════════════════════════════╣");
    println!("║ Nom                      │ Nice │ Description                             ║");
    println!("╠══════════════════════════╪══════╪═════════════════════════════════════════╣");

    for strategy_type in StrategyType::all() {
        let strategy = strategy_type.create();
        let nice = if strategy.is_nice() { "Oui" } else { "Non" };
        println!(
            "║ {:<24} │ {:>4} │ {:<39} ║",
            truncate_str(strategy.name(), 24),
            nice,
            truncate_str(strategy.description(), 39)
        );
    }

    println!("╚═══════════════════════════════════════════════════════════════════════════╝");

    println!("\nLégende:");
    println!("  Nice = Ne trahit jamais en premier (caractéristique gagnante selon Axelrod)");
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
