//! Informations 'atomiques' échangées par le protocole ALMA IE - ST2150

/// Structure pour toutes les informations 'atomiques'
///
/// En privé, la structure `Context`, toutes les informations sont du type `Option<T>` avec une valeur
/// à `None` tant qu'aucune valeur n'a été explicitement assignée à l'information qui devient alors
/// `Some(quelque_chose)`.
///
/// Pour l'utilisateur (accès public), le dictionnaire des données du contexte sont énumérées `IdInfo` et le type
/// du format de la donnée est également énumérée `InfoFormat`.
///
/// Des primitives sont disponibles pour accéder aux informations du contexte via ces énumérations.

///
/// Nota : C'est un peu relou à maintenir mais très pratique pour l'IHM...

/// Nombre max de produits
const NB_PRODUITS: usize = 16;

/// Nombre de caractères pour un libellé produit
const LIBELLE_PRODUIT_WIDTH: usize = 10;

/// Nombre max de compartiments
const NB_COMPARTIMENTS: usize = 9;

// Pour ajouter un nouveau format de données pour le contexte :
//
// Dans ce module (`context`) :
//  1 - Ajouter ce format dans la liste des enum de `FormatInfo`
//  2 - Créer au moins une information dans le contexte avec ce format pour pouvoir tester
//      (Voir ci-dessous les instructions pour créer une nouvelle information dans le contexte)
//  3 - Ajouter la fonction `get_info_mon_nouveau_format` dans l'implémentation de `Context`
//  4 - Ajouter la fonction `set_info_mon_nouveau_format` dans l'implémentation de `Context`
//  5 - Implémenter `CommonContextTrait` pour ce nouveau format
//      `impl CommonContextTrait<NouveauFormat> for Context { ...`
//  6 - Completer les tests pour ce nouveau format : `test_get_set` et `test_get_set_generic`
// Dans le module st2150::messages :
//  1 - Ajouter ce format dans la fonction `availability` pour `CommonMessageTrait`
// Dans le module `app_view::show_infos` :
//  1 - Ajouter une fonction `str_info_mon_nouveau_format`
//  2 - Ajouter ce format dans la fonction `str_info`
// Dans le module `app_view::input_infos` :
//  1 - Ajouter ce format dans la fonction `callback_input_info` de `app_view::input_infos`
// C'est tout :)
//
// Ou si on est courageux, une macro! qui fait tout ça...

/// Format possible d'une information du contexte
#[derive(Clone, Debug)]
pub enum FormatInfo {
    FormatBool,
    FormatChar,
    FormatU8,
    FormatU16,
    FormatU32,
    FormatU64,
    FormatF32,
    FormatString(usize),
}

// Pour ajouter une nouvelle information 'Xxx' d'un format `mon_format` dans le contexte :
// 1 - Ajouter `Xxx` dans l'énumération de `IdInfo`
// 2 - Ajouter 'xxx: `Option<mon_format>`` dans la structure `Context`
//      (d'autres implémentations sont également possibles. Adapter alors la suite des modifications)
// 3 - Ajouter le libellé de cette information Xxx dans `get_info_name`
// 4 - Ajouter le type de format de cette information Xxx dans `get_info_format`
// 5 - Ajouter le cas IdInfo::Xxx dans la fonction `get_info_format` dans l'implémentation de `Context`
// 6 - Ajouter le cas IdInfo::Xxx dans la fonction `set_info_format` dans l'implémentation de `Context`
// 7 - Pour les tests, ajouter `check_id_code(&mut context, IdInfo::Xxx);` dans la fonction `test_get_set`
// Et c'est tout :)

/// Énumération des informations du contexte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    HeureDebut,
    HeureFin,
    IdentificationTag,
    ReferenceEtImmatriculation,
    VersionLogiciel,
    DateHeure,
    TypeCompteur,
    NbMesuragesQuantieme,
    LibelleProduit,
    NbFractionnements,
    LibelleTableProduits(usize),
    IndexFractionnement,
    TypeDistribution,
    Date,
    Heure,
    NbJEvents,
    DataJEvent,
    LibelleJEvent,
    CodeProduitCompartiment(usize),
    QuantiteCompartiment(usize),
}

/// Dictionnaire des données pour les requêtes et les réponses
#[derive(Debug, Default)]
pub struct Context {
    /// ACK du dernier message
    ack: Option<bool>,

    /// NACK du dernier message
    nack: Option<bool>,

    /// En mesurage
    en_mesurage: Option<bool>,

    /// Code défaut en cours (Codage 'court' selon ST3274)
    /// 0 pour pas de défaut
    code_defaut: Option<u8>,

    /// En arrêt intermédiaire
    arret_intermediaire: Option<bool>,

    /// Forçage en petit débit
    forcage_petit_debit: Option<bool>,

    /// En mode connecté
    mode_connecte: Option<bool>,

    /// Totalisateur (en échelon = Litre ou kg)
    totalisateur: Option<u32>,

    /// Débit instantané en m3/h (ou tonne/h)
    debit_instant: Option<f32>,

    /// Quantité (volume mesuré, volume de base ou masse)  principale
    quantite_principale: Option<u32>,

    /// Quantité secondaire (alternative à la quantité principale)
    quantite_secondaire: Option<u32>,

    /// Température instantanée
    temperature_instant: Option<f32>,

    /// Température moyenne (d'un mesurage)
    temperature_moyen: Option<f32>,

    /// Prédétermination (en échelon = Litre ou kg)
    predetermination: Option<u32>,

    /// Code produit
    code_produit: Option<u8>,

    /// Index sans remise à 0
    index_sans_raz: Option<u16>,

    /// Index journalier
    index_journalier: Option<u16>,

    /// Quantième
    quantieme: Option<u16>,

    /// Heure de début (de mesurage)
    heure_debut: Option<u16>,

    /// Heure de fin (de mesurage)
    heure_fin: Option<u16>,

    /// Identification par TAG
    identification_tag: Option<String>,

    /// Version du logiciel
    version_logiciel: Option<String>,

    /// Numéro de référence du compteur et immatriculation du camion
    reference_et_immatriculation: Option<String>,

    /// Date et heure AAMMJJHHMMSS
    date_heure: Option<u64>,

    /// Type de compteur 0: Vm, 1:Vb, 2:Masse
    type_compteur: Option<u8>,

    /// Nombre de mesurages pour un quantième
    nb_mesurages_quantieme: Option<u16>,

    /// Libellé du produit
    libelle_produit: Option<String>,

    /// Nombre de fractionnements
    nb_fractionnements: Option<u16>,

    /// Libellés de la table des max. NB_PRODUITS produits
    libelle_table_produits: Vec<String>,

    /// Numéro de fractionnement pour un mesurage
    index_fractionnement: Option<u16>,

    /// Type de distribution ('P' pour purge, 'L' pour libre, etc.)
    type_distribution: Option<char>,

    /// Date (AAMMJJ)
    date: Option<u32>,

    /// Heure (HHMMSS)
    heure: Option<u32>,

    /// Nombre d'événements pour une journée
    nb_jevents: Option<u16>,

    /// Données techniques d'un événement (voir doc ST2150)
    data_jevent: Option<String>,

    /// Libellé d'un événement
    libelle_jevent: Option<String>,

    /// Code produit dans le compartiment #i
    code_produit_compartiment: Vec<u8>,

    /// Quantité dans le compartiment #i
    quantite_compartiment: Vec<u32>,
}

/// Retourne le libellé d'une information du contexte
pub fn get_info_name(id_info: IdInfo) -> String {
    match id_info {
        IdInfo::Ack => "Acquit message".to_string(),
        IdInfo::Nack => "Refus message".to_string(),
        IdInfo::EnMesurage => "En mesurage".to_string(),
        IdInfo::CodeDefaut => "Code défaut".to_string(),
        IdInfo::ArretIntermediaire => "Arrêt intermédiaire".to_string(),
        IdInfo::ForcagePetitDebit => "Forçage petit débit".to_string(),
        IdInfo::ModeConnecte => "Mode connecté".to_string(),
        IdInfo::Totalisateur => "Totalisateur".to_string(),
        IdInfo::DebitInstant => "Débit instantané".to_string(),
        IdInfo::QuantitePrincipale => "Quantité".to_string(),
        IdInfo::QuantiteSecondaire => "Quantité secondaire".to_string(),
        IdInfo::TemperatureInstant => "Température instantanée".to_string(),
        IdInfo::TemperatureMoyen => "Température moyenne".to_string(),
        IdInfo::Predetermination => "Prédétermination".to_string(),
        IdInfo::CodeProduit => "Code produit".to_string(),
        IdInfo::IndexSansRaz => "Index sans remise à zéro".to_string(),
        IdInfo::IndexJournalier => "Index journalier".to_string(),
        IdInfo::Quantieme => "Quantième".to_string(),
        IdInfo::HeureDebut => "Heure de début (HHMM)".to_string(),
        IdInfo::HeureFin => "Heure de fin (HHMM)".to_string(),
        IdInfo::IdentificationTag => "Identification TAG".to_string(),
        IdInfo::ReferenceEtImmatriculation => "Référence et immatriculation".to_string(),
        IdInfo::VersionLogiciel => "Version du logiciel".to_string(),
        IdInfo::DateHeure => "Date et Heure (AAMMJJHHMMSS)".to_string(),
        IdInfo::TypeCompteur => "Type de compteur (0:Vm, 1:Vb, 2:Masse)".to_string(),
        IdInfo::NbMesuragesQuantieme => "Nombre de mesurages pour un quantième".to_string(),
        IdInfo::LibelleProduit => "Libellé produit".to_string(),
        IdInfo::NbFractionnements => "Nombre de fractionnements".to_lowercase(),
        IdInfo::LibelleTableProduits(prod_num) => format!("Libellé table produit #{prod_num}"),
        IdInfo::IndexFractionnement => "Index fractionnement".to_string(),
        IdInfo::TypeDistribution => "(A)nticipation purge, li(B)ération, (C)hargement, pré(D)é, (L)ibre, (P)urge, (T)ransfert, (V)idange, ".to_string(),
        IdInfo::Date=> "Date (AAMMJJ)".to_string(),
        IdInfo::Heure=> "Heure (HHMMSS)".to_string(),
        IdInfo::NbJEvents => "Nombre d'événements".to_string(),
        IdInfo::DataJEvent => "Données techniques d'un événement".to_string(),
        IdInfo::LibelleJEvent => "Libellé d'un événement".to_string(),
        IdInfo::CodeProduitCompartiment(compart_num) => format!("Code produit du compartiment #{compart_num}"),
        IdInfo::QuantiteCompartiment(compart_num)=> format!("Quantité dans le compartiment #{compart_num}"),
    }
}

/// Retourne le type de format pour une information du contexte
pub fn get_info_format(id_info: IdInfo) -> FormatInfo {
    match id_info {
        /* Booléen */
        IdInfo::Ack
        | IdInfo::Nack
        | IdInfo::EnMesurage
        | IdInfo::ArretIntermediaire
        | IdInfo::ForcagePetitDebit
        | IdInfo::ModeConnecte => FormatInfo::FormatBool,

        /* Char */
        IdInfo::TypeDistribution => FormatInfo::FormatChar,

        /* U8 */
        IdInfo::CodeDefaut
        | IdInfo::CodeProduit
        | IdInfo::TypeCompteur
        | IdInfo::CodeProduitCompartiment(_) => FormatInfo::FormatU8,

        /* U16 */
        IdInfo::IndexSansRaz
        | IdInfo::IndexJournalier
        | IdInfo::Quantieme
        | IdInfo::HeureDebut
        | IdInfo::HeureFin
        | IdInfo::NbMesuragesQuantieme
        | IdInfo::NbFractionnements
        | IdInfo::IndexFractionnement
        | IdInfo::NbJEvents => FormatInfo::FormatU16,

        /* U32 */
        IdInfo::Totalisateur
        | IdInfo::QuantitePrincipale
        | IdInfo::QuantiteSecondaire
        | IdInfo::Predetermination
        | IdInfo::Date
        | IdInfo::Heure
        | IdInfo::QuantiteCompartiment(_) => FormatInfo::FormatU32,

        /* U64 */
        IdInfo::DateHeure => FormatInfo::FormatU64,

        /* F32 */
        IdInfo::DebitInstant | IdInfo::TemperatureInstant | IdInfo::TemperatureMoyen => {
            FormatInfo::FormatF32
        }

        /* String */
        IdInfo::IdentificationTag => FormatInfo::FormatString(100),
        IdInfo::ReferenceEtImmatriculation => FormatInfo::FormatString(15),
        IdInfo::VersionLogiciel => FormatInfo::FormatString(10),
        IdInfo::LibelleProduit => FormatInfo::FormatString(LIBELLE_PRODUIT_WIDTH),
        IdInfo::LibelleTableProduits(_prod_num) => FormatInfo::FormatString(LIBELLE_PRODUIT_WIDTH),
        IdInfo::DataJEvent => FormatInfo::FormatString(12),
        IdInfo::LibelleJEvent => FormatInfo::FormatString(40),
    }
}

impl Context {
    pub fn get_info_bool(&self, id_info: IdInfo) -> Option<bool> {
        match id_info {
            IdInfo::Ack => self.ack,
            IdInfo::Nack => self.nack,
            IdInfo::EnMesurage => self.en_mesurage,
            IdInfo::ArretIntermediaire => self.arret_intermediaire,
            IdInfo::ForcagePetitDebit => self.forcage_petit_debit,
            IdInfo::ModeConnecte => self.mode_connecte,

            _ => panic!("Cette information n'est pas booléenne : {id_info:?}"),
        }
    }

    pub fn set_info_bool(&mut self, id_info: IdInfo, value: bool) {
        match id_info {
            IdInfo::Ack => self.ack = Some(value),
            IdInfo::Nack => self.nack = Some(value),
            IdInfo::EnMesurage => self.en_mesurage = Some(value),
            IdInfo::ArretIntermediaire => self.arret_intermediaire = Some(value),
            IdInfo::ForcagePetitDebit => self.forcage_petit_debit = Some(value),
            IdInfo::ModeConnecte => self.mode_connecte = Some(value),

            _ => panic!("Cette information n'est pas booléenne : {id_info:?}"),
        }
    }

    pub fn get_info_char(&self, id_info: IdInfo) -> Option<char> {
        match id_info {
            IdInfo::TypeDistribution => self.type_distribution,

            _ => panic!("Cette information n'est pas char : {id_info:?}"),
        }
    }

    pub fn set_info_char(&mut self, id_info: IdInfo, value: char) {
        match id_info {
            IdInfo::TypeDistribution => self.type_distribution = Some(value),

            _ => panic!("Cette information n'est pas char : {id_info:?}"),
        }
    }

    /// Getter particulier pour le code produit d'un compartiment
    /// (la table des codes produit par compartiment est construite par morceaux...)
    fn get_info_code_produit_compartiment(&self, compart_num: usize) -> Option<u8> {
        assert!(compart_num <= NB_COMPARTIMENTS);
        if self.code_produit_compartiment.len() <= compart_num {
            None
        } else {
            Some(self.code_produit_compartiment[compart_num])
        }
    }

    pub fn get_info_u8(&self, id_info: IdInfo) -> Option<u8> {
        match id_info {
            IdInfo::CodeDefaut => self.code_defaut,
            IdInfo::CodeProduit => self.code_produit,
            IdInfo::TypeCompteur => self.type_compteur,
            IdInfo::CodeProduitCompartiment(compart_num) => {
                self.get_info_code_produit_compartiment(compart_num)
            }

            _ => panic!("Cette information n'est pas u8 : {id_info:?}"),
        }
    }

    /// Setter particulier pour le code produit d'un compartiment
    /// (la table des codes produit par compartiment est construite par morceaux...)
    fn set_info_code_produit_compartiment(&mut self, compart_num: usize, value: u8) {
        assert!(compart_num <= NB_COMPARTIMENTS);
        while self.code_produit_compartiment.len() <= compart_num {
            self.code_produit_compartiment.push(0);
        }
        self.code_produit_compartiment[compart_num] = value;
    }

    pub fn set_info_u8(&mut self, id_info: IdInfo, value: u8) {
        match id_info {
            IdInfo::CodeDefaut => self.code_defaut = Some(value),
            IdInfo::CodeProduit => self.code_produit = Some(value),
            IdInfo::TypeCompteur => self.type_compteur = Some(value),
            IdInfo::CodeProduitCompartiment(compart_num) => {
                self.set_info_code_produit_compartiment(compart_num, value);
            }

            _ => panic!("Cette information n'est pas u8 : {id_info:?}"),
        }
    }

    pub fn get_info_u16(&self, id_info: IdInfo) -> Option<u16> {
        match id_info {
            IdInfo::IndexSansRaz => self.index_sans_raz,
            IdInfo::IndexJournalier => self.index_journalier,
            IdInfo::Quantieme => self.quantieme,
            IdInfo::HeureDebut => self.heure_debut,
            IdInfo::HeureFin => self.heure_fin,
            IdInfo::NbMesuragesQuantieme => self.nb_mesurages_quantieme,
            IdInfo::NbFractionnements => self.nb_fractionnements,
            IdInfo::IndexFractionnement => self.index_fractionnement,
            IdInfo::NbJEvents => self.nb_jevents,

            _ => panic!("Cette information n'est pas u16 : {id_info:?}"),
        }
    }

    pub fn set_info_u16(&mut self, id_info: IdInfo, value: u16) {
        match id_info {
            IdInfo::IndexSansRaz => self.index_sans_raz = Some(value),
            IdInfo::IndexJournalier => self.index_journalier = Some(value),
            IdInfo::Quantieme => self.quantieme = Some(value),
            IdInfo::HeureDebut => self.heure_debut = Some(value),
            IdInfo::HeureFin => self.heure_fin = Some(value),
            IdInfo::NbMesuragesQuantieme => self.nb_mesurages_quantieme = Some(value),
            IdInfo::NbFractionnements => self.nb_fractionnements = Some(value),
            IdInfo::IndexFractionnement => self.index_fractionnement = Some(value),
            IdInfo::NbJEvents => self.nb_jevents = Some(value),

            _ => panic!("Cette information n'est pas u16 : {id_info:?}"),
        }
    }

    /// Getter particulier pour la quantité d'un compartiment
    /// (la table des quantités par compartiment est construite par morceaux...)
    fn get_info_quantite_compartiment(&self, compart_num: usize) -> Option<u32> {
        assert!(compart_num <= NB_COMPARTIMENTS);
        if self.quantite_compartiment.len() <= compart_num {
            None
        } else {
            Some(self.quantite_compartiment[compart_num])
        }
    }

    pub fn get_info_u32(&self, id_info: IdInfo) -> Option<u32> {
        match id_info {
            IdInfo::Totalisateur => self.totalisateur,
            IdInfo::QuantitePrincipale => self.quantite_principale,
            IdInfo::QuantiteSecondaire => self.quantite_secondaire,
            IdInfo::Predetermination => self.predetermination,
            IdInfo::Date => self.date,
            IdInfo::Heure => self.heure,
            IdInfo::QuantiteCompartiment(compart_num) => {
                self.get_info_quantite_compartiment(compart_num)
            }

            _ => panic!("Cette information n'est pas u32 : {id_info:?}"),
        }
    }

    /// Setter particulier pour la quantité d'un compartiment
    /// (la table des quantités par compartiment est construite par morceaux...)
    fn set_info_quantite_compartiment(&mut self, compart_num: usize, value: u32) {
        assert!(compart_num <= NB_COMPARTIMENTS);
        while self.quantite_compartiment.len() <= compart_num {
            self.quantite_compartiment.push(0);
        }
        self.quantite_compartiment[compart_num] = value;
    }

    pub fn set_info_u32(&mut self, id_info: IdInfo, value: u32) {
        match id_info {
            IdInfo::Totalisateur => self.totalisateur = Some(value),
            IdInfo::QuantitePrincipale => self.quantite_principale = Some(value),
            IdInfo::QuantiteSecondaire => self.quantite_secondaire = Some(value),
            IdInfo::Predetermination => self.predetermination = Some(value),
            IdInfo::Date => self.date = Some(value),
            IdInfo::Heure => self.heure = Some(value),
            IdInfo::QuantiteCompartiment(compart_num) => {
                self.set_info_quantite_compartiment(compart_num, value);
            }

            _ => panic!("Cette information n'est pas u32 : {id_info:?}"),
        }
    }

    pub fn get_info_u64(&self, id_info: IdInfo) -> Option<u64> {
        match id_info {
            IdInfo::DateHeure => self.date_heure,

            _ => panic!("Cette information n'est pas u64 : {id_info:?}"),
        }
    }

    pub fn set_info_u64(&mut self, id_info: IdInfo, value: u64) {
        match id_info {
            IdInfo::DateHeure => self.date_heure = Some(value),

            _ => panic!("Cette information n'est pas u64 : {id_info:?}"),
        }
    }

    pub fn get_info_f32(&self, id_info: IdInfo) -> Option<f32> {
        match id_info {
            IdInfo::DebitInstant => self.debit_instant,
            IdInfo::TemperatureInstant => self.temperature_instant,
            IdInfo::TemperatureMoyen => self.temperature_moyen,

            _ => panic!("Cette information n'est pas f32 : {id_info:?}"),
        }
    }

    pub fn set_info_f32(&mut self, id_info: IdInfo, value: f32) {
        match id_info {
            IdInfo::DebitInstant => self.debit_instant = Some(value),
            IdInfo::TemperatureInstant => self.temperature_instant = Some(value),
            IdInfo::TemperatureMoyen => self.temperature_moyen = Some(value),

            _ => panic!("Cette information n'est pas f32 : {id_info:?}"),
        }
    }

    /// Getter particulier pour la tables des produits
    /// (la table des produits est construite par morceaux...)
    fn get_info_libelle_table_produits(&self, prod_num: usize) -> Option<String> {
        assert!(prod_num <= NB_PRODUITS);
        if self.libelle_table_produits.len() <= prod_num {
            None
        } else {
            Some(self.libelle_table_produits[prod_num].clone())
        }
    }

    pub fn get_info_string(&self, id_info: IdInfo) -> Option<String> {
        match id_info {
            IdInfo::IdentificationTag => self.identification_tag.clone(),
            IdInfo::ReferenceEtImmatriculation => self.reference_et_immatriculation.clone(),
            IdInfo::VersionLogiciel => self.version_logiciel.clone(),
            IdInfo::LibelleProduit => self.libelle_produit.clone(),
            IdInfo::LibelleTableProduits(prod_num) => {
                self.get_info_libelle_table_produits(prod_num)
            }
            IdInfo::DataJEvent => self.data_jevent.clone(),
            IdInfo::LibelleJEvent => self.libelle_jevent.clone(),

            _ => panic!("Cette information n'est pas string : {id_info:?}"),
        }
    }

    /// Setter particulier pour table des produits
    /// (la table des produits est construite par morceaux...)
    fn set_info_libelle_table_produits(&mut self, prod_num: usize, value: &str) {
        assert!(prod_num <= NB_PRODUITS);
        while self.libelle_table_produits.len() <= prod_num {
            self.libelle_table_produits.push("???".to_string());
        }
        let txt = if value.len() > LIBELLE_PRODUIT_WIDTH {
            // Tronque si libellé trop long
            // /!\ format! ne le fait pas...
            value[..LIBELLE_PRODUIT_WIDTH].to_string()
        } else {
            value.to_string()
        };
        self.libelle_table_produits[prod_num] = txt;
    }

    pub fn set_info_string(&mut self, id_info: IdInfo, value: &str) {
        match id_info {
            IdInfo::IdentificationTag => self.identification_tag = Some(value.to_string()),
            IdInfo::ReferenceEtImmatriculation => {
                self.reference_et_immatriculation = Some(value.to_string());
            }
            IdInfo::VersionLogiciel => self.version_logiciel = Some(value.to_string()),
            IdInfo::LibelleProduit => self.libelle_produit = Some(value.to_string()),
            IdInfo::LibelleTableProduits(prod_num) => {
                self.set_info_libelle_table_produits(prod_num, value);
            }
            IdInfo::DataJEvent => self.data_jevent = Some(value.to_string()),
            IdInfo::LibelleJEvent => self.libelle_jevent = Some(value.to_string()),

            _ => panic!("Cette information n'est pas string : {id_info:?}"),
        };
    }
}

/// Implémentation générique des getters/setters
pub trait CommonContextTrait<T> {
    fn get_info(&self, id_info: IdInfo) -> Option<T>;

    fn set_info(&mut self, id_info: IdInfo, value: T);
}

impl CommonContextTrait<bool> for Context {
    fn get_info(&self, id_info: IdInfo) -> Option<bool> {
        self.get_info_bool(id_info)
    }

    fn set_info(&mut self, id_info: IdInfo, value: bool) {
        self.set_info_bool(id_info, value);
    }
}

impl CommonContextTrait<char> for Context {
    fn get_info(&self, id_info: IdInfo) -> Option<char> {
        self.get_info_char(id_info)
    }

    fn set_info(&mut self, id_info: IdInfo, value: char) {
        self.set_info_char(id_info, value);
    }
}

impl CommonContextTrait<u8> for Context {
    fn get_info(&self, id_info: IdInfo) -> Option<u8> {
        self.get_info_u8(id_info)
    }

    fn set_info(&mut self, id_info: IdInfo, value: u8) {
        self.set_info_u8(id_info, value);
    }
}

impl CommonContextTrait<u16> for Context {
    fn get_info(&self, id_info: IdInfo) -> Option<u16> {
        self.get_info_u16(id_info)
    }

    fn set_info(&mut self, id_info: IdInfo, value: u16) {
        self.set_info_u16(id_info, value);
    }
}

impl CommonContextTrait<u32> for Context {
    fn get_info(&self, id_info: IdInfo) -> Option<u32> {
        self.get_info_u32(id_info)
    }

    fn set_info(&mut self, id_info: IdInfo, value: u32) {
        self.set_info_u32(id_info, value);
    }
}

impl CommonContextTrait<u64> for Context {
    fn get_info(&self, id_info: IdInfo) -> Option<u64> {
        self.get_info_u64(id_info)
    }

    fn set_info(&mut self, id_info: IdInfo, value: u64) {
        self.set_info_u64(id_info, value);
    }
}

impl CommonContextTrait<f32> for Context {
    fn get_info(&self, id_info: IdInfo) -> Option<f32> {
        self.get_info_f32(id_info)
    }

    fn set_info(&mut self, id_info: IdInfo, value: f32) {
        self.set_info_f32(id_info, value);
    }
}

impl CommonContextTrait<String> for Context {
    fn get_info(&self, id_info: IdInfo) -> Option<String> {
        self.get_info_string(id_info)
    }

    fn set_info(&mut self, id_info: IdInfo, value: String) {
        self.set_info_string(id_info, &value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::context::CommonContextTrait;

    // Cette fonction devrait être appelée avec des `IdInfo` de tous les `FormatInfo` possibles
    // Voir `test_get_set` ci-dessous
    fn check_id_code(context: &mut Context, id_info: IdInfo) {
        match self::get_info_format(id_info) {
            FormatInfo::FormatBool => {
                assert!(context.get_info_bool(id_info).is_none());
                for value in [true, false] {
                    context.set_info_bool(id_info, value);
                    assert_eq!(context.get_info_bool(id_info), Some(value));
                }
            }
            FormatInfo::FormatChar => {
                assert!(context.get_info_char(id_info).is_none());
                for value in ['A', 'B', 'é'] {
                    context.set_info_char(id_info, value);
                    assert_eq!(context.get_info_char(id_info), Some(value));
                }
            }
            FormatInfo::FormatU8 => {
                assert!(context.get_info_u8(id_info).is_none());
                for value in [0_u8, 10_u8, 100_u8] {
                    context.set_info_u8(id_info, value);
                    assert_eq!(context.get_info_u8(id_info), Some(value));
                }
            }
            FormatInfo::FormatU16 => {
                assert!(context.get_info_u16(id_info).is_none());
                for value in [0_u16, 1000_u16, 10_000_u16] {
                    context.set_info_u16(id_info, value);
                    assert_eq!(context.get_info_u16(id_info), Some(value));
                }
            }
            FormatInfo::FormatU32 => {
                assert!(context.get_info_u32(id_info).is_none());
                for value in [0_u32, 1000_u32, 100_000_u32] {
                    context.set_info_u32(id_info, value);
                    assert_eq!(context.get_info_u32(id_info), Some(value));
                }
            }
            FormatInfo::FormatU64 => {
                assert!(context.get_info_u64(id_info).is_none());
                for value in [0_u64, 100_000_u64, 100_000_000_u64, 100_000_000_000_000_u64] {
                    context.set_info_u64(id_info, value);
                    assert_eq!(context.get_info_u64(id_info), Some(value));
                }
            }
            FormatInfo::FormatF32 => {
                assert!(context.get_info_f32(id_info).is_none());
                for value in [0.0_f32, 1000.0_f32, -1000.0_f32, 100_000.0_f32] {
                    context.set_info_f32(id_info, value);
                    assert_eq!(context.get_info_f32(id_info), Some(value));
                }
            }
            FormatInfo::FormatString(_width) => {
                assert!(context.get_info_string(id_info).is_none());
                for value in ["", "ABC"] {
                    context.set_info_string(id_info, value);
                    assert_eq!(context.get_info_string(id_info), Some(value.to_string()));
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
        check_id_code(&mut context, IdInfo::HeureDebut);
        check_id_code(&mut context, IdInfo::HeureFin);
        check_id_code(&mut context, IdInfo::IdentificationTag);
        check_id_code(&mut context, IdInfo::ReferenceEtImmatriculation);
        check_id_code(&mut context, IdInfo::VersionLogiciel);
        check_id_code(&mut context, IdInfo::DateHeure);
        check_id_code(&mut context, IdInfo::TypeCompteur);
        check_id_code(&mut context, IdInfo::NbMesuragesQuantieme);
        check_id_code(&mut context, IdInfo::LibelleProduit);
        check_id_code(&mut context, IdInfo::NbFractionnements);
        for prod_num in 0..=NB_PRODUITS {
            check_id_code(&mut context, IdInfo::LibelleTableProduits(prod_num));
        }
        check_id_code(&mut context, IdInfo::IndexFractionnement);
        check_id_code(&mut context, IdInfo::TypeDistribution);
        check_id_code(&mut context, IdInfo::Date);
        check_id_code(&mut context, IdInfo::Heure);
        check_id_code(&mut context, IdInfo::NbJEvents);
        check_id_code(&mut context, IdInfo::DataJEvent);
        check_id_code(&mut context, IdInfo::LibelleJEvent);
        for compart_num in 0..=NB_COMPARTIMENTS {
            check_id_code(&mut context, IdInfo::CodeProduitCompartiment(compart_num));
            check_id_code(&mut context, IdInfo::QuantiteCompartiment(compart_num));
        }
    }

    #[test]
    #[should_panic]
    fn test_get_set_panic() {
        // Le getter va panic! si on demande une information avec un format différent
        // du format de cette info

        let context: Context = Context::default();

        // Lecture d'une température (F_32) dans un bool
        let _ = context.get_info_bool(IdInfo::TemperatureInstant);
    }

    #[test]
    fn test_get_set_generic() {
        let mut context: Context = Context::default();

        // On utilise ici les getters et les setters génériques (via `CommonContextTrait`)
        // Il faut bien préciser les types des informations à gérer et ne pas se tromper
        // sinon panic!

        context.set_info(IdInfo::EnMesurage, true);
        let my_value: Option<bool> = context.get_info(IdInfo::EnMesurage);
        assert_eq!(my_value, Some(true));

        context.set_info(IdInfo::TypeDistribution, 'C');
        let my_value: Option<char> = context.get_info(IdInfo::TypeDistribution);
        assert_eq!(my_value, Some('C'));

        context.set_info(IdInfo::CodeDefaut, 10_u8);
        let my_value: Option<u8> = context.get_info(IdInfo::CodeDefaut);
        assert_eq!(my_value, Some(10_u8));

        context.set_info(IdInfo::Predetermination, 1000_u32);
        let my_value: Option<u32> = context.get_info(IdInfo::Predetermination);
        assert_eq!(my_value, Some(1000_u32));

        context.set_info(IdInfo::TemperatureInstant, -12.3);
        let my_value: Option<f32> = context.get_info(IdInfo::TemperatureInstant);
        assert_eq!(my_value, Some(-12.3));

        context.set_info(IdInfo::LibelleTableProduits(5), "TEST".to_string());
        let my_value: Option<String> = context.get_info(IdInfo::LibelleTableProduits(5));
        assert_eq!(my_value, Some("TEST".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_get_set_generic_panic() {
        // Le getter générique va panic! si on demande une information avec un format différent
        // du format de cette info

        let context: Context = Context::default();

        // Lecture d'une température (F_32) dans un u32
        let _: Option<u32> = context.get_info(IdInfo::TemperatureInstant);
    }
}
