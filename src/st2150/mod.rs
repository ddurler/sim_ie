//! Protocole ALMA IE selon ST 2150 (voir DOCS)
use std::error::Error;
use std::fmt::Display;

use crate::serial_com::SerialCom;
use crate::CommonSerialComTrait;

mod field;
mod frame;
mod protocol;

/// Erreur détectée
#[derive(Debug, PartialEq, Eq)]
pub enum ProtocolError {
    /// Pas de réponse du calculateur
    NoReply,

    /// Réponse incomplète du calculateur (nb octets reçus, attendus)
    ReplyTooShort(usize, usize),

    /// Longueur incorrecte de message (nb octets message, attendus)
    BadMessageLen(usize, usize),

    /// Checksum incorrect du message (checksum, attendu)
    BadChecksum(u8, u8),

    /// Pas de STX en début de message
    MissingSTX,

    /// Pas de ETX en fin de message
    MissingETX,

    /// Numéro de message incorrect (num, attendu)
    BadMessageNumber(u8, u8),

    /// Séparateur de champ attendu (position)
    SeparatorExpected(usize),
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::NoReply => write!(f, "Pas de réponse du calculateur"),
            ProtocolError::ReplyTooShort(nb, nb_expected) => write!(
                f,
                "Réponse incomplète du calculateur ({nb}/{nb_expected} cars)"
            ),
            ProtocolError::BadMessageLen(nb, nb_expected) => write!(
                f,
                "Longueur incorrecte du message ({nb}/{nb_expected} cars)"
            ),
            ProtocolError::BadChecksum(checksum, checksum_expected) => write!(
                f,
                "Checksum incorrect du message (0x{checksum:02X} vs 0x{checksum_expected:02X} attendu)"
            ),
            ProtocolError::MissingSTX => write!(f, "Pas de 'STX' en début de message"),
            ProtocolError::MissingETX => write!(f, "Pas de 'ETX' en fin de message"),
            ProtocolError::BadMessageNumber(num, num_expected) => write!(
                f,
                "Numéro incorrect du message ({num} vs {num_expected} attendu)"
            ),
            ProtocolError::SeparatorExpected(pos) => write!(
                f,
                "Séparateur de champ attendu en position {pos} dans le message"
            ),
        }
    }
}

impl Error for ProtocolError {}

/// Associe un port série au protocole ST2150 et la trace des dernières vacations
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
        let rep = protocol::waiting_frame(&mut self.port, buffer, expected_len);
        let len_rep = match rep {
            Err(e) => {
                match e {
                    ProtocolError::ReplyTooShort(len_rep, _) => {
                        self.set_last_rep(buffer, len_rep, format!("{e}"));
                    }
                    ProtocolError::NoReply => {
                        self.set_last_rep(&[], 0, format!("{e}"));
                    }
                    // Pas de trace dans les autres cas (qui ne peuvent pas revenir de `waiting_frame`)
                    _ => (),
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

    /// Conversion d'une message reçu
    pub fn try_from_buffer(
        &mut self,
        buffer: &[u8],
        num_message: u8,
        len_fields: &[usize],
    ) -> Result<frame::Frame, ProtocolError> {
        let ret = frame::Frame::try_from_buffer(buffer, num_message, len_fields);
        match ret {
            Ok(frame) => Ok(frame),
            Err(e) => {
                self.last_error = format!("{e}");
                Err(e)
            }
        }
    }

    /// Message 00 de signe de vie
    pub fn message00(&mut self) -> Result<(), ProtocolError> {
        // Création et envoi requête
        let req = frame::Frame::new(0);
        self.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = self.wait_rep(&mut buffer, 17)?;

        // Décodage de la réponse reçue
        let frame = self.try_from_buffer(&buffer[..len_rep], 0, &[1, 1, 1, 1, 1])?;

        dbg!(frame.to_frame());

        Ok(())
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
            b'D',
            b'E',
            protocol::ETX,
        ]);

        let mut st = ST2150::new(fake_port);
        assert!(st.message00().is_ok());
    }
}
