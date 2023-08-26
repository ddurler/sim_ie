/// Gestion d'un port série

// Vitesse du port géré
const BAUD_RATE: u32 = 9600;

/// Retourne la liste des ports séries disponibles sur cette machine
pub fn get_list() -> Vec<String> {
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

/// Structure pour gérer un port série à 9600Bd / 1 start / 8 bits data / 1 stop
pub struct SerialCom {
    // Nom du port série
    name: String,

    // Objet serial associé
    port: serial2::SerialPort,
}

impl SerialCom {
    /// Constructeur
    pub fn new(name: &str) -> Self {
        let port = serial2::SerialPort::open(name, BAUD_RATE);
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
                    port,
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
        match self.port.read(buffer) {
            Ok(n) => n,
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => 0,
            Err(e) => panic!("Erreur de lecture du port '{}' : {}", self.name, e),
        }
    }

    /// Écriture du port série
    /// `buffer` : Vec<u8> à écriture
    /// # panics
    /// panics si erreur d'écriture du port
    pub fn write(&self, buffer: &[u8]) {
        if let Err(e) = self.port.write_all(buffer) {
            panic!("Erreur d'écriture du port '{}' : {}", self.name, e);
        }
    }
}
