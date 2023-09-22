//! Informations 'atomiques' échangées par le protocole ALMA IE - ST2150

use std::collections::{HashMap, HashSet};

/// Nombre max de produits
pub const NB_PRODUITS: usize = 16;

/// Nombre de caractères pour un libellé produit
const LIBELLE_PRODUIT_WIDTH: usize = 10;

/// Nombre max de compartiments
pub const NB_COMPARTIMENTS: usize = 9;

/// Nombre max de flexibles
pub const NB_FLEXIBLES: usize = 3;

/// Format possible d'une information du contexte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatInfo {
    Bool,
    Char,
    U8,
    U16,
    U32,
    U64,
    F32,
    String(usize),
}

/// Énumération des informations du contexte
/// Cette énumération permet aux modules externes de désigner une information du contexte
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum IdInfo {
    Ack,
    Nack,
    EnMesurage,
    CodeDefaut,
    ArretIntermediaire,
    ForcagePetitDebit,
    ModeConnecte,
    Totalisateur,
    DebitInstant,
    QuantitePrincipale,
    QuantiteSecondaire,
    TemperatureInstant,
    TemperatureMoyen,
    Predetermination,
    CodeProduit,
    IndexSansRaz,
    IndexJournalier,
    Quantieme,
    HeureHHMMDebut,
    HeureHHMMFin,
    IdentificationTag,
    ReferenceEtImmatriculation,
    VersionLogiciel,
    DateAAMMJJHeureHHMMSS,
    TypeCompteur,
    NbMesuragesQuantieme,
    LibelleProduit,
    NbFractionnements,
    LibelleTableProduits(usize),
    IndexFractionnement,
    TypeDistribution,
    DateAAMMJJ,
    HeureHHMMSS,
    HeureHHMM,
    NbJEvents,
    DataJEvent,
    LibelleJEvent,
    CodeProduitCompartiment(usize),
    QuantiteCompartiment(usize),
    NombreCompartiments,
    PresenceRemorque,
    CodeProduitCollecteur,
    CodeProduitPartieCommune,
    CodeProduitFlexible1,
    CodeProduitFlexible2,
    CodeErreurMouvementProduit,
    CodeProduitFinal,
    NumeroCompartiment,
    NumeroCompartimentFinal,
    OrdreCompartiments,
    NumeroFlexible,
    NumeroFlexibleFinal,
    FinirFlexibleVide,
}

/// Container des différents types de valeurs possibles pour une information du contexte
#[derive(Clone, Debug)]
pub enum TValue {
    Bool(bool),
    Char(char),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    String(String),
}

/// Propriétés associées à chaque information de contexte
#[derive(Clone, Debug)]
struct Info {
    /// Libellé présenté pour l'information
    label: String,

    /// Format choisi pour l'information.
    /// Ce format doit être cohérent avec l'item utilisé pour la propriété `t_info`
    format_info: FormatInfo,

    /// Propriété à `false` tant qu'aucune valeur n'est attribuée à l'information
    is_none: bool,

    /// Valeur de l'information dans le format choisi
    /// En cohérence avec la propriété `format_info`
    t_value: TValue,

    /// Valeur max optionnelle
    option_max_t_value: Option<TValue>,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            label: "Non défini".to_string(), // Par défaut, à définir précisément
            format_info: FormatInfo::Bool,   // Par défaut, à définir précisément
            is_none: true,                   // Pour tous, au début...
            t_value: TValue::Bool(false),    // Don't care: Sera redéfini dès le 1er set_info_xx
            option_max_t_value: None,
        }
    }
}

/// Container pour toutes les informations du contexte
/// C'est l'implémentation de `Default` qui créé toutes les informations
/// atomiques du contexte
pub struct Context {
    /// Table des informations du contexte `IdInfo` -> `Info`
    hash_id_infos: HashMap<IdInfo, Info>,

    /// Liste des `IdInfo` en cours de traitement par la fonction de `callback` invoquée
    /// après une mise à jour de la valeur d'une information.
    /// Cette liste permet d'éviter une mise à jour par une récursion sans fin quand 2 (ou +)
    /// informations sont traitées par ce `callback`.
    /// Typiquement `HeureHHM` -> `HeureHHMMSS` -> `HeureHHM` -> `HeureHHMMSS`...
    set_id_infos_on_change: HashSet<IdInfo>,
}

impl Default for Context {
    #[allow(clippy::too_many_lines)]
    fn default() -> Self {
        let mut hash_id_infos: HashMap<_, _> = HashMap::new();

        // Construction du contexte par défaut
        hash_id_infos.insert(
            IdInfo::Ack,
            Info {
                label: "Acquit message".to_string(),
                format_info: FormatInfo::Bool,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::Nack,
            Info {
                label: "Refus message".to_string(),
                format_info: FormatInfo::Bool,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::EnMesurage,
            Info {
                label: "En mesurage".to_string(),
                format_info: FormatInfo::Bool,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::CodeDefaut,
            Info {
                label: "En Code défaut".to_string(),
                format_info: FormatInfo::U8,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::ArretIntermediaire,
            Info {
                label: "Arrêt intermédiaire".to_string(),
                format_info: FormatInfo::Bool,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::ForcagePetitDebit,
            Info {
                label: "Forçage petit débit".to_string(),
                format_info: FormatInfo::Bool,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::ModeConnecte,
            Info {
                label: "Mode connecté".to_string(),
                format_info: FormatInfo::Bool,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::Totalisateur,
            Info {
                label: "Totalisateur".to_string(),
                format_info: FormatInfo::U32,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::DebitInstant,
            Info {
                label: "Débit instantané".to_string(),
                format_info: FormatInfo::F32,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::QuantitePrincipale,
            Info {
                label: "Quantité".to_string(),
                format_info: FormatInfo::U32,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::QuantiteSecondaire,
            Info {
                label: "Quantité secondaire".to_string(),
                format_info: FormatInfo::U32,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::TemperatureInstant,
            Info {
                label: "Quantité Température instantanée".to_string(),
                format_info: FormatInfo::F32,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::TemperatureMoyen,
            Info {
                label: "Température moyenne".to_string(),
                format_info: FormatInfo::F32,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::Predetermination,
            Info {
                label: "Prédétermination".to_string(),
                format_info: FormatInfo::U32,
                option_max_t_value: Some(TValue::U32(99999)),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::CodeProduit,
            Info {
                label: "Code produit".to_string(),
                format_info: FormatInfo::U8,
                option_max_t_value: Some(TValue::U8(u8::try_from(NB_PRODUITS).unwrap())),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::IndexSansRaz,
            Info {
                label: "Index sans remise à zéro".to_string(),
                format_info: FormatInfo::U16,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::IndexJournalier,
            Info {
                label: "Index journalier".to_string(),
                format_info: FormatInfo::U16,
                option_max_t_value: Some(TValue::U16(999)),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::Quantieme,
            Info {
                label: "Quantième".to_string(),
                format_info: FormatInfo::U16,
                option_max_t_value: Some(TValue::U16(366)),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::HeureHHMMDebut,
            Info {
                label: "Heure de début (HHMM)".to_string(),
                format_info: FormatInfo::U16,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::HeureHHMMFin,
            Info {
                label: "Heure de fin (HHMM)".to_string(),
                format_info: FormatInfo::U16,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::IdentificationTag,
            Info {
                label: "Identification TAG".to_string(),
                format_info: FormatInfo::String(100),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::ReferenceEtImmatriculation,
            Info {
                label: "Référence et immatriculation".to_string(),
                format_info: FormatInfo::String(15),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::VersionLogiciel,
            Info {
                label: "Version du logiciel".to_string(),
                format_info: FormatInfo::String(10),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::DateAAMMJJHeureHHMMSS,
            Info {
                label: "Date et Heure (AAMMJJHHMMSS)".to_string(),
                format_info: FormatInfo::U64,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::TypeCompteur,
            Info {
                label: "Type de compteur (0:Vm, 1:Vb, 2:Masse)".to_string(),
                format_info: FormatInfo::U8,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::NbMesuragesQuantieme,
            Info {
                label: "Nombre de mesurages pour un quantième".to_string(),
                format_info: FormatInfo::U16,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::LibelleProduit,
            Info {
                label: "Libellé produit".to_string(),
                format_info: FormatInfo::String(LIBELLE_PRODUIT_WIDTH),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::NbFractionnements,
            Info {
                label: "Nombre de fractionnements".to_string(),
                format_info: FormatInfo::U16,
                ..Default::default()
            },
        );
        for prod_num in 0..=NB_PRODUITS {
            hash_id_infos.insert(
                IdInfo::LibelleTableProduits(prod_num),
                Info {
                    label: format!("Libellé table produit #{prod_num}"),
                    format_info: FormatInfo::String(LIBELLE_PRODUIT_WIDTH),
                    ..Default::default()
                },
            );
        }
        hash_id_infos.insert(
            IdInfo::IndexFractionnement,
            Info {
                label: "Index fractionnement".to_string(),
                format_info: FormatInfo::U16,
                option_max_t_value: Some(TValue::U16(999)),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::TypeDistribution,
            Info {
                label: "(A)nticipation purge, li(B)ération, (C)hargement, pré(D)é, (L)ibre, (P)urge, (T)ransfert, (V)idange".to_string(),
                format_info: FormatInfo::Char,
                ..Default::default()

            },
        );
        hash_id_infos.insert(
            IdInfo::DateAAMMJJ,
            Info {
                label: "Date (AAMMJJ)".to_string(),
                format_info: FormatInfo::U32,
                option_max_t_value: Some(TValue::U32(99_12_31)),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::HeureHHMMSS,
            Info {
                label: "Heure (HHMMSS)".to_string(),
                format_info: FormatInfo::U32,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::HeureHHMM,
            Info {
                label: "Heure (HHMM)".to_string(),
                format_info: FormatInfo::U16,
                option_max_t_value: Some(TValue::U16(2359)),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::NbJEvents,
            Info {
                label: "Nombre d'événements".to_string(),
                format_info: FormatInfo::U16,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::DataJEvent,
            Info {
                label: "Données techniques événement".to_string(),
                format_info: FormatInfo::String(12),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::LibelleJEvent,
            Info {
                label: "Libellé événement".to_string(),
                format_info: FormatInfo::String(40),
                ..Default::default()
            },
        );
        for compart_num in 0..=NB_COMPARTIMENTS {
            hash_id_infos.insert(
                IdInfo::CodeProduitCompartiment(compart_num),
                Info {
                    label: format!("Code produit cpt #{compart_num}"),
                    format_info: FormatInfo::U8,
                    option_max_t_value: Some(TValue::U8(u8::try_from(NB_PRODUITS).unwrap())),
                    ..Default::default()
                },
            );
            hash_id_infos.insert(
                IdInfo::QuantiteCompartiment(compart_num),
                Info {
                    label: format!("Quantité cpt #{compart_num}"),
                    format_info: FormatInfo::U32,
                    option_max_t_value: Some(TValue::U32(99999)),
                    ..Default::default()
                },
            );
        }
        hash_id_infos.insert(
            IdInfo::NombreCompartiments,
            Info {
                label: "Nombre de compartiments".to_string(),
                format_info: FormatInfo::U8,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::CodeProduitCollecteur,
            Info {
                label: "Code produit dans le collecteur".to_string(),
                format_info: FormatInfo::U8,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::CodeProduitPartieCommune,
            Info {
                label: "Code produit dans la partie commune".to_string(),
                format_info: FormatInfo::U8,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::CodeProduitFlexible1,
            Info {
                label: "Code produit dans le flexible #1".to_string(),
                format_info: FormatInfo::U8,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::CodeProduitFlexible2,
            Info {
                label: "Code produit dans le flexible #2".to_string(),
                format_info: FormatInfo::U8,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::CodeErreurMouvementProduit,
            Info {
                label: "Code erreur (1:Non supporté, 2:En opération)".to_string(),
                format_info: FormatInfo::U8,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::CodeProduitFinal,
            Info {
                label: "Code produit final".to_string(),
                format_info: FormatInfo::U8,
                option_max_t_value: Some(TValue::U8(u8::try_from(NB_PRODUITS).unwrap())),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::NumeroCompartiment,
            Info {
                label: "Numéro de compartiment".to_string(),
                format_info: FormatInfo::U8,
                option_max_t_value: Some(TValue::U8(u8::try_from(NB_COMPARTIMENTS).unwrap())),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::PresenceRemorque,
            Info {
                label: "Présence d'un remorque".to_string(),
                format_info: FormatInfo::Bool,
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::NumeroCompartimentFinal,
            Info {
                label: "Numéro de compartiment final".to_string(),
                format_info: FormatInfo::U8,
                option_max_t_value: Some(TValue::U8(u8::try_from(NB_COMPARTIMENTS).unwrap())),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::OrdreCompartiments,
            Info {
                label: "Ordre des compartiments".to_string(),
                format_info: FormatInfo::U32,
                option_max_t_value: Some(TValue::U32(987_654_321)),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::NumeroFlexible,
            Info {
                label: "Numéro de flexible".to_string(),
                format_info: FormatInfo::U8,
                option_max_t_value: Some(TValue::U8(u8::try_from(NB_FLEXIBLES).unwrap())),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::NumeroFlexibleFinal,
            Info {
                label: "Numéro de flexible final".to_string(),
                format_info: FormatInfo::U8,
                option_max_t_value: Some(TValue::U8(u8::try_from(NB_FLEXIBLES).unwrap())),
                ..Default::default()
            },
        );
        hash_id_infos.insert(
            IdInfo::FinirFlexibleVide,
            Info {
                label: "Finir flexible vide".to_string(),
                format_info: FormatInfo::Bool,
                ..Default::default()
            },
        );

        Self {
            hash_id_infos,
            set_id_infos_on_change: HashSet::new(),
        }
    }
}

impl Context {
    /* ----------- */
    /** GÉNÉRIQUE **/
    /* ----------- */

    /// Getter générique (non mutable) d'une information du contexte (tout format)
    /// # panics
    /// panic! si l'`IdInfo` n'est pas reconnu
    fn get_inner_info(&self, id_info: IdInfo) -> &Info {
        match self.hash_id_infos.get(&id_info) {
            Some(inner_info) => inner_info,
            None => panic!("IdInfo {id_info:?} inconnue"),
        }
    }

    /// Getter générique (mutable) d'une information du contexte (tout format)
    /// # panics
    /// panic! si l'`IdInfo` n'est pas reconnu
    fn get_mut_inner_info(&mut self, id_info: IdInfo) -> &mut Info {
        match self.hash_id_infos.get_mut(&id_info) {
            Some(inner_info) => inner_info,
            None => panic!("IdInfo {id_info:?} inconnue"),
        }
    }

    /// Getter générique (non mutable) d'une information du contexte d'un format spécifique
    /// # panics
    /// panic! si l'`IdInfo` n'est pas reconnu
    /// panic! si l'`IdInfo` n'est pas du format attendu
    fn get_inner_info_with_format(&self, id_info: IdInfo, format_info: FormatInfo) -> &Info {
        match self.hash_id_infos.get(&id_info) {
            Some(inner_info) => {
                if inner_info.format_info == format_info {
                    inner_info
                } else {
                    panic!("IdInfo {id_info:?} n'est pas {format_info:?}")
                }
            }
            None => panic!("IdInfo {id_info:?} inconnue"),
        }
    }

    /// Getter générique (mutable) d'une information du contexte d'un format spécifique
    /// # panics
    /// panic! si l'`IdInfo` n'est pas reconnu
    /// panic! si l'`IdInfo` n'est pas du format attendu
    fn get_mut_inner_info_with_format(
        &mut self,
        id_info: IdInfo,
        format_info: FormatInfo,
    ) -> &mut Info {
        match self.hash_id_infos.get_mut(&id_info) {
            Some(inner_info) => {
                if inner_info.format_info == format_info {
                    inner_info
                } else {
                    panic!("IdInfo {id_info:?} n'est pas {format_info:?}")
                }
            }
            None => panic!("IdInfo {id_info:?} inconnue"),
        }
    }

    /// Libellé d'une information du contexte
    /// # panics
    /// panic! si l'`IdInfo` n'est pas reconnu
    pub fn get_info_label(&self, id_info: IdInfo) -> String {
        let inner_info = self.get_inner_info(id_info);
        inner_info.label.clone()
    }

    /// Format d'une information du contexte
    /// # panics
    /// panic! si l'`IdInfo` n'est pas reconnu
    pub fn get_info_format(&self, id_info: IdInfo) -> FormatInfo {
        let inner_info = self.get_inner_info(id_info);
        inner_info.format_info
    }

    /// Action en callback pour une information du contexte
    /// L'accès à cette fonction est protégé contre une ré-entrance récursive
    fn do_callback_info_on_change(&mut self, id_info: IdInfo, t_value: &TValue) {
        match id_info {
            // HeureHHMMSS => HeureHHMM
            IdInfo::HeureHHMMSS => {
                if let TValue::U32(heure_minute_seconde) = t_value {
                    let heure_minute = u16::try_from(*heure_minute_seconde / 100).unwrap();
                    self.set_info_u16(IdInfo::HeureHHMM, heure_minute);
                }
            }
            // DateAAMMJJHeureHHMMSS -> DateAAMMJJ, HeureHHMMSS -> HeureHHMM
            IdInfo::DateAAMMJJHeureHHMMSS => {
                if let TValue::U64(an_mois_jour_heure_minute_seconde) = t_value {
                    let an_mois_jour = an_mois_jour_heure_minute_seconde / 1_00_00_00;
                    let heure_minute_seconde = an_mois_jour_heure_minute_seconde % 1_00_00_00;
                    let an_mois_jour = u32::try_from(an_mois_jour).unwrap();
                    let heure_minute_seconde = u32::try_from(heure_minute_seconde).unwrap();
                    self.set_info_u32(IdInfo::DateAAMMJJ, an_mois_jour);
                    self.set_info_u32(IdInfo::HeureHHMMSS, heure_minute_seconde);
                }
            }

            // Pas d'action en callback pour toutes les autres informations
            _ => (),
        }
    }

    /// Callback lors d'une mise à jour d'une information du contexte
    fn callback_info_on_change(&mut self, id_info: IdInfo, t_value: &TValue) {
        // Protection contre ré-entrance par récursion
        if self.set_id_infos_on_change.contains(&id_info) {
            return;
        }
        self.set_id_infos_on_change.insert(id_info);
        self.do_callback_info_on_change(id_info, t_value);
        self.set_id_infos_on_change.remove(&id_info);
    }

    /* --------------------- */
    /** STRING INPUT/OUTPUT **/
    /* --------------------- */

    /// Getter de la représentation 'textuelle' d'une information du contexte
    pub fn get_info_to_string(&self, id_info: IdInfo, output_none: &str) -> String {
        let inner_info = self.get_inner_info(id_info);
        if inner_info.is_none {
            output_none.to_string()
        } else {
            match &inner_info.t_value {
                TValue::Bool(value) => {
                    if *value {
                        "Oui".to_string()
                    } else {
                        "Non".to_string()
                    }
                }
                TValue::Char(value) => format!("{value}"),
                TValue::U8(value) => format!("{value}"),
                TValue::U16(value) => format!("{value}"),
                TValue::U32(value) => format!("{value}"),
                TValue::U64(value) => format!("{value}"),
                TValue::F32(value) => format!("{value:.1}"),
                TValue::String(value) => value.trim_end().to_string(),
            }
        }
    }

    /// Setter d'une information du contexte depuis une représentation 'textuelle'
    /// Une `input.is_empty()` ré-initialise l'information à `None` (non définie)
    pub fn set_info_from_string(&mut self, id_info: IdInfo, input: &str) {
        let inner_info = self.get_mut_inner_info(id_info);
        if input.is_empty() {
            inner_info.is_none = true;
        } else {
            // L'input n'est pas vide
            match inner_info.format_info {
                FormatInfo::Bool => {
                    let value = ['o', 'O', '1'].contains(&input.chars().next().unwrap());
                    self.set_info_bool(id_info, value);
                }
                FormatInfo::Char => {
                    let value = input.chars().next().unwrap();
                    self.set_info_char(id_info, value);
                }
                FormatInfo::U8 => {
                    if let Ok(value) = input.parse::<u8>() {
                        self.set_info_u8(id_info, value);
                    }
                }
                FormatInfo::U16 => {
                    if let Ok(value) = input.parse::<u16>() {
                        self.set_info_u16(id_info, value);
                    }
                }
                FormatInfo::U32 => {
                    if let Ok(value) = input.parse::<u32>() {
                        self.set_info_u32(id_info, value);
                    }
                }
                FormatInfo::U64 => {
                    if let Ok(value) = input.parse::<u64>() {
                        self.set_info_u64(id_info, value);
                    }
                }
                FormatInfo::F32 => {
                    if let Ok(value) = input.parse::<f32>() {
                        self.set_info_f32(id_info, value);
                    }
                }
                FormatInfo::String(width) => {
                    let input = input.trim_end();
                    let value = if input.len() > width {
                        // Tronque si trop long
                        // /!\ format! ne le fait pas...
                        input[..width].to_string()
                    } else {
                        input.to_string()
                    };
                    self.set_info_string(id_info, &value);
                }
            }
        }
    }

    /* ------ */
    /** BOOL **/
    /* ------ */

    /// Getter d'une information de type `bool`
    pub fn get_option_info_bool(&self, id_info: IdInfo) -> Option<bool> {
        let inner_info = self.get_inner_info_with_format(id_info, FormatInfo::Bool);
        if inner_info.is_none {
            None
        } else {
            match inner_info.t_value {
                TValue::Bool(value) => Some(value),
                _ => panic!("{id_info:?} n'est pas un bool"),
            }
        }
    }

    /// Setter d'une information de type `bool`
    pub fn set_info_bool(&mut self, id_info: IdInfo, value: bool) {
        let inner_info = self.get_mut_inner_info_with_format(id_info, FormatInfo::Bool);
        inner_info.is_none = false;
        inner_info.t_value = TValue::Bool(value);
        let t_value = inner_info.t_value.clone();
        self.callback_info_on_change(id_info, &t_value);
    }

    /* -------*/
    /** CHAR **/
    /* -------*/

    /// Getter d'une information de type `char`
    pub fn get_option_info_char(&self, id_info: IdInfo) -> Option<char> {
        let inner_info = self.get_inner_info_with_format(id_info, FormatInfo::Char);
        if inner_info.is_none {
            None
        } else {
            match inner_info.t_value {
                TValue::Char(value) => Some(value),
                _ => panic!("{id_info:?} n'est pas un char"),
            }
        }
    }

    /// Setter d'une information de type `char`
    pub fn set_info_char(&mut self, id_info: IdInfo, value: char) {
        let inner_info = self.get_mut_inner_info_with_format(id_info, FormatInfo::Char);
        if let Some(TValue::Char(max_value)) = inner_info.option_max_t_value {
            if value > max_value {
                return;
            }
        }
        inner_info.is_none = false;
        inner_info.t_value = TValue::Char(value);
        let t_value = inner_info.t_value.clone();
        self.callback_info_on_change(id_info, &t_value);
    }

    /* -----*/
    /** U8 **/
    /* -----*/

    /// Getter d'une information de type `u8`
    pub fn get_option_info_u8(&self, id_info: IdInfo) -> Option<u8> {
        let inner_info = self.get_inner_info_with_format(id_info, FormatInfo::U8);
        if inner_info.is_none {
            None
        } else {
            match inner_info.t_value {
                TValue::U8(value) => Some(value),
                _ => panic!("{id_info:?} n'est pas un u8"),
            }
        }
    }

    /// Setter d'une information de type `u8`
    pub fn set_info_u8(&mut self, id_info: IdInfo, value: u8) {
        let inner_info = self.get_mut_inner_info_with_format(id_info, FormatInfo::U8);
        if let Some(TValue::U8(max_value)) = inner_info.option_max_t_value {
            if value > max_value {
                return;
            }
        }
        inner_info.is_none = false;
        inner_info.t_value = TValue::U8(value);
        let t_value = inner_info.t_value.clone();
        self.callback_info_on_change(id_info, &t_value);
    }

    /* ------*/
    /** U16 **/
    /* ------*/

    /// Getter d'une information de type `u16`
    pub fn get_option_info_u16(&self, id_info: IdInfo) -> Option<u16> {
        let inner_info = self.get_inner_info_with_format(id_info, FormatInfo::U16);
        if inner_info.is_none {
            None
        } else {
            match inner_info.t_value {
                TValue::U16(value) => Some(value),
                _ => panic!("{id_info:?} n'est pas un u16"),
            }
        }
    }

    /// Setter d'une information de type `u16`
    pub fn set_info_u16(&mut self, id_info: IdInfo, value: u16) {
        let inner_info = self.get_mut_inner_info_with_format(id_info, FormatInfo::U16);
        if let Some(TValue::U16(max_value)) = inner_info.option_max_t_value {
            if value > max_value {
                return;
            }
        }
        inner_info.is_none = false;
        inner_info.t_value = TValue::U16(value);
        let t_value = inner_info.t_value.clone();
        self.callback_info_on_change(id_info, &t_value);
    }

    /* ------*/
    /** U32 **/
    /* ------*/

    /// Getter d'une information de type `u32`
    pub fn get_option_info_u32(&self, id_info: IdInfo) -> Option<u32> {
        let inner_info = self.get_inner_info_with_format(id_info, FormatInfo::U32);
        if inner_info.is_none {
            None
        } else {
            match inner_info.t_value {
                TValue::U32(value) => Some(value),
                _ => panic!("{id_info:?} n'est pas un u32"),
            }
        }
    }

    /// Setter d'une information de type `u32`
    pub fn set_info_u32(&mut self, id_info: IdInfo, value: u32) {
        let inner_info = self.get_mut_inner_info_with_format(id_info, FormatInfo::U32);
        if let Some(TValue::U32(max_value)) = inner_info.option_max_t_value {
            if value > max_value {
                return;
            }
        }
        inner_info.is_none = false;
        inner_info.t_value = TValue::U32(value);
        let t_value = inner_info.t_value.clone();
        self.callback_info_on_change(id_info, &t_value);
    }

    /* ------*/
    /** U64 **/
    /* ------*/

    /// Getter d'une information de type `u64`
    pub fn get_option_info_u64(&self, id_info: IdInfo) -> Option<u64> {
        let inner_info = self.get_inner_info_with_format(id_info, FormatInfo::U64);
        if inner_info.is_none {
            None
        } else {
            match inner_info.t_value {
                TValue::U64(value) => Some(value),
                _ => panic!("{id_info:?} n'est pas un u64"),
            }
        }
    }

    /// Setter d'une information de type `u64`
    pub fn set_info_u64(&mut self, id_info: IdInfo, value: u64) {
        let inner_info = self.get_mut_inner_info_with_format(id_info, FormatInfo::U64);
        if let Some(TValue::U64(max_value)) = inner_info.option_max_t_value {
            if value > max_value {
                return;
            }
        }
        inner_info.is_none = false;
        inner_info.t_value = TValue::U64(value);
        let t_value = inner_info.t_value.clone();
        self.callback_info_on_change(id_info, &t_value);
    }

    /* ------*/
    /** F32 **/
    /* ------*/

    /// Getter d'une information de type `f32`
    pub fn get_option_info_f32(&self, id_info: IdInfo) -> Option<f32> {
        let inner_info = self.get_inner_info_with_format(id_info, FormatInfo::F32);
        if inner_info.is_none {
            None
        } else {
            match inner_info.t_value {
                TValue::F32(value) => Some(value),
                _ => panic!("{id_info:?} n'est pas un f32"),
            }
        }
    }

    /// Setter d'une information de type `f32`
    pub fn set_info_f32(&mut self, id_info: IdInfo, value: f32) {
        let inner_info = self.get_mut_inner_info_with_format(id_info, FormatInfo::F32);

        if let Some(TValue::F32(max_value)) = inner_info.option_max_t_value {
            if value > max_value {
                return;
            }
        }
        inner_info.is_none = false;
        inner_info.t_value = TValue::F32(value);
        let t_value = inner_info.t_value.clone();
        self.callback_info_on_change(id_info, &t_value);
    }

    /* ---------*/
    /** String **/
    /* ---------*/

    /// Getter d'une information de type `string`
    pub fn get_option_info_string(&self, id_info: IdInfo) -> Option<String> {
        let inner_info = self.get_inner_info(id_info);
        if inner_info.is_none {
            None
        } else {
            match &inner_info.t_value {
                TValue::String(value) => Some(value.clone()),
                _ => panic!("{id_info:?} n'est pas un string"),
            }
        }
    }

    /// Setter d'une information de type `string`
    pub fn set_info_string(&mut self, id_info: IdInfo, value: &str) {
        let inner_info = self.get_mut_inner_info(id_info);
        let value = if let FormatInfo::String(width) = inner_info.format_info {
            if value.len() > width {
                &value[0..width]
            } else {
                value
            }
        } else {
            value
        };
        if let Some(TValue::String(max_value)) = &inner_info.option_max_t_value {
            if value > &max_value[..] {
                return;
            }
        }
        inner_info.is_none = false;
        inner_info.t_value = TValue::String(value.to_string());
        let t_value = inner_info.t_value.clone();
        self.callback_info_on_change(id_info, &t_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::context::CommonContextTrait;

    // Cette fonction devrait être appelée avec des `IdInfo` de tous les `FormatInfo` possibles
    // Voir `test_get_set` ci-dessous
    fn check_id_code(context: &mut Context, id_info: IdInfo) {
        let option_max_t_value = { context.get_inner_info(id_info).option_max_t_value.clone() };
        match context.get_info_format(id_info) {
            FormatInfo::Bool => {
                assert!(context.get_option_info_bool(id_info).is_none());
                for value in [true, false] {
                    context.set_info_bool(id_info, value);
                    assert_eq!(context.get_option_info_bool(id_info), Some(value));
                }
            }
            FormatInfo::Char => {
                assert!(context.get_option_info_char(id_info).is_none());
                for value in ['A', 'B', 'e', 'é'] {
                    if let Some(TValue::Char(max_value)) = option_max_t_value {
                        if value > max_value {
                            continue;
                        }
                    }
                    context.set_info_char(id_info, value);
                    assert_eq!(context.get_option_info_char(id_info), Some(value));
                }
            }
            FormatInfo::U8 => {
                assert!(context.get_option_info_u8(id_info).is_none());
                for value in [0_u8, 10_u8, 100_u8] {
                    if let Some(TValue::U8(max_value)) = option_max_t_value {
                        if value > max_value {
                            continue;
                        }
                    }
                    context.set_info_u8(id_info, value);
                    assert_eq!(context.get_option_info_u8(id_info), Some(value));
                }
            }
            FormatInfo::U16 => {
                assert!(context.get_option_info_u16(id_info).is_none());
                for value in [0_u16, 1000_u16, 10_000_u16] {
                    if let Some(TValue::U16(max_value)) = option_max_t_value {
                        if value > max_value {
                            continue;
                        }
                    }
                    context.set_info_u16(id_info, value);
                    assert_eq!(context.get_option_info_u16(id_info), Some(value));
                }
            }
            FormatInfo::U32 => {
                assert!(context.get_option_info_u32(id_info).is_none());
                for value in [0_u32, 1000_u32, 100_000_u32] {
                    if let Some(TValue::U32(max_value)) = option_max_t_value {
                        if value > max_value {
                            continue;
                        }
                    }
                    context.set_info_u32(id_info, value);
                    assert_eq!(context.get_option_info_u32(id_info), Some(value));
                }
            }
            FormatInfo::U64 => {
                assert!(context.get_option_info_u64(id_info).is_none());
                for value in [0_u64, 100_000_u64, 100_000_000_u64, 100_000_000_000_000_u64] {
                    if let Some(TValue::U64(max_value)) = option_max_t_value {
                        if value > max_value {
                            continue;
                        }
                    }
                    context.set_info_u64(id_info, value);
                    assert_eq!(context.get_option_info_u64(id_info), Some(value));
                }
            }
            FormatInfo::F32 => {
                assert!(context.get_option_info_f32(id_info).is_none());
                for value in [0.0_f32, 1000.0_f32, -1000.0_f32, 100_000.0_f32] {
                    if let Some(TValue::F32(max_value)) = option_max_t_value {
                        if value > max_value {
                            continue;
                        }
                    }
                    context.set_info_f32(id_info, value);
                    assert_eq!(context.get_option_info_f32(id_info), Some(value));
                }
            }
            FormatInfo::String(_width) => {
                assert!(context.get_option_info_string(id_info).is_none());
                for value in ["", "ABC"] {
                    if let Some(TValue::String(max_value)) = &option_max_t_value {
                        if value > &max_value[..] {
                            continue;
                        }
                    }
                    context.set_info_string(id_info, value);
                    assert_eq!(
                        context.get_option_info_string(id_info),
                        Some(value.to_string())
                    );
                }
            }
        }
    }

    #[test]
    fn test_get_set() {
        let mut context = Context::default();

        // Idéalement, mettre ici tous les IdInfos... Au moins tester les différents formats :)
        // TODO : Pas réussi à mettre en oeuvre le crate `enum-iterator` qui permettrait d'itérer
        //        sur toutes les valeurs d'un Enum :(
        check_id_code(&mut context, IdInfo::Ack);
        check_id_code(&mut context, IdInfo::Nack);
        check_id_code(&mut context, IdInfo::EnMesurage);
        check_id_code(&mut context, IdInfo::CodeDefaut);
        check_id_code(&mut context, IdInfo::ArretIntermediaire);
        check_id_code(&mut context, IdInfo::ForcagePetitDebit);
        check_id_code(&mut context, IdInfo::ModeConnecte);
        check_id_code(&mut context, IdInfo::Totalisateur);
        check_id_code(&mut context, IdInfo::DebitInstant);
        check_id_code(&mut context, IdInfo::QuantitePrincipale);
        check_id_code(&mut context, IdInfo::QuantiteSecondaire);
        check_id_code(&mut context, IdInfo::TemperatureInstant);
        check_id_code(&mut context, IdInfo::TemperatureMoyen);
        check_id_code(&mut context, IdInfo::Predetermination);
        check_id_code(&mut context, IdInfo::CodeProduit);
        check_id_code(&mut context, IdInfo::IndexSansRaz);
        check_id_code(&mut context, IdInfo::IndexJournalier);
        check_id_code(&mut context, IdInfo::Quantieme);
        check_id_code(&mut context, IdInfo::HeureHHMMDebut);
        check_id_code(&mut context, IdInfo::HeureHHMMFin);
        // Attention, la mise à jour date/heure peut se faire en callback
        // (il faut tester les date/heure du moins précis au plus précis)
        check_id_code(&mut context, IdInfo::HeureHHMM);
        check_id_code(&mut context, IdInfo::HeureHHMMSS);
        check_id_code(&mut context, IdInfo::DateAAMMJJ);
        check_id_code(&mut context, IdInfo::DateAAMMJJHeureHHMMSS);
        check_id_code(&mut context, IdInfo::IdentificationTag);
        check_id_code(&mut context, IdInfo::ReferenceEtImmatriculation);
        check_id_code(&mut context, IdInfo::VersionLogiciel);
        check_id_code(&mut context, IdInfo::TypeCompteur);
        check_id_code(&mut context, IdInfo::NbMesuragesQuantieme);
        check_id_code(&mut context, IdInfo::LibelleProduit);
        check_id_code(&mut context, IdInfo::NbFractionnements);
        for prod_num in 0..=NB_PRODUITS {
            check_id_code(&mut context, IdInfo::LibelleTableProduits(prod_num));
        }
        check_id_code(&mut context, IdInfo::IndexFractionnement);
        check_id_code(&mut context, IdInfo::TypeDistribution);
        check_id_code(&mut context, IdInfo::NbJEvents);
        check_id_code(&mut context, IdInfo::DataJEvent);
        check_id_code(&mut context, IdInfo::LibelleJEvent);
        for compart_num in 0..=NB_COMPARTIMENTS {
            check_id_code(&mut context, IdInfo::CodeProduitCompartiment(compart_num));
            check_id_code(&mut context, IdInfo::QuantiteCompartiment(compart_num));
        }
        check_id_code(&mut context, IdInfo::NombreCompartiments);
        check_id_code(&mut context, IdInfo::PresenceRemorque);
        check_id_code(&mut context, IdInfo::CodeProduitCollecteur);
        check_id_code(&mut context, IdInfo::CodeProduitPartieCommune);
        check_id_code(&mut context, IdInfo::CodeProduitFlexible1);
        check_id_code(&mut context, IdInfo::CodeProduitFlexible2);
        check_id_code(&mut context, IdInfo::CodeErreurMouvementProduit);
        check_id_code(&mut context, IdInfo::CodeProduitFinal);
        check_id_code(&mut context, IdInfo::NumeroCompartiment);
        check_id_code(&mut context, IdInfo::NumeroCompartimentFinal);
        check_id_code(&mut context, IdInfo::OrdreCompartiments);
        check_id_code(&mut context, IdInfo::NumeroFlexible);
        check_id_code(&mut context, IdInfo::NumeroFlexibleFinal);
        check_id_code(&mut context, IdInfo::FinirFlexibleVide);
    }

    #[test]
    #[should_panic]
    fn test_get_set_panic() {
        // Le getter va panic! si on demande une information avec un format différent
        // du format de cette info

        let context: Context = Context::default();

        // Lecture d'une température (F_32) dans un bool
        let _ = context.get_option_info_bool(IdInfo::TemperatureInstant);
    }

    #[test]
    fn test_context_string_bool() {
        let mut context = Context::default();

        assert!(context.get_option_info_bool(IdInfo::Ack).is_none());
        assert_eq!(context.get_info_to_string(IdInfo::Ack, "None"), "None");

        context.set_info_bool(IdInfo::Ack, true);
        assert_eq!(context.get_option_info_bool(IdInfo::Ack), Some(true));
        assert_eq!(context.get_info_to_string(IdInfo::Ack, "None"), "Oui");

        context.set_info_from_string(IdInfo::Ack, "Non");
        assert_eq!(context.get_option_info_bool(IdInfo::Ack), Some(false));
        assert_eq!(context.get_info_to_string(IdInfo::Ack, "None"), "Non");

        context.set_info_from_string(IdInfo::Ack, "");
        assert_eq!(context.get_option_info_bool(IdInfo::Ack), None);
    }

    #[test]
    fn test_context_string_char() {
        let mut context = Context::default();

        assert!(context
            .get_option_info_char(IdInfo::TypeDistribution)
            .is_none());
        assert_eq!(
            context.get_info_to_string(IdInfo::TypeDistribution, "None"),
            "None"
        );

        context.set_info_from_string(IdInfo::TypeDistribution, "X");
        assert_eq!(
            context.get_option_info_char(IdInfo::TypeDistribution),
            Some('X')
        );
        assert_eq!(
            context.get_info_to_string(IdInfo::TypeDistribution, "None"),
            "X"
        );

        context.set_info_from_string(IdInfo::TypeDistribution, "");
        assert_eq!(context.get_option_info_char(IdInfo::TypeDistribution), None);
    }

    #[test]
    fn test_context_string_u8() {
        let mut context = Context::default();

        assert!(context.get_option_info_u8(IdInfo::CodeProduit).is_none());
        assert_eq!(
            context.get_info_to_string(IdInfo::CodeProduit, "None"),
            "None"
        );

        context.set_info_from_string(IdInfo::CodeProduit, "3");
        assert_eq!(context.get_option_info_u8(IdInfo::CodeProduit), Some(3));
        assert_eq!(context.get_info_to_string(IdInfo::CodeProduit, "None"), "3");

        context.set_info_from_string(IdInfo::CodeProduit, "");
        assert_eq!(context.get_option_info_u8(IdInfo::CodeProduit), None);
    }

    #[test]
    fn test_context_string_u16() {
        let mut context = Context::default();

        assert!(context.get_option_info_u16(IdInfo::HeureHHMM).is_none());
        assert_eq!(
            context.get_info_to_string(IdInfo::HeureHHMM, "None"),
            "None"
        );

        context.set_info_from_string(IdInfo::HeureHHMM, "1234");
        assert_eq!(context.get_option_info_u16(IdInfo::HeureHHMM), Some(1234));
        assert_eq!(
            context.get_info_to_string(IdInfo::HeureHHMM, "None"),
            "1234"
        );

        context.set_info_from_string(IdInfo::HeureHHMM, "");
        assert_eq!(context.get_option_info_u16(IdInfo::HeureHHMM), None);
    }

    #[test]
    fn test_context_string_u32() {
        let mut context = Context::default();

        assert!(context
            .get_option_info_u32(IdInfo::QuantitePrincipale)
            .is_none());
        assert_eq!(
            context.get_info_to_string(IdInfo::QuantitePrincipale, "None"),
            "None"
        );

        context.set_info_from_string(IdInfo::QuantitePrincipale, "12345");
        assert_eq!(
            context.get_option_info_u32(IdInfo::QuantitePrincipale),
            Some(12345)
        );
        assert_eq!(
            context.get_info_to_string(IdInfo::QuantitePrincipale, "None"),
            "12345"
        );

        context.set_info_from_string(IdInfo::QuantitePrincipale, "");
        assert_eq!(
            context.get_option_info_u32(IdInfo::QuantitePrincipale),
            None
        );
    }

    #[test]
    fn test_context_string_u64() {
        let mut context = Context::default();

        assert!(context
            .get_option_info_u64(IdInfo::DateAAMMJJHeureHHMMSS)
            .is_none());
        assert_eq!(
            context.get_info_to_string(IdInfo::DateAAMMJJHeureHHMMSS, "None"),
            "None"
        );

        context.set_info_from_string(IdInfo::DateAAMMJJHeureHHMMSS, "991231235959");
        assert_eq!(
            context.get_option_info_u64(IdInfo::DateAAMMJJHeureHHMMSS),
            Some(99_12_31_23_59_59)
        );
        assert_eq!(
            context.get_info_to_string(IdInfo::DateAAMMJJHeureHHMMSS, "None"),
            "991231235959"
        );

        context.set_info_from_string(IdInfo::DateAAMMJJHeureHHMMSS, "");
        assert_eq!(
            context.get_option_info_u64(IdInfo::DateAAMMJJHeureHHMMSS),
            None
        );
    }

    #[test]
    fn test_context_string_f32() {
        let mut context = Context::default();

        assert!(context
            .get_option_info_f32(IdInfo::TemperatureInstant)
            .is_none());
        assert_eq!(
            context.get_info_to_string(IdInfo::TemperatureInstant, "None"),
            "None"
        );

        context.set_info_from_string(IdInfo::TemperatureInstant, "-12.3");
        assert_eq!(
            context.get_option_info_f32(IdInfo::TemperatureInstant),
            Some(-12.3)
        );
        assert_eq!(
            context.get_info_to_string(IdInfo::TemperatureInstant, "None"),
            "-12.3"
        );

        context.set_info_from_string(IdInfo::TemperatureInstant, "");
        assert_eq!(
            context.get_option_info_f32(IdInfo::TemperatureInstant),
            None
        );
    }

    #[test]
    fn test_context_string_string() {
        let mut context = Context::default();

        assert!(context
            .get_option_info_string(IdInfo::LibelleProduit)
            .is_none());
        assert_eq!(
            context.get_info_to_string(IdInfo::LibelleProduit, "None"),
            "None"
        );

        context.set_info_from_string(IdInfo::LibelleProduit, "ABCDE");
        assert_eq!(
            context.get_option_info_string(IdInfo::LibelleProduit),
            Some("ABCDE".to_string())
        );
        assert_eq!(
            context.get_info_to_string(IdInfo::LibelleProduit, "None"),
            "ABCDE"
        );

        context.set_info_from_string(IdInfo::LibelleProduit, "");
        assert_eq!(context.get_option_info_string(IdInfo::LibelleProduit), None);
    }

    #[test]
    fn test_get_set_date_heure() {
        let mut context: Context = Context::default();

        // Par défaut, rien n'est défini
        assert!(context
            .get_option_info_u64(IdInfo::DateAAMMJJHeureHHMMSS)
            .is_none());
        assert!(context.get_option_info_u32(IdInfo::DateAAMMJJ).is_none());
        assert!(context.get_option_info_u32(IdInfo::HeureHHMMSS).is_none());
        assert!(context.get_option_info_u16(IdInfo::HeureHHMM).is_none());

        // Si on définit l'heure_minute_seconde, heure_minute est maintenant défini
        context.set_info_u32(IdInfo::HeureHHMMSS, 13_14_15);
        assert_eq!(context.get_option_info_u16(IdInfo::HeureHHMM), Some(13_14));

        // Si on définit an_mois_jour_heure_minute_seconde, an_mois_jour est défini...
        context.set_info_u64(IdInfo::DateAAMMJJHeureHHMMSS, 1_02_03_10_11_12);
        assert_eq!(
            context.get_option_info_u32(IdInfo::DateAAMMJJ),
            Some(1_02_03)
        );
        // ... ainsi que heure_minute_seconde...
        assert_eq!(
            context.get_option_info_u32(IdInfo::HeureHHMMSS),
            Some(10_11_12)
        );
        // ... ainsi que l'heure sans les secondes
        assert_eq!(context.get_option_info_u16(IdInfo::HeureHHMM), Some(10_11));
    }
}
