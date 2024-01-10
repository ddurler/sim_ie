//! Message 32 : Relevé mesurage

use crate::context::Context;

use super::field::Field;
use super::frame;
use super::CommonMessageTrait;
use super::Edition2150;
use super::ProtocolError;
use super::ST2150;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 32;

/// Message 32 : Relevé mesurage
#[derive(Default)]
pub struct Message32 {}

impl CommonMessageTrait for Message32 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn edition_st2150(&self) -> Edition2150 {
        Edition2150::A
    }

    fn message_str(&self) -> &'static str {
        "Relevé mesurage"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![IdInfo::Quantieme, IdInfo::IndexJournalier]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![
            IdInfo::LibelleProduit,
            IdInfo::QuantitePrincipale,
            IdInfo::TemperatureMoyen,
            IdInfo::NbFractionnements,
            IdInfo::HeureHHMMDebut,
            IdInfo::HeureHHMMFin,
        ]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Self::availability(self, context)?;

        // Création et envoi requête
        let mut req = frame::Frame::new(MESSAGE_NUM);

        // #0 : Quantième
        let quantieme = context.get_option_info_u16(IdInfo::Quantieme).unwrap();
        req.add_field(Field::encode_number(quantieme, 3)?);

        // #1 : Numéro d'ordre dans la journée
        let index_journalier = context
            .get_option_info_u16(IdInfo::IndexJournalier)
            .unwrap();
        req.add_field(Field::encode_number(index_journalier, 3)?);

        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let lens_expected = &[5, 5, 4, 3, 4, 4];
        let len_rep = st2150.wait_rep(&mut buffer, lens_expected)?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(&buffer[..len_rep], MESSAGE_NUM, lens_expected)?;

        // Mise à jour du contexte

        // #0 : Libellé produit
        context.set_info_string(IdInfo::LibelleProduit, &frame.fields[0].decode_str()?);

        // #1 : Quantité livrée
        context.set_info_u32(
            IdInfo::QuantitePrincipale,
            frame.fields[1].decode_number::<u32>()?,
        );

        // #2 : Température moyenne +123 pour 12.3°C
        let tempe10 = frame.fields[2].decode_signed_number::<i16>()?;
        let tempe10 = f32::from(tempe10);
        context.set_info_f32(IdInfo::TemperatureMoyen, tempe10 / 10_f32);

        // #3 : Nombre de fractionnements
        context.set_info_u16(
            IdInfo::NbFractionnements,
            frame.fields[3].decode_number::<u16>()?,
        );

        // #4 : Heure de début
        context.set_info_u16(
            IdInfo::HeureHHMMDebut,
            frame.fields[4].decode_number::<u16>()?,
        );

        // #5 : Heure de fin
        context.set_info_u16(
            IdInfo::HeureHHMMFin,
            frame.fields[5].decode_number::<u16>()?,
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
    fn test_message32() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        context.set_info_u16(IdInfo::Quantieme, 123);
        context.set_info_u16(IdInfo::IndexJournalier, 1);

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'3', //  Numéro de message
            b'2',
            protocol::SEPARATOR,
            b'1', // Quantième
            b'2',
            b'3',
            protocol::SEPARATOR,
            b'0', // Numéro d'ordre
            b'0',
            b'1',
            protocol::SEPARATOR,
            70, // Checksum
            69,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'3', // Numéro de message
            b'2',
            protocol::SEPARATOR,
            b'A', // Libellé produit
            b'B',
            b'C',
            b'D',
            b'E',
            protocol::SEPARATOR,
            b'1', // Quantité livrée
            b'2',
            b'3',
            b'4',
            b'5',
            protocol::SEPARATOR,
            b'+', // Température moyenne +123 pour 12.3°C
            b'1',
            b'2',
            b'3',
            protocol::SEPARATOR,
            b'0', // Nombre de fractionnements
            b'0',
            b'1',
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
            b'A', // Checksum
            b'5',
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
            context.get_option_info_string(IdInfo::LibelleProduit),
            Some("ABCDE".to_string())
        );
        assert_eq!(
            context.get_option_info_u32(IdInfo::QuantitePrincipale),
            Some(12_345)
        );
        assert_eq!(
            context.get_option_info_f32(IdInfo::TemperatureMoyen),
            Some(12.3)
        );
        assert_eq!(
            context.get_option_info_u16(IdInfo::NbFractionnements),
            Some(1)
        );
        assert_eq!(
            context.get_option_info_u16(IdInfo::HeureHHMMDebut),
            Some(12_34)
        );
        assert_eq!(
            context.get_option_info_u16(IdInfo::HeureHHMMFin),
            Some(12_34)
        );
    }
}
