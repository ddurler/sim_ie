//! Message 21 : Informations dernier mesurage, demande de solde

use crate::context::Context;

use super::frame;
use super::CommonMessageTrait;
use super::ProtocolError;
use super::ST2150;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 21;

/// Message 21 : Informations dernier mesurage, demande de solde
#[derive(Default)]
pub struct Message21 {}

impl CommonMessageTrait for Message21 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn str_message(&self) -> &'static str {
        "Dernier mesurage, demande de solde"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![
            // 1ere réponse possible
            IdInfo::Nack,
            // 2eme réponse possible
            IdInfo::QuantitePrincipale,
            IdInfo::TemperatureMoyen,
            IdInfo::QuantiteSecondaire,
            IdInfo::Totalisateur,
            IdInfo::IndexSansRaz,
            IdInfo::IndexJournalier,
            IdInfo::Quantieme,
            IdInfo::CodeProduit,
            IdInfo::HeureDebut,
            IdInfo::HeureFin,
        ]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Message21::availability(self, context)?;

        // Création et envoi requête
        let req = frame::Frame::new(MESSAGE_NUM);
        st2150.send_req(&req);

        // Réception réponse (2 réponses possibles)
        let mut buffer = [0; 200];
        let len_rep = st2150.wait_rep(&mut buffer, &[9, 57])?;

        // Décodage de la réponse reçue : 2 réponses possibles : NACK ou compte rendu de mesurage
        // On tente d'abord de décoder un NACK (dans une trame correcte)
        if let Ok(frame) = st2150.try_from_buffer(&buffer[..len_rep], MESSAGE_NUM, &[1]) {
            context.set_info_bool(IdInfo::Nack, frame.is_nack());
            return Ok(());
        }

        // Sinon...
        context.set_info_bool(IdInfo::Nack, false);
        let frame = st2150.try_from_buffer(
            &buffer[..len_rep],
            MESSAGE_NUM,
            &[5, 4, 5, 8, 3, 3, 3, 1, 4, 4],
        )?;

        // Mise à jour du contexte

        // #0 : Quantité principale
        context.set_info_u32(
            IdInfo::QuantitePrincipale,
            frame.fields[0].decode_number::<u32>()?,
        );

        // #1 : Température moyenne +123 pour 12.3°C
        let tempe10 = frame.fields[1].decode_signed_number::<i16>()?;
        let tempe10 = f32::try_from(tempe10).map_err(|_e| {
            ProtocolError::ErrFieldConversion("température".to_string(), frame.fields[1].clone())
        })?;
        context.set_info_f32(IdInfo::TemperatureMoyen, tempe10 / 10_f32);

        // #2 : Quantité secondaire
        context.set_info_u32(
            IdInfo::QuantiteSecondaire,
            frame.fields[2].decode_number::<u32>()?,
        );

        // #2 : Quantité courante
        context.set_info_u32(
            IdInfo::QuantitePrincipale,
            frame.fields[2].decode_number::<u32>()?,
        );

        // #3 : Totalisateur
        context.set_info_u32(
            IdInfo::Totalisateur,
            frame.fields[3].decode_number::<u32>()?,
        );

        // #4 : Index sans Raz
        context.set_info_u16(
            IdInfo::IndexSansRaz,
            frame.fields[4].decode_number::<u16>()?,
        );

        // #5 : Index journalier
        context.set_info_u16(
            IdInfo::IndexJournalier,
            frame.fields[5].decode_number::<u16>()?,
        );

        // #6 : Quantième
        context.set_info_u16(IdInfo::Quantieme, frame.fields[6].decode_number::<u16>()?);

        // #7: Code produit
        context.set_info_u8(IdInfo::CodeProduit, frame.fields[7].decode_number::<u8>()?);

        // #8 : Heure de début
        context.set_info_u16(IdInfo::HeureDebut, frame.fields[8].decode_number::<u16>()?);

        // #8 : Heure de fin
        context.set_info_u16(IdInfo::HeureFin, frame.fields[9].decode_number::<u16>()?);

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
    fn test_message21_ok() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'2', //  Numéro de message
            b'1',
            protocol::SEPARATOR,
            70, // Checksum
            68,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'2', // Numéro de message
            b'1',
            protocol::SEPARATOR,
            b'1', // Volume mesurage
            b'2',
            b'3',
            b'4',
            b'5',
            protocol::SEPARATOR,
            b'+', // Température +123 pour 12.3°C
            b'1',
            b'2',
            b'3',
            protocol::SEPARATOR,
            b'1', // Volume secondaire
            b'2',
            b'3',
            b'4',
            b'5',
            protocol::SEPARATOR,
            b'1', // Totalisateur
            b'2',
            b'3',
            b'4',
            b'5',
            b'6',
            b'7',
            b'8',
            protocol::SEPARATOR,
            b'1', // Index sans Raz
            b'2',
            b'3',
            protocol::SEPARATOR,
            b'1', // Index journalier
            b'2',
            b'3',
            protocol::SEPARATOR,
            b'1', // Quantième
            b'2',
            b'3',
            protocol::SEPARATOR,
            b'1', // Code produit
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
            b'E', // Checksum
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
        assert_eq!(
            context.get_info_u32(IdInfo::QuantitePrincipale),
            Some(12_345)
        );
        assert_eq!(context.get_info_f32(IdInfo::TemperatureMoyen), Some(12.3));
        assert_eq!(
            context.get_info_u32(IdInfo::QuantiteSecondaire),
            Some(12_345)
        );
        assert_eq!(context.get_info_u32(IdInfo::Totalisateur), Some(12_345_678));
        assert_eq!(context.get_info_u16(IdInfo::IndexSansRaz), Some(123));
        assert_eq!(context.get_info_u16(IdInfo::IndexJournalier), Some(123));
        assert_eq!(context.get_info_u16(IdInfo::Quantieme), Some(123));
        assert_eq!(context.get_info_u8(IdInfo::CodeProduit), Some(1));
        assert_eq!(context.get_info_u16(IdInfo::HeureDebut), Some(12_34));
        assert_eq!(context.get_info_u16(IdInfo::HeureFin), Some(12_34));
    }

    #[test]
    fn test_message21_nack() {
        // Contexte pour le protocole
        let mut context = Context::default();

        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'2', //  Numéro de message
            b'1',
            protocol::SEPARATOR,
            70, // Checksum
            68,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'2', // Numéro de message
            b'1',
            protocol::SEPARATOR,
            protocol::NACK, // NACK
            protocol::SEPARATOR,
            b'1', // Checksum
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
        assert_eq!(context.get_info_bool(IdInfo::Nack), Some(true));
    }
}
