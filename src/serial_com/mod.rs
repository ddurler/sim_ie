//! Gestion d'un port série réel ou virtuel
//!
//! Un port nommé 'FAKE' a un comportement spécifique pour les besoins de tests. Voir `FakeSerialPort`.
//! Sinon, il s'agit d'un port réel de la machine qu'on cherche à gérer. Voir `TrueSerialPort`.
//! Dans tous les 2 cas, le port implémente le trait `CommonSerialComTrait`.
mod fake_serial_com;
mod true_serial_com;

/// Retourne la liste des noms des ports séries disponibles sur cette machine
pub fn available_names_list() -> Vec<String> {
    true_serial_com::available_names_list()
}

/// Façade entre un 'true' port d'un FAKE port
pub struct SerialCom {
    /// Nom du port
    pub name: String,

    /// Port 'true' ou FAKE sous-jacent
    port: Box<dyn CommonSerialComTrait>,
}

impl Default for SerialCom {
    fn default() -> Self {
        SerialCom {
            name: "FAKE".to_string(),
            port: Box::<fake_serial_com::FakeSerialCom>::default(),
        }
    }
}

/// Trait à implémenter pour les `SerialCom` (true ou FAKE)
pub trait CommonSerialComTrait {
    /// Lecture du port
    fn read(&mut self, buffer: &mut [u8]) -> usize;

    /// Écriture du port
    fn write(&mut self, buffer: &[u8]);

    ///FAKE panic! si la prochaine écriture n'est pas celle attendue
    fn should_write(&mut self, buffer: &[u8]);

    /// FAKE : Force les lectures à suivre
    fn will_read(&mut self, buffer: &[u8]);
}

impl SerialCom {
    /// Constructeur
    /// Le `name` "FAKE" permet ici de créer un port pour faire des tests
    pub fn new(name: &str, baud_rate: u32) -> Self {
        if name.to_uppercase() == "FAKE" {
            // Cas d'un FAKE port série
            SerialCom {
                name: name.to_string(),
                port: Box::<fake_serial_com::FakeSerialCom>::default(),
            }
        } else {
            SerialCom {
                name: name.to_string(),
                port: Box::new(true_serial_com::TrueSerialCom::new(name, baud_rate)),
            }
        }
    }
}

impl CommonSerialComTrait for SerialCom {
    /// Lecture du port série
    /// `buffer` : `Vec<u8>` qu'on peut initialiser par `let mut buffer = [0; 512]`
    /// Return : Nombre d'octets lus
    /// # Panics
    /// panic! si erreur de lecture du port réel de la machine
    fn read(&mut self, buffer: &mut [u8]) -> usize {
        self.port.read(buffer)
    }

    /// Écriture du port série
    /// `buffer` : `Vec<u8>` à écriture
    /// # Panics
    /// panics si erreur d'écriture d'un port réel de la machine
    fn write(&mut self, buffer: &[u8]) {
        self.port.write(buffer);
    }

    /// Primitive pour les FAKE ports uniquement
    /// Sans effet si le port n'est pas un FAKE port
    fn should_write(&mut self, buffer: &[u8]) {
        self.port.should_write(buffer);
    }

    /// Primitive pour les FAKE ports uniquement
    /// Sans effet si le port n'est pas un FAKE port
    fn will_read(&mut self, buffer: &[u8]) {
        self.port.will_read(buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_com_new() {
        // Création d'un FAKE port si le nom est "FAKE" ou "fake"
        let mut serial_com = SerialCom::new("fake", 9600);

        // Nom du port machine utilisé
        assert_eq!(serial_com.name.to_uppercase(), "fake".to_uppercase());

        // On vérifie que c'est un FAKE port en testant la fonction 'will_read' qui n'a
        // de sens que pour les FAKE ports
        let mut buffer: [u8; 512] = [0; 512];
        serial_com.will_read(&[1, 2, 3]);
        assert_eq!(serial_com.read(&mut buffer), 3);
        assert_eq!(buffer[..3], [1, 2, 3]);
    }
}
