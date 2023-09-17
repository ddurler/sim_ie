//! Helpers pour l'édition des informations du contexte

use iced::widget::{Row, Text, TextInput};

use super::{Element, Message};
use crate::context;
use context::{Context, IdInfo};

/// Affichage en édition d'un champ non défini
const STR_INPUT_INFO_NONE: &str = "";

/// Largeur harmonisée de tous les libellés en saisie
const LABEL_WIDTH: f32 = 120.0;
/// Largeur harmonisée de tous les champs de saisie
const INPUT_WIDTH: f32 = 80.0;

/// Edition IHM d'un champ `IdInfo`
pub fn input_info(context: &Context, id_info: IdInfo) -> Element<Message> {
    let row = Row::new();

    let txt = format!("{} : ", context.get_info_label(id_info));
    let txt: Text = Text::new(txt).width(LABEL_WIDTH);
    let row = row.push(txt);

    let str_place_holder = context.get_info_label(id_info);
    let str_value = context.get_info_to_string(id_info, STR_INPUT_INFO_NONE);
    let txt_input = TextInput::new(&str_place_holder, &str_value)
        .width(INPUT_WIDTH)
        .on_input(move |str| Message::InputInfo(str, id_info));
    let row = row.push(txt_input);

    row.into()
}

/// Callback IHM modification de la valeur d'un champ identifié par son `IdInfo`
pub fn callback_input_info(context: &mut Context, input: &str, id_info: IdInfo) {
    context.set_info_from_string(id_info, input);
}
