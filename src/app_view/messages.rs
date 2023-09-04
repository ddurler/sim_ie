//! Structure IHM générique pour afficher/éditer un message de l'Informatique Embarquée - ST2150

use super::Message;
use iced::Element;

pub trait CommonMessageTrait {
    /// Numéro de message
    fn message_num(&self) -> u8;

    /// Libellé (Quelques mots) décrivant le message
    fn str_message(&self) -> &'static str;

    /// View pour la partie 'requête' du message
    fn view_request(&self) -> Element<Message>;

    /// View pour la partie 'réponse' du message
    fn view_response(&self) -> Element<Message>;
}
