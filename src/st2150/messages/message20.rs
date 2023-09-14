//! Message 20 : Présélection

use crate::context;
use crate::context::{Context, IdInfo};

use super::field::Field;
use super::frame;
use super::CommonMessageTrait;
use super::Edition2150;
use super::ProtocolError;
use super::ST2150;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 20;

/// Message 20 : Présélection
#[derive(Default)]
pub struct Message20 {}

impl CommonMessageTrait for Message20 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn edition_st2150(&self) -> Edition2150 {
        Edition2150::A
    }

    fn message_str(&self) -> &'static str {
        "Présélection"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![IdInfo::Predetermination, IdInfo::CodeProduit]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![IdInfo::Ack, IdInfo::Nack]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Self::availability(self, context)?;

        // Création et envoi requête
        let mut req = frame::Frame::new(MESSAGE_NUM);

        // Valeur de la prédétermination
        let prede = context.get_info_u32(IdInfo::Predetermination).unwrap();
        req.add_field(Field::encode_number(prede, 5)?);

        // Code produit
        let code_prod = context.get_info_u8(IdInfo::CodeProduit).unwrap();
        Field::check_binary_domain(
            "code produit",
            code_prod,
            0_u8..=u8::try_from(context::NB_PRODUITS).unwrap(),
        )?;
        req.add_field(Field::encode_binary(code_prod + b'0'));

        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = st2150.wait_rep(&mut buffer, 9)?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(&buffer[..len_rep], MESSAGE_NUM, &[1])?;

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
    fn test_message20() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        // Infos pour la requête de test
        context.set_info_u32(IdInfo::Predetermination, 12345);
        context.set_info_u8(IdInfo::CodeProduit, 1);

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'2', //  Numéro de message
            b'0',
            protocol::SEPARATOR,
            b'1', // Prédétermination
            b'2',
            b'3',
            b'4',
            b'5',
            protocol::SEPARATOR,
            b'1', // Code produit
            protocol::SEPARATOR,
            70, // Checksum
            67,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'2', // Numéro de message
            b'0',
            protocol::SEPARATOR,
            protocol::ACK, // ACK
            protocol::SEPARATOR,
            b'0', // Checksum
            b'4',
            protocol::ETX,
        ]);

        // Création du protocole ST2150 avec ce FAKE port
        let mut st = ST2150::new(fake_port);

        // Le message est possible
        assert!(ST2150::message_availability(&context, MESSAGE_NUM).is_ok());

        // Vacation requête/réponse du message via le FAKE port
        assert_eq!(st.do_message_vacation(&mut context, MESSAGE_NUM), Ok(()));

        // Vérification de ce qui a été mis à jour dans le contexte
        assert_eq!(context.get_info_bool(IdInfo::Nack), Some(false));
        assert_eq!(context.get_info_bool(IdInfo::Ack), Some(true));
    }
}
