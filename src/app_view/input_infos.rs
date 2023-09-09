//! Helpers pour l'édition des informations du contexte

use iced::widget::{Row, Text, TextInput};

use super::show_infos;
use super::{Element, Message};
use crate::context;
use context::{CommonContextTrait, Context, FormatInfo, IdInfo};

/// Affichage en édition d'un champ non défini
const STR_INPUT_INFO_NONE: &str = "";

/// Largeur harmonisée de tous les libellés en saisie
const LABEL_WIDTH: f32 = 100.0;
/// Largeur harmonisée de tous les champs de saisie
const INPUT_WIDTH: f32 = 100.0;

/// Edition IHM d'un champ `IdInfo`
pub fn input_info(context: &Context, id_info: IdInfo) -> Element<Message> {
    let row = Row::new();

    let txt = format!("{} : ", context::get_info_name(id_info));
    let txt: Text = Text::new(txt).width(LABEL_WIDTH);
    let row = row.push(txt);

    let str_place_holder = crate::context::get_info_name(id_info);
    let str_value = show_infos::str_info(context, id_info, STR_INPUT_INFO_NONE);
    let txt_input = TextInput::new(&str_place_holder, &str_value)
        .width(INPUT_WIDTH)
        .on_input(move |str| Message::InputInfo(str, id_info));
    let row = row.push(txt_input);

    row.into()
}

/// Callback IHM modification de la valeur d'un champ bool
/// Ce callback est différent des callback's génériques pour des nombres car on
/// interprète ici la saisie de l'utilisateur (pas de parse)
fn callback_input_info_bool(context: &mut Context, input: &str, id_info: IdInfo) {
    let value = !input.is_empty() && ['o', 'O', '1'].contains(&input.chars().next().unwrap());
    context.set_info_bool(id_info, value);
}

/// Callback IHM modification de la valeur d'un champ String
fn callback_input_info_string(context: &mut Context, input: &str, id_info: IdInfo, width: usize) {
    let input = input.trim_end();
    let value = if input.len() > width {
        // Tronque si trop long
        // /!\ format! ne le fait pas...
        input[..width].to_string()
    } else {
        input.to_string()
    };
    context.set_info_string(id_info, &value);
}

/// Callback IHM modification de la valeur d'un champ numérique (U8, U32, F32, etc...)
/// (Le type `<T>` doit être également géré par `context.set_info`)
fn callback_input_info_generic<T>(context: &mut Context, input: &str, id_info: IdInfo)
where
    T: std::str::FromStr,
    context::Context: context::CommonContextTrait<T>,
{
    match input.parse::<T>() {
        Ok(value) => context.set_info(id_info, value),
        Err(_e) => (),
    };
}

/// Callback IHM modification de la valeur d'un champ identifié par son `IdInfo`
pub fn callback_input_info(context: &mut Context, input: &str, id_info: IdInfo) {
    match context::get_info_format(id_info) {
        FormatInfo::FormatBool => callback_input_info_bool(context, input, id_info),
        FormatInfo::FormatU8 => callback_input_info_generic::<u8>(context, input, id_info),
        FormatInfo::FormatU16 => callback_input_info_generic::<u16>(context, input, id_info),
        FormatInfo::FormatU32 => callback_input_info_generic::<u32>(context, input, id_info),
        FormatInfo::FormatF32 => callback_input_info_generic::<f32>(context, input, id_info),
        FormatInfo::FormatString(width) => {
            callback_input_info_string(context, input, id_info, width);
        }
    }
}
