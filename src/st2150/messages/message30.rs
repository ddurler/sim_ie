//! Message 30 : Information compteur

use crate::context::Context;

use super::frame;
use super::CommonMessageTrait;
use super::ProtocolError;
use super::ST2150;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 30;

/// Message 30 : Information compteur
#[derive(Default)]
pub struct Message30 {}

impl CommonMessageTrait for Message30 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn str_message(&self) -> &'static str {
        "Information compteur"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![
            IdInfo::ReferenceEtImmatriculation,
            IdInfo::VersionLogiciel,
            IdInfo::DateHeure,
            IdInfo::TypeCompteur,
        ]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Message30::availability(self, context)?;

        // Création et envoi requête
        let req = frame::Frame::new(MESSAGE_NUM);
        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = st2150.wait_rep(&mut buffer, 49)?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(&buffer[..len_rep], MESSAGE_NUM, &[15, 10, 12, 1])?;

        // Mise à jour du contexte

        // #0 : Référence compteur et immatriculation véhicule
        context.set_info_string(
            IdInfo::ReferenceEtImmatriculation,
            &frame.fields[0].decode_str()?,
        );

        // #1 : Version logiciel
        context.set_info_string(IdInfo::VersionLogiciel, &frame.fields[1].decode_str()?);

        // #2 : Date & heure
        context.set_info_u64(IdInfo::DateHeure, frame.fields[2].decode_number::<u64>()?);

        // #3 : Type compteur
        context.set_info_u8(IdInfo::TypeCompteur, frame.fields[3].decode_number::<u8>()?);

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
    fn test_message30() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'3', //  Numéro de message
            b'0',
            protocol::SEPARATOR,
            70, // Checksum
            68,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'3', // Numéro de message
            b'0',
            protocol::SEPARATOR,
            b'1', // Référence compteur (5 cars)
            b'2',
            b'3',
            b'4',
            b'5',
            b'A', // Immatriculation camion (10 cars)
            b'B',
            b'C',
            b'1',
            b'2',
            b'3',
            b'4',
            b'X',
            b'Y',
            b'Z',
            protocol::SEPARATOR,
            b'1', // Version logiciel (10 cars)
            b'.',
            b'0',
            b'0',
            b'0',
            b'1',
            b'0',
            b'1',
            b'0',
            b'1',
            protocol::SEPARATOR,
            b'9', // Date & Heure AAMMJJHHMMSS
            b'9',
            b'1',
            b'2',
            b'3',
            b'1',
            b'2',
            b'3',
            b'5',
            b'9',
            b'5',
            b'9',
            protocol::SEPARATOR,
            b'0', // Type compteur 0:Vm, 1:Vb, 2:Masse
            protocol::SEPARATOR,
            b'F', // Checksum
            b'D',
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
            context.get_info_string(IdInfo::ReferenceEtImmatriculation),
            Some("12345ABC1234XYZ".to_string())
        );
        assert_eq!(
            context.get_info_string(IdInfo::VersionLogiciel),
            Some("1.00010101".to_string())
        );
        assert_eq!(
            context.get_info_u64(IdInfo::DateHeure),
            Some(99_12_31_23_59_59_u64)
        );
        assert_eq!(context.get_info_u8(IdInfo::TypeCompteur), Some(0));
    }
}
