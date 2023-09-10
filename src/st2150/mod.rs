//! Protocole ALMA IE selon ST 2150 (voir DOCS)
use std::error::Error;
use std::fmt::Display;

use crate::context;

use crate::serial_com::SerialCom;
use crate::CommonSerialComTrait;
use context::{Context, IdInfo};

pub mod field;
pub mod frame;
pub mod messages;
pub mod protocol;

use field::Field;
use frame::Frame;

/// Erreur détectée
#[derive(Debug, PartialEq, Eq)]
pub enum ProtocolError {
    /// Pas de réponse du calculateur
    NoReply,

    /// Longueur inattendue de la trame reçue (nb octets lus)
    BadFrameLen(usize, Vec<usize>),

    /// Longueur incorrecte de message (nb octets message, attendus)
    BadMessageLen(usize, usize),

    /// Checksum incorrect du message (checksum, attendu)
    BadChecksum(u8, u8),

    /// Pas de STX en début de message
    MissingSTX,

    /// Pas de ETX en fin de message
    MissingETX,

    /// Réponse avec un message d'erreur 50
    ErrorMessage50(String),

    /// Numéro de message incorrect (num, attendu)
    BadMessageNumber(u8, u8),

    /// Séparateur de champ attendu (position)
    SeparatorExpected(usize),

    /// Caractère incorrect dans un champ lors du décodage (type_de_champ, champ, caractère)
    IllegalFieldCharDecode(String, Field, u8),

    /// Échec conversion d'un champ dans un type (type_de_champ, champ),
    ErrFieldConversion(String, Field),

    /// Valeur incorrecte dans un champ (champ, nom, domaine_valeurs)
    IllegalFieldValue(Field, String, String),

    /// Information manquante dans le contexte (nom_de_l_info)
    ContextMissing(String),
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::NoReply => write!(f, "Pas de réponse du calculateur"),
            ProtocolError::BadFrameLen(nb, expected_lens) => write!(
                f,
                "Longueur inattendue de la trame ({nb} / {expected_lens:?} cars)"
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
            ProtocolError::ErrorMessage50(txt) => write!(f, "Réponse avec un message 50 d'erreur : {txt}"),
            ProtocolError::BadMessageNumber(num, num_expected) => write!(
                f,
                "Numéro incorrect du message ({num} vs {num_expected} attendu)"
            ),
            ProtocolError::SeparatorExpected(pos) => write!(
                f,
                "Séparateur de champ attendu en position {pos} dans le message"
            ),
            ProtocolError::IllegalFieldCharDecode(str_decode, field, car) => write!(
                f,
                "Contenu '0x{car:02X}' incorrect pour décodage en {str_decode} du champ {field:?}"
            ),
            ProtocolError::ErrFieldConversion(str_decode, field) => write!(
                f,
                "Erreur lors de la conversion en {str_decode} du champ {field:?}"
            ),
            ProtocolError::IllegalFieldValue(field, nom, domaine_valeurs) => write!(
                f,
                "Valeur incorrecte du champ '{nom}'={field:?} : {domaine_valeurs}"
            ),
            ProtocolError::ContextMissing(nom) => write!(
                f,
                "Valeur non renseignée du champ '{nom}'"
            ),
        }
    }
}

impl Error for ProtocolError {}

/// Associe un port série pour le protocole ALMA IE ST2150  et la trace des dernières vacations
#[derive(Default)]
pub struct ST2150 {
    /// Port série de communication
    pub port: SerialCom,

    /// Dernière requête envoyée
    pub last_req: Vec<u8>,

    /// Dernière réponse reçue
    pub last_rep: Vec<u8>,

    /// Libellé de la dernière erreur relevée
    pub last_error: String,
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
    fn send_req(&mut self, req: &Frame) {
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
    fn wait_rep(
        &mut self,
        buffer: &mut [u8],
        expected_lens: &[usize],
    ) -> Result<usize, ProtocolError> {
        self.last_rep = vec![];
        let max_expected_len = *expected_lens.iter().max().unwrap_or(&0);
        let len_rep = protocol::waiting_frame(&mut self.port, buffer, max_expected_len);
        if len_rep == 0 {
            let e = ProtocolError::NoReply;
            self.set_last_rep(buffer, len_rep, format!("{e}"));
            return Err(e);
        }
        if !expected_lens.contains(&len_rep) {
            let e = ProtocolError::BadFrameLen(len_rep, expected_lens.to_vec());
            self.set_last_rep(buffer, len_rep, format!("{e}"));
            return Err(e);
        }

        self.set_last_rep(buffer, len_rep, String::new());
        Ok(len_rep)
    }

    /// Décodage et conversion d'un message reçu en une `Frame`
    pub fn try_from_buffer(
        &mut self,
        buffer: &[u8],
        num_message: u8,
        len_fields: &[usize],
    ) -> Result<Frame, ProtocolError> {
        let ret = Frame::try_from_buffer(buffer, num_message, len_fields);
        match ret {
            Ok(frame) => Ok(frame),
            Err(e) => {
                self.last_error = format!("{e}");
                Err(e)
            }
        }
    }

    /// Message disponible (toutes les informations nécessaires disponibles dans le contexte) ?
    pub fn message_availability(context: &Context, message_num: u8) -> Result<(), ProtocolError> {
        messages::get_dyn_message(message_num).availability(context)
    }

    /// Vacation (requête/réponse) d'un message
    pub fn do_message_vacation(
        &mut self,
        context: &mut Context,
        message_num: u8,
    ) -> Result<(), ProtocolError> {
        messages::get_dyn_message(message_num).do_vacation(self, context)
    }
}
