//! Message 37 : Mise à jour du plan

use crate::context::Context;
use crate::st2150::field::Field;

use super::frame;
use super::CommonMessageTrait;
use super::ProtocolError;
use super::ST2150;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 37;

/// Message 37 : Mise à jour du plan
#[derive(Default)]
pub struct Message37 {}

impl CommonMessageTrait for Message37 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn str_message(&self) -> &'static str {
        "Mise à jour du plan"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![
            IdInfo::CodeProduitCompartiment(1),
            IdInfo::QuantiteCompartiment(1),
            IdInfo::CodeProduitCompartiment(2),
            IdInfo::QuantiteCompartiment(2),
            IdInfo::CodeProduitCompartiment(3),
            IdInfo::QuantiteCompartiment(3),
            IdInfo::CodeProduitCompartiment(4),
            IdInfo::QuantiteCompartiment(4),
            IdInfo::CodeProduitCompartiment(5),
            IdInfo::QuantiteCompartiment(5),
            IdInfo::CodeProduitCompartiment(6),
            IdInfo::QuantiteCompartiment(6),
            IdInfo::CodeProduitCompartiment(7),
            IdInfo::QuantiteCompartiment(7),
            IdInfo::CodeProduitCompartiment(8),
            IdInfo::QuantiteCompartiment(8),
            IdInfo::CodeProduitCompartiment(9),
            IdInfo::QuantiteCompartiment(9),
        ]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![IdInfo::Ack, IdInfo::Nack]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Message37::availability(self, context)?;

        // Création et envoi requête
        let mut req = frame::Frame::new(MESSAGE_NUM);

        // 9 x code produit et quantité des compartiments
        for compart_num in 1..=9 {
            let code_produit = context
                .get_info_u8(IdInfo::CodeProduitCompartiment(compart_num))
                .unwrap();
            req.add_field(Field::encode_binary(b'0' + code_produit));

            let quantite = context
                .get_info_u32(IdInfo::QuantiteCompartiment(compart_num))
                .unwrap();
            req.add_field(Field::encode_number(quantite, 5)?);
        }

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

    #[rustfmt::skip]  // On demande à 'cargo fmt' de ne pas arranger le code parce que sinon ça dépasse 100 lignes :)
    #[test]
    fn test_message37() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        // On met code produit i & quantité 1000 * i dans le compartiment #i
        for num_compart in 1_u8..=9 {
            context.set_info_u8(
                IdInfo::CodeProduitCompartiment(num_compart as usize),
                num_compart,
            );
            context.set_info_u32(
                IdInfo::QuantiteCompartiment(num_compart as usize),
                1000_u32 * u32::from(num_compart),
            );
        }

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'3', //  Numéro de message
            b'7',
            protocol::SEPARATOR,
            b'1', // Code produit dans compartiment #1
            protocol::SEPARATOR,
            b'0', b'1', b'0', b'0', b'0', // Quantité dans compartiment #1
            protocol::SEPARATOR,
            b'2', // Code produit dans compartiment #2
            protocol::SEPARATOR,
            b'0', b'2', b'0', b'0', b'0',  // Quantité dans compartiment #2
            protocol::SEPARATOR,
            b'3', // Code produit dans compartiment #3
            protocol::SEPARATOR,
            b'0', b'3', b'0', b'0', b'0', // Quantité dans compartiment #3
            protocol::SEPARATOR,
            b'4', // Code produit dans compartiment #4
            protocol::SEPARATOR,
            b'0', b'4', b'0', b'0', b'0', // Quantité dans compartiment #4
            protocol::SEPARATOR,
            b'5', // Code produit dans compartiment #5
            protocol::SEPARATOR,
            b'0', b'5', b'0', b'0', b'0', // Quantité dans compartiment #5
            protocol::SEPARATOR,
            b'6', // Code produit dans compartiment #6
            protocol::SEPARATOR,
            b'0', b'6', b'0', b'0', b'0', // Quantité dans compartiment #6
            protocol::SEPARATOR,
            b'7', // Code produit dans compartiment #7
            protocol::SEPARATOR,
            b'0', b'7', b'0', b'0', b'0', // Quantité dans compartiment #7
            protocol::SEPARATOR,
            b'8', // Code produit dans compartiment #8
            protocol::SEPARATOR,
            b'0', b'8', b'0', b'0', b'0', // Quantité dans compartiment #8
            protocol::SEPARATOR,
            b'9', // Code produit dans compartiment #9
            protocol::SEPARATOR,
            b'0', b'9', b'0', b'0', b'0', // Quantité dans compartiment #9
            protocol::SEPARATOR,
            70, // Checksum
            65,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'3', // Numéro de message
            b'7',
            protocol::SEPARATOR,
            protocol::ACK, // ACK
            protocol::SEPARATOR,
            b'0', // Checksum
            b'2',
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
