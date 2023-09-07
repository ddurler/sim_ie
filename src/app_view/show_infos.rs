//! Helpers pour l'affichage des informations du contexte

use iced::widget::Text;

use super::{Element, Message};
use crate::context;
use context::{Context, FormatInfo, IdInfo};

/// Visualisation IHM d'un champ booléen
fn show_info_bool(context: &Context, id_info: IdInfo) -> Element<'static, Message> {
    let txt = match context.get_info_bool(id_info) {
        None => "???",
        Some(value) => {
            if value {
                "Oui"
            } else {
                "Non"
            }
        }
    };
    let txt = format!("{} : {}", context::get_info_name(id_info), txt);
    Text::new(txt).into()
}

/// Visualisation IHM d'un champ U8
fn show_info_u8(context: &Context, id_info: IdInfo) -> Element<'static, Message> {
    let txt = match context.get_info_u8(id_info) {
        None => "???".to_string(),
        Some(value) => format!("{value}"),
    };
    let txt = format!("{} : {}", context::get_info_name(id_info), txt);
    Text::new(txt).into()
}

/// Visualisation IHM d'un champ U32
fn show_info_u32(context: &Context, id_info: IdInfo) -> Element<'static, Message> {
    let txt = match context.get_info_u32(id_info) {
        None => "???".to_string(),
        Some(value) => format!("{value}"),
    };
    let txt = format!("{} : {}", context::get_info_name(id_info), txt);
    Text::new(txt).into()
}

/// Visualisation IHM d'un champ F32
fn show_info_f32(context: &Context, id_info: IdInfo) -> Element<'static, Message> {
    let txt = match context.get_info_f32(id_info) {
        None => "???".to_string(),
        Some(value) => format!("{value:.1}"),
    };
    let txt = format!("{} : {}", context::get_info_name(id_info), txt);
    Text::new(txt).into()
}

/// Visualisation IHM de la valeur du champ `IdInfo`
pub fn show_info(context: &Context, id_info: IdInfo) -> Element<'static, Message> {
    match context::get_info_format(id_info) {
        FormatInfo::FormatBool => show_info_bool(context, id_info),
        FormatInfo::FormatU8 => show_info_u8(context, id_info),
        FormatInfo::FormatU32 => show_info_u32(context, id_info),
        FormatInfo::FormatF32 => show_info_f32(context, id_info),
    }
}
