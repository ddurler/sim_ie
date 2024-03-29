//! Message 70 : Mouvement de produit - Prédétermination gravitaire

use crate::context::Context;

use super::helper_messages60_79 as helper;
use super::CommonMessageTrait;
use super::Edition2150;
use super::ProtocolError;
use super::ST2150;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 70;

/// Message 70 : Mouvement de produit - prédétermination gravitaire
#[derive(Default)]
pub struct Message70;

impl CommonMessageTrait for Message70 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn edition_st2150(&self) -> Edition2150 {
        helper::edition_st2150(MESSAGE_NUM)
    }

    fn message_str(&self) -> &'static str {
        helper::message_str(MESSAGE_NUM)
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        helper::id_infos_request(MESSAGE_NUM)
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        helper::id_infos_response(MESSAGE_NUM)
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Self::availability(self, context)?;

        // Création et envoi requête
        let req = helper::create_frame_request(MESSAGE_NUM, context)?;

        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = st2150.wait_rep(&mut buffer, helper::rep_len_fields(MESSAGE_NUM))?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(
            &buffer[..len_rep],
            MESSAGE_NUM,
            helper::rep_len_fields(MESSAGE_NUM),
        )?;

        // Mise à jour du contexte selon la réponse
        helper::update_context_from_rep(MESSAGE_NUM, context, &frame)?;

        // C'est tout bon
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::context::U8OrT;
    use crate::st2150::protocol;
    use crate::CommonSerialComTrait;
    use crate::SerialCom;

    #[test]
    fn test_message70() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        // Infos pour la requête
        context.set_info_u32(IdInfo::Predetermination, 1000);
        context.set_info_u8(IdInfo::CodeProduit, 3);
        context.set_info_u8_or_t(IdInfo::NumeroCompartiment, U8OrT::U8(2));
        context.set_info_bool(IdInfo::FinirFlexibleVide, true);

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'7', //  Numéro de message
            b'0',
            protocol::SEPARATOR,
            b'0', // Limitation
            b'1',
            b'0',
            b'0',
            b'0',
            protocol::SEPARATOR,
            b'3', // Code produit
            protocol::SEPARATOR,
            b'2', // Numéro compartiment
            protocol::SEPARATOR,
            b'V', // Finir vide
            protocol::SEPARATOR,
            57, // Checksum
            70,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'7', // Numéro de message
            b'0',
            protocol::SEPARATOR,
            protocol::ACK, // ACK
            protocol::SEPARATOR,
            b'0', // Code erreur mouvement de produit
            b'0',
            protocol::SEPARATOR,
            b'F', // Checksum
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
        assert_eq!(context.get_option_info_bool(IdInfo::Nack), Some(false));
        assert_eq!(context.get_option_info_bool(IdInfo::Ack), Some(true));
        assert_eq!(
            context.get_option_info_u8(IdInfo::CodeErreurMouvementProduit),
            Some(0)
        );
    }
}
