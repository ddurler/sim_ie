//! Message 00 : Signe de vie

use crate::context::Context;

use super::frame;
use super::CommonMessageTrait;
use super::ProtocolError;
use super::ST2150;

pub struct Message00<'a> {
    /// Structure ST2150
    st2150: &'a mut ST2150<'a>,
}

impl<'a> Message00<'a> {
    /// Constructeur
    pub fn new(st2150: &'a mut ST2150<'a>) -> Self {
        Self { st2150 }
    }
}

impl<'a> CommonMessageTrait for Message00<'a> {
    fn availability(_context: &Context) -> Result<(), ProtocolError> {
        // Toujours possible car pas d'information dans la requête
        Ok(())
    }

    fn do_vacation(&mut self) -> Result<(), ProtocolError> {
        // Contexte OK ?
        Message00::availability(self.st2150.context)?;

        // Création et envoi requête
        let req = frame::Frame::new(0);
        self.st2150.send_req(&req);

        // Réception réponse
        let mut buffer = [0; 200];
        let len_rep = self.st2150.wait_rep(&mut buffer, 17)?;

        // Décodage de la réponse reçue
        let frame = self
            .st2150
            .try_from_buffer(&buffer[..len_rep], 0, &[1, 1, 1, 1, 1])?;

        // Mise à jour du contexte

        // #0 : En mesurage
        match frame.fields[0].decode_char()? {
            '0' => self.st2150.context.en_mesurage = Some(false),
            '1' => self.st2150.context.en_mesurage = Some(true),
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
            self.st2150.context.code_defaut = Some(code_defaut - 0x20);
        } else {
            return Err(ProtocolError::IllegalFieldValue(
                frame.fields[1].clone(),
                "code défaut".to_string(),
                "Valeur entre 0x20 et 0x9F".to_string(),
            ));
        }

        // #2 : Arrêt intermédiaire
        match frame.fields[2].decode_char()? {
            '0' => self.st2150.context.arret_intermediaire = Some(false),
            '1' => self.st2150.context.arret_intermediaire = Some(true),
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
            '0' => self.st2150.context.forcage_petit_debit = Some(false),
            '1' => self.st2150.context.forcage_petit_debit = Some(true),
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
            '0' => self.st2150.context.mode_connecte = Some(false),
            '1' => self.st2150.context.mode_connecte = Some(true),
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

        // Trame pour message 00 (le checksum est 0xFE)
        fake_port.should_write(&[
            protocol::STX,
            0x30,
            0x30,
            protocol::SEPARATOR,
            0x46,
            0x45,
            protocol::ETX,
        ]);

        // Réponse simulée (le checksum est 0x20)
        fake_port.will_read(&[
            protocol::STX,
            0x30,
            0x30,
            protocol::SEPARATOR,
            0x30, // Hors mesurage
            protocol::SEPARATOR,
            0x20, // Pas de défaut
            protocol::SEPARATOR,
            0x30, // Pas en arrêt intermédiaire
            protocol::SEPARATOR,
            0x30, // Pas de forçage PD
            protocol::SEPARATOR,
            0x30, // En mode autonome
            protocol::SEPARATOR,
            b'2',
            b'0',
            protocol::ETX,
        ]);

        // Création du protocol ST2150 avec ce FAKE port
        let mut context = Context::default();
        let mut st = ST2150::new(fake_port, &mut context);

        // Le message 00 est possible
        assert!(st.message_availability(0).is_ok());

        // Vacation requête/réponse du message 00 via le FAKE port
        assert_eq!(st.message00(), Ok(()));

        // Vérification de ce qui a été mis à jour dans le contexte
        assert_eq!(context.en_mesurage, Some(false));
        assert_eq!(context.code_defaut, Some(0));
        assert_eq!(context.arret_intermediaire, Some(false));
        assert_eq!(context.forcage_petit_debit, Some(false));
        assert_eq!(context.mode_connecte, Some(false));
    }
}
