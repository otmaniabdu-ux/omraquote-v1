use crate::db::DbState;
use crate::error::{AppResult, AppError};
use crate::models::catalogue::{CompagnieCatalogue, CompagnieCatalogueCreate, CompagnieCatalogueUpdate};
use crate::services::db::compagnies as db_compagnies;
use tauri::State;

#[tauri::command]
pub fn create_compagnie(
    state: State<DbState>,
    compagnie_data: CompagnieCatalogueCreate,
) -> AppResult<CompagnieCatalogue> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    db_compagnies::create_compagnie(&conn, compagnie_data)
}

#[tauri::command]
pub fn get_compagnie_by_id(state: State<DbState>, id: i64) -> AppResult<CompagnieCatalogue> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    db_compagnies::get_compagnie_by_id(&conn, id)
}

#[tauri::command]
pub fn list_compagnies(state: State<DbState>, actif_seulement: Option<bool>) -> AppResult<Vec<CompagnieCatalogue>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    db_compagnies::list_compagnies(&conn, actif_seulement)
}

#[tauri::command]
pub fn update_compagnie(
    state: State<DbState>,
    id: i64,
    update_data: CompagnieCatalogueUpdate,
) -> AppResult<CompagnieCatalogue> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    db_compagnies::update_compagnie(&conn, id, update_data)
}

#[tauri::command]
pub fn delete_compagnie(state: State<DbState>, id: i64) -> AppResult<()> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    db_compagnies::delete_compagnie(&conn, id)
}