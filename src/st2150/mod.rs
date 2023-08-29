//! Protocole ALMA IE selon ST 2150 (voir DOCS)
use std::error::Error;
use std::fmt::Display;

use crate::serial_com::SerialCom;
use crate::CommonSerialComTrait;

mod field;
mod frame;
mod protocol;

/// Erreur détectée
#[derive(Debug)]
pub enum ProtocolError {
    /// Pas de réponse du calculateur
    NoReply,

    /// Réponse incomplète du calculateur (nb octets reçus, attendus)
    ReplyTooShort(usize, usize),
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::NoReply => write!(f, "Pas de réponse du calculateur"),
            ProtocolError::ReplyTooShort(nb_rec, nb_expected) => write!(
                f,
                "Réponse incomplète du calculateur ({nb_rec}/{nb_expected} cars)"
            ),
        }
    }
}

impl Error for ProtocolError {}

/// Associe un port série au protocole ST2150
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

    /// Helper pour renseigner la trace de ce qu'on a reçu
    fn set_last_rep(&mut self, buffer: &[u8], len_buffer: usize, error: String) {
        self.last_rep = Vec::with_capacity(len_buffer);
        for v in &buffer[0..len_buffer] {
            self.last_rep.push(*v);
        }
        self.last_error = error;
    }

    /// Attente d'un message (réponse)
    fn wait_rep(&mut self, buffer: &mut [u8], expected_len: usize) -> Result<usize, ProtocolError> {
        self.last_rep = vec![];
        self.last_error = "TODO Erreur".to_string();
        let rep = protocol::waiting_frame(&self.port, buffer, expected_len);
        let len_rep = match rep {
            Err(e) => {
                match e {
                    ProtocolError::ReplyTooShort(len_rep, _) => {
                        self.set_last_rep(buffer, len_rep, format!("{e}"));
                    }
                    ProtocolError::NoReply => {
                        self.set_last_rep(&[], 0, format!("{e}"));
                    }
                }
                return Err(e);
            }
            Ok(n) => {
                self.set_last_rep(buffer, n, String::new());
                n // Nombre de caractères reçus
            }
        };

        Ok(len_rep)
    }

    /// Message 00 de signe de vie
    pub fn message00(&mut self) {
        // Création et envoi requête
        let req = frame::Frame::new(0);
        self.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let Ok(len_rep) = self.wait_rep(&mut buffer, 17) else {
            return;
        };

        // TODO : Décodage de la répons reçue
        dbg!(&buffer[..len_rep]);
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

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            0x30,
            0x30,
            protocol::SEPARATOR,
            0x30, // Hors mesurage
            protocol::SEPARATOR,
            0x20, // Pas de défaut
            protocol::SEPARATOR,
            0x30, // Pas en arrêt intermédiaire
            protocol::SEPARATOR,
            0x30, // Pas de forçage PD
            protocol::SEPARATOR,
            0x30, // En mode autonome
            protocol::SEPARATOR,
            0x00,
            0x00,
            protocol::ETX,
        ]);

        let mut st = ST2150::new(fake_port);
        st.message00();
    }
}
