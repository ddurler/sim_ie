//! Gestion du contenu d'un champ du protocole ST2150

/// Champ d'une requête ou d'une réponse
pub struct Field {
    /// Table des octets d'un champ
    data: Vec<u8>,
}

impl Field {
    /// Constructeur champ avec un array de u8
    pub fn new(field: &[u8]) -> Self {
        let mut data = vec![];
        for v in field {
            data.push(*v);
        }
        Self { data }
    }

    /// Constructeur champ avec une valeur binaire (typiquement ACK ou NACK)
    /// (Ne peut donner qu'un champ d'une longueur de 1 octet)
    pub fn encode_binary(value: u8) -> Self {
        Self { data: vec![value] }
    }

    /// Constructeur champ numérique entier (supposé positif)
    /// Transforme une valeur entière en un champ ASCII d'une taille définie (0 padded à gauche)
    /// Par exemple la valeur 2 sur une width de 2 retourne vec![0x30, 0x32]
    /// # panics
    /// panic! si la valeur est trop grande pour la taille demandée
    /// panic! si taille demandée = 0
    pub fn encode_number<T>(value: T, width: usize) -> Self
    where
        T: std::fmt::Display,
    {
        assert!(width > 0);
        let str = format!("{value:0width$}");
        let data = str.as_bytes().to_vec();
        assert_eq!(data.len(), width);
        Self { data }
    }

    /// Constructeur champ numérique entier signé (Le 1er car est un signe '+' ou '-')
    /// Transforme une valeur entière en un champ ASCII d'une taille définie (0 padded à gauche)
    /// Par exemple la valeur 2 sur une width de 2 retourne vec![0x30, 0x32]
    /// # panics
    /// panic! si la valeur est trop grande pour la taille demandée
    /// panic! si taille demandée = 0
    pub fn encode_signed_number<T>(value: T, width: usize) -> Self
    where
        T: std::fmt::Display,
    {
        assert!(width > 0);
        let str = format!("{value:+0width$}");
        let data = str.as_bytes().to_vec();
        assert_eq!(data.len(), width);
        Self { data }
    }

    /// Constructeur champ chaîne de caractères
    /// Transforme une chaîne en un champ ASCII d'une taille définie (space padded à droite)
    /// Par exemple la valeur "ABC" sur une width de 4 retourne vec![0x41, 0x42, 0x43, 0x20]
    /// La chaîne est tronquée si trop grande pour la taille définie
    /// panic! si taille demandée = 0
    pub fn encode_str(value: &str, width: usize) -> Self {
        assert!(width > 0);
        let str = format!("{value:width$}");
        // Tronque les caractères de fin car format! ne le fait pas...
        let data: Vec<u8> = str.as_bytes().iter().copied().take(width).collect();
        assert_eq!(data.len(), width);
        Self { data }
    }

    /// Constructeur champ en hexadécimal
    /// Transforme une valeur entière en un champ hexadécimal d'une taille définie (cars hexadécimal en majuscule)
    /// Par exemple 0xA23 sur une width de 4 retourne vec![0x30, 0x41, 0x32, 0x33]
    /// # panics
    /// panic! si la valeur est trop grande pour la taille demandée
    /// panic! si taille demandée = 0
    pub fn encode_hexa<T>(value: T, width: usize) -> Self
    where
        T: std::fmt::Display + std::fmt::UpperHex,
    {
        assert!(width > 0);
        // format! va ajouter un "0x" devant (on ajoute +2 à la taille demandée)
        let w = width + 2;
        let str = format!("{value:#0w$X}");
        // Supprime les 2 premiers caractères
        let data: Vec<u8> = str.as_bytes().iter().copied().skip(2).collect();
        assert_eq!(data.len(), width);
        Self { data }
    }

    /// Trame pour la requête
    pub fn to_frame(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let f = Field::new(&[1, 2, 3]);
        assert_eq!(f.to_frame(), vec![1, 2, 3]);
    }

    #[test]
    fn test_encode_binary() {
        let f = Field::encode_binary(0xAB);
        assert_eq!(f.to_frame(), vec![0xAB]);
    }

    #[test]
    fn test_encode_number() {
        // 0, width=1 -> '0'
        let f = Field::encode_number(0, 1);
        assert_eq!(f.to_frame(), vec![0x30]);

        // 0, width=2 -> '00'
        let f = Field::encode_number(0, 2);
        assert_eq!(f.to_frame(), vec![0x30, 0x30]);

        // 5, width=1 -> '5'
        let f = Field::encode_number(5, 1);
        assert_eq!(f.to_frame(), vec![0x35]);

        // 5, width=2 -> '05'
        let f = Field::encode_number(5, 2);
        assert_eq!(f.to_frame(), vec![0x30, 0x35]);

        // 5, width=2 -> '05'
        let f = Field::encode_number(5, 2);
        assert_eq!(f.to_frame(), vec![0x30, 0x35]);

        // 56, width=2 -> '56'
        let f = Field::encode_number(56, 2);
        assert_eq!(f.to_frame(), vec![0x35, 0x36]);
    }

    #[test]
    #[should_panic]
    fn test_panic_encode_number() {
        // 567, width=2 -> '56'  /!\ ça dépasse en panic!
        let _ = Field::encode_number(567, 2);
    }

    #[test]
    fn test_encode_hexa() {
        // 0x1A, width=2 -> '1A'
        let f = Field::encode_hexa(0x1A, 2);
        assert_eq!(f.to_frame(), vec![0x31, 0x41]);

        // 0x1A, width=3 -> '01A'
        let f = Field::encode_hexa(0x1A, 3);
        assert_eq!(f.to_frame(), vec![0x30, 0x31, 0x41]);

        // 0x1A, width=4 -> '001A'
        let f = Field::encode_hexa(0x1A, 4);
        assert_eq!(f.to_frame(), vec![0x30, 0x30, 0x31, 0x41]);

        // 0xABCD, width=4 -> 'ABCD'
        let f = Field::encode_hexa(0xABCD, 4);
        assert_eq!(f.to_frame(), vec![0x41, 0x42, 0x43, 0x44]);
    }

    #[test]
    #[should_panic]
    fn test_panic_encode_hexa() {
        // 0x1234, width=2 -> '1234'  /!\ ça dépasse en panic!
        let _ = Field::encode_hexa(0x1234, 2);
    }

    #[test]
    fn test_encode_signed_number() {
        // 0, width=3 -> '+00'
        let f = Field::encode_signed_number(0, 3);
        assert_eq!(f.to_frame(), vec![b'+', 0x30, 0x30]);

        // 12, width=4 -> '+012'
        let f = Field::encode_signed_number(12, 4);
        assert_eq!(f.to_frame(), vec![b'+', 0x30, 0x31, 0x32]);

        // -12, width=4 -> '-012'
        let f = Field::encode_signed_number(-12, 4);
        assert_eq!(f.to_frame(), vec![b'-', 0x30, 0x31, 0x32]);

        // -123, width=4 -> '-123'
        let f = Field::encode_signed_number(-123, 4);
        assert_eq!(f.to_frame(), vec![b'-', 0x31, 0x32, 0x33]);
    }

    #[test]
    #[should_panic]
    fn test_panic_encode_signed_number() {
        // -1234, width=2 -> '-1234'  /!\ ça dépasse en panic!
        let _ = Field::encode_hexa(-1234, 2);
    }

    #[test]
    fn test_encode_str() {
        // "ABC", width 2 -> 'AB'
        let f = Field::encode_str("ABC", 2);
        assert_eq!(f.to_frame(), vec![0x41, 0x42]);

        // "ABC", width 3 -> 'ABC'
        let f = Field::encode_str("ABC", 3);
        assert_eq!(f.to_frame(), vec![0x41, 0x42, 0x43]);

        // "ABC", width 4 -> 'ABC '
        let f = Field::encode_str("ABC", 4);
        assert_eq!(f.to_frame(), vec![0x41, 0x42, 0x43, 0x20]);

        // "ABC", width 5 -> 'ABC '
        let f = Field::encode_str("ABC", 5);
        assert_eq!(f.to_frame(), vec![0x41, 0x42, 0x43, 0x20, 0x20]);
    }
}
