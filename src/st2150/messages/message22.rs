//! Message 22 : Identification TAG

use crate::context::Context;

use super::field::Field;
use super::frame;
use super::CommonMessageTrait;
use super::ProtocolError;
use super::ST2150;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 22;

/// Message 22 : Identification TAG
#[derive(Default)]
pub struct Message22 {}

impl CommonMessageTrait for Message22 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn str_message(&self) -> &'static str {
        "Identification TAG"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![IdInfo::IdentificationTag]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![IdInfo::Nack, IdInfo::Ack]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Message22::availability(self, context)?;

        // Création et envoi requête
        let mut req = frame::Frame::new(MESSAGE_NUM);

        let identification_tag = match context.get_info_string(IdInfo::IdentificationTag) {
            None => String::new(),
            Some(txt) => txt.trim().to_string(),
        };

        // #0 : Longueur de l'identification tag sur 3
        req.add_field(Field::encode_number(identification_tag.len(), 3));

        // #1 : Identification tag
        if identification_tag.is_empty() {
            req.add_field(Field::new(&[]));
        } else {
            req.add_field(Field::encode_str(
                &identification_tag,
                identification_tag.len(),
            ));
        }

        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = st2150.wait_rep(&mut buffer, &[9])?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(&buffer[..len_rep], MESSAGE_NUM, &[1])?;

        // Mise à jour du contexte

        // #0 : ACK ou NACK
        context.set_info_bool(IdInfo::Nack, frame.is_nack());
        context.set_info_bool(IdInfo::Ack, frame.is_ack());

        // C'est tout bon
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::st2150::protocol;
    use crate::CommonSerialComTrait;
    use crate::SerialCom;

    #[test]
    fn test_message22() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        context.set_info_string(IdInfo::IdentificationTag, "ABCDE");

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'2', //  Numéro de message
            b'2',
            protocol::SEPARATOR,
            b'0', // Longueur du TAG
            b'0',
            b'5',
            protocol::SEPARATOR,
            b'A',
            b'B',
            b'C',
            b'D',
            b'E',
            protocol::SEPARATOR,
            56, // Checksum
            65,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'2', // Numéro de message
            b'2',
            protocol::SEPARATOR,
            protocol::ACK, // ACK
            protocol::SEPARATOR,
            b'0', // Checksum
            b'6',
            protocol::ETX,
        ]);

        // Création du protocole ST2150 avec ce FAKE port
        let mut st = ST2150::new(fake_port);

        // Le message est possible
        assert!(ST2150::message_availability(&context, MESSAGE_NUM).is_ok());

        // Vacation requête/réponse du message via le FAKE port
        assert_eq!(st.do_message_vacation(&mut context, MESSAGE_NUM), Ok(()));

        // Vérification de ce qui a été mis à jour dans le contexte
        // Vérification de ce qui a été mis à jour dans le contexte
        assert_eq!(context.get_info_bool(IdInfo::Nack), Some(false));
        assert_eq!(context.get_info_bool(IdInfo::Ack), Some(true));
    }
}