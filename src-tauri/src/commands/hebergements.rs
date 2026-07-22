use crate::db::DbState;
use crate::error::{AppResult, AppError};
use crate::models::hebergement::{Hebergement, HebergementCreate};
use crate::services::db::hebergements as db_hebergements;
use tauri::State;

/// Cree un nouvel hebergement lie a un devis.
#[tauri::command]
pub fn create_hebergement(
    state: State<DbState>,
    hebergement_data: HebergementCreate,
) -> AppResult<Hebergement> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    db_hebergements::create_hebergement(&conn, hebergement_data)
}

/// Retourne un hebergement par son identifiant.
#[tauri::command]
pub fn get_hebergement_by_id(
    state: State<DbState>,
    id: i64,
) -> AppResult<Hebergement> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    db_hebergements::get_hebergement_by_id(&conn, id)
}

/// Retourne la liste des hebergements lies a un devis donne.
#[tauri::command]
pub fn list_hebergements_by_devis(
    state: State<DbState>,
    devis_id: i64,
) -> AppResult<Vec<Hebergement>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    db_hebergements::list_hebergements_by_devis(&conn, devis_id)
}

/// Met a jour un hebergement existant.
#[tauri::command]
pub fn update_hebergement(
    state: State<DbState>,
    id: i64,
    update_data: HebergementCreate,
) -> AppResult<Hebergement> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    db_hebergements::update_hebergement(&conn, id, update_data)
}

/// Supprime un hebergement par son identifiant.
#[tauri::command]
pub fn delete_hebergement(
    state: State<DbState>,
    id: i64,
) -> AppResult<()> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    db_hebergements::delete_hebergement(&conn, id)
}
