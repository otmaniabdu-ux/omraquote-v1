use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::models::catalogue::{HotelCatalogue, HotelCatalogueCreate, HotelCatalogueUpdate};
use crate::services::db::hotels;
use tauri::State;

#[tauri::command]
pub fn create_hotel(
    state: State<DbState>,
    hotel_data: HotelCatalogueCreate,
) -> AppResult<HotelCatalogue> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    hotels::create(&conn, hotel_data)
}

#[tauri::command]
pub fn get_hotel_by_id(state: State<DbState>, id: i64) -> AppResult<HotelCatalogue> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    hotels::get_by_id(&conn, id)
}

#[tauri::command]
pub fn list_hotels(state: State<DbState>, actif_seulement: Option<bool>) -> AppResult<Vec<HotelCatalogue>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    hotels::list(&conn, actif_seulement)
}

#[tauri::command]
pub fn update_hotel(
    state: State<DbState>,
    id: i64,
    update_data: HotelCatalogueUpdate,
) -> AppResult<HotelCatalogue> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    hotels::update(&conn, id, update_data)
}

#[tauri::command]
pub fn delete_hotel(state: State<DbState>, id: i64) -> AppResult<()> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    hotels::delete(&conn, id)
}
