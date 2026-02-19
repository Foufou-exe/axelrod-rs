use inquire::Select;

fn main() {
    // 1. Affichage du titre en ASCII Art avec une "raw string" (r#""#)
    let titre = r#"                                                                                                
 ▄   ▄▄▄▄                                                                                          
 ▀██████▀                        █▄ █▄                                                             
   ██   ▄       ▄               ▄██▄██                ▄                                            
   ██  ██ ▄▀▀█▄ ███▄███▄ ▄█▀█▄   ██ ████▄ ▄█▀█▄ ▄███▄ ████▄██ ██                                   
   ██  ██ ▄█▀██ ██ ██ ██ ██▄█▀   ██ ██ ██ ██▄█▀ ██ ██ ██   ██▄██                                   
   ▀█████▄▀█▄██▄██ ██ ▀█▄▀█▄▄▄  ▄██▄██ ██▄▀█▄▄▄▄▀███▀▄█▀  ▄▄▀██▀                                   
   ▄   ██                                                    ██                                    
   ▀████▀                                                  ▀▀▀                                                                                                                                                                                                                                                                     
"#;

    println!("{}", titre);
    println!("Bienvenue dans l'arène !\n");

    // 2. Définition des choix d'adversaires
    let options_adversaire = vec![
        "Joueur Local (1v1 sur le même écran)",
        "IA - Facile (Coups aléatoires)",
        "IA - Difficile (Stratégique)",
        "Quitter le jeu",
    ];

    // 3. Affichage du menu interactif
    let choix = Select::new("Quel type d'adversaire souhaitez-vous affronter ?", options_adversaire)
        .prompt();

    // 4. Gestion de la réponse
    match choix {
        Ok(selection) => {
            if selection == "Quitter le jeu" {
                println!("À bientôt !");
                return;
            }
            
            println!("\n=> Préparation de la partie contre : {}", selection);
        }
        Err(_) => {
            println!("Erreur lors de la sélection ou annulation (Ctrl+C).");
        }
    }
}