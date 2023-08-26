/// Simulateur d'informatique embarquée
mod serial_com;

fn main() {
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
