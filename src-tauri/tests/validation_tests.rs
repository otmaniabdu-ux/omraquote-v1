use omravip_quotes::services::validations::*;
use chrono::NaiveDate;

#[test]
fn test_validations_dates() {
    let depart = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
    let retour = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();
    assert!(valider_dates_sejour(depart, retour).is_ok());
}

// ... plus de tests