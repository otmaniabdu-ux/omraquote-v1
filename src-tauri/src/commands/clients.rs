use crate::db::DbState;
use crate::models::client::{Client, ClientCreate, ClientUpdate};
use crate::error::{AppResult, AppError};
use crate::services::db::clients;
use tauri::State;

#[tauri::command]
pub fn create_client(
    state: State<DbState>,
    client_data: ClientCreate,
) -> AppResult<Client> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    clients::create(&conn, client_data)
}

#[tauri::command]
pub fn get_client_by_id(state: State<DbState>, id: i64) -> AppResult<Client> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    clients::get_by_id(&conn, id)
}

#[tauri::command]
pub fn list_clients(state: State<DbState>) -> AppResult<Vec<Client>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    clients::list(&conn)
}

#[tauri::command]
pub fn update_client(
    state: State<DbState>,
    id: i64,
    update_data: ClientUpdate,
) -> AppResult<Client> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    clients::update(&conn, id, update_data)
}

#[tauri::command]
pub fn delete_client(state: State<DbState>, id: i64) -> AppResult<()> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    clients::delete(&conn, id)
}

#[tauri::command]
pub fn generate_client_code(state: State<DbState>) -> AppResult<String> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    clients::generate_code(&conn)
}