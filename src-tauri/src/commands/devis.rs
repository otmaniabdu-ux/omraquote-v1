use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::models::devis::{Devis, DevisCreate, DevisUpdate};
use crate::services::db::devis;
use tauri::State;

#[tauri::command]
pub fn get_alertes_tous_devis(state: State<DbState>) -> AppResult<Vec<serde_json::Value>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    devis::get_alertes_tous_devis(&conn)
}

#[tauri::command]
pub fn get_alertes_devis(state: State<DbState>, devis_id: i64) -> AppResult<Vec<serde_json::Value>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    devis::get_alertes_devis(&conn, devis_id)
}

#[tauri::command]
pub fn create_devis(
    state: State<DbState>,
    devis_data: DevisCreate,
) -> AppResult<Devis> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    devis::create(&conn, devis_data)
}

#[tauri::command]
pub fn get_devis_by_id(state: State<DbState>, id: i64) -> AppResult<Devis> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    devis::get_by_id(&conn, id)
}

#[tauri::command]
pub fn list_devis(state: State<DbState>) -> AppResult<Vec<Devis>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    devis::list(&conn)
}

#[tauri::command]
pub fn update_devis(
    state: State<DbState>,
    id: i64,
    update_data: DevisUpdate,
) -> AppResult<Devis> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    devis::update(&conn, id, update_data)
}

#[tauri::command]
pub fn delete_devis(state: State<DbState>, id: i64) -> AppResult<()> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    devis::delete(&conn, id)
}

#[tauri::command]
pub fn calculate_totals(
    state: State<DbState>,
    devis_id: i64,
) -> AppResult<Devis> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    devis::calculate_totals(&conn, devis_id)
}
