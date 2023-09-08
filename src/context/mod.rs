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

/// Nombre max de produit
const NB_PRODUITS: usize = 16;

/// Nombre de caractères pour un libellé produit
const LIBELLE_PRODUIT_WIDTH: usize = 10;

/// Format possible d'une information du contexte
#[derive(Clone, Debug)]
pub enum FormatInfo {
    FormatBool,
    FormatU8,
    FormatU32,
    FormatF32,
    FormatString(usize),
}

/// Énumération des informations du contexte
#[derive(Clone, Copy, Debug)]
pub enum IdInfo {
    EnMesurage,
    CodeDefaut,
    ArretIntermediaire,
    ForcagePetitDebit,
    ModeConnecte,
    Totalisateur,
    DebitInstant,
    QuantiteChargee,
    TemperatureInstant,
    Predetermination,
    CodeProduit,
    Ack,
    LibelleProduit(usize),
}

/// Dictionnaire des données pour les requêtes et les réponses
#[derive(Debug, Default)]
pub struct Context {
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

    /// Quantité chargée (en échelon = Litre ou kg)
    quantite_chargee: Option<u32>,

    /// Température instantanée
    temperature_instant: Option<f32>,

    /// Prédétermination (en échelon = Litre ou kg)
    predetermination: Option<u32>,

    /// Code produit
    code_produit: Option<u8>,

    /// ACK du dernier message
    ack: Option<bool>,

    /* Pour + tard... */
    /// Libellés des max. NB_PRODUITS produits
    libelle_produits: Vec<String>,
}

/// Retourne le libellé d'un information du contexte
pub fn get_info_name(id_info: IdInfo) -> String {
    match id_info {
        IdInfo::EnMesurage => "En mesurage".to_string(),
        IdInfo::CodeDefaut => "Code défaut".to_string(),
        IdInfo::ArretIntermediaire => "Arrêt intermédiaire".to_string(),
        IdInfo::ForcagePetitDebit => "Forçage petit débit".to_string(),
        IdInfo::ModeConnecte => "Mode connecté".to_string(),
        IdInfo::Totalisateur => "Totalisateur".to_string(),
        IdInfo::DebitInstant => "Débit instantané".to_string(),
        IdInfo::QuantiteChargee => "Quantité chargée".to_string(),
        IdInfo::TemperatureInstant => "Température instantanée".to_string(),
        IdInfo::Predetermination => "Prédétermination".to_string(),
        IdInfo::CodeProduit => "Code produit".to_string(),
        IdInfo::Ack => "Acquit message".to_string(),
        IdInfo::LibelleProduit(prod_num) => format!("Libellé produit #{prod_num}"),
    }
}

/// Retourne le type de format pour une information du contexte
pub fn get_info_format(id_info: IdInfo) -> FormatInfo {
    match id_info {
        /* Booléen */
        IdInfo::EnMesurage
        | IdInfo::ArretIntermediaire
        | IdInfo::ForcagePetitDebit
        | IdInfo::ModeConnecte
        | IdInfo::Ack => FormatInfo::FormatBool,

        /* U8 */
        IdInfo::CodeDefaut | IdInfo::CodeProduit => FormatInfo::FormatU8,

        /* U32 */
        IdInfo::Totalisateur | IdInfo::QuantiteChargee | IdInfo::Predetermination => {
            FormatInfo::FormatU32
        }

        /* F32 */
        IdInfo::DebitInstant | IdInfo::TemperatureInstant => FormatInfo::FormatF32,

        /* String */
        IdInfo::LibelleProduit(_prod_num) => FormatInfo::FormatString(LIBELLE_PRODUIT_WIDTH),
    }
}

impl Context {
    pub fn get_info_bool(&self, id_info: IdInfo) -> Option<bool> {
        match id_info {
            IdInfo::EnMesurage => self.en_mesurage,
            IdInfo::ArretIntermediaire => self.arret_intermediaire,
            IdInfo::ForcagePetitDebit => self.forcage_petit_debit,
            IdInfo::ModeConnecte => self.mode_connecte,
            IdInfo::Ack => self.ack,

            _ => panic!("Cette information n'est pas booléenne : {id_info:?}"),
        }
    }

    pub fn set_info_bool(&mut self, id_info: IdInfo, value: bool) {
        match id_info {
            IdInfo::EnMesurage => self.en_mesurage = Some(value),
            IdInfo::ArretIntermediaire => self.arret_intermediaire = Some(value),
            IdInfo::ForcagePetitDebit => self.forcage_petit_debit = Some(value),
            IdInfo::ModeConnecte => self.mode_connecte = Some(value),
            IdInfo::Ack => self.ack = Some(value),

            _ => panic!("Cette information n'est pas booléenne : {id_info:?}"),
        }
    }

    pub fn get_info_u8(&self, id_info: IdInfo) -> Option<u8> {
        match id_info {
            IdInfo::CodeDefaut => self.code_defaut,
            IdInfo::CodeProduit => self.code_produit,

            _ => panic!("Cette information n'est pas u8 : {id_info:?}"),
        }
    }

    pub fn set_info_u8(&mut self, id_info: IdInfo, value: u8) {
        match id_info {
            IdInfo::CodeDefaut => self.code_defaut = Some(value),
            IdInfo::CodeProduit => self.code_produit = Some(value),

            _ => panic!("Cette information n'est pas u8 : {id_info:?}"),
        }
    }

    pub fn get_info_u32(&self, id_info: IdInfo) -> Option<u32> {
        match id_info {
            IdInfo::Totalisateur => self.totalisateur,
            IdInfo::QuantiteChargee => self.quantite_chargee,
            IdInfo::Predetermination => self.predetermination,

            _ => panic!("Cette information n'est pas u32 : {id_info:?}"),
        }
    }

    pub fn set_info_u32(&mut self, id_info: IdInfo, value: u32) {
        match id_info {
            IdInfo::Totalisateur => self.totalisateur = Some(value),
            IdInfo::QuantiteChargee => self.quantite_chargee = Some(value),
            IdInfo::Predetermination => self.predetermination = Some(value),

            _ => panic!("Cette information n'est pas u32 : {id_info:?}"),
        }
    }

    pub fn get_info_f32(&self, id_info: IdInfo) -> Option<f32> {
        match id_info {
            IdInfo::DebitInstant => self.debit_instant,
            IdInfo::TemperatureInstant => self.temperature_instant,

            _ => panic!("Cette information n'est pas f32 : {id_info:?}"),
        }
    }

    pub fn set_info_f32(&mut self, id_info: IdInfo, value: f32) {
        match id_info {
            IdInfo::DebitInstant => self.debit_instant = Some(value),
            IdInfo::TemperatureInstant => self.temperature_instant = Some(value),

            _ => panic!("Cette information n'est pas f32 : {id_info:?}"),
        }
    }

    /// Getter particulier pour les produits
    /// (la table des produits est construite par morceaux...)
    fn get_info_libelle_produits(&self, prod_num: usize) -> Option<String> {
        assert!(prod_num <= NB_PRODUITS);
        if self.libelle_produits.len() <= prod_num {
            None
        } else {
            Some(self.libelle_produits[prod_num].clone())
        }
    }

    pub fn get_info_string(&self, id_info: IdInfo) -> Option<String> {
        match id_info {
            IdInfo::LibelleProduit(prod_num) => self.get_info_libelle_produits(prod_num),

            _ => panic!("Cette information n'est pas string : {id_info:?}"),
        }
    }

    /// Setter particulier pour les produits
    /// (la table des produits est construite par morceaux...)
    fn set_info_libelle_produits(&mut self, prod_num: usize, value: &str) {
        assert!(prod_num <= NB_PRODUITS);
        while self.libelle_produits.len() <= prod_num {
            self.libelle_produits.push("???".to_string());
        }
        let txt = if value.len() > LIBELLE_PRODUIT_WIDTH {
            // Tronque si libellé trop long
            // /!\ format! ne le fait pas...
            value[..LIBELLE_PRODUIT_WIDTH].to_string()
        } else {
            value.to_string()
        };
        self.libelle_produits[prod_num] = txt;
    }

    pub fn set_info_string(&mut self, id_info: IdInfo, value: &str) {
        match id_info {
            IdInfo::LibelleProduit(prod_num) => self.set_info_libelle_produits(prod_num, value),

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

impl CommonContextTrait<u8> for Context {
    fn get_info(&self, id_info: IdInfo) -> Option<u8> {
        self.get_info_u8(id_info)
    }

    fn set_info(&mut self, id_info: IdInfo, value: u8) {
        self.set_info_u8(id_info, value);
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

    #[test]
    fn test_get_set() {
        fn check_id_code(context: &mut Context, id_info: IdInfo) {
            match self::get_info_format(id_info) {
                FormatInfo::FormatBool => {
                    assert!(context.get_info_bool(id_info).is_none());
                    for value in [true, false] {
                        context.set_info_bool(id_info, value);
                        assert_eq!(context.get_info_bool(id_info), Some(value));
                    }
                }
                FormatInfo::FormatU8 => {
                    assert!(context.get_info_u8(id_info).is_none());
                    for value in [0_u8, 10_u8, 100_u8] {
                        context.set_info_u8(id_info, value);
                        assert_eq!(context.get_info_u8(id_info), Some(value));
                    }
                }
                FormatInfo::FormatU32 => {
                    assert!(context.get_info_u32(id_info).is_none());
                    for value in [0_u32, 1000_u32, 100_000_u32] {
                        context.set_info_u32(id_info, value);
                        assert_eq!(context.get_info_u32(id_info), Some(value));
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

        let mut context = Context::default();

        // Pas besoin de mettre ici tous les IdInfos... Au moins tester les différents formats :)
        // TODO : Pas réussi à mettre en oeuvre le crate `enum-iterator` qui permettrait d'itérer
        //        sur toutes les valeurs d'un Enum :(
        check_id_code(&mut context, IdInfo::EnMesurage);
        check_id_code(&mut context, IdInfo::CodeDefaut);
        check_id_code(&mut context, IdInfo::ArretIntermediaire);
        check_id_code(&mut context, IdInfo::ForcagePetitDebit);
        check_id_code(&mut context, IdInfo::ModeConnecte);
        check_id_code(&mut context, IdInfo::Totalisateur);
        check_id_code(&mut context, IdInfo::DebitInstant);
        check_id_code(&mut context, IdInfo::QuantiteChargee);
        check_id_code(&mut context, IdInfo::TemperatureInstant);
        check_id_code(&mut context, IdInfo::Predetermination);
        check_id_code(&mut context, IdInfo::CodeProduit);
        check_id_code(&mut context, IdInfo::Ack);

        for prod_num in 0..=NB_PRODUITS {
            check_id_code(&mut context, IdInfo::LibelleProduit(prod_num));
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

        context.set_info(IdInfo::CodeDefaut, 10_u8);
        let my_value: Option<u8> = context.get_info(IdInfo::CodeDefaut);
        assert_eq!(my_value, Some(10_u8));

        context.set_info(IdInfo::Predetermination, 1000_u32);
        let my_value: Option<u32> = context.get_info(IdInfo::Predetermination);
        assert_eq!(my_value, Some(1000_u32));

        context.set_info(IdInfo::TemperatureInstant, -12.3);
        let my_value: Option<f32> = context.get_info(IdInfo::TemperatureInstant);
        assert_eq!(my_value, Some(-12.3));

        context.set_info(IdInfo::LibelleProduit(5), "TEST".to_string());
        let my_value: Option<String> = context.get_info(IdInfo::LibelleProduit(5));
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
