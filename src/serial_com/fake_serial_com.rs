//! Port série fictif pour les besoins de test
use crate::CommonSerialComTrait;

/// Port série fictif
#[derive(Default)]
pub struct FakeSerialCom {
    /// Ecriture attendue pour les 'write'
    should_write: Vec<u8>,

    /// Réponse à faire pour un `read`
    will_read: Vec<u8>,
}

impl CommonSerialComTrait for FakeSerialCom {
    /// Fake read
    fn read(&self, buffer: &mut [u8]) -> usize {
        for (dst, src) in buffer.iter_mut().zip(self.will_read.iter()) {
            *dst = *src;
        }
        self.will_read.len()
    }

    /// Fake write
    #[allow(clippy::unused_self)]
    fn write(&self, buffer: &[u8]) {
        if !self.should_write.is_empty() {
            // Si un 'should_write' a été défini, on doit le retrouver ici
            assert_eq!(buffer, self.should_write);
        }
    }

    /// Prédéfini la prochaine écriture du FAKE port
    fn should_write(&mut self, buffer: &[u8]) {
        self.should_write = Vec::new();
        for byte in buffer {
            self.should_write.push(*byte);
        }
    }

    /// Prédéfini la prochaine lecture du FAKE port
    fn will_read(&mut self, buffer: &[u8]) {
        self.will_read = Vec::new();
        for byte in buffer {
            self.will_read.push(*byte);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fake_serial_port_will_read() {
        let mut fake = FakeSerialCom::default();

        // On peut écrire un FAKE port (c'est sans effet sans should_write)
        fake.write(&[1, 2, 3]);

        // Par défaut, on ne lit rien
        let mut buffer = [0; 512];
        assert_eq!(fake.read(&mut buffer), 0);

        // Mais on peut forcer ce qu'on va lire
        fake.will_read(&[1, 2, 3]);
        assert_eq!(fake.read(&mut buffer), 3);
        assert_eq!(buffer[..3], [1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_fake_serial_port_should_write() {
        let mut fake = FakeSerialCom::default();

        // On peut écrire un FAKE port (c'est sans effet sans should_write)
        fake.write(&[1, 2, 3]);

        // Par contre, si on indique le contenu de la prochaine écriture
        fake.should_write(&[1, 2, 3]);

        // panic! si ce n'est pas ce qui est écrit
        fake.write(&[2, 3, 4]);
    }
}
