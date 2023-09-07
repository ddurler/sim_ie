//! Helpers pour l'édition des informations du contexte dans l'IHM

use iced::widget::{Row, Text, TextInput};

use super::{Element, Message};
use crate::context;
use context::{Context, FormatInfo, IdInfo};

/// Edition IHM d'un champ U8 du contexte
fn input_info_u8<'a>(context: &'a Context, id_info: &'a IdInfo) -> Element<'a, Message> {
    let row = Row::new();

    let txt = format!("{} : ", context::get_info_name(id_info));
    let txt: Text = Text::new(txt);
    let row = row.push(txt);

    let str_place_holder = crate::context::get_info_name(id_info);
    let option_value = context.get_info_u8(id_info);
    let txt_value = match option_value {
        None => String::new(),
        Some(value) => format!("{value}"),
    };
    let txt_input = TextInput::new(str_place_holder, &txt_value)
        .width(100)
        .on_input(|str| Message::InputInfo(str, id_info.clone()));
    let row = row.push(txt_input);

    row.into()
}

// Edition IHM de la valeur d'un champ du contexte identifié par son `IdInfo`
pub fn input_info<'a>(context: &'a Context, id_info: &'a IdInfo) -> Element<'a, Message> {
    match context::get_info_format(id_info) {
        // FormatInfo::FormatU32 => input_info_u32(context, id_info),
        FormatInfo::FormatU8 => input_info_u8(context, id_info),
        // FormatInfo::FormatBool => input_info_bool(context, id_info),
        // FormatInfo::FormatF32 => show_info_f32(context, id_info),
        _ => todo!(),
    }
}
