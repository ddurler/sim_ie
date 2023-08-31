//! Helper pour l'encodage/décodage des trames du protocole ALMA IE ST2150

use crate::st2150::field::Field;
use crate::st2150::protocol;

use super::field;
use super::ProtocolError;

/// Support générique pour un message du protocole
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Frame {
    /// Numéro de message
    pub num_message: u8,

    /// Champ de la requête
    pub fields: Vec<Field>,
}

/// Helper pour convertir un caractère hexadécimal en binaire décimal
fn car_hexa_to_value(car: u8) -> u8 {
    match car {
        b'0'..=b'9' => car - b'0',
        b'A'..=b'F' => car - b'A' + 10,
        b'a'..=b'f' => car - b'a' + 10,
        _ => 0,
    }
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

    /// Décodage et validation d'une trame d'un message
    /// `buffer` : Message à décoder
    /// `num_message` : Numéro de message attendu
    /// `len_fields` : Nombre et taille (en octet) des différents champs attendus dans le message
    ///
    /// Rappel : Un message ST2150 est : STX + num(2) + { SEPARATOR + champ(n) }* + SEPARATOR + checksum(2) + ETX
    ///
    pub fn try_from_buffer(
        buffer: &[u8],
        num_message: u8,
        len_fields: &[usize],
    ) -> Result<Self, ProtocolError> {
        // Commence par STX ?
        if buffer[0] != protocol::STX {
            return Err(ProtocolError::MissingSTX);
        }

        // Termine par ETX ?
        if buffer[buffer.len() - 1] != protocol::ETX {
            return Err(ProtocolError::MissingETX);
        }

        // Longueur du message OK ?
        let mut rec_len = 1 + 2; // STX + numéro de message
        rec_len += len_fields.len() + len_fields.iter().sum::<usize>(); // SEPARATOR avant chaque champs + champs
        rec_len += 1 + 2 + 1; // SEPARATOR + checksum + ETX
        if buffer.len() != rec_len {
            return Err(ProtocolError::BadMessageLen(buffer.len(), rec_len));
        }

        // checksum OK ?
        let rec_checksum =
            car_hexa_to_value(buffer[rec_len - 3]) * 16 + car_hexa_to_value(buffer[rec_len - 2]);
        let checksum = protocol::calcul_checksum(&buffer[1..rec_len - 3]);
        if checksum != rec_checksum {
            return Err(ProtocolError::BadChecksum(rec_checksum, checksum));
        }

        // Numéro de message OK ?
        let rec_num_message = (buffer[1] - b'0') * 10 + (buffer[2] - b'0');
        if rec_num_message != num_message {
            return Err(ProtocolError::BadMessageNumber(
                rec_num_message,
                num_message,
            ));
        }

        // On est plutôt bien parti, reste à valider les champs..
        let mut frame = Self::new(num_message);

        let mut cur_position = 1 + 2; /* Après le STX num(2) */
        for len_field in len_fields {
            // On doit trouver un SEPARATOR avant le champ
            if buffer[cur_position] != protocol::SEPARATOR {
                return Err(ProtocolError::SeparatorExpected(cur_position));
            }
            cur_position += 1;
            // Les len_field caractères qui suivent sont un champ
            let field = field::Field::new(&buffer[cur_position..cur_position + len_field]);
            frame.add_field(field);
            cur_position += len_field;
        }

        // On doit trouver encore un SEPARATOR avant le checksum
        if buffer[cur_position] != protocol::SEPARATOR {
            return Err(ProtocolError::SeparatorExpected(cur_position));
        }

        // On est tout bon :)
        Ok(frame)
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

    #[test]
    fn test_try_from_buffer_is_ok() {
        // On utilise ici la possibilité de créer une trame et son message et
        // on décode le message pour vérifier qu'on retrouve bien la trame créée

        // Trame num = 12 sans champ
        let frame = Frame::new(12);
        let ret = Frame::try_from_buffer(&frame.to_frame(), 12, &[]);
        assert_eq!(ret.unwrap(), frame);

        // Trame num = 12 avec un champ texte de 5 cars
        let mut frame = Frame::new(12);
        frame.add_field(Field::encode_str("VALUE", 5));
        let ret = Frame::try_from_buffer(&frame.to_frame(), 12, &[5]);
        assert_eq!(ret.unwrap(), frame);

        // Trame num = 12 avec 2 champ texte de 1 car
        let mut frame = Frame::new(12);
        frame.add_field(Field::encode_str("0", 1));
        frame.add_field(Field::encode_str("1", 1));
        let ret = Frame::try_from_buffer(&frame.to_frame(), 12, &[1, 1]);
        assert_eq!(ret.unwrap(), frame);

        // Trame num = 12 avec 2 champs de longueur [5, 10]
        let mut frame = Frame::new(12);
        frame.add_field(Field::encode_number(12345, 5));
        frame.add_field(Field::encode_str("VALUE", 10));
        let ret = Frame::try_from_buffer(&frame.to_frame(), 12, &[5, 10]);
        assert_eq!(ret.unwrap(), frame);

        // Trame num = 12 avec 3 champs de longueur [10, 0, 3]
        let mut frame = Frame::new(12);
        frame.add_field(Field::encode_str("VALUE", 10));
        frame.add_field(Field::new(&[]));
        frame.add_field(Field::encode_number(123, 3));
        let ret = Frame::try_from_buffer(&frame.to_frame(), 12, &[10, 0, 3]);
        assert_eq!(ret.unwrap(), frame);
    }

    #[test]
    fn test_try_from_buffer_is_err() {
        /* buffer / num message / len_fields / ProtocolError */
        let err_tests: Vec<(&[u8], u8, &[usize], ProtocolError)> = vec![
            // Message sans STX au début
            (&[0x01], 0, &[], ProtocolError::MissingSTX),
            // Message dans ETX à la fin
            (&[protocol::STX, 0x01], 0, &[], ProtocolError::MissingETX),
            // Message avec une longueur inattendue (selon la longueur attendue des champs (ici aucun))
            (
                &[protocol::STX, 0x01, protocol::ETX],
                0,
                &[],
                ProtocolError::BadMessageLen(3, 7),
            ),
            // Message avec un mauvais checksum
            (
                &[
                    protocol::STX,
                    b'0',
                    b'0',
                    protocol::SEPARATOR,
                    0x00, /* Checksum de 00, FE attendu */
                    0x00,
                    protocol::ETX,
                ],
                0,
                &[],
                ProtocolError::BadChecksum(0, 0xFE),
            ),
            // Message avec un mauvais numéro (autre que celui attendu)
            (
                &[
                    protocol::STX,
                    b'0', /* Message 00... */
                    b'0',
                    protocol::SEPARATOR,
                    b'F',
                    b'E',
                    protocol::ETX,
                ],
                12, /* ... alors que message 12 attendu */
                &[],
                ProtocolError::BadMessageNumber(0, 12),
            ),
            // Message avec un séparateur au mauvais endroit (au moins 2 champs)
            (
                &[
                    protocol::STX,
                    b'0',
                    b'0',
                    protocol::SEPARATOR,
                    b'1', /* Champ #1 d'une longueur de 1  */
                    protocol::SEPARATOR,
                    b'2', /* Champ #2 d'une longueur de 2 */
                    b'2',
                    protocol::SEPARATOR,
                    b'C',
                    b'F',
                    protocol::ETX,
                ],
                0,
                &[2, 1], /* Inversion des longueurs des champs #1/#2 ici */
                ProtocolError::SeparatorExpected(6),
            ),
        ];

        for (buffer, num_message, len_fields, expected) in err_tests {
            let res = Frame::try_from_buffer(buffer, num_message, len_fields);
            assert!(res.is_err());
            assert_eq!(expected, res.err().unwrap());
        }
    }
}
