//! Message 00 : Signe de vie

use crate::context::Context;

use super::frame;
use super::CommonMessageTrait;
use super::ProtocolError;
use super::ST2150;

use crate::context::IdInfo;

/// Message 00 : Signe de vie
#[derive(Default)]
pub struct Message00 {}

/// Numéro de ce message
const MESSAGE_NUM: u8 = 0;

impl CommonMessageTrait for Message00 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn str_message(&self) -> &'static str {
        "Signe de vie"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![
            IdInfo::EnMesurage,
            IdInfo::CodeDefaut,
            IdInfo::ArretIntermediaire,
            IdInfo::ForcagePetitDebit,
            IdInfo::ModeConnecte,
        ]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Message00::availability(self, context)?;

        // Création et envoi requête
        let req = frame::Frame::new(MESSAGE_NUM);
        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = st2150.wait_rep(&mut buffer, 17)?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(&buffer[..len_rep], MESSAGE_NUM, &[1, 1, 1, 1, 1])?;

        // Mise à jour du contexte

        // #0 : En mesurage
        match frame.fields[0].decode_char()? {
            '0' => context.set_info_bool(&IdInfo::EnMesurage, false),
            '1' => context.set_info_bool(&IdInfo::EnMesurage, true),
            _ => {
                return Err(ProtocolError::IllegalFieldValue(
                    frame.fields[0].clone(),
                    "en mesurage".to_string(),
                    "'0' ou '1'".to_string(),
                ))
            }
        }

        // #1 : Code défaut
        let code_defaut = frame.fields[1].decode_binary()?;
        if (0x20..=0x9F).contains(&code_defaut) {
            context.set_info_u8(&IdInfo::CodeDefaut, code_defaut - 0x20);
        } else {
            return Err(ProtocolError::IllegalFieldValue(
                frame.fields[1].clone(),
                "code défaut".to_string(),
                "Valeur entre 0x20 et 0x9F".to_string(),
            ));
        }

        // #2 : Arrêt intermédiaire
        match frame.fields[2].decode_char()? {
            '0' => context.set_info_bool(&IdInfo::ArretIntermediaire, false),
            '1' => context.set_info_bool(&IdInfo::ArretIntermediaire, true),
            _ => {
                return Err(ProtocolError::IllegalFieldValue(
                    frame.fields[2].clone(),
                    "arrêt intermédiaire".to_string(),
                    "'0' ou '1'".to_string(),
                ))
            }
        }

        // #3 : Forçage petit débit
        match frame.fields[3].decode_char()? {
            '0' => context.set_info_bool(&IdInfo::ForcagePetitDebit, false),
            '1' => context.set_info_bool(&IdInfo::ForcagePetitDebit, true),
            _ => {
                return Err(ProtocolError::IllegalFieldValue(
                    frame.fields[3].clone(),
                    "forçage petit debit".to_string(),
                    "'0' ou '1'".to_string(),
                ))
            }
        }

        // #4 : Mode connecté
        match frame.fields[4].decode_char()? {
            '0' => context.set_info_bool(&IdInfo::ModeConnecte, false),
            '1' => context.set_info_bool(&IdInfo::ModeConnecte, true),
            _ => {
                return Err(ProtocolError::IllegalFieldValue(
                    frame.fields[4].clone(),
                    "mode connecté".to_string(),
                    "'0' ou '1'".to_string(),
                ))
            }
        }

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
    fn test_message00() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'0', // Numéro de message
            b'0',
            protocol::SEPARATOR,
            b'F', // Checksum
            b'E',
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'0', // Numéro de message
            b'0',
            protocol::SEPARATOR,
            b'0', // Hors mesurage
            protocol::SEPARATOR,
            0x20, // Pas de défaut
            protocol::SEPARATOR,
            b'0', // Pas en arrêt intermédiaire
            protocol::SEPARATOR,
            b'0', // Pas de forçage PD
            protocol::SEPARATOR,
            b'0', // En mode autonome
            protocol::SEPARATOR,
            b'2', // Checksum
            b'0',
            protocol::ETX,
        ]);

        // Création du protocole ST2150 avec ce FAKE port
        let mut st = ST2150::new(fake_port);

        // Contexte pour le protocole
        let mut context = Context::default();

        // Le message est possible
        assert!(ST2150::message_availability(&context, MESSAGE_NUM).is_ok());

        // Vacation requête/réponse du message via le FAKE port
        assert_eq!(st.do_message_vacation(&mut context, MESSAGE_NUM), Ok(()));

        // Vérification de ce qui a été mis à jour dans le contexte
        assert_eq!(context.get_info_bool(&IdInfo::EnMesurage), Some(false));
        assert_eq!(context.get_info_u8(&IdInfo::CodeDefaut), Some(0));
        assert_eq!(
            context.get_info_bool(&IdInfo::ArretIntermediaire),
            Some(false)
        );
        assert_eq!(
            context.get_info_bool(&IdInfo::ForcagePetitDebit),
            Some(false)
        );
        assert_eq!(context.get_info_bool(&IdInfo::ModeConnecte), Some(false));
    }
}
