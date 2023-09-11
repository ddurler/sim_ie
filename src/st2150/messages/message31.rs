//! Message 31 : Nombre de mesurages pour un quantième

use crate::context::Context;

use super::field::Field;
use super::frame;
use super::CommonMessageTrait;
use super::ProtocolError;
use super::ST2150;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 31;

/// Message 31 : Nombre de mesurages pour un quantième
#[derive(Default)]
pub struct Message31 {}

impl CommonMessageTrait for Message31 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn str_message(&self) -> &'static str {
        "Nb mesurages pour un quantième"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![IdInfo::Quantieme]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![IdInfo::NbMesuragesQuantieme]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Message31::availability(self, context)?;

        // Création et envoi requête
        let mut req = frame::Frame::new(MESSAGE_NUM);

        let quantieme = context.get_info_u16(IdInfo::Quantieme).unwrap();
        req.add_field(Field::encode_number(quantieme, 3)?);

        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = st2150.wait_rep(&mut buffer, &[11])?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(&buffer[..len_rep], MESSAGE_NUM, &[3])?;

        // Mise à jour du contexte

        // #0 : Nombre de mesurages pour le quantième
        context.set_info_u16(
            IdInfo::NbMesuragesQuantieme,
            frame.fields[0].decode_number()?,
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
    fn test_message31() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        context.set_info_u16(IdInfo::Quantieme, 123);

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'3', //  Numéro de message
            b'1',
            protocol::SEPARATOR,
            b'1', // Quantième
            b'2',
            b'3',
            protocol::SEPARATOR,
            51, // Checksum
            50,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'3', // Numéro de message
            b'1',
            protocol::SEPARATOR,
            b'0', // Nombre de mesurages pour ce quantième
            b'1',
            b'2',
            protocol::SEPARATOR,
            b'3', // Checksum
            b'1',
            protocol::ETX,
        ]);

        // Création du protocole ST2150 avec ce FAKE port
        let mut st = ST2150::new(fake_port);

        // Le message est possible
        assert!(ST2150::message_availability(&context, MESSAGE_NUM).is_ok());

        // Vacation requête/réponse du message via le FAKE port
        assert_eq!(st.do_message_vacation(&mut context, MESSAGE_NUM), Ok(()));

        // Vérification de ce qui a été mis à jour dans le contexte
        assert_eq!(context.get_info_u16(IdInfo::NbMesuragesQuantieme), Some(12));
    }
}
