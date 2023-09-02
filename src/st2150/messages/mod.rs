//! Façade pour tous les messages du protocole Informatique Embarqué - ST2150

use super::frame;
use super::Context;
use super::ProtocolError;
use super::ST2150;

pub mod message00;
pub mod message10;

/// Trait à implémenter pour chaque type de message
/// Les structures `MessageXX` doivent implémenter le `Default` trait
pub trait CommonMessageTrait {
    /// Indique si le contexte permet d'effectuer une requête avec ce message
    /// (note: pas de `self` dans cette fonction)
    fn availability(context: &Context) -> Result<(), ProtocolError>;

    /// Tente une vacation sur un port avec un contexte de ce message
    /// (note: pas de `self` dans cette fonction)
    fn do_vacation(st2150: &mut ST2150, context: &mut Context) -> Result<(), ProtocolError>;
}
