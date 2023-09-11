//! Message 33 : Table produits (court)

use crate::context::Context;

use super::frame;
use super::CommonMessageTrait;
use super::ProtocolError;
use super::ST2150;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 33;

/// Message 33 : Table produits (court)
#[derive(Default)]
pub struct Message33 {}

impl CommonMessageTrait for Message33 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn str_message(&self) -> &'static str {
        "Tables des produits (court)"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![
            IdInfo::LibelleTableProduits(1),
            IdInfo::LibelleTableProduits(2),
            IdInfo::LibelleTableProduits(3),
            IdInfo::LibelleTableProduits(4),
            IdInfo::LibelleTableProduits(5),
            IdInfo::LibelleTableProduits(6),
            IdInfo::LibelleTableProduits(7),
            IdInfo::LibelleTableProduits(8),
        ]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Message33::availability(self, context)?;

        // Création et envoi requête
        let req = frame::Frame::new(MESSAGE_NUM);

        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = st2150.wait_rep(&mut buffer, &[55])?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(&buffer[..len_rep], MESSAGE_NUM, &[5; 8])?;

        // Mise à jour du contexte

        // #0 - #7 : Libellé table produits(i+1)
        for indice_champ in 0_usize..=7 {
            context.set_info_string(
                IdInfo::LibelleTableProduits(indice_champ + 1),
                &frame.fields[indice_champ].decode_str()?,
            );
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
    fn test_message33() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'3', //  Numéro de message
            b'3',
            protocol::SEPARATOR,
            70, // Checksum
            69,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'3', // Numéro de message
            b'3',
            protocol::SEPARATOR,
            b'P', // Libellé produit #1
            b'R',
            b'O',
            b'D',
            b'1',
            protocol::SEPARATOR,
            b'P', // Libellé produit #2
            b'R',
            b'O',
            b'D',
            b'2',
            protocol::SEPARATOR,
            b'P', // Libellé produit #3
            b'R',
            b'O',
            b'D',
            b'3',
            protocol::SEPARATOR,
            b'P', // Libellé produit #4
            b'R',
            b'O',
            b'D',
            b'4',
            protocol::SEPARATOR,
            b'P', // Libellé produit #5
            b'R',
            b'O',
            b'D',
            b'5',
            protocol::SEPARATOR,
            b'P', // Libellé produit #6
            b'R',
            b'O',
            b'D',
            b'6',
            protocol::SEPARATOR,
            b'P', // Libellé produit #7
            b'R',
            b'O',
            b'D',
            b'7',
            protocol::SEPARATOR,
            b'P', // Libellé produit #8
            b'R',
            b'O',
            b'D',
            b'8',
            protocol::SEPARATOR,
            b'F', // Checksum
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
        for indice_produit in 1_usize..=8 {
            assert_eq!(
                context.get_info_string(IdInfo::LibelleTableProduits(indice_produit)),
                Some(format!("PROD{indice_produit}"))
            );
        }
    }
}
