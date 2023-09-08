//! Façade pour tous les messages du protocole Informatique Embarqué - ST2150

use super::context;
use super::context::Context;
use super::frame;
use super::IdInfo;
use super::ProtocolError;
use super::ST2150;

// Pour implémenter un nouveau message XX, il suffit de :

// 1 - Mettre à jour la liste des numéros de messages implémentés `ST2150_MESSAGE_NUMBERS`
// 2 - implémenter un nouveau module messageXX.rs à l'image de ceux déjà existants
// 3 - Ajout pub messageXX et use messageXX::MessageXX ci-dessous
// 4 - Ajout MessageXX dans la primitive `get_dyn_message` ci-dessous
// C'est tout...

/// Liste des numéros de messages implémentés
pub const ST2150_MESSAGE_NUMBERS: &[u8] = &[0_u8, 10_u8, 20_u8];

pub mod message00;
use message00::Message00;
pub mod message10;
use message10::Message10;
pub mod message20;
use message20::Message20;

use super::field;

/// Accès au `CommonMessageTrait` des différents messages gérés
pub fn get_dyn_message(message_num: u8) -> Box<dyn CommonMessageTrait> {
    match message_num {
        0 => Box::<Message00>::default(),
        10 => Box::<Message10>::default(),
        20 => Box::<Message20>::default(),
        _ => panic!("Numéro de message non géré {message_num}"),
    }
}

/// Trait à implémenter pour chaque type de message
/// Les structures `MessageXX` doivent implémenter le `Default` trait
pub trait CommonMessageTrait {
    /// Numéro de message
    fn message_num(&self) -> u8;

    /// Libellé (Quelques mots) décrivant le message
    fn str_message(&self) -> &'static str;

    /// Id des informations contexte nécessaire pour la 'requête' du message
    fn id_infos_request(&self) -> Vec<IdInfo>;

    /// Id des informations contexte nécessaire pour la 'réponse' du message
    fn id_infos_response(&self) -> Vec<IdInfo>;

    /// Indique si le contexte permet d'effectuer une requête avec ce message
    /// (note: pas de `self` dans cette fonction)
    fn availability(&self, context: &Context) -> Result<(), ProtocolError> {
        for id_info in self.id_infos_request() {
            let info_name = context::get_info_name(id_info);
            if match context::get_info_format(id_info) {
                context::FormatInfo::FormatBool => context.get_info_bool(id_info).is_none(),
                context::FormatInfo::FormatU8 => context.get_info_u8(id_info).is_none(),
                context::FormatInfo::FormatU32 => context.get_info_u32(id_info).is_none(),
                context::FormatInfo::FormatF32 => context.get_info_f32(id_info).is_none(),
                context::FormatInfo::FormatString(_width) => {
                    context.get_info_string(id_info).is_none()
                }
            } {
                return Err(ProtocolError::ContextMissing(info_name.to_string()));
            }
        }

        Ok(())
    }

    /// Tente une vacation sur un port avec un contexte de ce message
    /// (note: pas de `self` dans cette fonction)
    fn do_vacation(&self, st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError>;
}
