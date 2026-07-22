use chrono::NaiveDate;
use crate::error::{AppResult, AppError};

/// Calcule le nombre de nuits entre deux dates (checkout - checkin) en jours entiers.
pub fn calculer_nuitees(date_checkin: NaiveDate, date_checkout: NaiveDate) -> AppResult<i64> {
    if date_checkout <= date_checkin {
        return Err(AppError::Validation(
            "La date de checkout doit être postérieure à la date de checkin".to_string()
        ));
    }
    let diff = date_checkout.signed_duration_since(date_checkin);
    Ok(diff.num_days())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_nuitees_normales() {
        let checkin = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        let checkout = NaiveDate::from_ymd_opt(2026, 8, 5).unwrap();
        assert_eq!(calculer_nuitees(checkin, checkout).unwrap(), 4);
    }

    #[test]
    fn test_changement_mois() {
        let checkin = NaiveDate::from_ymd_opt(2026, 7, 28).unwrap();
        let checkout = NaiveDate::from_ymd_opt(2026, 8, 2).unwrap();
        assert_eq!(calculer_nuitees(checkin, checkout).unwrap(), 5); // 31-28=3 + 2 = 5
    }

    #[test]
    fn test_annee_bissextile() {
        let checkin = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();
        let checkout = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        assert_eq!(calculer_nuitees(checkin, checkout).unwrap(), 2); // 29 fév + 1 mars = 2 nuits
    }

    #[test]
    fn test_erreur_dates_egales() {
        let checkin = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        let checkout = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        let result = calculer_nuitees(checkin, checkout);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("postérieure"));
    }
}
