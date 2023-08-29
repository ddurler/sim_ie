//! Helper pour l'encodage/décodage des trames du protocole ALMA IE ST2150

use crate::st2150::field::Field;
use crate::st2150::protocol;

/// Support générique pour un message du protocole
pub struct Frame {
    /// Numéro de message
    pub num_message: u8,

    /// Champ de la requête
    pub fields: Vec<Field>,
}

impl Frame {
    /// Constructeur
    pub fn new(numero: u8) -> Self {
        Self {
            num_message: numero,
            fields: vec![],
        }
    }

    /// Est-ce un message ACK ?
    pub fn is_ack(&self) -> bool {
        !self.fields.is_empty() && self.fields[0].to_frame() == vec![protocol::ACK]
    }

    /// Est-ce un message NACK ?
    pub fn is_nack(&self) -> bool {
        !self.fields.is_empty() && self.fields[0].to_frame() == vec![protocol::NACK]
    }

    /// Ajout d'un champ dans le message
    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    /// Création de la trame pour le message
    pub fn to_frame(&self) -> Vec<u8> {
        let mut req = vec![];

        // STX au début
        req.push(protocol::STX);

        // Numéro de message sur 2 octets
        let command_field = Field::encode_number(self.num_message, 2);
        req.extend(command_field.to_frame());

        // Tous le champs du message précédés d'un SEPARATOR
        for field in &self.fields {
            req.push(protocol::SEPARATOR);
            req.extend(field.to_frame());
        }

        // checksum précédé d'un SEPARATOR
        req.push(protocol::SEPARATOR);
        // Le checksum est calculé sur l'ensemble de la trame sans le STX initial mais
        // avec le SEPARATOR avant le checksum
        let checksum = protocol::calcul_checksum(&req[1..]);
        let checksum_field = Field::encode_hexa(checksum, 2);
        req.extend(checksum_field.to_frame());

        // ETX final
        req.push(protocol::ETX);

        // et voilà :)
        req
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction_message() {
        // Message 12...
        let mut req = Frame::new(12);

        // Avec un champ ABCD...
        req.add_field(Field::encode_str("ABCD", 4));

        let trame = req.to_frame();

        // Le checksum de la trame
        let checksum = protocol::calcul_checksum(&[
            0x31,
            0x32,
            protocol::SEPARATOR,
            0x41,
            0x42,
            0x43,
            0x44,
            protocol::SEPARATOR,
        ]);
        assert_eq!(checksum, 0x07); // Ici la valeur du checksum de cette trame

        // Donc la trame doit être
        assert_eq!(
            trame,
            vec![
                protocol::STX,
                0x31,
                0x32,
                protocol::SEPARATOR,
                0x41,
                0x42,
                0x43,
                0x44,
                protocol::SEPARATOR,
                0x30,
                0x37,
                protocol::ETX
            ]
        );
    }

    #[test]
    fn test_message_is_ack() {
        // Message 12...
        let mut req = Frame::new(12);

        // Avec un champ binaire ACK...
        req.add_field(Field::encode_binary(protocol::ACK));

        assert!(req.is_ack());
    }

    #[test]
    fn test_message_is_nack() {
        // Message 12...
        let mut req = Frame::new(12);

        // Avec un champ binaire NACK...
        req.add_field(Field::encode_binary(protocol::NACK));

        // Et un message d'erreur
        req.add_field(Field::encode_str("ERREUR", 6));

        assert!(req.is_nack());
    }
}
