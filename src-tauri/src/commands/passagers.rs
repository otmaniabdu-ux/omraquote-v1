use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::models::passager::{Passager, PassagerCreate};
use crate::services::db::passagers;
use tauri::State;

#[tauri::command]
pub fn create_passager(
    state: State<DbState>,
    passager_data: PassagerCreate,
) -> AppResult<Passager> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    passagers::create(&conn, passager_data)
}

#[tauri::command]
pub fn get_passager_by_id(state: State<DbState>, id: i64) -> AppResult<Passager> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    passagers::get_by_id(&conn, id)
}

#[tauri::command]
pub fn list_passagers_by_devis(state: State<DbState>, devis_id: i64) -> AppResult<Vec<Passager>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    passagers::list_by_devis(&conn, devis_id)
}

#[tauri::command]
pub fn delete_passager(state: State<DbState>, id: i64) -> AppResult<()> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    passagers::delete(&conn, id)
}
