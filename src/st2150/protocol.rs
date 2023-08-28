/// Helper pour les requêtes et les réponses du protocole ALMA IE selon la ST 2150
///
/// Une trame du protocole IE est au format :
/// `STX + numéro_command_as_N2 + { SEPARATOR + champ }* + SEPARATOR + lrc_as_Hex2) + STX`
///
/// (Le LRC n'intègre pas le STX et intègre le SEPARATOR qui le précède)

/// Début de message
pub const STX: u8 = 0x02;

/// Séparateur de champ
pub const SEPARATOR: u8 = 0xFE;

/// Fin de message
pub const ETX: u8 = 0x03;

/// Acquit de message
pub const ACK: u8 = 0x06;

/// Non-acquit de message
pub const NACK: u8 = 0x15;

/// Calcul du checksum pour trame (XOR des octets)
/// Attention : Dans la ST2150, le checksum n'intègre pas le STX initial et contient un SEPARATOR avant
pub fn calcul_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0_u8, |lrc, byte| lrc ^ (*byte))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        // Tests de la ST 2150 §7
        assert_eq!(calcul_checksum(&[0x32, 0x32, 0xFE]), 0xFE);
        assert_eq!(calcul_checksum(&[0x32, 0x32, 0xFE, 0x06, 0xFE]), 0x06);
        assert_eq!(
            calcul_checksum(&[
                0x32, 0x31, 0xFE, 0x30, 0x31, 0x30, 0x30, 0x30, 0xFE, 0x31, 0xFE, 0x30, 0xFE, 0x31,
                0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0xFE
            ]),
            0xC5
        );
    }
}
