//! Helper pour les requêtes et les réponses du protocole ALMA IE selon la ST 2150
//!
//! Une trame du protocole IE est au format :
//! `STX + numéro_command_as_N2 + { SEPARATOR + champ }* + SEPARATOR + lrc_as_Hex2) + STX`
//!
//! (Le LRC n'intègre pas le STX et intègre le SEPARATOR qui le précède)

use std::time::SystemTime;

use crate::serial_com::{CommonSerialComTrait, SerialCom};

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

/// Timeout réception réponse (en seconde) : Absence de toute réponse
pub const TIMEOUT_READ_FRAME: f32 = 1.0;

/// Timeout fin de trame (en seconde) : Si reçu quelque chose mais pas assez
pub const TIMEOUT_END_FRAME: f32 = 0.3;

/// Helper pour vérifier qu'un caractère est de l'hexadécimal
pub fn is_car_hexa(car: u8) -> bool {
    matches!(car, b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f')
}

/// Helper pour convertir un caractère hexadécimal en binaire décimal
pub fn car_hexa_to_value(car: u8) -> u8 {
    match car {
        b'0'..=b'9' => car - b'0',
        b'A'..=b'F' => car - b'A' + 10,
        b'a'..=b'f' => car - b'a' + 10,
        _ => 0,
    }
}

/// Calcul du checksum pour trame (XOR des octets)
/// Attention : Dans la ST2150, le checksum n'intègre pas le STX initial et contient un SEPARATOR avant
pub fn calcul_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0_u8, |lrc, byte| lrc ^ (*byte))
}

/// Primitive générique pour attendre une réponse sur la liaison série
/// `port` : Référence au port série (true ou FAKE) à utiliser
/// `buffer` : Buffer pour les octets reçus sur le port
/// `max_expected_len` : Longueur max. de la réponse attendue. Dès que ce nombre max. est reçu,
/// la fonction retourne. Sinon, c'est le timeout qui agit (un timeout différent entre aucune réponse
/// et un timeout inter-caractères)
pub fn waiting_frame(port: &mut SerialCom, buffer: &mut [u8], max_expected_len: usize) -> usize {
    let mut total_len_received = 0;
    let mut start_time = SystemTime::now();

    // Boucle de lecture du port série
    loop {
        let len_received = port.read(&mut buffer[total_len_received..]);
        if len_received > 0 {
            // Ré-arme le timer si on a reçu qq. chose
            total_len_received += len_received;
            start_time = SystemTime::now();
        }
        if total_len_received >= max_expected_len {
            // On a reçu au moins le nombre max d'octets attendus, on retourne
            return total_len_received;
        }
        if total_len_received > 0 {
            // On a reçu qq. chose (mais pas le max assez), c'est le timeout fin de trame qui compte
            if let Ok(elapsed) = start_time.elapsed() {
                if elapsed.as_secs_f32() > TIMEOUT_END_FRAME {
                    return total_len_received;
                }
            }
        } else {
            // Absolument rien reçu en réponse
            if let Ok(elapsed) = start_time.elapsed() {
                if elapsed.as_secs_f32() > TIMEOUT_READ_FRAME {
                    return 0;
                }
            }
        }
    }
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

    #[test]
    fn test_waiting_frame() {
        // On utilise le FAKE serial port pour simuler un réponse...
        let mut fake_port = SerialCom::new("FAKE", 9600);

        fake_port.will_read(&[0x01, 0x02, 0x03]);

        let mut buffer = [0; 500];
        let rep_len = waiting_frame(&mut fake_port, &mut buffer, 3);

        assert_eq!(rep_len, 3);
        assert_eq!(buffer[0..3], [0x01, 0x02, 0x03]);
    }
}
