//! Façade pour tous les messages du protocole Informatique Embarqué - ST2150

use super::frame;
use super::Context;
use super::ProtocolError;
use super::ST2150;

pub mod message00;

/// Trait à implémenter pour chaque type de message
pub trait CommonMessageTrait {
    /// Indique si le contexte permet d'effectuer une requête avec ce message
    /// (note: pas de `self` dans cette fonction)
    /// Retourne `Ok(())` ou `Err(ProtocolError::ContextMissing)`
    fn availability(context: &Context) -> Result<(), ProtocolError>;

    /// Tente une vacation avec ce message
    fn do_vacation(&mut self) -> Result<(), ProtocolError>;
}
