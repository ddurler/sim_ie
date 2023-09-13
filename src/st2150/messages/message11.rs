//! Message 11 : État cargaison

use crate::context::Context;

use super::frame;
use super::CommonMessageTrait;
use super::Edition2150;
use super::ProtocolError;
use super::ST2150;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 11;

/// Message 11 : État cargaison
#[derive(Default)]
pub struct Message11 {}

impl CommonMessageTrait for Message11 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn edition_st2150(&self) -> Edition2150 {
        Edition2150::C
    }

    fn str_message(&self) -> &'static str {
        "État cargaison"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![
            IdInfo::NombreCompartiments,
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
            IdInfo::PresenceRemorque,
            IdInfo::CodeProduitCollecteur,
            IdInfo::CodeProduitPartieCommune,
            IdInfo::CodeProduitFlexible1,
            IdInfo::CodeProduitFlexible2,
        ]
    }

    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Message11::availability(self, context)?;

        // Création et envoi requête
        let req = frame::Frame::new(MESSAGE_NUM);
        st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = st2150.wait_rep(&mut buffer, 88)?;

        // Décodage de la réponse reçue
        let frame = st2150.try_from_buffer(
            &buffer[..len_rep],
            MESSAGE_NUM,
            &[
                1, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 4,
            ],
        )?;

        // Mise à jour du contexte

        // #0 - Nombre compartiments
        context.set_info_u8(
            IdInfo::NombreCompartiments,
            frame.fields[0].decode_number::<u8>()?,
        );

        // #1 à ?? : 9 fois par compartiment codeProduit et Quantité
        let mut index_champ = 1;
        for compart_num in 1..=9 {
            context.set_info_u8(
                IdInfo::CodeProduitCompartiment(compart_num),
                frame.fields[index_champ].decode_number::<u8>()?,
            );
            context.set_info_u32(
                IdInfo::QuantiteCompartiment(compart_num),
                frame.fields[index_champ + 1].decode_number::<u32>()?,
            );
            index_champ += 2;
        }

        // #ensuite #(index_champ) : Présence remorque
        let presence_remorque = matches!(frame.fields[index_champ].decode_char()?, 'T');
        context.set_info_bool(IdInfo::PresenceRemorque, presence_remorque);

        // #et enfin (index_champ+1) : Les codes produits dans la tuyauterie
        // 4 fois un u8 avec #0: Collecteur, #1: partie commune, #2: flexible1 et #3: flexible2
        let code_produits_tuyauterie = frame.fields[index_champ + 1].decode_as_vec();
        if code_produits_tuyauterie.len() != 4 {
            return Err(ProtocolError::IllegalFieldValue(
                frame.fields[index_champ + 1].clone(),
                "produits tuyauterie".to_string(),
                "4 x code produit".to_string(),
            ));
        }
        let code_produits_tuyauterie: Vec<u8> = code_produits_tuyauterie
            .iter()
            .map(|code| *code - b'0')
            .collect();
        context.set_info_u8(IdInfo::CodeProduitCollecteur, code_produits_tuyauterie[0]);
        context.set_info_u8(
            IdInfo::CodeProduitPartieCommune,
            code_produits_tuyauterie[1],
        );
        context.set_info_u8(IdInfo::CodeProduitFlexible1, code_produits_tuyauterie[2]);
        context.set_info_u8(IdInfo::CodeProduitFlexible2, code_produits_tuyauterie[3]);

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
    fn test_message11() {
        // On utilise le FAKE serial port pour contrôler ce qui circule...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        // Contexte pour le protocole
        let mut context = Context::default();

        // Trame pour message
        fake_port.should_write(&[
            protocol::STX,
            b'1', //  Numéro de message
            b'1',
            protocol::SEPARATOR,
            70, // Checksum
            69,
            protocol::ETX,
        ]);

        // Réponse simulée
        fake_port.will_read(&[
            protocol::STX,
            b'1', // Numéro de message
            b'1',
            protocol::SEPARATOR,
            b'9', // Nombre de compartiments
            protocol::SEPARATOR,
            b'1', // Code produit compartiment #1
            protocol::SEPARATOR,
            b'0',  b'1', b'0', b'0', b'0', // Quantité compartiment #1
            protocol::SEPARATOR,
            b'2', // Code produit compartiment #2
            protocol::SEPARATOR,
            b'0', b'2', b'0', b'0', b'0', // Quantité compartiment #2
            protocol::SEPARATOR,
            b'3', // Code produit compartiment #3
            protocol::SEPARATOR,
            b'0', b'3', b'0', b'0', b'0', // Quantité compartiment #3
            protocol::SEPARATOR,
            b'4', // Code produit compartiment #4
            protocol::SEPARATOR,
            b'0', b'4', b'0', b'0', b'0', // Quantité compartiment #4
            protocol::SEPARATOR,
            b'5', // Code produit compartiment #5
            protocol::SEPARATOR,
            b'0', b'5', b'0', b'0', b'0', // Quantité compartiment #5
            protocol::SEPARATOR,
            b'6', // Code produit compartiment #6
            protocol::SEPARATOR,
            b'0', b'6', b'0', b'0', b'0', // Quantité compartiment #6
            protocol::SEPARATOR,
            b'7', // Code produit compartiment #7
            protocol::SEPARATOR,
            b'0', b'7', b'0', b'0', b'0', // Quantité compartiment #7
            protocol::SEPARATOR,
            b'8', // Code produit compartiment #8
            protocol::SEPARATOR,
            b'0', b'8', b'0', b'0', b'0', // Quantité compartiment #8
            protocol::SEPARATOR,
            b'9', // Code produit compartiment #9
            protocol::SEPARATOR,
            b'0',  b'9', b'0', b'0', b'0', // Quantité compartiment #9
            protocol::SEPARATOR,
            b'T', // Présence remorque
            protocol::SEPARATOR,
            b'1', // Codes produits tuyauterie
            b'2',
            b'3',
            b'4',
            protocol::SEPARATOR,
            b'6', // Checksum
            b'9',
            protocol::ETX,
        ]);

        // Création du protocole ST2150 avec ce FAKE port
        let mut st = ST2150::new(fake_port);

        // Le message est possible
        assert!(ST2150::message_availability(&context, MESSAGE_NUM).is_ok());

        // Vacation requête/réponse du message via le FAKE port
        assert_eq!(st.do_message_vacation(&mut context, MESSAGE_NUM), Ok(()));

        // Vérification de ce qui a été mis à jour dans le contexte
        assert_eq!(context.get_info_u8(IdInfo::NombreCompartiments), Some(9));
        for compart_num in 1..=9 {
            assert_eq!(
                context.get_info_u8(IdInfo::CodeProduitCompartiment(compart_num)),
                Some(u8::try_from(compart_num).unwrap())
            );
            assert_eq!(
                context.get_info_u32(IdInfo::QuantiteCompartiment(compart_num)),
                Some(u32::try_from(compart_num).unwrap() * 1000_u32)
            );
        }
        assert_eq!(context.get_info_bool(IdInfo::PresenceRemorque), Some(true));

        assert_eq!(context.get_info_u8(IdInfo::CodeProduitCollecteur), Some(1));
        assert_eq!(
            context.get_info_u8(IdInfo::CodeProduitPartieCommune),
            Some(2)
        );
        assert_eq!(context.get_info_u8(IdInfo::CodeProduitFlexible1), Some(3));
        assert_eq!(context.get_info_u8(IdInfo::CodeProduitFlexible2), Some(4));
    }
}
