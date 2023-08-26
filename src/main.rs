/// Simulateur d'informatique embarquée
use std::env;

mod serial_com;
use serial_com::SerialCom;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let _port = SerialCom::new(&args[1]);
    } else {
        // Sans argument ou avec trop d'argument, on affiche l'aide à l'utilisateur
        print_help();
        print_serial_com_list();
        println!();
    }
}

/// Affiche l'aide pour l'utilisateur
fn print_help() {
    println!(
        r#"
Simulateur d'informatique embarquée - ALMA 2023.

Usage :
    sim_ie COM1             # Pour une machine Windows avec un port série 'COM1'
    sim_ie "\.\COM10"       # Après COM9, Windows impose cette syntaxe (étrange, mais bon...)
    sim_ie "/dev/ttyUSB0"   # Pour une machine Linux

    sim --help              # Pour ce message d'aide
    sim --ports             # Pour avoir la liste des ports de la machine
"#
    );
}

/// Affiche la liste des ports séries de la machine
fn print_serial_com_list() {
    let ports_list = serial_com::get_list();
    if ports_list.is_empty() {
        println!("Désolé, pas de port série sur cette machine :(");
    } else {
        println!("Ports séries de cette machine :");
        for port in ports_list {
            println!("{port}");
        }
    }
}
