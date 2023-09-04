//! Implémentation IHM pour le message 10

use super::messages::CommonMessageTrait;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 10;

/// Structure pour ce message
#[derive(Default)]
pub struct Message10 {}

impl CommonMessageTrait for Message10 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn str_message(&self) -> &'static str {
        "Informations instantanées"
    }
}
