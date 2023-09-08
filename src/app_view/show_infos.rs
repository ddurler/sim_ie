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

/// Affichage d'un champ `IdInfo`
pub fn str_info(context: &Context, id_info: IdInfo, str_none: &str) -> String {
    match context::get_info_format(id_info) {
        FormatInfo::FormatBool => str_info_bool(context, id_info, str_none),
        FormatInfo::FormatU8 => str_info_u8(context, id_info, str_none),
        FormatInfo::FormatU32 => str_info_u32(context, id_info, str_none),
        FormatInfo::FormatF32 => str_info_f32(context, id_info, str_none),
    }
}

/// Visualisation IHM de la valeur du champ `IdInfo`
pub fn show_info(context: &Context, id_info: IdInfo) -> Element<'static, Message> {
    let txt = str_info(context, id_info, STR_INFO_NONE);
    let txt = format!("{} : {}", context::get_info_name(id_info), txt);
    Text::new(txt).into()
}
