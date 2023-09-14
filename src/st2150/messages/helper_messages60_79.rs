//! Helper pour tous les les messages de mouvement de produit - Message 60 à 79
//!
//! Ce helper simplifie la gestion des messages de mouvement de produit et prend à sa charge
//! les traitements communs ou très similaires pour les messages de mouvement de produit.
//!
//! De fait, toute la définition de ses messages est factorisée dans ce module.

use super::field::Field;
use super::frame::Frame;
use super::Edition2150;
use super::IdInfo;
use crate::context;
use crate::context::Context;
use crate::st2150::protocol;
use crate::st2150::ProtocolError;

/// Configuration des différents messages pour un mouvement de produit
struct MessageDefinition {
    message_str: &'static str,
    id_infos_request: Vec<IdInfo>,
}

/// Getter configuration d'un message de mouvement de produit
fn definition_message(message_num: u8) -> MessageDefinition {
    match message_num {
        60 => MessageDefinition {
            message_str: "Prédétermination pompée",
            id_infos_request: vec![
                IdInfo::Predetermination,
                IdInfo::CodeProduit,
                IdInfo::NumeroCompartiment,
                IdInfo::NumeroFlexible,
                IdInfo::FinirFlexibleVide,
            ],
        },
        61 => MessageDefinition {
            message_str: "Prédétermination pompée multi-compartiments",
            id_infos_request: vec![
                IdInfo::Predetermination,
                IdInfo::CodeProduit,
                IdInfo::OrdreCompartiments,
                IdInfo::NumeroFlexible,
                IdInfo::FinirFlexibleVide,
            ],
        },
        62 => MessageDefinition {
            message_str: "Prédétermination pompée libre",
            id_infos_request: vec![
                IdInfo::CodeProduit,
                IdInfo::NumeroCompartiment,
                IdInfo::NumeroFlexible,
            ],
        },
        63 => MessageDefinition {
            message_str: "Prédétermination pompée libre multi-compartiments",
            id_infos_request: vec![
                IdInfo::CodeProduit,
                IdInfo::OrdreCompartiments,
                IdInfo::NumeroFlexible,
            ],
        },
        65 => MessageDefinition {
            message_str: "Purge",
            id_infos_request: vec![
                IdInfo::CodeProduit,
                IdInfo::NumeroCompartiment,
                IdInfo::NumeroCompartimentFinal,
                IdInfo::NumeroFlexible,
                IdInfo::NumeroFlexibleFinal,
                IdInfo::FinirFlexibleVide,
            ],
        },
        66 => MessageDefinition {
            message_str: "Prédétermination avec anticipation de purge",
            id_infos_request: vec![
                IdInfo::Predetermination,
                IdInfo::CodeProduit,
                IdInfo::CodeProduitFinal,
                IdInfo::NumeroCompartiment,
                IdInfo::NumeroCompartimentFinal,
                IdInfo::NumeroFlexible,
                IdInfo::NumeroFlexibleFinal,
                IdInfo::FinirFlexibleVide,
            ],
        },

        _ => panic!("Message de mouvement produit inconnu : {message_num}"),
    }
}

/// Liste des éditions de la ST2150 pour chaque requête d'un mouvement de produit
pub fn edition_st2150(_message_num: u8) -> Edition2150 {
    Edition2150::C
}

/// Liste des libellés de chaque requête d'un mouvement de produit
pub fn message_str(message_num: u8) -> &'static str {
    definition_message(message_num).message_str
}

/// Liste des informations nécessaires pour chaque requête d'un mouvement de produit
pub fn id_infos_request(message_num: u8) -> Vec<IdInfo> {
    definition_message(message_num).id_infos_request
}

/// Liste des infos dans la réponse à un message mouvement de produit
pub fn id_infos_response(_message_num: u8) -> Vec<IdInfo> {
    vec![
        IdInfo::Ack,
        IdInfo::Nack,
        IdInfo::CodeErreurMouvementProduit,
    ]
}

/// Création de la trame pour la requête
pub fn create_frame_request(message_num: u8, context: &Context) -> Result<Frame, ProtocolError> {
    let mut req = Frame::new(message_num);

    for id_info in id_infos_request(message_num) {
        match id_info {
            IdInfo::Predetermination => {
                let prede = context.get_info_u32(IdInfo::Predetermination).unwrap();
                req.add_field(Field::encode_number(prede, 5)?);
            }
            IdInfo::CodeProduit => {
                let code_prod = context.get_info_u8(IdInfo::CodeProduit).unwrap();
                Field::check_binary_domain(
                    "code produit",
                    code_prod,
                    0_u8..=u8::try_from(context::NB_PRODUITS).unwrap(),
                )?;
                req.add_field(Field::encode_binary(code_prod + b'0'));
            }
            IdInfo::CodeProduitFinal => {
                let code_prod = context.get_info_u8(IdInfo::CodeProduitFinal).unwrap();
                Field::check_binary_domain(
                    "code produit",
                    code_prod,
                    0_u8..=u8::try_from(context::NB_PRODUITS).unwrap(),
                )?;
                req.add_field(Field::encode_binary(code_prod + b'0'));
            }
            IdInfo::NumeroCompartiment => {
                let compart_num = context.get_info_u8(IdInfo::NumeroCompartiment).unwrap();
                Field::check_binary_domain(
                    "numéro compartiment",
                    compart_num,
                    0_u8..=u8::try_from(context::NB_COMPARTIMENTS).unwrap(),
                )?;
                req.add_field(Field::encode_binary(compart_num + b'0'));
            }
            IdInfo::NumeroCompartimentFinal => {
                let compart_num = context
                    .get_info_u8(IdInfo::NumeroCompartimentFinal)
                    .unwrap();
                Field::check_binary_domain(
                    "numéro compartiment",
                    compart_num,
                    0_u8..=u8::try_from(context::NB_COMPARTIMENTS).unwrap(),
                )?;
                req.add_field(Field::encode_binary(compart_num + b'0'));
            }
            IdInfo::OrdreCompartiments => {
                let compart_order = context.get_info_u32(IdInfo::OrdreCompartiments).unwrap();
                req.add_field(Field::encode_number(compart_order, 9)?);
            }
            IdInfo::NumeroFlexible => {
                let flexible_num = context.get_info_u8(IdInfo::NumeroFlexible).unwrap();
                Field::check_binary_domain(
                    "numéro flexible",
                    flexible_num,
                    0_u8..=u8::try_from(context::NB_FLEXIBLES).unwrap(),
                )?;
                req.add_field(Field::encode_binary(flexible_num + b'0'));
            }
            IdInfo::NumeroFlexibleFinal => {
                let flexible_num = context.get_info_u8(IdInfo::NumeroFlexibleFinal).unwrap();
                Field::check_binary_domain(
                    "numéro flexible",
                    flexible_num,
                    0_u8..=u8::try_from(context::NB_FLEXIBLES).unwrap(),
                )?;
                req.add_field(Field::encode_binary(flexible_num + b'0'));
            }
            IdInfo::FinirFlexibleVide => {
                let finir_vide = context.get_info_bool(IdInfo::FinirFlexibleVide).unwrap();
                let finir_vide = if finir_vide { 'V' } else { '0' };
                req.add_field(Field::encode_char(finir_vide)?);
            }
            _ => {
                panic!("IdInfo {id_info:?} n'est pas valide pour une requête mouvement de produit");
            }
        }
    }

    Ok(req)
}

/// Longueur max du message de réponse à un message de mouvement de produit
pub fn max_expected_rep_len(_message_num: u8) -> usize {
    12
}

/// Longueur des différents champs dans la réponse à un message de mouvement de produit
pub fn rep_len_fields(_message_num: u8) -> &'static [usize] {
    &[1, 2]
}

/// Mise à jour du contexte selon la réponse reçue à un message de mouvement de produit
pub fn update_context_from_rep(
    _message_num: u8,
    context: &mut Context,
    frame: &Frame,
) -> Result<(), ProtocolError> {
    assert!(
        frame.fields.len() == 2,
        "Frame avec 2 champs attendus en réponse d'un message de mouvement de produit"
    );

    // #0 - Ack ou Nack
    match frame.fields[0].decode_binary()? {
        protocol::ACK => {
            context.set_info_bool(IdInfo::Ack, true);
            context.set_info_bool(IdInfo::Nack, false);
        }
        protocol::NACK => {
            context.set_info_bool(IdInfo::Ack, false);
            context.set_info_bool(IdInfo::Nack, true);
        }
        n => {
            context.set_info_bool(IdInfo::Ack, false);
            context.set_info_bool(IdInfo::Nack, false);
            return Err(ProtocolError::IllegalFieldCharDecode(
                "Ack/Nack".to_string(),
                frame.fields[0].clone(),
                n,
            ));
        }
    }

    // #1 - Code erreur spécifique pour les requêtes de mouvement de produit
    let code_erreur: u8 = frame.fields[1].decode_number()?;
    context.set_info_u8(IdInfo::CodeErreurMouvementProduit, code_erreur);

    Ok(())
}
