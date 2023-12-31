//! Gestion d'un port série
//!
//! Ce module gère un port série de manière synchrone.
//!
//! Le port est identifié par son nom (COM1, COM2, etc.).
//! Sous windows, les liaisons après COM9 doivent s'identifier par "\.\COM10"
//! selon [la description de Microsoft](https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file)
//! Sous Linux, le nom d'un port est du style "/dev/ttyUSB0".
//!
//! La primitive `available_names_list` est disponible pour obtenir la liste des noms
//! des ports séries disponibles sur la machine.
//!
//! La création d'un port à 9600Bd (1 start, 8 data, 1 stop, sans parité ni contrôle) :
//! ```rs
//! use serial_com;
//! let port = serial_com::SerialCom::new("COM1", 9600);
//! ```
//!
//! Les primitives `read`, `write` permettent de lire et d'écrire des vecteurs de `u8`.
//! TODO : Expliquer si le `read` est bloquant...

use std::time::Duration;

use crate::CommonSerialComTrait;

/// Retourne la liste des noms des ports séries disponibles sur cette machine
pub fn available_names_list() -> Vec<String> {
    let mut ret_list = vec![];
    match serialport::available_ports() {
        Err(e) => {
            eprintln!("Erreur fatal lors de la recherche des ports séries de cette machine : {e}");
            std::process::exit(1);
        }
        Ok(ports) => {
            ports
                .iter()
                .for_each(|port| ret_list.push(port.port_name.to_string()));
            ret_list
        }
    }
}

/// Structure pour gérer un port série à 9600Bd / 1 start / 8 bits data / 1 stop
pub struct TrueSerialCom {
    /// Nom du port série
    pub name: String,

    /// Objet serial associé
    pub port: Box<dyn serialport::SerialPort>,
}

impl TrueSerialCom {
    /// Constructeur
    pub fn new(name: &str, baud_rate: u32) -> Self {
        let port = serialport::new(name, baud_rate)
            .timeout(Duration::from_millis(10))
            .flow_control(serialport::FlowControl::None)
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::Two)
            .parity(serialport::Parity::None)
            .open();
        match port {
            Err(e) => {
                eprintln!("Erreur lors de l'ouverture du port '{name}' : {e}");
                std::process::exit(1);
            }
            Ok(port) => Self {
                name: name.to_owned(),
                port,
            },
        }
    }
}

impl CommonSerialComTrait for TrueSerialCom {
    /// Lecture du port série
    /// `buffer` : `Vec<u8>` qu'on peut initialiser par `let mut buffer = [0; 512]`
    /// Return : Nombre d'octets lus
    /// # Panics
    /// panic! si erreur de lecture du port
    fn read(&mut self, buffer: &mut [u8]) -> usize {
        match &self.port.read(buffer) {
            Ok(n) => *n,
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => 0,
            Err(e) => panic!("Erreur de lecture du port '{}' : {}", self.name, e),
        }
    }

    /// Écriture du port série
    /// `buffer` : `Vec<u8>` à écriture
    /// # Panics
    /// panics! si erreur d'écriture du port
    fn write(&mut self, buffer: &[u8]) {
        if let Err(e) = self.port.write_all(buffer) {
            panic!("Erreur d'écriture du port '{}' : {}", self.name, e);
        }
    }

    /// Primitive pour les FAKE ports uniquement
    /// Sans effet si le port n'est pas un FAKE port
    fn should_write(&mut self, _buffer: &[u8]) {
        eprint!(
            "Usage inattendu de 'should_write' avec un port existant ({})",
            self.name
        );
    }

    /// Primitive pour les FAKE ports uniquement
    /// Sans effet si le port n'est pas un FAKE port
    fn will_read(&mut self, _buffer: &[u8]) {
        eprint!(
            "Usage inattendu de 'will_read' avec un port existant ({})",
            self.name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_serial_com_new() {
        let list_port_names = available_names_list();
        for name in list_port_names {
            let _serial_com = TrueSerialCom::new(&name, 9600);
        }
    }
}
