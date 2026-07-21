use crate::db::DbState;
use crate::models::catalogue::{CompagnieCatalogue, CompagnieCatalogueCreate, CompagnieCatalogueUpdate};
use rusqlite::{params, Connection, Result};
use tauri::State;
use std::sync::Mutex;

#[tauri::command]
pub fn create_compagnie(
    state: State<DbState>,
    compagnie_data: CompagnieCatalogueCreate,
) -> Result<CompagnieCatalogue, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let actif = compagnie_data.actif.unwrap_or(true);

    let mut stmt = conn.prepare(
        "INSERT INTO catalogue_compagnies (code_iata, nom_compagnie, pays, site_web, actif)
         VALUES (?1, ?2, ?3, ?4, ?5)"
    ).map_err(|e| e.to_string())?;

    stmt.execute(params![
        compagnie_data.code_iata,
        compagnie_data.nom_compagnie,
        compagnie_data.pays,
        compagnie_data.site_web,
        actif,
    ]).map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    get_compagnie_by_id(state, id)
}

#[tauri::command]
pub fn get_compagnie_by_id(state: State<DbState>, id: i64) -> Result<CompagnieCatalogue, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, code_iata, nom_compagnie, pays, site_web, actif
         FROM catalogue_compagnies WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(CompagnieCatalogue {
            id: Some(row.get(0)?),
            code_iata: row.get(1)?,
            nom_compagnie: row.get(2)?,
            pays: row.get(3)?,
            site_web: row.get(4)?,
            actif: row.get(5)?,
        })
    } else {
        Err("Compagnie non trouvée".to_string())
    }
}

#[tauri::command]
pub fn list_compagnies(state: State<DbState>, actif_seulement: Option<bool>) -> Result<Vec<CompagnieCatalogue>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut query = String::from(
        "SELECT id, code_iata, nom_compagnie, pays, site_web, actif
         FROM catalogue_compagnies"
    );
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(actif) = actif_seulement {
        query.push_str(" WHERE actif = ?");
        params.push(Box::new(actif));
    }

    query.push_str(" ORDER BY nom_compagnie");

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let mut rows = stmt.query(&params).map_err(|e| e.to_string())?;
    let mut compagnies = Vec::new();

    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        compagnies.push(CompagnieCatalogue {
            id: Some(row.get(0)?),
            code_iata: row.get(1)?,
            nom_compagnie: row.get(2)?,
            pays: row.get(3)?,
            site_web: row.get(4)?,
            actif: row.get(5)?,
        });
    }
    Ok(compagnies)
}

#[tauri::command]
pub fn update_compagnie(
    state: State<DbState>,
    id: i64,
    update_data: CompagnieCatalogueUpdate,
) -> Result<CompagnieCatalogue, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut sets = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(code_iata) = update_data.code_iata {
        sets.push("code_iata = ?");
        params.push(Box::new(code_iata));
    }
    if let Some(nom_compagnie) = update_data.nom_compagnie {
        sets.push("nom_compagnie = ?");
        params.push(Box::new(nom_compagnie));
    }
    if let Some(pays) = update_data.pays {
        sets.push("pays = ?");
        params.push(Box::new(pays));
    }
    if let Some(site_web) = update_data.site_web {
        sets.push("site_web = ?");
        params.push(Box::new(site_web));
    }
    if let Some(actif) = update_data.actif {
        sets.push("actif = ?");
        params.push(Box::new(actif));
    }

    if sets.is_empty() {
        return Err("Aucune donnée à mettre à jour".to_string());
    }

    sets.push("updated_at = CURRENT_TIMESTAMP");
    let query = format!(
        "UPDATE catalogue_compagnies SET {} WHERE id = ?",
        sets.join(", ")
    );

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let mut final_params: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    final_params.push(&id);
    stmt.execute(&final_params).map_err(|e| e.to_string())?;

    get_compagnie_by_id(state, id)
}

#[tauri::command]
pub fn delete_compagnie(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM catalogue_compagnies WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}