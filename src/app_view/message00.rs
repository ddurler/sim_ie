//! Implémentation IHM pour le message 00

use iced::widget::Text;
use iced::Element;

use super::messages::CommonMessageTrait;
use super::Message;

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

    fn view_request(&self) -> Element<Message> {
        Text::new("(Pas de champ pour cette requête)").into()
    }

    fn view_response(&self) -> Element<Message> {
        Text::new("(TODO message 00)").into()
    }
}
