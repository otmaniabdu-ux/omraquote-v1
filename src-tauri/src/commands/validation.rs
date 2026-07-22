use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::models::passager::Passager;
use crate::services::validation::{valider_dates_devis, valider_hebergement};
use crate::services::db::validation as db_validation;
use chrono::NaiveDate;
use tauri::State;

#[tauri::command]
pub fn valider_dates_devis_command(
    date_depart: NaiveDate,
    date_retour: NaiveDate,
) -> AppResult<()> {
    valider_dates_devis(date_depart, date_retour)
}

#[tauri::command]
pub fn valider_hebergement_command(
    checkin: NaiveDate,
    checkout: NaiveDate,
    date_depart: NaiveDate,
    date_retour: NaiveDate,
) -> AppResult<()> {
    valider_hebergement(checkin, checkout, date_depart, date_retour)
}

#[tauri::command]
pub fn get_passeport_alertes(
    state: State<DbState>,
    devis_id: i64,
) -> AppResult<Vec<(i64, String)>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    db_validation::get_passeport_alertes(&conn, devis_id)
}

#[tauri::command]
pub fn check_passager_passeport(
    passager: Passager,
    date_retour: NaiveDate,
) -> AppResult<(bool, Option<String>)> {
    use crate::services::validation::verifier_alerte_passeport_passager;
    Ok(verifier_alerte_passeport_passager(&passager, date_retour))
}
