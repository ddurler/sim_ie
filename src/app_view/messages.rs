//! Structure IHM générique pour afficher/éditer un message de l'Informatique Embarquée - ST2150

pub trait CommonMessageTrait {
    /// Numéro de message
    fn message_num(&self) -> u8;

    /// Libellé (Quelques mots) décrivant le message
    fn str_message(&self) -> &'static str;
}
