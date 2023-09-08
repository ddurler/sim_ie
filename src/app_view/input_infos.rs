//! Helpers pour l'édition des informations du contexte

use iced::widget::{Row, Text, TextInput};

use super::show_infos;
use super::{Element, Message};
use crate::context;
use context::{Context, FormatInfo, IdInfo};

/// Affichage en édition d'un champ non défini
const STR_INPUT_INFO_NONE: &str = "";

/// Largeur harmonisée de tous les libellés en saisie
const LABEL_WIDTH: f32 = 100.0;
/// Largeur harmonisée de tous les champs de saisie
const INPUT_WIDTH: f32 = 100.0;

/// Edition IHM d'un champ bool
fn input_info_bool(context: &Context, id_info: IdInfo) -> Element<Message> {
    let row = Row::new();

    let txt = format!("{} : ", context::get_info_name(id_info));
    let txt: Text = Text::new(txt).width(LABEL_WIDTH);
    let row = row.push(txt);

    let str_place_holder = crate::context::get_info_name(id_info);
    let str_value = &show_infos::str_info_bool(context, id_info, STR_INPUT_INFO_NONE);
    let txt_input = TextInput::new(str_place_holder, str_value)
        .width(INPUT_WIDTH)
        .on_input(move |str| Message::InputInfo(str, id_info));
    let row = row.push(txt_input);

    row.into()
}

/// Callback IHM modification de la valeur d'un champ bool
fn callback_input_info_bool(context: &mut Context, input: &str, id_info: IdInfo) {
    let value = !input.is_empty() && ['o', 'O', '1'].contains(&input.chars().next().unwrap());
    context.set_info_bool(id_info, value);
}

/// Edition IHM d'un champ U8
fn input_info_u8(context: &Context, id_info: IdInfo) -> Element<Message> {
    let row = Row::new();

    let txt = format!("{} : ", context::get_info_name(id_info));
    let txt: Text = Text::new(txt).width(LABEL_WIDTH);
    let row = row.push(txt);

    let str_place_holder = crate::context::get_info_name(id_info);
    let str_value = show_infos::str_info_u8(context, id_info, STR_INPUT_INFO_NONE);
    let txt_input = TextInput::new(str_place_holder, &str_value)
        .width(INPUT_WIDTH)
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

/// Edition IHM d'un champ U32
fn input_info_u32(context: &Context, id_info: IdInfo) -> Element<Message> {
    let row = Row::new();

    let txt = format!("{} : ", context::get_info_name(id_info));
    let txt: Text = Text::new(txt).width(LABEL_WIDTH);
    let row = row.push(txt);

    let str_place_holder = crate::context::get_info_name(id_info);
    let str_value = show_infos::str_info_u32(context, id_info, STR_INPUT_INFO_NONE);
    let txt_input = TextInput::new(str_place_holder, &str_value)
        .width(INPUT_WIDTH)
        .on_input(move |str| Message::InputInfo(str, id_info));
    let row = row.push(txt_input);

    row.into()
}

/// Callback IHM modification de la valeur d'un champ U32
fn callback_input_info_u32(context: &mut Context, input: &str, id_info: IdInfo) {
    match input.parse::<u32>() {
        Ok(value) => context.set_info_u32(id_info, value),
        Err(_e) => (),
    };
}

/// Edition IHM d'un champ F32
fn input_info_f32(context: &Context, id_info: IdInfo) -> Element<Message> {
    let row = Row::new();

    let txt = format!("{} : ", context::get_info_name(id_info));
    let txt: Text = Text::new(txt).width(LABEL_WIDTH);
    let row = row.push(txt);

    let str_place_holder = crate::context::get_info_name(id_info);
    let str_value = show_infos::str_info_f32(context, id_info, STR_INPUT_INFO_NONE);
    let txt_input = TextInput::new(str_place_holder, &str_value)
        .width(INPUT_WIDTH)
        .on_input(move |str| Message::InputInfo(str, id_info));
    let row = row.push(txt_input);

    row.into()
}

/// Callback IHM modification de la valeur d'un champ F32
fn callback_input_info_f32(context: &mut Context, input: &str, id_info: IdInfo) {
    match input.parse::<f32>() {
        Ok(value) => context.set_info_f32(id_info, value),
        Err(_e) => (),
    };
}

/// Edition IHM de la valeur d'un champ identifié par son `IdInfo`
pub fn input_info(context: &Context, id_info: IdInfo) -> Element<Message> {
    match context::get_info_format(id_info) {
        FormatInfo::FormatBool => input_info_bool(context, id_info),
        FormatInfo::FormatU8 => input_info_u8(context, id_info),
        FormatInfo::FormatU32 => input_info_u32(context, id_info),
        FormatInfo::FormatF32 => input_info_f32(context, id_info),
    }
}

/// Callback IHM modification de la valeur d'un champ identifié par son `IdInfo`
pub fn callback_input_info(context: &mut Context, input: &str, id_info: IdInfo) {
    match context::get_info_format(id_info) {
        FormatInfo::FormatBool => callback_input_info_bool(context, input, id_info),
        FormatInfo::FormatU8 => callback_input_info_u8(context, input, id_info),
        FormatInfo::FormatU32 => callback_input_info_u32(context, input, id_info),
        FormatInfo::FormatF32 => callback_input_info_f32(context, input, id_info),
    }
}
