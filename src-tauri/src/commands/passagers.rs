use crate::db::DbState;
use crate::models::passager::{Passager, PassagerCreate};
use rusqlite::{params, Connection, Result};
use tauri::State;
use std::sync::Mutex;

#[tauri::command]
pub fn create_passager(
    state: State<DbState>,
    passager_data: PassagerCreate,
) -> Result<Passager, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "INSERT INTO passagers (devis_id, categorie, nom_complet, date_naissance, nationalite,
                                numero_passeport, date_expiration_passeport, lieu_delivrance, remarques)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
    ).map_err(|e| e.to_string())?;

    stmt.execute(params![
        passager_data.devis_id,
        passager_data.categorie,
        passager_data.nom_complet,
        passager_data.date_naissance,
        passager_data.nationalite,
        passager_data.numero_passeport,
        passager_data.date_expiration_passeport,
        passager_data.lieu_delivrance,
        passager_data.remarques,
    ]).map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    get_passager_by_id(state, id)
}

#[tauri::command]
pub fn get_passager_by_id(state: State<DbState>, id: i64) -> Result<Passager, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, categorie, nom_complet, date_naissance, nationalite,
                numero_passeport, date_expiration_passeport, lieu_delivrance, remarques
         FROM passagers WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(Passager {
            id: Some(row.get(0)?),
            devis_id: row.get(1)?,
            categorie: row.get(2)?,
            nom_complet: row.get(3)?,
            date_naissance: row.get(4)?,
            nationalite: row.get(5)?,
            numero_passeport: row.get(6)?,
            date_expiration_passeport: row.get(7)?,
            lieu_delivrance: row.get(8)?,
            remarques: row.get(9)?,
        })
    } else {
        Err("Passager non trouvé".to_string())
    }
}

#[tauri::command]
pub fn list_passagers_by_devis(state: State<DbState>, devis_id: i64) -> Result<Vec<Passager>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, categorie, nom_complet, date_naissance, nationalite,
                numero_passeport, date_expiration_passeport, lieu_delivrance, remarques
         FROM passagers WHERE devis_id = ?1 ORDER BY id"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(params![devis_id]).map_err(|e| e.to_string())?;
    let mut passagers = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        passagers.push(Passager {
            id: Some(row.get(0)?),
            devis_id: row.get(1)?,
            categorie: row.get(2)?,
            nom_complet: row.get(3)?,
            date_naissance: row.get(4)?,
            nationalite: row.get(5)?,
            numero_passeport: row.get(6)?,
            date_expiration_passeport: row.get(7)?,
            lieu_delivrance: row.get(8)?,
            remarques: row.get(9)?,
        });
    }
    Ok(passagers)
}

#[tauri::command]
pub fn delete_passager(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM passagers WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}