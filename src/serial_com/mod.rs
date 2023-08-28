/// Gestion d'un port série réel ou virtuel
///
/// Un port nommé 'FAKE' a un comportement spécifique pour les besoins de tests. Voir `FakeSerialPort`
/// Sinon, il s'agit d'un port réel de la machine qu'on cherche à gérer
mod fake_serial_com;
mod true_serial_com;

/// Retourne la liste des noms des ports séries disponibles sur cette machine
pub fn available_names_list() -> Vec<String> {
    true_serial_com::available_names_list()
}

/// Distingue un 'true' port d'un FAKE port
pub enum SerialCom {
    FakePort(fake_serial_com::FakeSerialCom),
    TruePort(true_serial_com::TrueSerialCom),
}

/// Trait à implémenter pour les `SerialCom` (true ou FAKE)
pub trait CommonSerialComTrait {
    /// Lecture du port
    fn read(&self, buffer: &mut [u8]) -> usize;

    /// Écriture du port
    fn write(&self, buffer: &[u8]);

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
            SerialCom::FakePort(fake_serial_com::FakeSerialCom::default())
        } else {
            SerialCom::TruePort(true_serial_com::TrueSerialCom::new(name, baud_rate))
        }
    }
}

impl CommonSerialComTrait for SerialCom {
    /// Lecture du port série
    /// `buffer` : Vec<u8> qu'on peut initialiser par `let mut buffer = [0; 512]`
    /// Return : Nombre d'octets lus
    /// # panics
    /// panic! si erreur de lecture du port réel de la machine
    fn read(&self, buffer: &mut [u8]) -> usize {
        match &self {
            SerialCom::FakePort(fake) => fake.read(buffer),
            SerialCom::TruePort(port) => port.read(buffer),
        }
    }

    /// Écriture du port série
    /// `buffer` : Vec<u8> à écriture
    /// # panics
    /// panics si erreur d'écriture d'un port réel de la machine
    fn write(&self, buffer: &[u8]) {
        match &self {
            SerialCom::FakePort(fake) => fake.write(buffer),
            SerialCom::TruePort(port) => port.write(buffer),
        }
    }

    /// Primitive pour les FAKE ports uniquement
    /// Sans effet si le port n'est pas un FAKE port
    fn should_write(&mut self, buffer: &[u8]) {
        match self {
            SerialCom::FakePort(fake) => fake.should_write(buffer),
            SerialCom::TruePort(port) => {
                eprint!(
                    "Usage inattendu de 'should_write' avec un port existant ({})",
                    port.name
                );
            }
        }
    }

    /// Primitive pour les FAKE ports uniquement
    /// Sans effet si le port n'est pas un FAKE port
    fn will_read(&mut self, buffer: &[u8]) {
        match self {
            SerialCom::FakePort(fake) => fake.will_read(buffer),
            SerialCom::TruePort(port) => {
                eprint!(
                    "Usage inattendu de 'will_read' avec un port existant ({})",
                    port.name
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_com_new() {
        // Création d'un FAKE port si le nom est "FAKE" ou "fake"
        let serial_com = SerialCom::new("fake", 9600);
        if let SerialCom::FakePort(_) = serial_com {
        } else {
            panic!("Expected FakeSerialCom !!!");
        }
    }
}
