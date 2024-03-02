# README pour les développeurs

Ce projet est en [Rust](https://www.rust-lang.org/) avec son gestionnaire [Cargo](https://doc.rust-lang.org/cargo/).

Ce code est portable Windows ou Linux.
Mais la librairie WSL pour la gestion des ports séries est manquante dans l'installation par défaut de WSL.

Toutes les commandes dont identique entre Windows et Linux.

Voir le fichier [readme.md](../README.md) pour l'usage de l'outil.

## Instructions pour le développeur

L'environnement de développement recommandé est [VSCode](https://code.visualstudio.com/).

Pour [Rust](https://www.rust-lang.org/), les extensions suivantes sont recommandées:

* `rust-analyzer` : Support VSCode pour le [Rust](https://www.rust-lang.org/)
* `crates` : Vérifie si les dépendances sont valides et à jour
* `Code Spell Checker` + `French - Code Spell Checker` : Pour éviter les typos

Pour les opérations de développement :

* La version du logiciel est dans `Cargo.toml`
* Pour compiler un livrable dans `target/release` : `cargo build --release`
* Pour formatter proprement le code source du projet : `cargo fmt`
* Pour vérifier rapidement la cohérence du projet : `cargo check`
* Pour vérifier la cohérence du projet avec quelques conseils d'amélioration : `cargo clippy`
* Pour des instructions plus fines des améliorations possibles dans le code : `cargo clippy -- W clippy::pedantic`
* Idem en incluant le code pour les tests : `cargo clippy --test -- W clippy::pedantic`
* Pour exécuter tous les tests unitaires du projet : `cargo test`
* Pour lancer l'application (mode console) : `cargo run`
* Pour lancer l'application en mode graphique avec un port série 'bidon' : `cargo run -- FAKE`
* Idem avec un vrai port série 'COM1' :`cargo run -- COM1`
* Pour générer et visualiser la documentation du projet (sans les dépendances)  : `cargo doc --open --no-deps`
* Pour calculer la couverture de code des tests unitaires : `cargo tarpaulin`

Nota : Certaines commandes `cargo` ne sont pas installées par défaut : `cargo install cargo-XXX` pour avoir `cargo XXX`

Rappel: Pour la mise à jour de l'environnement [Rust](https://www.rust-lang.org/) : `rustup check` ou `rustup update`

Attention : Le fichier `aaa.json` pour l'archivage du projet par `rsaaa` doit être au format `Windows 1252` (ou `ISO-8859-1`). Par défaut, il est ouvert en `UTF-8` par [VSCode](https://code.visualstudio.com/) et ce format introduit des soucis avec le chemin du livrable `Générique` qui est accentué. En cas de problème, il est nécessaire d'éditer ce fichier `aaa.json` avec l'encodage correct.

Pour le calcul des 'checksum' des messages pour les tests, ne pas perdre du temps à calculer cette valeur manuellement: Mettre une valeur 'bidon' et exécuter le test qui va échouer en indiquant la valeur attendue...