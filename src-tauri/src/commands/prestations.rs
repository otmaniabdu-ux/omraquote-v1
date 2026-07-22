use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::models::prestation_vip::{PrestationVip, PrestationVipCreate, PrestationVipUpdate};
use crate::services::db::prestations;
use tauri::State;

#[tauri::command]
pub fn create_prestation(
    state: State<DbState>,
    prestation_data: PrestationVipCreate,
) -> AppResult<PrestationVip> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    prestations::create(&conn, prestation_data)
}

#[tauri::command]
pub fn get_prestation_by_id(
    state: State<DbState>,
    id: i64,
) -> AppResult<PrestationVip> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    prestations::get_by_id(&conn, id)
}

#[tauri::command]
pub fn list_prestations_by_devis(
    state: State<DbState>,
    devis_id: i64,
) -> AppResult<Vec<PrestationVip>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    prestations::list_by_devis(&conn, devis_id)
}

#[tauri::command]
pub fn update_prestation(
    state: State<DbState>,
    id: i64,
    update_data: PrestationVipUpdate,
) -> AppResult<PrestationVip> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    prestations::update(&conn, id, update_data)
}

#[tauri::command]
pub fn delete_prestation(
    state: State<DbState>,
    id: i64,
) -> AppResult<()> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    prestations::delete(&conn, id)
}
