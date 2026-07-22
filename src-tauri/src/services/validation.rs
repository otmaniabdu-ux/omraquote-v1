use chrono::{NaiveDate, Months};
use rust_decimal::Decimal;
use crate::error::{AppResult, AppError};

// Re-export de la source unique pour maintenir l'API publique (rétrocompatibilité)
pub use crate::services::alerte_passeport::alerte_passeport;

/// Valide qu'une date de départ est antérieure à une date de retour.
pub fn valider_dates_sejour(date_depart: NaiveDate, date_retour: NaiveDate) -> AppResult<()> {
    if date_retour <= date_depart {
        return Err(AppError::Validation(
            "La date de retour doit être postérieure à la date de départ.".to_string()
        ));
    }
    Ok(())
}

/// Valide qu'une date de checkin est antérieure à une date de checkout.
pub fn valider_dates_hebergement(checkin: NaiveDate, checkout: NaiveDate) -> AppResult<()> {
    if checkout <= checkin {
        return Err(AppError::Validation(
            "La date de checkout doit être postérieure à la date de checkin.".to_string()
        ));
    }
    Ok(())
}

/// Valide qu'une date de vol est comprise dans la période du séjour (optionnel).
pub fn valider_date_vol_dans_sejour(date_vol: NaiveDate, depart: NaiveDate, retour: NaiveDate) -> AppResult<()> {
    if date_vol < depart || date_vol > retour {
        return Err(AppError::Validation(format!(
            "La date de vol ({}) est en dehors de la période du séjour ({} - {}).",
            date_vol, depart, retour
        )));
    }
    Ok(())
}

/// Wrapper pour les commandes Tauri : valide les dates d'un devis.
/// Signature compatible avec `commands::validation::valider_dates_devis_command`.
pub fn valider_dates_devis(date_depart: NaiveDate, date_retour: NaiveDate) -> AppResult<()> {
    valider_dates_sejour(date_depart, date_retour)
}

/// Wrapper pour les commandes Tauri : valide les dates d'un hébergement.
pub fn valider_hebergement(
    checkin: NaiveDate,
    checkout: NaiveDate,
    _date_depart: NaiveDate,
    _date_retour: NaiveDate,
) -> AppResult<()> {
    valider_dates_hebergement(checkin, checkout)
}

/// Vérifie les alertes passeport pour tous les passagers d'un devis.
pub fn verifier_alertes_passeports(
    passagers: &[crate::models::passager::Passager],
    date_retour: NaiveDate,
) -> Vec<(i64, String)> {
    passagers
        .iter()
        .filter_map(|p| {
            if let Some(exp) = p.date_expiration_passeport {
                if alerte_passeport(exp, date_retour) {
                    return Some((p.id.unwrap_or(0), format!("Passeport expirant le {} pour le passager {}", exp, p.nom_complet)));
                }
            }
            None
        })
        .collect()
}

/// Vérifie un seul passager (utile lors de la saisie).
pub fn verifier_alerte_passeport_passager(
    passager: &crate::models::passager::Passager,
    date_retour: NaiveDate,
) -> (bool, Option<String>) {
    if let Some(exp) = passager.date_expiration_passeport {
        if alerte_passeport(exp, date_retour) {
            return (true, Some(format!("Attention : le passeport de {} expire le {} (moins de 6 mois après la date de retour).", passager.nom_complet, exp)));
        }
    }
    (false, None)
}

/// Valide qu'un nombre de passagers par catégorie n'est pas négatif.
pub fn valider_nombre_passagers(nb_adultes: i32, nb_enfants: i32, nb_bebes: i32) -> AppResult<()> {
    if nb_adultes < 0 || nb_enfants < 0 || nb_bebes < 0 {
        return Err(AppError::Validation(
            "Le nombre de passagers ne peut pas être négatif.".to_string()
        ));
    }
    if nb_adultes == 0 && nb_enfants == 0 && nb_bebes == 0 {
        return Err(AppError::Validation(
            "Au moins un passager est requis.".to_string()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_valid_dates() {
        let depart = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        let retour = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();
        assert!(valider_dates_sejour(depart, retour).is_ok());
    }

    #[test]
    fn test_invalid_dates() {
        let depart = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();
        let retour = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        assert!(valider_dates_sejour(depart, retour).is_err());
    }

    #[test]
    fn test_alerte_passeport_reexport() {
        // Vérifier que le re-export fonctionne
        let retour = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();
        let expiration_ok = NaiveDate::from_ymd_opt(2027, 2, 15).unwrap(); // 6 mois exact
        let expiration_alerte = NaiveDate::from_ymd_opt(2027, 2, 14).unwrap(); // 6 mois -1 jour
        assert!(!alerte_passeport(expiration_ok, retour));
        assert!(alerte_passeport(expiration_alerte, retour));
    }

    #[test]
    fn test_valider_dates_devis() {
        let depart = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        let retour = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();
        assert!(valider_dates_devis(depart, retour).is_ok());
    }

    #[test]
    fn test_valider_hebergement() {
        let checkin = NaiveDate::from_ymd_opt(2026, 8, 10).unwrap();
        let checkout = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();
        let depart = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        let retour = NaiveDate::from_ymd_opt(2026, 8, 20).unwrap();
        assert!(valider_hebergement(checkin, checkout, depart, retour).is_ok());
    }

    #[test]
    fn test_verifier_alertes_passeports() {
        use crate::models::passager::Passager;

        let date_retour = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();

        let passager_ok = Passager {
            id: Some(1),
            devis_id: 1,
            categorie: "adulte".to_string(),
            nom_complet: "OK Expired".to_string(),
            date_naissance: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
            nationalite: Some("FR".to_string()),
            numero_passeport: Some("PK1".to_string()),
            date_expiration_passeport: Some(NaiveDate::from_ymd_opt(2028, 1, 1).unwrap()),
            lieu_delivrance: Some("Paris".to_string()),
            remarques: None,
        };

        let passager_alerte = Passager {
            id: Some(2),
            devis_id: 1,
            categorie: "adulte".to_string(),
            nom_complet: "Alert Expired".to_string(),
            date_naissance: NaiveDate::from_ymd_opt(1985, 1, 1).unwrap(),
            nationalite: Some("FR".to_string()),
            numero_passeport: Some("PK2".to_string()),
            date_expiration_passeport: Some(NaiveDate::from_ymd_opt(2026, 12, 1).unwrap()),
            lieu_delivrance: Some("Paris".to_string()),
            remarques: None,
        };

        let passager_no_pass = Passager {
            id: Some(3),
            devis_id: 1,
            categorie: "adulte".to_string(),
            nom_complet: "No Passport".to_string(),
            date_naissance: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            nationalite: None,
            numero_passeport: None,
            date_expiration_passeport: None,
            lieu_delivrance: None,
            remarques: None,
        };

        let alertes = verifier_alertes_passeports(&[passager_ok, passager_alerte, passager_no_pass], date_retour);

        // Seul le 2ème passager devrait avoir une alerte
        assert_eq!(alertes.len(), 1);
        assert_eq!(alertes[0].0, 2);
    }

    #[test]
    fn test_verifier_alerte_passeport_passager() {
        use crate::models::passager::Passager;

        let date_retour = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();

        let passager_ok = Passager {
            id: Some(1),
            devis_id: 1,
            categorie: "adulte".to_string(),
            nom_complet: "OK Expired".to_string(),
            date_naissance: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
            nationalite: Some("FR".to_string()),
            numero_passeport: Some("PK1".to_string()),
            date_expiration_passeport: Some(NaiveDate::from_ymd_opt(2028, 1, 1).unwrap()),
            lieu_delivrance: Some("Paris".to_string()),
            remarques: None,
        };

        let passager_alerte = Passager {
            id: Some(2),
            devis_id: 1,
            categorie: "adulte".to_string(),
            nom_complet: "Alert Expired".to_string(),
            date_naissance: NaiveDate::from_ymd_opt(1985, 1, 1).unwrap(),
            nationalite: Some("FR".to_string()),
            numero_passeport: Some("PK2".to_string()),
            date_expiration_passeport: Some(NaiveDate::from_ymd_opt(2026, 12, 1).unwrap()),
            lieu_delivrance: Some("Paris".to_string()),
            remarques: None,
        };

        let (ok, msg) = verifier_alerte_passeport_passager(&passager_ok, date_retour);
        assert!(!ok);
        assert!(msg.is_none());

        let (alert, msg_alert) = verifier_alerte_passeport_passager(&passager_alerte, date_retour);
        assert!(alert);
        assert!(msg_alert.is_some());
        assert!(msg_alert.unwrap().contains("2026-12-01"));
    }
}
