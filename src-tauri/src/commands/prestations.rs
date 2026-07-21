use crate::db::DbState;
use crate::models::prestation_vip::{PrestationVip, PrestationVipCreate, PrestationVipUpdate};
use rusqlite::params;
use tauri::State;

/// Cree une nouvelle prestation VIP liee a un devis.
#[tauri::command]
pub fn create_prestation(
    state: State<DbState>,
    prestation_data: PrestationVipCreate,
) -> Result<PrestationVip, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "INSERT INTO prestations_vip (devis_id, type_prestation, description,
                                       prix_unitaire, quantite, devise_prix, remarques)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
    ).map_err(|e| e.to_string())?;

    stmt.execute(params![
        prestation_data.devis_id,
        prestation_data.type_prestation,
        prestation_data.description,
        prestation_data.prix_unitaire,
        prestation_data.quantite,
        prestation_data.devise_prix,
        prestation_data.remarques,
    ]).map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    get_prestation_by_id(state, id)
}

/// Retourne une prestation VIP par son identifiant.
#[tauri::command]
pub fn get_prestation_by_id(
    state: State<DbState>,
    id: i64,
) -> Result<PrestationVip, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, type_prestation, description,
                prix_unitaire, quantite, devise_prix, remarques
         FROM prestations_vip WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(PrestationVip {
            id: Some(row.get(0)?),
            devis_id: row.get(1)?,
            type_prestation: row.get(2)?,
            description: row.get(3)?,
            prix_unitaire: row.get(4)?,
            quantite: row.get(5)?,
            devise_prix: row.get(6)?,
            remarques: row.get(7)?,
        })
    } else {
        Err("Prestation VIP non trouvee".to_string())
    }
}

/// Retourne la liste des prestations VIP liees a un devis donne.
#[tauri::command]
pub fn list_prestations_by_devis(
    state: State<DbState>,
    devis_id: i64,
) -> Result<Vec<PrestationVip>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, type_prestation, description,
                prix_unitaire, quantite, devise_prix, remarques
         FROM prestations_vip WHERE devis_id = ?1 ORDER BY id"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(params![devis_id]).map_err(|e| e.to_string())?;
    let mut list = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        list.push(PrestationVip {
            id: Some(row.get(0)?),
            devis_id: row.get(1)?,
            type_prestation: row.get(2)?,
            description: row.get(3)?,
            prix_unitaire: row.get(4)?,
            quantite: row.get(5)?,
            devise_prix: row.get(6)?,
            remarques: row.get(7)?,
        });
    }
    Ok(list)
}

/// Met a jour une prestation VIP existante.
#[tauri::command]
pub fn update_prestation(
    state: State<DbState>,
    id: i64,
    update_data: PrestationVipUpdate,
) -> Result<PrestationVip, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut sets = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(type_prestation) = update_data.type_prestation {
        sets.push("type_prestation = ?");
        params_vec.push(Box::new(type_prestation));
    }
    if let Some(description) = update_data.description {
        sets.push("description = ?");
        params_vec.push(Box::new(description));
    }
    if let Some(prix_unitaire) = update_data.prix_unitaire {
        sets.push("prix_unitaire = ?");
        params_vec.push(Box::new(prix_unitaire));
    }
    if let Some(quantite) = update_data.quantite {
        sets.push("quantite = ?");
        params_vec.push(Box::new(quantite));
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
        "UPDATE prestations_vip SET {} WHERE id = ?",
        sets.join(", ")
    );

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let mut final_params: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    final_params.push(&id);
    stmt.execute(&final_params).map_err(|e| e.to_string())?;

    get_prestation_by_id(state, id)
}

/// Supprime une prestation VIP par son identifiant.
#[tauri::command]
pub fn delete_prestation(
    state: State<DbState>,
    id: i64,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM prestations_vip WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
