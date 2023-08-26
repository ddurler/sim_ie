/// Port série fictif pour les besoins de test

#[derive(Debug, Default)]
pub struct FakeSerialCom {
    // Réponse à faire pour un `read`
    will_read: Vec<u8>,
}

impl FakeSerialCom {
    // Fake read
    pub fn read(&self, buffer: &mut [u8]) -> usize {
        for (dst, src) in buffer.iter_mut().zip(self.will_read.iter()) {
            *dst = *src;
        }
        self.will_read.len()
    }

    // Fake write
    pub fn write(&self, _buffer: &[u8]) {}

    pub fn will_read(&mut self, buffer: &[u8]) {
        self.will_read = Vec::new();
        for byte in buffer.iter() {
            self.will_read.push(*byte)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fake_serial_port() {
        let mut fake = FakeSerialCom::default();

        // On peut écrire un FAKE port (c'est sans effet)
        fake.write(&[1, 2, 3]);

        // Par défaut, on ne lit rien
        let mut buffer = [0; 512];
        assert_eq!(fake.read(&mut buffer), 0);

        // Mais on peut forcer ce qu'on va lire
        fake.will_read(&[1, 2, 3]);
        assert_eq!(fake.read(&mut buffer), 3);
        assert_eq!(buffer[..3], [1, 2, 3])
    }
}
