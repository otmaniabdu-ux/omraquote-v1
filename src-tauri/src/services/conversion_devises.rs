use rust_decimal::Decimal;
use crate::models::devis::Devis;

/// Convertit un montant exprimé dans une devise source en DZD en utilisant le taux verrouillé du devis.
pub fn convertir_vers_dzd(
    montant: Decimal,
    devise_source: &str,
    devis: &Devis,
) -> Result<Decimal, String> {
    let taux = match devise_source {
        "SAR" => devis.taux_sar_dzd,
        "USD" => devis.taux_usd_dzd,
        "EUR" => devis.taux_eur_dzd,
        "DZD" => Decimal::ONE, // Pas de conversion pour DZD
        _ => return Err(format!("Devise inconnue: {}", devise_source)),
    };
    Ok(montant * taux)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use crate::models::devis::Devis;
    use chrono::NaiveDate;

    fn create_test_devis() -> Devis {
        Devis {
            id: Some(1),
            numero_devis: "DEVIS-2026-07-001".to_string(),
            client_id: 1,
            date_creation: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            date_depart: NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
            date_retour: NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
            type_visa: "omra_standard".to_string(),
            assurance_medicale: true,
            devise_achat: "SAR".to_string(),
            taux_sar_dzd: dec!(30.5),
            taux_usd_dzd: dec!(140.0),
            taux_eur_dzd: dec!(150.0),
            marge_type: "pourcentage".to_string(),
            marge_valeur: dec!(10.0),
            cout_net_total: None,
            montant_marge: None,
            prix_vente_total: None,
            statut: "brouillon".to_string(),
            remise: Some(dec!(0.0)),
            notes_internes: None,
        }
    }

    #[test]
    fn test_convertir_sar_to_dzd() {
        let devis = create_test_devis();
        let montant = dec!(1000.0);
        let result = convertir_vers_dzd(montant, "SAR", &devis).unwrap();
        assert_eq!(result, dec!(30500.0)); // 1000 * 30.5
    }

    #[test]
    fn test_convertir_usd_to_dzd() {
        let devis = create_test_devis();
        let montant = dec!(100.0);
        let result = convertir_vers_dzd(montant, "USD", &devis).unwrap();
        assert_eq!(result, dec!(14000.0));
    }

    #[test]
    fn test_convertir_dzd_to_dzd() {
        let devis = create_test_devis();
        let montant = dec!(5000.0);
        let result = convertir_vers_dzd(montant, "DZD", &devis).unwrap();
        assert_eq!(result, dec!(5000.0));
    }

    #[test]
    fn test_devise_inconnue() {
        let devis = create_test_devis();
        let result = convertir_vers_dzd(dec!(100.0), "GBP", &devis);
        assert!(result.is_err());
    }
}