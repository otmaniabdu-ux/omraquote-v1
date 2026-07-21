use crate::db::DbState;
use crate::models::catalogue::{HotelCatalogue, HotelCatalogueCreate, HotelCatalogueUpdate};
use rusqlite::{params, Connection, Result};
use tauri::State;
use std::sync::Mutex;

#[tauri::command]
pub fn create_hotel(
    state: State<DbState>,
    hotel_data: HotelCatalogueCreate,
) -> Result<HotelCatalogue, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let actif = hotel_data.actif.unwrap_or(true);

    let mut stmt = conn.prepare(
        "INSERT INTO catalogue_hotels (nom_hotel, ville, categorie, adresse, contact, site_web, remarques, actif)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
    ).map_err(|e| e.to_string())?;

    stmt.execute(params![
        hotel_data.nom_hotel,
        hotel_data.ville,
        hotel_data.categorie,
        hotel_data.adresse,
        hotel_data.contact,
        hotel_data.site_web,
        hotel_data.remarques,
        actif,
    ]).map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    get_hotel_by_id(state, id)
}

#[tauri::command]
pub fn get_hotel_by_id(state: State<DbState>, id: i64) -> Result<HotelCatalogue, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, nom_hotel, ville, categorie, adresse, contact, site_web, remarques, actif
         FROM catalogue_hotels WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(HotelCatalogue {
            id: Some(row.get(0)?),
            nom_hotel: row.get(1)?,
            ville: row.get(2)?,
            categorie: row.get(3)?,
            adresse: row.get(4)?,
            contact: row.get(5)?,
            site_web: row.get(6)?,
            remarques: row.get(7)?,
            actif: row.get(8)?,
        })
    } else {
        Err("Hôtel non trouvé".to_string())
    }
}

#[tauri::command]
pub fn list_hotels(state: State<DbState>, actif_seulement: Option<bool>) -> Result<Vec<HotelCatalogue>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut query = String::from(
        "SELECT id, nom_hotel, ville, categorie, adresse, contact, site_web, remarques, actif
         FROM catalogue_hotels"
    );
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(actif) = actif_seulement {
        query.push_str(" WHERE actif = ?");
        params.push(Box::new(actif));
    }

    query.push_str(" ORDER BY nom_hotel");

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let mut rows = stmt.query(&params).map_err(|e| e.to_string())?;
    let mut hotels = Vec::new();

    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        hotels.push(HotelCatalogue {
            id: Some(row.get(0)?),
            nom_hotel: row.get(1)?,
            ville: row.get(2)?,
            categorie: row.get(3)?,
            adresse: row.get(4)?,
            contact: row.get(5)?,
            site_web: row.get(6)?,
            remarques: row.get(7)?,
            actif: row.get(8)?,
        });
    }
    Ok(hotels)
}

#[tauri::command]
pub fn update_hotel(
    state: State<DbState>,
    id: i64,
    update_data: HotelCatalogueUpdate,
) -> Result<HotelCatalogue, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut sets = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(nom_hotel) = update_data.nom_hotel {
        sets.push("nom_hotel = ?");
        params.push(Box::new(nom_hotel));
    }
    if let Some(ville) = update_data.ville {
        sets.push("ville = ?");
        params.push(Box::new(ville));
    }
    if let Some(categorie) = update_data.categorie {
        sets.push("categorie = ?");
        params.push(Box::new(categorie));
    }
    if let Some(adresse) = update_data.adresse {
        sets.push("adresse = ?");
        params.push(Box::new(adresse));
    }
    if let Some(contact) = update_data.contact {
        sets.push("contact = ?");
        params.push(Box::new(contact));
    }
    if let Some(site_web) = update_data.site_web {
        sets.push("site_web = ?");
        params.push(Box::new(site_web));
    }
    if let Some(remarques) = update_data.remarques {
        sets.push("remarques = ?");
        params.push(Box::new(remarques));
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
        "UPDATE catalogue_hotels SET {} WHERE id = ?",
        sets.join(", ")
    );

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let mut final_params: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    final_params.push(&id);
    stmt.execute(&final_params).map_err(|e| e.to_string())?;

    get_hotel_by_id(state, id)
}

#[tauri::command]
pub fn delete_hotel(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM catalogue_hotels WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}