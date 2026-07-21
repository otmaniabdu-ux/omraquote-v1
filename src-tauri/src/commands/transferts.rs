use crate::db::DbState;
use crate::models::transfert::{Transfert, TransfertCreate, TransfertUpdate};
use rusqlite::params;
use tauri::State;

/// Cree un nouveau transfert lie a un devis.
#[tauri::command]
pub fn create_transfert(
    state: State<DbState>,
    transfert_data: TransfertCreate,
) -> Result<Transfert, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "INSERT INTO transferts (devis_id, type_transfert, trajet, type_vehicule,
                                   date_transfert, heure_transfert, nombre_vehicules,
                                   prix_unitaire, devise_prix, remarques)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
    ).map_err(|e| e.to_string())?;

    stmt.execute(params![
        transfert_data.devis_id,
        transfert_data.type_transfert,
        transfert_data.trajet,
        transfert_data.type_vehicule,
        transfert_data.date_transfert,
        transfert_data.heure_transfert,
        transfert_data.nombre_vehicules,
        transfert_data.prix_unitaire,
        transfert_data.devise_prix,
        transfert_data.remarques,
    ]).map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    get_transfert_by_id(state, id)
}

/// Retourne un transfert par son identifiant.
#[tauri::command]
pub fn get_transfert_by_id(
    state: State<DbState>,
    id: i64,
) -> Result<Transfert, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, type_transfert, trajet, type_vehicule,
                date_transfert, heure_transfert, nombre_vehicules,
                prix_unitaire, devise_prix, remarques
         FROM transferts WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(Transfert {
            id: Some(row.get(0)?),
            devis_id: row.get(1)?,
            type_transfert: row.get(2)?,
            trajet: row.get(3)?,
            type_vehicule: row.get(4)?,
            date_transfert: row.get(5)?,
            heure_transfert: row.get(6)?,
            nombre_vehicules: row.get(7)?,
            prix_unitaire: row.get(8)?,
            devise_prix: row.get(9)?,
            remarques: row.get(10)?,
        })
    } else {
        Err("Transfert non trouve".to_string())
    }
}

/// Retourne la liste des transferts lies a un devis donne.
#[tauri::command]
pub fn list_transferts_by_devis(
    state: State<DbState>,
    devis_id: i64,
) -> Result<Vec<Transfert>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, type_transfert, trajet, type_vehicule,
                date_transfert, heure_transfert, nombre_vehicules,
                prix_unitaire, devise_prix, remarques
         FROM transferts WHERE devis_id = ?1 ORDER BY id"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(params![devis_id]).map_err(|e| e.to_string())?;
    let mut list = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        list.push(Transfert {
            id: Some(row.get(0)?),
            devis_id: row.get(1)?,
            type_transfert: row.get(2)?,
            trajet: row.get(3)?,
            type_vehicule: row.get(4)?,
            date_transfert: row.get(5)?,
            heure_transfert: row.get(6)?,
            nombre_vehicules: row.get(7)?,
            prix_unitaire: row.get(8)?,
            devise_prix: row.get(9)?,
            remarques: row.get(10)?,
        });
    }
    Ok(list)
}

/// Met a jour un transfert existant.
#[tauri::command]
pub fn update_transfert(
    state: State<DbState>,
    id: i64,
    update_data: TransfertUpdate,
) -> Result<Transfert, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut sets = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(type_transfert) = update_data.type_transfert {
        sets.push("type_transfert = ?");
        params_vec.push(Box::new(type_transfert));
    }
    if let Some(trajet) = update_data.trajet {
        sets.push("trajet = ?");
        params_vec.push(Box::new(trajet));
    }
    if let Some(type_vehicule) = update_data.type_vehicule {
        sets.push("type_vehicule = ?");
        params_vec.push(Box::new(type_vehicule));
    }
    if let Some(date_transfert) = update_data.date_transfert {
        sets.push("date_transfert = ?");
        params_vec.push(Box::new(date_transfert));
    }
    if let Some(heure_transfert) = update_data.heure_transfert {
        sets.push("heure_transfert = ?");
        params_vec.push(Box::new(heure_transfert));
    }
    if let Some(nombre_vehicules) = update_data.nombre_vehicules {
        sets.push("nombre_vehicules = ?");
        params_vec.push(Box::new(nombre_vehicules));
    }
    if let Some(prix_unitaire) = update_data.prix_unitaire {
        sets.push("prix_unitaire = ?");
        params_vec.push(Box::new(prix_unitaire));
    }
    if let Some(devise_prix) = update_data.devise_prix {
        sets.push("devise_prix = ?");
        params_vec.push(Box::new(devise_prix));
    }
    if let Some(remarques) = update_data.remarques {
        sets.push("remarques = ?");
        params_vec.push(Box::new(remarques));
    }

    if sets.is_empty() {
        return Err("Aucune donnee a mettre a jour".to_string());
    }

    sets.push("updated_at = CURRENT_TIMESTAMP");
    let query = format!(
        "UPDATE transferts SET {} WHERE id = ?",
        sets.join(", ")
    );

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let mut final_params: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    final_params.push(&id);
    stmt.execute(&final_params).map_err(|e| e.to_string())?;

    get_transfert_by_id(state, id)
}

/// Supprime un transfert par son identifiant.
#[tauri::command]
pub fn delete_transfert(
    state: State<DbState>,
    id: i64,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM transferts WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
