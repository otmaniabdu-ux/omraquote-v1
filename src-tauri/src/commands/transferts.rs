use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::models::transfert::{Transfert, TransfertCreate, TransfertUpdate};
use crate::services::db::transferts;
use tauri::State;

#[tauri::command]
pub fn create_transfert(
    state: State<DbState>,
    transfert_data: TransfertCreate,
) -> AppResult<Transfert> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    transferts::create(&conn, transfert_data)
}

#[tauri::command]
pub fn get_transfert_by_id(
    state: State<DbState>,
    id: i64,
) -> AppResult<Transfert> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    transferts::get_by_id(&conn, id)
}

#[tauri::command]
pub fn list_transferts_by_devis(
    state: State<DbState>,
    devis_id: i64,
) -> AppResult<Vec<Transfert>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    transferts::list_by_devis(&conn, devis_id)
}

#[tauri::command]
pub fn update_transfert(
    state: State<DbState>,
    id: i64,
    update_data: TransfertUpdate,
) -> AppResult<Transfert> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    transferts::update(&conn, id, update_data)
}

#[tauri::command]
pub fn delete_transfert(
    state: State<DbState>,
    id: i64,
) -> AppResult<()> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    transferts::delete(&conn, id)
}
