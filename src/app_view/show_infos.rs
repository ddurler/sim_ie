//! Helpers pour l'affichage des informations du contexte

use iced::widget::Text;

use super::{Element, Message};
use crate::context;
use context::{Context, IdInfo};

/// Affichage d'un champ non défini
const STR_INFO_NONE: &str = "???";

/// Visualisation IHM de la valeur du champ `IdInfo`
pub fn show_info(context: &Context, id_info: IdInfo) -> Element<'static, Message> {
    let txt = format!(
        "{} : {}",
        context.get_info_label(id_info),
        context.get_info_to_string(id_info, STR_INFO_NONE)
    );
    Text::new(txt).into()
}
