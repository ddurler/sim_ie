//! Implémentation IHM pour le message 10

use super::messages::CommonMessageTrait;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 10;

/// Structure pour ce message
#[derive(Default)]
pub struct Message10 {}

impl CommonMessageTrait for Message10 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn str_message(&self) -> &'static str {
        "Informations instantanées"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![
            IdInfo::Totalisateur,
            IdInfo::DebitInstant,
            IdInfo::QuantiteChargee,
            IdInfo::TemperatureInstant,
            IdInfo::Predetermination,
        ]
    }
}
