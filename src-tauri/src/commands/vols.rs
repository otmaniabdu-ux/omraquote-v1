use crate::db::DbState;
use crate::models::segment_vol::SegmentVol;
use crate::error::{AppError, AppResult};
use crate::services::db::vols;
use tauri::State;

pub fn list_segments_by_devis(state: State<DbState>, devis_id: i64) -> AppResult<Vec<SegmentVol>> {
    let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    vols::list_by_devis(&conn, devis_id)
}