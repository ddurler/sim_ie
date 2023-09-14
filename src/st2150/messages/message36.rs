//! Message 36 : Relevé d'un événement

use crate::context::Context;
use crate::st2150::field::Field;

use super::frame;
use super::CommonMessageTrait;
use super::Edition2150;
use super::ProtocolError;
use super::ST2150;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 36;

/// Message 36 : Relevé d'un événement
#[derive(Default)]
pub struct Message36 {}

impl CommonMessageTrait for Message36 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn edition_st2150(&self) -> Edition2150 {
        Edition2150::C
    }

    fn message_str(&self) -> &'static str {
        "Relevé d'un événement"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![IdInfo::DateAAMMJJ, IdInfo::IndexJournalier]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![
            IdInfo::NbJEvents,
            IdInfo::HeureHHMMSS,
            IdInfo::DataJEvent,
            IdInfo::LibelleJEvent,
        ]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Self::availability(self, context)?;

        // Création et envoi requête
        let mut req = frame::Frame::new(MESSAGE_NUM);

        // #0 - Date
        let data = context.get_info_u32(IdInfo::DateAAMMJJ).unwrap();
        req.add_field(Field::encode_number(data, 6)?);

        // #1 - Numéro d'ordre dans le jour
        let index_journalier = context.get_info_u16(IdInfo::IndexJournalier).unwrap();
        req.add_field(Field::encode_number(index_journalier, 3)?);

        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = st2150.wait_rep(&mut buffer, 72)?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(&buffer[..len_rep], MESSAGE_NUM, &[3, 6, 12, 40])?;

        // Mise à jour du contexte

        // #0 - Nombre d'événements pour la journée demandée
        context.set_info_u16(IdInfo::NbJEvents, frame.fields[0].decode_number::<u16>()?);

        // #1 - Heure
        context.set_info_u32(IdInfo::HeureHHMMSS, frame.fields[1].decode_number::<u32>()?);

        // #2 - Data techniques de l'événement
        context.set_info_string(IdInfo::DataJEvent, &frame.fields[2].decode_str()?);

        // #3 - Libellé de l'événement
        context.set_info_string(IdInfo::LibelleJEvent, &frame.fields[3].decode_str()?);

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

    #[rustfmt::skip]  // On demande à 'cargo fmt' de ne pas arranger le code parce que sinon ça dépasse 100 lignes :)
    #[test]
    fn test_message36() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        context.set_info_u32(IdInfo::DateAAMMJJ, 12_03_04);
        context.set_info_u16(IdInfo::IndexJournalier, 1);

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'3', //  Numéro de message
            b'6',
            protocol::SEPARATOR,
            b'1', // Date AA MM JJ
            b'2',
            b'0',
            b'3',
            b'0',
            b'4',
            protocol::SEPARATOR,
            b'0', // Numéro d'ordre dans la journée
            b'0',
            b'1',
            protocol::SEPARATOR,
            67, // Checksum
            69,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'3', // Numéro de message
            b'6',
            protocol::SEPARATOR,
            b'0', // Nombre d'événements dans cette journée
            b'1',
            b'2',
            protocol::SEPARATOR,
            b'1', // Heure de l'événement (HHMMSS)
            b'2',
            b'3',
            b'4',
            b'5',
            b'6',
            protocol::SEPARATOR,
            b'A',  b'A', b'B', b'B', b'C', b'C', b'D', b'D', b'E', b'E', b'F', b'F', // Données techniques de l'événement (12 cars)
            protocol::SEPARATOR,
            b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', // Libellé événement (40 cars)
            b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
            b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
            b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
            protocol::SEPARATOR,
            b'C', // Checksum
            b'F',
            protocol::ETX,
        ]);

        // Création du protocole ST2150 avec ce FAKE port
        let mut st = ST2150::new(fake_port);

        // Le message est possible
        assert!(ST2150::message_availability(&context, MESSAGE_NUM).is_ok());

        // Vacation requête/réponse du message via le FAKE port
        assert_eq!(st.do_message_vacation(&mut context, MESSAGE_NUM), Ok(()));

        // Vérification de ce qui a été mis à jour dans le contexte
        assert_eq!(context.get_info_u16(IdInfo::NbJEvents), Some(12));
        assert_eq!(context.get_info_u32(IdInfo::HeureHHMMSS), Some(12_34_56));
        assert_eq!(
            context.get_info_string(IdInfo::DataJEvent),
            Some("AABBCCDDEEFF".to_string())
        );
        assert_eq!(
            context.get_info_string(IdInfo::LibelleJEvent),
            Some("0123456789012345678901234567890123456789".to_string())
        );
    }
}
