/// Builder de requête ST2150 pour le protocole ALMA IE
use crate::st2150::{field, protocol};

pub struct Requete {
    // Numéro de requête
    numero: u8,

    // Champ de la requête
    fields: Vec<field::Field>,
}

impl Requete {
    /// Constructeur
    pub fn new(numero: u8) -> Self {
        Self {
            numero,
            fields: vec![],
        }
    }

    /// Ajout d'un champ dans la requête
    pub fn add_field(&mut self, field: field::Field) {
        self.fields.push(field);
    }

    /// Création de la trame pour la requête
    pub fn to_frame(&self) -> Vec<u8> {
        let mut req = vec![];

        // STX au début
        req.push(protocol::STX);

        // Numéro de commande sur 2 octets
        let command_field = field::Field::from_number(self.numero, 2);
        req.extend(command_field.to_frame());

        // Toius le champs de la commande précédés d'un SEPARATOR
        for field in &self.fields {
            req.push(protocol::SEPARATOR);
            req.extend(field.to_frame());
        }

        // checksum précédé d'un SEPARATOR
        req.push(protocol::SEPARATOR);
        // Le checksum est calculé sur l'ensemble de la trame sans le STX initial mais
        // avec le SEPARATOR avant le checksum
        let checksum = protocol::calcul_checksum(&req[1..]);
        let checksum_field = field::Field::from_hexa(checksum, 2);
        req.extend(checksum_field.to_frame());

        // ETX final
        req.push(protocol::ETX);

        // et voilà :)
        req
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requete() {
        // Message 12...
        let mut req = Requete::new(12);

        // Avec un champ ABCD...
        let my_field = field::Field::from_str("ABCD", 4);
        req.add_field(my_field);

        let trame = req.to_frame();

        // Le checksum de la trame
        let checksum = protocol::calcul_checksum(&[
            0x31,
            0x32,
            protocol::SEPARATOR,
            0x41,
            0x42,
            0x43,
            0x44,
            protocol::SEPARATOR,
        ]);
        assert_eq!(checksum, 0x07); // Ici la valeur du checksum de cette trame

        // Donc la trame doit être
        assert_eq!(
            trame,
            vec![
                protocol::STX,
                0x31,
                0x32,
                protocol::SEPARATOR,
                0x41,
                0x42,
                0x43,
                0x44,
                protocol::SEPARATOR,
                0x30,
                0x37,
                protocol::ETX
            ]
        );
    }
}
