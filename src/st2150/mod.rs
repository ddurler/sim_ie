/// Protocole ALMA IE selon ST 2150 (voir DOCS)
use std::error::Error;
use std::fmt::Display;

use crate::serial_com::SerialCom;
use crate::CommonSerialComTrait;

mod field;
mod frame;
mod protocol;

/// Erreur détectée par le module
#[derive(Debug, Default)]
struct ModError {}

impl Display for ModError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Erreur")
    }
}

impl Error for ModError {}

pub struct ST2150 {
    /// Port série de communication
    port: SerialCom,

    /// Dernière requête envoyée
    last_req: Vec<u8>,

    /// Dernière réponse reçue
    last_rep: Vec<u8>,

    /// Libellé de la dernière erreur relevée
    last_error: String,
}

impl ST2150 {
    /// Constructeur
    pub fn new(port: SerialCom) -> Self {
        Self {
            port,
            last_req: vec![],
            last_rep: vec![],
            last_error: String::new(),
        }
    }

    /// Envoi d'un message (requête)
    fn send_req(&mut self, req: &frame::Frame) {
        self.last_req = req.to_frame();
        self.last_rep = vec![];
        self.last_error = String::new();

        self.port.write(&req.to_frame());
    }

    /// Attente d'un message (réponse)
    fn wait_rep(&mut self) -> Result<frame::Frame, Box<dyn Error>> {
        self.last_rep = vec![];
        self.last_error = "TODO Erreur".to_string();
        Err(Box::<ModError>::default())
    }

    /// Message de signe de vie
    pub fn message00(&mut self) {
        let requete = frame::Frame::new(0);
        self.send_req(&requete);
        if let Ok(rep) = self.wait_rep() {
            dbg!(rep.to_frame());
        }
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
