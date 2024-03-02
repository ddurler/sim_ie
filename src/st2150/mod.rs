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

/// Énumération des éditions de la spécification ST 2150
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Edition2150 {
    /// Révision A de la ST2150 (historique)
    A,

    /// Révision B de la ST2150 (2021)
    B,

    /// Révision C de la ST2150 (2023 - Mode étendu / Préliminaire non production)
    C,

    /// Révision D de la ST2150 (2024 - Mode étendu / change message 75, 77 et 78)
    D,

    /// Révision E de la ST2150 (2024 / Ajout message 38)
    E,
}

impl Display for Edition2150 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Edition2150::A => write!(f, "A"),
            Edition2150::B => write!(f, "B"),
            Edition2150::C => write!(f, "C"),
            Edition2150::D => write!(f, "D"),
            Edition2150::E => write!(f, "E"),
        }
    }
}

/// Erreur détectée
#[derive(Debug, PartialEq, Eq)]
pub enum ProtocolError {
    /// Pas de réponse du calculateur
    NoReply,

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

    /// Impossible d'encoder le nombre dans le format demandé
    IllegalNumberEncoding(String),

    /// Caractère incorrect dans un champ lors du décodage (type_de_champ, champ, caractère)
    IllegalFieldCharDecode(String, Field, u8),

    /// Échec conversion d'un champ dans un type (type_de_champ, champ),
    ErrFieldConversion(String, Field),

    /// Valeur incorrecte dans un champ de la réponse (champ, nom, domaine_valeurs)
    IllegalRepFieldValue(Field, String, String),

    /// Information manquante dans le contexte (nom_de_l_info)
    ContextMissing(String),
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::NoReply => write!(f, "Pas de réponse du calculateur"),
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
            ProtocolError::IllegalNumberEncoding(txt) => write!(
                f,
                "Encodage trame impossible : {txt}"
            ),
            ProtocolError::IllegalFieldCharDecode(str_decode, field, car) => write!(
                f,
                "Contenu '0x{car:02X}' incorrect pour décodage en {str_decode} du champ {field:?}"
            ),
            ProtocolError::ErrFieldConversion(str_decode, field) => write!(
                f,
                "Erreur lors de la conversion en {str_decode} du champ {field:?}"
            ),
            ProtocolError::IllegalRepFieldValue(field, nom, domaine_valeurs) => write!(
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
    fn set_last_rep(&mut self, buffer: &[u8], len_buffer: usize) {
        self.last_rep = Vec::with_capacity(len_buffer);
        for v in &buffer[0..len_buffer] {
            self.last_rep.push(*v);
        }
    }

    /// Attente d'un message (réponse)
    fn wait_rep(
        &mut self,
        buffer: &mut [u8],
        len_fields: &[usize],
    ) -> Result<usize, ProtocolError> {
        let max_expected_len = Frame::len_expected_response(len_fields);
        self.last_rep = vec![];
        let len_rep = protocol::waiting_frame(&mut self.port, buffer, max_expected_len);
        self.set_last_rep(buffer, len_rep);
        if len_rep == 0 {
            return Err(ProtocolError::NoReply);
        }

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
            Ok(frame) => {
                self.last_error = String::new();
                Ok(frame)
            }
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
        self.last_req = vec![];
        self.last_rep = vec![];
        self.last_error = String::new();
        match messages::get_dyn_message(message_num).do_vacation(self, context) {
            Ok(()) => {
                self.last_error = String::new();
                Ok(())
            }
            Err(e) => {
                self.last_error = format!("{e}");
                Err(e)
            }
        }
    }
}
