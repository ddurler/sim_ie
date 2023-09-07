//! Helpers pour l'édition des informations du contexte

use iced::widget::{Row, Text, TextInput};

use super::{Element, Message};
use crate::context;
use context::{Context, FormatInfo, IdInfo};

/// Edition IHM d'un champ U8
fn input_info_u8(context: &Context, id_info: IdInfo) -> Element<Message> {
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
        .on_input(move |str| Message::InputInfo(str, id_info));
    let row = row.push(txt_input);

    row.into()
}

/// Callback IHM modification de la valeur d'un champ U8
fn callback_input_info_u8(context: &mut Context, input: &str, id_info: IdInfo) {
    match input.parse::<u8>() {
        Ok(value) => context.set_info_u8(id_info, value),
        Err(_e) => (),
    };
}

/// Edition IHM de la valeur d'un champ identifié par son `IdInfo`
pub fn input_info(context: &Context, id_info: IdInfo) -> Element<Message> {
    match context::get_info_format(id_info) {
        // FormatInfo::FormatU32 => input_info_u32(context, id_info),
        FormatInfo::FormatU8 => input_info_u8(context, id_info),
        // FormatInfo::FormatBool => input_info_bool(context, id_info),
        // FormatInfo::FormatF32 => show_info_f32(context, id_info),
        _ => todo!(),
    }
}

/// Callback IHM modification de la valeur d'un champ identifié par son `IdInfo`
pub fn callback_input_info(context: &mut Context, input: &str, id_info: IdInfo) {
    match context::get_info_format(id_info) {
        // FormatInfo::FormatU32 => input_info_u32(context, id_info),
        FormatInfo::FormatU8 => callback_input_info_u8(context, input, id_info),
        // FormatInfo::FormatBool => input_info_bool(context, id_info),
        // FormatInfo::FormatF32 => show_info_f32(context, id_info),
        _ => todo!(),
    }
}
