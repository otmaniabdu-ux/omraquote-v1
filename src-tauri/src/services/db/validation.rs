use crate::models::passager::Passager;
use crate::error::{AppResult, AppError};
use crate::services::validation::verifier_alertes_passeports;
use crate::services::db::passagers;
use rusqlite::{params, Connection};
use chrono::NaiveDate;

pub fn get_passeport_alertes(conn: &Connection, devis_id: i64) -> AppResult<Vec<(i64, String)>> {
    let date_retour: NaiveDate = conn.query_row(
        "SELECT date_retour FROM devis WHERE id = ?1",
        params![devis_id],
        |row| row.get(0),
    )?;

    let passagers = passagers::list_by_devis(conn, devis_id)?;
    Ok(verifier_alertes_passeports(&passagers, date_retour))
}
