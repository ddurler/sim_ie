//! Message 34 : Relevé fractionnement

use crate::context::Context;

use super::field::Field;
use super::frame;
use super::CommonMessageTrait;
use super::Edition2150;
use super::ProtocolError;
use super::ST2150;

use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 34;

/// Message 34 : Relevé fractionnement
#[derive(Default)]
pub struct Message34 {}

impl CommonMessageTrait for Message34 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn edition_st2150(&self) -> Edition2150 {
        Edition2150::A
    }

    fn message_str(&self) -> &'static str {
        "Relevé fractionnement"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![
            IdInfo::Quantieme,
            IdInfo::IndexJournalier,
            IdInfo::IndexFractionnement,
        ]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![
            IdInfo::QuantitePrincipale,
            IdInfo::TypeDistribution,
            IdInfo::HeureHHMMDebut,
            IdInfo::HeureHHMMFin,
        ]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Message34::availability(self, context)?;

        // Création et envoi requête
        let mut req = frame::Frame::new(MESSAGE_NUM);

        // #0 : Quantième
        let quantieme = context.get_info_u16(IdInfo::Quantieme).unwrap();
        req.add_field(Field::encode_number(quantieme, 3)?);

        // #1 : Numéro d'ordre dans la journée
        let index_journalier = context.get_info_u16(IdInfo::IndexJournalier).unwrap();
        req.add_field(Field::encode_number(index_journalier, 3)?);

        // #2 : Numéro du fractionnement
        let index_fractionnement = context.get_info_u16(IdInfo::IndexFractionnement).unwrap();
        req.add_field(Field::encode_number(index_fractionnement, 3)?);

        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = st2150.wait_rep(&mut buffer, 25)?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(&buffer[..len_rep], MESSAGE_NUM, &[5, 1, 4, 4])?;

        // Mise à jour du contexte

        // #0 - Quantité livrée
        context.set_info_u32(
            IdInfo::QuantitePrincipale,
            frame.fields[0].decode_number::<u32>()?,
        );

        // #1 : Type de distribution
        context.set_info_char(IdInfo::TypeDistribution, frame.fields[1].decode_char()?);

        // #2 : Heure de début
        context.set_info_u16(
            IdInfo::HeureHHMMDebut,
            frame.fields[2].decode_number::<u16>()?,
        );

        // #3 : Heure de fin
        context.set_info_u16(
            IdInfo::HeureHHMMFin,
            frame.fields[3].decode_number::<u16>()?,
        );

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
    fn test_message34() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        context.set_info_u16(IdInfo::Quantieme, 123);
        context.set_info_u16(IdInfo::IndexJournalier, 1);
        context.set_info_u16(IdInfo::IndexFractionnement, 2);

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'3', //  Numéro de message
            b'4',
            protocol::SEPARATOR,
            b'1', // Quantième
            b'2',
            b'3',
            protocol::SEPARATOR,
            b'0', // No ordre dans le jour
            b'0',
            b'1',
            protocol::SEPARATOR,
            b'0', // No fractionnement pour ce mesurage
            b'0',
            b'2',
            protocol::SEPARATOR,
            51, // Checksum
            52,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'3', // Numéro de message
            b'4',
            protocol::SEPARATOR,
            b'1', // Quantité livrée
            b'2',
            b'3',
            b'4',
            b'5',
            protocol::SEPARATOR,
            b'D', // Type de distribution
            protocol::SEPARATOR,
            b'1', // Heure de début
            b'2',
            b'3',
            b'4',
            protocol::SEPARATOR,
            b'1', // Heure de fin
            b'2',
            b'3',
            b'4',
            protocol::SEPARATOR,
            b'8', // Checksum
            b'C',
            protocol::ETX,
        ]);

        // Création du protocole ST2150 avec ce FAKE port
        let mut st = ST2150::new(fake_port);

        // Le message est possible
        assert!(ST2150::message_availability(&context, MESSAGE_NUM).is_ok());

        // Vacation requête/réponse du message via le FAKE port
        assert_eq!(st.do_message_vacation(&mut context, MESSAGE_NUM), Ok(()));

        // Vérification de ce qui a été mis à jour dans le contexte
        assert_eq!(
            context.get_info_u32(IdInfo::QuantitePrincipale),
            Some(12_345)
        );
        assert_eq!(context.get_info_char(IdInfo::TypeDistribution), Some('D'));
        assert_eq!(context.get_info_u16(IdInfo::HeureHHMMDebut), Some(12_34));
        assert_eq!(context.get_info_u16(IdInfo::HeureHHMMFin), Some(12_34));
    }
}
