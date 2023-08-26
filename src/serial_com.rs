/// Gestion d'un port série
///
/// Ce module gère un port série de manière synchrone.
///
/// Le port est identifié par son nom (COM1, COM2, etc.).
/// Sous windows, les liaisons après COM9 doivent s'identifier par "\.\COM10"
/// selon [la description de Microsoft](https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file)
/// Sous Linux, le nom d'un port est du style "/dev/ttyUSB0".
///
/// La primitive `available_names_list` est disponible pour obtenir la liste des noms
/// des ports séries disponibles sur la machine.
///
/// La création d'un port à 9600Bd (1 start, 8 data, 1 stop, sans parité ni contrôle) :
/// ```rs
/// use serial_com;
/// let port = serial_com::SerialCom::new("COM1", 9600);
/// ```
///
/// Les primitives `read`, `write` permettent de lire et d'écrire des vecteurs de `u8`.
/// TODO : Expliquer si le `read` est bloquant...
///
/// Le port nommé 'FAKE' a un comportement spécifique pour les besoins de tests. Voir `FakeSerialPort`
use crate::fake_serial_com::FakeSerialCom;

/// Retourne la liste des noms des ports séries disponibles sur cette machine
pub fn available_names_list() -> Vec<String> {
    let mut ret_list = vec![];
    match serial2::SerialPort::available_ports() {
        Err(e) => {
            eprintln!("Erreur fatal lors de la recherche des ports séries de cette machine : {e}");
            std::process::exit(1);
        }
        Ok(ports) => {
            ports
                .iter()
                .for_each(|port| ret_list.push(format!("{}", port.display())));
            ret_list
        }
    }
}

/// Distingue un FAKE port et un port existant
enum TypeSerialCom {
    FakePort(FakeSerialCom),
    RealPort(serial2::SerialPort),
}

/// Structure pour gérer un port série à 9600Bd / 1 start / 8 bits data / 1 stop
pub struct SerialCom {
    // Nom du port série
    name: String,

    // Objet serial associé
    port: TypeSerialCom,
}

impl SerialCom {
    /// Constructeur
    pub fn new(name: &str, baud_rate: u32) -> Self {
        if name.to_uppercase() == "FAKE" {
            // Cas d'un FAKE port série
            Self {
                name: name.to_owned(),
                port: TypeSerialCom::FakePort(FakeSerialCom::default()),
            }
        } else {
            let port = serial2::SerialPort::open(name, baud_rate);
            match port {
                Err(e) => {
                    eprintln!("Erreur lors de l'ouverture du port '{name}' : {e}");
                    std::process::exit(1);
                }
                Ok(port) => {
                    // Nécessaire ?
                    // let mut settings = serial2::Settings::from(port.get_configuration().unwrap());
                    // settings.set_raw();  // 1 start, 8 data, 1 stop, pas de parité ni de contrôle
                    Self {
                        name: name.to_owned(),
                        port: TypeSerialCom::RealPort(port),
                    }
                }
            }
        }
    }

    /// Lecture du port série
    /// `buffer` : Vec<u8> qu'on peut initialiser par `let mut buffer = [0; 512]`
    /// Return : Nombre d'octets lus
    /// # panics
    /// panic! si erreur de lecture du port
    pub fn read(&self, buffer: &mut [u8]) -> usize {
        match &self.port {
            TypeSerialCom::FakePort(fake) => fake.read(buffer),
            TypeSerialCom::RealPort(port) => match port.read(buffer) {
                Ok(n) => n,
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => 0,
                Err(e) => panic!("Erreur de lecture du port '{}' : {}", self.name, e),
            },
        }
    }

    /// Écriture du port série
    /// `buffer` : Vec<u8> à écriture
    /// # panics
    /// panics si erreur d'écriture du port
    pub fn write(&self, buffer: &[u8]) {
        match &self.port {
            TypeSerialCom::FakePort(fake) => fake.write(buffer),
            TypeSerialCom::RealPort(port) => {
                if let Err(e) = port.write_all(buffer) {
                    panic!("Erreur d'écriture du port '{}' : {}", self.name, e);
                }
            }
        }
    }

    /// Primitive pour les FAKE ports uniquement
    /// Sans effet si le port n'est pas un FAKE port
    pub fn will_read(&mut self, buffer: &[u8]) {
        match &mut self.port {
            TypeSerialCom::FakePort(fake) => fake.will_read(buffer),
            TypeSerialCom::RealPort(_port) => {
                eprint!(
                    "Usage inattendu de 'will_read' avec un port existant ({})",
                    self.name
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_serial_com_new() {
        let list_port_names = available_names_list();
        for name in list_port_names {
            let _serial_com = SerialCom::new(&name, 9600);
        }
    }

    #[test]
    fn test_fake_serial_com_new() {
        let _serial_com = SerialCom::new("FAKE", 9600);
    }
}
