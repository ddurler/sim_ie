//! Implémentation IHM pour le message 00

use super::messages::CommonMessageTrait;
use crate::context::IdInfo;

/// Numéro de ce message
const MESSAGE_NUM: u8 = 0;

/// Structure pour ce message
#[derive(Default)]
pub struct Message00 {}

impl CommonMessageTrait for Message00 {
    fn message_num(&self) -> u8 {
        MESSAGE_NUM
    }

    fn str_message(&self) -> &'static str {
        "Signe de vie"
    }

    fn id_infos_request(&self) -> Vec<IdInfo> {
        vec![]
    }

    fn id_infos_response(&self) -> Vec<IdInfo> {
        vec![
            IdInfo::EnMesurage,
            IdInfo::CodeDefaut,
            IdInfo::ArretIntermediaire,
            IdInfo::ForcagePetitDebit,
            IdInfo::ModeConnecte,
        ]
    }
}
