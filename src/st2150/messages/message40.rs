//! Message 40 : Synchronisation heure

use crate::context::Context;
use crate::st2150::field::Field;

use super::frame;
use super::CommonMessageTrait;
use super::Edition2150;
use super::ProtocolError;
use super::ST2150;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 40;

/// Message 40 : Synchronisation heure
#[derive(Default)]
pub struct Message40 {}

impl CommonMessageTrait for Message40 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn edition_st2150(&self) -> Edition2150 {
        Edition2150::A
    }

    fn message_str(&self) -> &'static str {
        "Synchronisation heure"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![IdInfo::HeureHHMM]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![IdInfo::Ack, IdInfo::Nack]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Self::availability(self, context)?;

        // Création et envoi requête
        let mut req = frame::Frame::new(MESSAGE_NUM);

        // #0 - Heure (HHMM)
        let heure = context.get_option_info_u16(IdInfo::HeureHHMM).unwrap();
        req.add_field(Field::encode_number(heure, 4)?);

        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let lens_expected = &[1];
        let len_rep = st2150.wait_rep(&mut buffer, lens_expected)?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(&buffer[..len_rep], MESSAGE_NUM, lens_expected)?;

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
    fn test_message40() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        // On met code produit i & quantité 1000 * i dans le compartiment #i
        context.set_info_u16(IdInfo::HeureHHMM, 1234);

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'4', //  Numéro de message
            b'0',
            protocol::SEPARATOR,
            b'1', // Heure
            b'2',
            b'3',
            b'4',
            protocol::SEPARATOR,
            48, // Checksum
            48,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'4', // Numéro de message
            b'0',
            protocol::SEPARATOR,
            protocol::ACK, // ACK
            protocol::SEPARATOR,
            b'0', // Checksum
            b'2',
            protocol::ETX,
        ]);

        // Création du protocole ST2150 avec ce FAKE port
        let mut st = ST2150::new(fake_port);

        // Le message est possible
        assert!(ST2150::message_availability(&context, MESSAGE_NUM).is_ok());

        // Vacation requête/réponse du message via le FAKE port
        assert_eq!(st.do_message_vacation(&mut context, MESSAGE_NUM), Ok(()));

        // Vérification de ce qui a été mis à jour dans le contexte
        assert_eq!(context.get_option_info_bool(IdInfo::Nack), Some(false));
        assert_eq!(context.get_option_info_bool(IdInfo::Ack), Some(true));
    }
}
