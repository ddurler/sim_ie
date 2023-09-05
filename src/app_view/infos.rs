//! Helpers pour l'affichage et/ou l'édition des informations du contexte dans l'IHM

use iced::widget::Text;

use super::{Element, Message};
use crate::context;
use context::IdInfo;

/// Visualisation IHM de la valeur d'un champ du contexte identifié par son `IdInfo`
pub fn show_info(id_info: &IdInfo) -> Element<'static, Message> {
    let txt = format!("{:?}", id_info.clone());
    Text::new(txt).into()
}
