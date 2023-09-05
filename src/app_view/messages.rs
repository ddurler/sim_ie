//! Structure IHM générique pour afficher/éditer un message de l'Informatique Embarquée - ST2150

use crate::context::IdInfo;

pub trait CommonMessageTrait {
    /// Numéro de message
    fn message_num(&self) -> u8;

    /// Libellé (Quelques mots) décrivant le message
    fn str_message(&self) -> &'static str;

    /// Informations contexte nécessaire pour la 'requête' du message
    fn id_infos_request(&self) -> Vec<IdInfo>;

    /// Informations contexte nécessaire pour la 'réponse' du message
    fn id_infos_response(&self) -> Vec<IdInfo>;
}
