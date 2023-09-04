//! Implémentation IHM pour le message 00

use super::messages::CommonMessageTrait;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 0;

/// Structure pour ce message
#[derive(Default)]
pub struct Message00 {}

impl CommonMessageTrait for Message00 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn str_message(&self) -> &'static str {
        "Signe de vie"
    }
}
