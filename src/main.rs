/// Simulateur d'informatique embarquée
use std::env;

mod serial_com;
mod st2150;

use serial_com::{CommonSerialComTrait, SerialCom};

fn main() {
    let command_args: Vec<String> = env::args().collect();

    if command_args.len() == 2 {
        if vec![
            // Aide utilisateur
            "--HELP".to_string(),
            "HELP".to_string(),
            "-H".to_string(),
        ]
        .contains(&command_args[1].to_uppercase())
        {
            print_help();
        } else if vec![
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
            // Protocole ST2150 sur cette liaison série
            let _protocol = st2150::ST2150::new(port);
        }
    } else {
        // Sans argument ou avec trop d'arguments, on affiche l'aide à l'utilisateur
        print_help();
        print_serial_com_name_list();
        eprintln!();
    }
}

/// Affiche l'aide pour l'utilisateur
fn print_help() {
    eprintln!(
        r#"
Simulateur d'informatique embarquée - ALMA 2023.

Usage :
    sim_ie COM1             # Pour une machine Windows avec un port série 'COM1'
    sim_ie "\.\COM10"       # Syntaxe de Windows après COM9 (étrange, mais bon...)
    sim_ie "/dev/ttyUSB0"   # Pour une machine Linux

    sim --help              # Pour ce message d'aide
    sim --ports ou --list   #Liste des ports de la machine
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
