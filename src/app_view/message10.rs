//! Implémentation IHM pour le message 10

use iced::widget::Text;
use iced::Element;

use super::messages::CommonMessageTrait;
use super::Message;

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

    fn view_request(&self) -> Element<Message> {
        Text::new("(Pas de champ pour cette requête)").into()
    }

    fn view_response(&self) -> Element<Message> {
        Text::new("(TODO message 10)").into()
    }
}
