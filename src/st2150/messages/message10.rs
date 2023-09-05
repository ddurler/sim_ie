//! Message 10 : Informations instantanées

use crate::context::Context;

use super::frame;
use super::CommonMessageTrait;
use super::ProtocolError;
use super::ST2150;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 10;

/// Message 10 : Informations instantanées
#[derive(Default)]
pub struct Message10 {}

impl CommonMessageTrait for Message10 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn str_message(&self) -> &'static str {
        "Informations instantanées"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![
            IdInfo::Totalisateur,
            IdInfo::DebitInstant,
            IdInfo::QuantiteChargee,
            IdInfo::TemperatureInstant,
            IdInfo::Predetermination,
        ]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Message10::availability(self, context)?;

        // Création et envoi requête
        let req = frame::Frame::new(10);
        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = st2150.wait_rep(&mut buffer, 38)?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(&buffer[..len_rep], 10, &[8, 4, 5, 4, 5])?;

        // Mise à jour du contexte

        // #0 : Totalisateur
        context.set_info_u32(
            &IdInfo::Totalisateur,
            frame.fields[0].decode_number::<u32>()?,
        );

        // #1 : Débit instantanée (1234 pour 123.4 m3/h)
        let debit10 = frame.fields[1].decode_number::<u16>()?;
        let debit10 = f32::try_from(debit10).map_err(|_e| {
            ProtocolError::ErrFieldConversion("débit".to_string(), frame.fields[1].clone())
        })?;

        context.set_info_f32(&IdInfo::DebitInstant, debit10 / 10_f32);

        // #2 : Quantité courante
        context.set_info_u32(
            &IdInfo::QuantiteChargee,
            frame.fields[2].decode_number::<u32>()?,
        );

        // #3 : Température instantanée +123 pour 12.3°C
        let tempe10 = frame.fields[3].decode_signed_number::<i16>()?;
        let tempe10 = f32::try_from(tempe10).map_err(|_e| {
            ProtocolError::ErrFieldConversion("température".to_string(), frame.fields[1].clone())
        })?;
        context.set_info_f32(&IdInfo::TemperatureInstant, tempe10 / 10_f32);

        // #4 : Prédétermination
        context.set_info_u32(
            &IdInfo::Predetermination,
            frame.fields[4].decode_number::<u32>()?,
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
    fn test_message10() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'1', //  Numéro de message
            b'0',
            protocol::SEPARATOR,
            b'F', // Checksum
            b'F',
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'1', // Numéro de message
            b'0',
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
            b'1', // Débit 1234 pour 123.4
            b'2',
            b'3',
            b'4',
            protocol::SEPARATOR,
            b'1', // Quantité
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
            b'1', // Prédétermination
            b'2',
            b'3',
            b'4',
            b'5',
            protocol::SEPARATOR,
            b'1', // Checksum
            b'6',
            protocol::ETX,
        ]);

        // Création du protocole ST2150 avec ce FAKE port
        let mut st = ST2150::new(fake_port);

        // Contexte pour le protocole
        let mut context = Context::default();

        // Le message est possible
        assert!(ST2150::message_availability(&context, 10).is_ok());

        // Vacation requête/réponse du message via le FAKE port
        assert_eq!(st.do_message_vacation(&mut context, 10), Ok(()));

        // Vérification de ce qui a été mis à jour dans le contexte
        assert_eq!(
            context.get_info_u32(&IdInfo::Totalisateur),
            Some(12_345_678)
        );
        assert_eq!(context.get_info_f32(&IdInfo::DebitInstant), Some(123.4));
        assert_eq!(context.get_info_u32(&IdInfo::QuantiteChargee), Some(12345));
        assert_eq!(
            context.get_info_f32(&IdInfo::TemperatureInstant),
            Some(12.3)
        );
        assert_eq!(context.get_info_u32(&IdInfo::Predetermination), Some(12345));
    }
}
