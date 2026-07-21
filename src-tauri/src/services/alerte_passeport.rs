use chrono::{NaiveDate, Months};

/// Vérifie si un passeport expire avant la date de retour + 6 mois.
/// Retourne true si alerte (passeport expirant dans moins de 6 mois après le retour).
pub fn alerte_passeport(
    date_expiration: NaiveDate,
    date_retour: NaiveDate,
) -> bool {
    let seuil = date_retour
        .checked_add_months(Months::new(6))
        .unwrap_or(date_retour);
    date_expiration < seuil
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_alerte_exact_6_mois() {
        let date_retour = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();
        let date_expiration = NaiveDate::from_ymd_opt(2027, 2, 15).unwrap(); // exactement 6 mois après
        assert!(!alerte_passeport(date_expiration, date_retour)); // pas d'alerte car >= seuil
    }

    #[test]
    fn test_alerte_moins_6_mois_un_jour() {
        let date_retour = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();
        let date_expiration = NaiveDate::from_ymd_opt(2027, 2, 14).unwrap(); // 6 mois - 1 jour
        assert!(alerte_passeport(date_expiration, date_retour));
    }

    #[test]
    fn test_alerte_plus_6_mois_un_jour() {
        let date_retour = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();
        let date_expiration = NaiveDate::from_ymd_opt(2027, 2, 16).unwrap(); // 6 mois + 1 jour
        assert!(!alerte_passeport(date_expiration, date_retour));
    }

    #[test]
    fn test_alerte_passeport_deja_expire() {
        let date_retour = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();
        let date_expiration = NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();
        assert!(alerte_passeport(date_expiration, date_retour));
    }
}