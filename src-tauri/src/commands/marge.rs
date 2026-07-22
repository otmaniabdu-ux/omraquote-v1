use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::services::agregats_marge::{get_statistiques_marges, get_marges_par_client, StatistiquesMarges};
use chrono::NaiveDate;
use tauri::State;

#[tauri::command]
pub fn get_statistiques(
    state: State<DbState>,
    date_debut: String,
    date_fin: String,
) -> AppResult<StatistiquesMarges> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;

    let debut = NaiveDate::parse_from_str(&date_debut, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("Date de début invalide : {}", e)))?;
    let fin = NaiveDate::parse_from_str(&date_fin, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("Date de fin invalide : {}", e)))?;

    get_statistiques_marges(&conn, debut, fin)
}

#[tauri::command]
pub fn get_top_clients(
    state: State<DbState>,
    date_debut: String,
    date_fin: String,
    limit: Option<i64>,
) -> AppResult<Vec<(String, String, i64)>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;

    let debut = NaiveDate::parse_from_str(&date_debut, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("Date de début invalide : {}", e)))?;
    let fin = NaiveDate::parse_from_str(&date_fin, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("Date de fin invalide : {}", e)))?;

    let result = get_marges_par_client(&conn, debut, fin, limit)?;
    Ok(result.iter().map(|(client, marge, _, nb)| {
        (client.clone(), marge.to_string(), *nb)
    }).collect())
}
