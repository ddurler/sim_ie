//! Gestion du contenu d'un champ du protocole ST2150

use crate::st2150::protocol;

/// Champ d'une requête ou d'une réponse
#[derive(Debug, PartialEq, PartialOrd)]
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
    #[allow(dead_code)]
    pub fn encode_binary(value: u8) -> Self {
        Self { data: vec![value] }
    }

    /// Extraction champ d'une valeur binaire (typiquement ACK ou NACK)
    /// # Panics
    /// panic! si le champ est d'une taille autre qu'un seul caractère
    #[allow(dead_code)]
    pub fn decode_binary(&self) -> u8 {
        assert_eq!(self.data.len(), 1);
        self.data[0]
    }

    /// Constructeur champ numérique entier (supposé positif)
    /// Transforme une valeur entière en un champ ASCII d'une taille définie (0 padded à gauche)
    /// Par exemple la valeur 2 sur une width de 2 retourne vec![0x30, 0x32]
    /// # panics
    /// panic! si la valeur est trop grande pour la taille demandée
    /// panic! si taille demandée = 0
    #[allow(dead_code)]
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

    /// Extraction d'un valeur numérique entière encodée en ASCII
    /// # panics
    /// panic! si les caractères ne sont pas des chiffres `b'0'..=b'9'`
    /// panic! si la valeur ne peut pas être convertie dans le type attendu
    #[allow(dead_code)]
    pub fn decode_number<T>(&self) -> T
    where
        T: std::convert::TryFrom<u64>,
    {
        let mut ret = 0_u64;
        for value in &self.data {
            assert!(*value >= b'0' && *value <= b'9');
            let value = u64::from(*value - b'0');
            ret = 10_u64 * ret + value;
        }
        ret.try_into()
            .unwrap_or_else(|_| panic!("Erreur conversion {ret}"))
    }

    /// Constructeur champ numérique entier signé (Le 1er car est un signe '+' ou '-')
    /// Transforme une valeur entière en un champ ASCII d'une taille définie (0 padded à gauche)
    /// Par exemple la valeur 2 sur une width de 2 retourne vec![0x30, 0x32]
    /// # panics
    /// panic! si la valeur est trop grande pour la taille demandée
    /// panic! si taille demandée = 0
    #[allow(dead_code)]
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

    /// Extraction d'un valeur numérique entière et signée encodée en ASCII
    /// # panics
    /// panic! si pas au moins un caractère
    /// panic! si le premier caractère n'est pas '+', '-' ou un chiffre
    /// panic! si les autres caractères ne sont pas des chiffres
    /// panic! si la valeur ne peut pas être convertie dans le type attendu
    #[allow(dead_code)]
    pub fn decode_signed_number<T>(&self) -> T
    where
        T: std::convert::TryFrom<i64>,
    {
        // Commence optionnellement par un signe + ou -
        let (is_negative, index_start) = match self.data.first().expect("Champ signé vide") {
            b'-' => (true, 1),
            b'+' => (false, 1),
            _ => (false, 0),
        };
        let mut ret = 0_i64;
        for value in &self.data[index_start..] {
            assert!(*value >= b'0' && *value <= b'9');
            let value = i64::from(*value - b'0');
            ret = 10_i64 * ret + value;
        }
        if is_negative {
            ret = -ret;
        }
        ret.try_into()
            .unwrap_or_else(|_| panic!("Erreur conversion {ret}"))
    }

    /// Constructeur champ chaîne de caractères
    /// Transforme une chaîne en un champ ASCII d'une taille définie (space padded à droite)
    /// Par exemple la valeur "ABC" sur une width de 4 retourne vec![0x41, 0x42, 0x43, 0x20]
    /// La chaîne est tronquée si trop grande pour la taille définie
    /// panic! si taille demandée = 0
    #[allow(dead_code)]
    pub fn encode_str(value: &str, width: usize) -> Self {
        assert!(width > 0);
        let str = format!("{value:width$}");
        // Tronque les caractères de fin car format! ne le fait pas...
        let data: Vec<u8> = str.as_bytes().iter().copied().take(width).collect();
        assert_eq!(data.len(), width);
        Self { data }
    }

    /// Extraction d'une chaîne de caractère
    #[allow(dead_code)]
    pub fn decode_str(&self) -> String {
        String::from_utf8(self.data.clone()).expect("Erreur conversion")
    }

    /// Constructeur champ en hexadécimal
    /// Transforme une valeur entière en un champ hexadécimal d'une taille définie (cars hexadécimal en majuscule)
    /// Par exemple 0xA23 sur une width de 4 retourne vec![0x30, 0x41, 0x32, 0x33]
    /// # panics
    /// panic! si la valeur est trop grande pour la taille demandée
    /// panic! si taille demandée = 0
    #[allow(dead_code)]
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

    /// Extraction d'un valeur hexa en ASCII
    /// # panics
    /// panic! si les caractères ne sont pas des chiffres hexadécimaux `b'0'..=b'9'`, `b'A'..=b'F'` ou `b'a'..=b'f'`
    /// panic! si la valeur ne peut pas être convertie dans le type attendu
    #[allow(dead_code)]
    pub fn decode_hexa<T>(&self) -> T
    where
        T: std::convert::TryFrom<u64>,
    {
        let mut ret = 0_u64;
        for value in &self.data {
            assert!(protocol::is_car_hexa(*value));
            let value = u64::from(protocol::car_hexa_to_value(*value));
            ret = 16_u64 * ret + value;
        }
        ret.try_into()
            .unwrap_or_else(|_| panic!("Erreur conversion {ret}"))
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
    fn test_decode_binary() {
        for value in [0_u8, 10_u8, 100_u8] {
            assert_eq!(Field::encode_binary(value).decode_binary(), value);
        }
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
    fn test_decode_number() {
        // Test type u8
        for value in [0_u8, 10_u8, 100_u8] {
            assert_eq!(Field::encode_number(value, 3).decode_number::<u8>(), value);
        }
        // Test type i8
        for value in [0_i8, 10_i8, 100_i8] {
            assert_eq!(Field::encode_number(value, 3).decode_number::<i8>(), value);
        }
        // Test type u16
        for value in [0_u16, 10_u16, 1_000_u16, 10_000_u16] {
            assert_eq!(Field::encode_number(value, 5).decode_number::<u16>(), value);
        }
        // Test type i16
        for value in [0_i16, 10_i16, 1_000_i16, 10_000_i16] {
            assert_eq!(Field::encode_number(value, 5).decode_number::<i16>(), value);
        }
        // Test type u32
        for value in [
            0_u32,
            10_u32,
            1000_u32,
            10_000_u32,
            100_000_u32,
            1_000_000_000_u32,
        ] {
            assert_eq!(
                Field::encode_number(value, 10).decode_number::<u32>(),
                value
            );
        }
        // Test type i32
        for value in [
            0_i32,
            10_i32,
            1000_i32,
            10_000_i32,
            100_000_i32,
            1_000_000_000_i32,
        ] {
            assert_eq!(
                Field::encode_number(value, 10).decode_number::<i32>(),
                value
            );
        }
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
    fn test_decode_hexa() {
        // Test type u8
        for value in [0x00_u8, 0xAB_u8, 0x9A_u8] {
            assert_eq!(Field::encode_hexa(value, 2).decode_hexa::<u8>(), value);
        }
        // Test type i8
        for value in [0x00_i8, 0x12_i8, 0x23_i8] {
            assert_eq!(Field::encode_hexa(value, 3).decode_hexa::<i8>(), value);
        }
        // Test type u16
        for value in [0x1234_u16, 0xABCD_u16] {
            assert_eq!(Field::encode_hexa(value, 4).decode_hexa::<u16>(), value);
        }
        // Test type i16
        for value in [0x123_i16, 0x9AB_i16] {
            assert_eq!(Field::encode_hexa(value, 4).decode_hexa::<i16>(), value);
        }
        // Test type u32
        for value in [0x0_u32, 0x1234_ABCD_u32] {
            assert_eq!(Field::encode_hexa(value, 8).decode_hexa::<u32>(), value);
        }
        // Test type i32
        for value in [0x4567, 0xB_A987] {
            assert_eq!(Field::encode_hexa(value, 10).decode_hexa::<i32>(), value);
        }
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
    fn test_decode_signed_number() {
        // Test type u8
        for value in [0_u8, 10_u8, 100_u8] {
            assert_eq!(
                Field::encode_signed_number(value, 4).decode_signed_number::<u8>(),
                value
            );
        }
        // Test type i8
        for value in [-10_i8, 0_i8, 10_i8] {
            assert_eq!(
                Field::encode_signed_number(value, 4).decode_signed_number::<i8>(),
                value
            );
        }
        // Test type u16
        for value in [0_u16, 10_u16, 1_000_u16] {
            assert_eq!(
                Field::encode_signed_number(value, 5).decode_signed_number::<u16>(),
                value
            );
        }
        // Test type i16
        for value in [-1_000_i16, -10_i16, 0_i16, 10_i16, 1_000_i16] {
            assert_eq!(
                Field::encode_signed_number(value, 5).decode_signed_number::<i16>(),
                value
            );
        }
        // Test type u32
        for value in [0_u32, 10_u32, 1000_u32, 10_000_u32, 100_000_u32] {
            assert_eq!(
                Field::encode_signed_number(value, 10).decode_signed_number::<u32>(),
                value
            );
        }
        // Test type i32
        for value in [
            -100_000_i32,
            -10_000_i32,
            -1000_i32,
            -10_i32,
            0_i32,
            10_i32,
            1000_i32,
            10_000_i32,
            100_000_i32,
        ] {
            assert_eq!(
                Field::encode_signed_number(value, 10).decode_signed_number::<i32>(),
                value
            );
        }
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

    #[test]
    fn test_decode_str() {
        for value in ["HELLO", "123  ", " TOTO"] {
            assert_eq!(Field::encode_str(value, 5).decode_str(), value);
        }
    }
}
