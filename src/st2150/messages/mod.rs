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
//     Les messages pour les mouvements de produit (60-79) ont une construction factorisée
// 3 - Ajout pub messageXX et use messageXX::MessageXX ci-dessous
// 4 - Ajout MessageXX dans la primitive `get_dyn_message` ci-dessous
// C'est tout...

pub mod message00;
use message00::Message00;
pub mod message10;
use message10::Message10;
pub mod message11;
use message11::Message11;
pub mod message20;
use message20::Message20;
pub mod message21;
use message21::Message21;
pub mod message22;
use message22::Message22;
pub mod message30;
use message30::Message30;
pub mod message31;
use message31::Message31;
pub mod message32;
use message32::Message32;
pub mod message33;
use message33::Message33;
pub mod message34;
use message34::Message34;
pub mod message35;
use message35::Message35;
pub mod message36;
use message36::Message36;
pub mod message37;
use message37::Message37;
pub mod message38;
use message38::Message38;
pub mod message40;
use message40::Message40;
pub mod helper_messages60_79;
pub mod message60;
use message60::Message60;
pub mod message61;
use message61::Message61;
pub mod message62;
use message62::Message62;
pub mod message63;
use message63::Message63;
// Message64 réservé
pub mod message65;
use message65::Message65;
pub mod message66;
use message66::Message66;
pub mod message67;
use message67::Message67;
// Message68-69 réservés
pub mod message70;
use message70::Message70;
pub mod message71;
use message71::Message71;
// Messages72-73-74 réservés
pub mod message75;
use message75::Message75;
pub mod message76;
use message76::Message76;
pub mod message77;
use message77::Message77;
pub mod message78;
use message78::Message78;
// Messages79 réservé

use super::field;
use super::Edition2150;

/// Liste des numéros de messages implémentés
pub const ST2150_MESSAGE_NUMBERS: &[u8] = &[
    0, 10, 11, 20, 21, 22, 30, 31, 32, 33, 34, 35, 36, 37, 38, 40, 60, 61, 62, 63, 65, 66, 67, 70,
    71, 75, 76, 77, 78,
];

/// Accès au `CommonMessageTrait` des différents messages gérés
pub fn get_dyn_message(message_num: u8) -> Box<dyn CommonMessageTrait> {
    match message_num {
        0 => Box::<Message00>::default(),
        10 => Box::<Message10>::default(),
        11 => Box::<Message11>::default(),
        20 => Box::<Message20>::default(),
        21 => Box::<Message21>::default(),
        22 => Box::<Message22>::default(),
        30 => Box::<Message30>::default(),
        31 => Box::<Message31>::default(),
        32 => Box::<Message32>::default(),
        33 => Box::<Message33>::default(),
        34 => Box::<Message34>::default(),
        35 => Box::<Message35>::default(),
        36 => Box::<Message36>::default(),
        37 => Box::<Message37>::default(),
        38 => Box::<Message38>::default(),
        40 => Box::<Message40>::default(),
        60 => Box::<Message60>::default(),
        61 => Box::<Message61>::default(),
        62 => Box::<Message62>::default(),
        63 => Box::<Message63>::default(),
        65 => Box::<Message65>::default(),
        66 => Box::<Message66>::default(),
        67 => Box::<Message67>::default(),
        70 => Box::<Message70>::default(),
        71 => Box::<Message71>::default(),
        75 => Box::<Message75>::default(),
        76 => Box::<Message76>::default(),
        77 => Box::<Message77>::default(),
        78 => Box::<Message78>::default(),

        _ => panic!("Numéro de message non géré {message_num}"),
    }
}

/// Trait à implémenter pour chaque type de message
/// Les structures `MessageXX` doivent implémenter le `Default` trait
pub trait CommonMessageTrait {
    /// Numéro de message
    fn message_num(&self) -> u8;

    /// Edition de la ST2150
    fn edition_st2150(&self) -> Edition2150;

    /// Libellé (Quelques mots) décrivant le message
    fn message_str(&self) -> &'static str;

    /// Id des informations contexte nécessaire pour la 'requête' du message
    fn id_infos_request(&self) -> Vec<IdInfo>;

    /// Id des informations contexte nécessaire pour la 'réponse' du message
    fn id_infos_response(&self) -> Vec<IdInfo>;

    /// Indique si le contexte permet d'effectuer une requête avec ce message
    /// (note: pas de `self` dans cette fonction)
    fn availability(&self, context: &Context) -> Result<(), ProtocolError> {
        for id_info in self.id_infos_request() {
            let info_name = context.get_info_label(id_info);
            if match context.get_info_format(id_info) {
                context::FormatInfo::Bool => context.get_option_info_bool(id_info).is_none(),
                context::FormatInfo::Char => context.get_option_info_char(id_info).is_none(),
                context::FormatInfo::U8 => context.get_option_info_u8(id_info).is_none(),
                context::FormatInfo::U8OrT => context.get_option_info_u8_or_t(id_info).is_none(),
                context::FormatInfo::U16 => context.get_option_info_u16(id_info).is_none(),
                context::FormatInfo::U32 => context.get_option_info_u32(id_info).is_none(),
                context::FormatInfo::U64 => context.get_option_info_u64(id_info).is_none(),
                context::FormatInfo::F32 => context.get_option_info_f32(id_info).is_none(),
                context::FormatInfo::String(_width) => {
                    context.get_option_info_string(id_info).is_none()
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
