//! Helpers pour l'affichage des informations du contexte

use iced::widget::Text;

use super::{Element, Message};
use crate::context;
use context::{Context, FormatInfo, IdInfo};

/// Affichage d'un champ non défini
const STR_INFO_NONE: &str = "???";

/// Affichage d'un champ booléen
pub fn str_info_bool(context: &Context, id_info: IdInfo, str_none: &str) -> String {
    match context.get_info_bool(id_info) {
        None => str_none,
        Some(value) => {
            if value {
                "Oui"
            } else {
                "Non"
            }
        }
    }
    .to_string()
}

/// Affichage d'un champ U8
pub fn str_info_u8(context: &Context, id_info: IdInfo, str_none: &str) -> String {
    match context.get_info_u8(id_info) {
        None => str_none.to_string(),
        Some(value) => format!("{value}"),
    }
}

/// Affichage d'un champ U32
pub fn str_info_u32(context: &Context, id_info: IdInfo, str_none: &str) -> String {
    match context.get_info_u32(id_info) {
        None => str_none.to_string(),
        Some(value) => format!("{value}"),
    }
}

/// Affichage d'un champ F32
pub fn str_info_f32(context: &Context, id_info: IdInfo, str_none: &str) -> String {
    match context.get_info_f32(id_info) {
        None => str_none.to_string(),
        Some(value) => format!("{value:.1}"),
    }
}

/// Visualisation IHM générique de la valeur du champ `IdInfo`
/// On utilise ici une fonction qui retourne le texte à afficher pour un `IdInfo`
fn show_info_generic(
    context: &Context,
    id_info: IdInfo,
    f: fn(&Context, IdInfo, &str) -> String,
    str_none: &str,
) -> Element<'static, Message> {
    let txt = f(context, id_info, str_none);
    let txt = format!("{} : {}", context::get_info_name(id_info), txt);
    Text::new(txt).into()
}

/// Visualisation IHM de la valeur du champ `IdInfo`
pub fn show_info(context: &Context, id_info: IdInfo) -> Element<'static, Message> {
    match context::get_info_format(id_info) {
        FormatInfo::FormatBool => show_info_generic(context, id_info, str_info_bool, STR_INFO_NONE),
        FormatInfo::FormatU8 => show_info_generic(context, id_info, str_info_u8, STR_INFO_NONE),
        FormatInfo::FormatU32 => show_info_generic(context, id_info, str_info_u32, STR_INFO_NONE),
        FormatInfo::FormatF32 => show_info_generic(context, id_info, str_info_f32, STR_INFO_NONE),
    }
}
