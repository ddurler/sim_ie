/// Protocole ALMA IE selon ST 2150 (voir DOCS)
use crate::serial_com::SerialCom;
use crate::CommonSerialComTrait;

mod field;
mod protocol;
mod requete;

pub struct ST2150 {
    /// Port série de communication
    port: SerialCom,
}

impl ST2150 {
    /// Constructeur
    pub fn new(port: SerialCom) -> Self {
        Self { port }
    }

    /// Message de signe de vie
    pub fn message00(&mut self) {
        let requete = requete::Requete::new(0);
        self.port.write(&requete.to_frame());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requete00() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Trame pour message 00 (le checksum est 0xFE)
        fake_port.should_write(&[
            protocol::STX,
            0x30,
            0x30,
            protocol::SEPARATOR,
            0x46,
            0x45,
            protocol::ETX,
        ]);

        let mut st = ST2150::new(fake_port);
        st.message00();
    }
}
