# axelrod-rs

[![CI](https://github.com/foufou-exe/axelrod-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/foufou-exe/axelrod-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Quality gate](https://sonarcloud.io/api/project_badges/quality_gate?project=Foufou-exe_axelrod-rs)](https://sonarcloud.io/summary/new_code?id=Foufou-exe_axelrod-rs)
[![Coverage](https://sonarcloud.io/api/project_badges/measure?project=Foufou-exe_axelrod-rs&metric=coverage)](https://sonarcloud.io/summary/new_code?id=Foufou-exe_axelrod-rs)
[![SonarQube Cloud](https://sonarcloud.io/images/project_badges/sonarcloud-highlight.svg)](https://sonarcloud.io/summary/new_code?id=Foufou-exe_axelrod-rs)


*[English version](README.md)*

Un simulateur haute performance de theorie des jeux evolutionnaire ecrit en Rust. Ce projet explore le **Dilemme du Prisonnier Itere** en confrontant diverses strategies automatisees pour observer l'emergence de la cooperation, de la confiance et de la trahison dans un environnement competitif.

Inspire des travaux pionniers de Robert Axelrod *"The Evolution of Cooperation"* (1984) et de ses tournois informatiques (1980-1984).

## Table des Matieres

- [Fonctionnalites](#fonctionnalites)
- [Installation](#installation)
- [Demarrage Rapide](#demarrage-rapide)
- [Strategies](#strategies)
- [Modes de Tournoi](#modes-de-tournoi)
- [Reference CLI](#reference-cli)
- [Le Dilemme du Prisonnier](#le-dilemme-du-prisonnier)
- [Contribuer](#contribuer)
- [Licence](#licence)

## Fonctionnalites

- **20 strategies classiques** des tournois originaux d'Axelrod (1980-1984)
- **Tournois Round-Robin** - Chaque strategie affronte toutes les autres
- **Tournois ecologiques/evolutionnaires** - Dynamique de population sur plusieurs generations
- **Mode Match 1v1** - Confrontation directe entre deux strategies
- **Execution parallele** - Traitement multi-thread avec Rayon
- **Simulation de bruit** - Introduction d'erreurs aleatoires pour tester la robustesse
- **Resultats reproductibles** - Support de seed pour simulations deterministes
- **Export des resultats** - Formats CSV et JSON
- **Mode interactif** - Interface CLI conviviale avec `inquire`

## Installation

### Depuis les Sources

Necessite [Rust](https://rustup.rs/) (1.85+ recommande pour l'edition Rust 2024).

```bash
git clone https://github.com/foufou-exe/axelrod-rs.git
cd axelrod-rs
cargo build --release
```

Le binaire sera disponible dans `target/release/axelrod-rs`.

### Depuis les Releases

Telechargez les binaires pre-compiles depuis la page [Releases](https://github.com/foufou-exe/axelrod-rs/releases).

## Demarrage Rapide

### Mode Interactif

Lancez simplement sans arguments pour une interaction guidee :

```bash
./axelrod-rs
```

### Lancer un Tournoi Round-Robin

```bash
./axelrod-rs round-robin --rounds 200
```

### Lancer un Tournoi Ecologique

```bash
./axelrod-rs ecological --rounds 200 --generations 100
```

### Lancer un Match 1v1

```bash
./axelrod-rs match --strategy1 tit-for-tat --strategy2 random --rounds 200
```

### Lister Toutes les Strategies

```bash
./axelrod-rs strategies
```

## Strategies

Les 20 strategies implementees depuis les tournois d'Axelrod :

| Strategie | Description | Gentille* |
|-----------|-------------|-----------|
| **Always Cooperate** | Coopere toujours | Oui |
| **Always Defect** | Trahit toujours | Non |
| **Tit for Tat** | Coopere d'abord, puis copie le dernier coup adverse | Oui |
| **Suspicious Tit for Tat** | Comme TFT mais trahit en premier | Non |
| **Tit for Two Tats** | Trahit seulement apres deux trahisons consecutives | Oui |
| **Generous Tit for Tat** | TFT avec 10% de chance de pardonner une trahison | Oui |
| **Grudger** | Coopere jusqu'a etre trahi, puis trahit toujours | Oui |
| **Random** | 50/50 cooperation/trahison | Non |
| **Pavlov** | Strategie Gagnant-Reste, Perdant-Change | Oui |
| **Prober** | Teste l'adversaire avec D-C-C, puis exploite ou joue TFT | Non |
| **Hard Go By Majority** | Trahit si l'adversaire a trahi >= 50% | Non |
| **Soft Go By Majority** | Coopere si l'adversaire a coopere >= 50% | Oui |
| **Joss** | TFT sournois - trahit aleatoirement ~10% du temps | Non |
| **Graaskamp** | TFT avec une trahison test au tour 50 | Non |
| **Tullock** | Coopere 11 tours, puis trahit principalement | Non |
| **Feld** | TFT avec cooperation decroissante (100% vers 50%) | Non |
| **Nydegger** | Table de correspondance basee sur les 3 derniers tours | Non |
| **Grofman** | Coopere si les deux joueurs ont fait la meme action au tour precedent | Oui |
| **Shubik** | Represailles croissantes - N represailles apres la Neme trahison | Oui |
| **Davis** | 10 tours de grace, puis comportement Grudger | Oui |

*\*Gentille : Ne trahit jamais en premier*

## Modes de Tournoi

### Tournoi Round-Robin

Chaque strategie affronte toutes les autres strategies (y compris elle-meme). Les resultats sont classes par score total.

```bash
./axelrod-rs round-robin --rounds 200 --output resultats.csv
```

### Tournoi Ecologique

Simule l'evolution : les strategies avec des scores eleves se reproduisent, celles avec des scores faibles disparaissent. Observez l'evolution des populations au fil des generations.

```bash
./axelrod-rs ecological --rounds 200 --generations 100 --initial-population 100
```

### Match 1v1

Confrontation directe entre deux strategies specifiques avec une analyse coup par coup detaillee.

```bash
./axelrod-rs match -1 grudger -2 prober --rounds 50
```

## Reference CLI

```
axelrod-rs [COMMANDE]

Commandes:
  round-robin  Lancer un tournoi round-robin
  ecological   Lancer un tournoi ecologique/evolutionnaire
  match        Lancer un match 1v1 entre deux strategies
  strategies   Lister toutes les strategies disponibles
  help         Afficher l'aide

Options Globales:
  --rounds <N>       Nombre de tours par match (defaut: 200)
  --noise <PROB>     Probabilite d'erreur 0.0-1.0 (defaut: 0.0)
  --seed <N>         Seed aleatoire pour reproductibilite
  --output <FICHIER> Exporter les resultats en CSV ou JSON
  --quiet            Sortie minimale
  -h, --help         Afficher l'aide
  -V, --version      Afficher la version
```

### Exemples

```bash
# Tournoi avec bruit (5% d'erreur)
./axelrod-rs round-robin --noise 0.05 --rounds 200

# Simulation ecologique reproductible
./axelrod-rs ecological --seed 42 --generations 500

# Exporter les resultats d'un match en JSON
./axelrod-rs match -1 tit-for-tat -2 pavlov --output match.json
```

## Le Dilemme du Prisonnier

Le Dilemme du Prisonnier est un jeu fondamental en theorie des jeux. Deux joueurs choisissent simultanement de **Cooperer (C)** ou **Trahir (D)**.

### Matrice des Gains

|  | Adversaire Coopere | Adversaire Trahit |
|--|---------------------|------------------|
| **Vous Cooperez** | R=3, R=3 | S=0, T=5 |
| **Vous Trahissez** | T=5, S=0 | P=1, P=1 |

- **R (Recompense)**: 3 points - Cooperation mutuelle
- **T (Tentation)**: 5 points - Vous trahissez, l'adversaire coopere
- **S (Dupe)**: 0 points - Vous cooperez, l'adversaire trahit
- **P (Punition)**: 1 point - Trahison mutuelle

Le dilemme : la trahison est individuellement rationnelle, mais la cooperation mutuelle produit de meilleurs resultats collectifs.

### Decouverte d'Axelrod

Dans ses tournois des annees 1980, Axelrod a decouvert que les strategies **"gentilles"** (celles qui ne trahissent jamais en premier) surpassaient systematiquement les strategies agressives. **Tit for Tat** (Donnant-Donnant) - la strategie gentille la plus simple - a remporte les deux tournois.

Proprietes cles des strategies gagnantes :
1. **Gentille** - Ne jamais trahir en premier
2. **Retaliative** - Punir la trahison
3. **Pardonnante** - Revenir a la cooperation apres punition
4. **Claire** - Etre previsible pour que l'adversaire s'adapte

## Contribuer

Les contributions sont les bienvenues ! N'hesitez pas a soumettre des issues ou des pull requests.

```bash
# Lancer les tests
cargo test

# Lancer avec toutes les verifications
cargo fmt && cargo clippy && cargo test
```

## Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de details.

---

*Cree avec passion pour la theorie des jeux et l'evolution de la cooperation.*
