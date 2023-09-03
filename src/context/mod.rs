//! Informations 'atomiques' échangées par le protocole ALMA IE - ST2150

/// Structure pour toutes les informations 'atomiques'
///
/// Toutes les informations sont du type `Option<T>` avec une valeur à `None`
/// tant qu'aucune valeur n'a été explicitement assignée à l'information qui devient alors
/// `Some(quelque_chose)`
#[derive(Debug, Default)]
pub struct Context {
    /* Réponse Message 00 */
    /* ------------------ */
    /// En mesurage
    pub en_mesurage: Option<bool>,

    /// Code défaut en cours (Codage 'court' selon ST3274)
    /// 0 pour pas de défaut
    pub code_defaut: Option<u8>,

    /// En arrêt intermédiaire
    pub arret_intermediaire: Option<bool>,

    /// Forçage en petit débit
    pub forcage_petit_debit: Option<bool>,

    /// En mode connecté
    pub mode_connecte: Option<bool>,

    /* Réponse Message 10 */
    /* ------------------ */
    /// Totalisateur (en échelon = Litre ou kg)
    pub totalisateur: Option<u32>,

    /// Débit instantané en m3/h (ou tonne/h)
    pub debit_instant: Option<f32>,

    /// Quantité chargée (en échelon = Litre ou kg)
    pub quantite_chargee: Option<u32>,

    /// Température instantanée
    pub temperature_instant: Option<f32>,

    /// Prédétermination (en échelon = Litre ou kg)
    pub predetermination: Option<u32>,
}
