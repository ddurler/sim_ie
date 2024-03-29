//! Simulateur d'informatique embarquée ALMA - ST 2150
use std::env;

mod app_view;
mod context;
mod serial_com;
mod st2150;

use context::Context;
use serial_com::{CommonSerialComTrait, SerialCom};
use st2150::ST2150;

/// Version de l'application (selon définition dans Cargo.toml)
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Point d'entrée de l'outil
fn main() {
    let command_args: Vec<String> = env::args().collect();

    if command_args.len() == 2 {
        if [
            // Aide utilisateur
            "--HELP".to_string(),
            "HELP".to_string(),
            "-H".to_string(),
        ]
        .contains(&command_args[1].to_uppercase())
        {
            print_help();
        } else if [
            // Liste ports de la machine
            "--PORTS".to_string(),
            "--LIST".to_string(),
            "PORTS".to_string(),
            "LIST".to_string(),
        ]
        .contains(&command_args[1].to_uppercase())
        {
            print_serial_com_name_list();
        } else if command_args[1].starts_with('-') {
            // Option inconnue
            print_help();
            eprintln!();
            eprintln!("Erreur option inconnue : '{}'\n", command_args[1]);
        } else {
            // port série défini en ligne de commande
            let port = SerialCom::new(&command_args[1], 9600);

            // Protocole ALMA IE - ST2150 sur cette liaison série
            let st2150 = ST2150::new(port);

            // Application de test sur le terminal
            // run_on_terminal(&mut st2150);

            // IHM application avec l'utilisateur
            app_view::run(st2150);
        }
    } else {
        // Sans argument ou avec trop d'arguments, on affiche l'aide à l'utilisateur
        print_help();
        print_serial_com_name_list();
        eprintln!();
    }
}

/// Fonction pour test sur le terminal (sans IHM)
#[allow(dead_code)]
fn run_on_terminal(st2150: &mut ST2150) {
    // Création d'un contexte
    let mut context = Context::default();

    for message_num in [0_u8, 10_u8] {
        assert!(ST2150::message_availability(&context, message_num).is_ok());

        println!("Trying message #{message_num}");

        let ret = st2150.do_message_vacation(&mut context, message_num);

        if ret.is_err() {
            dbg!(ret.err());
        }
    }
}

/// Affiche l'aide pour l'utilisateur
fn print_help() {
    eprintln!(
        r#"
Simulateur d'informatique embarquée v{APP_VERSION} - ALMA 2023-2024.

Usage en mode graphique :
    sim_ie COM1               # Pour une machine Windows avec un port série 'COM1'
    sim_ie "\.\COM10"         # Syntaxe de Windows après COM9 (étrange, mais bon...)
    sim_ie "/dev/ttyUSB0"     # Pour une machine Linux

Usage en mode terminal :
    sim_ie --help             # Pour ce message d'aide
    sim_ie --ports ou --list  # Liste des ports de la machine
"#
    );
}

/// Affiche la liste des noms des ports séries de la machine
fn print_serial_com_name_list() {
    let port_names_list = serial_com::available_names_list();
    if port_names_list.is_empty() {
        eprintln!("Désolé, pas de port série sur cette machine :(");
    } else {
        eprintln!("Ports séries de cette machine :");
        for name in port_names_list {
            eprintln!("{name}");
        }
    }
}
