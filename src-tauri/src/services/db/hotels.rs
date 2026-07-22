use crate::models::catalogue::{HotelCatalogue, HotelCatalogueCreate, HotelCatalogueUpdate};
use crate::error::{AppResult, AppError};
use rusqlite::{params, Connection};

pub fn create(conn: &Connection, hotel_data: HotelCatalogueCreate) -> AppResult<HotelCatalogue> {
    let actif = hotel_data.actif.unwrap_or(true);

    let mut stmt = conn.prepare(
        "INSERT INTO catalogue_hotels (nom_hotel, ville, categorie, adresse, contact, site_web, remarques, actif)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
    )?;

    stmt.execute(params![
        hotel_data.nom_hotel,
        hotel_data.ville,
        hotel_data.categorie,
        hotel_data.adresse,
        hotel_data.contact,
        hotel_data.site_web,
        hotel_data.remarques,
        actif,
    ])?;

    let id = conn.last_insert_rowid();
    get_by_id(conn, id)
}

pub fn get_by_id(conn: &Connection, id: i64) -> AppResult<HotelCatalogue> {
    let mut stmt = conn.prepare(
        "SELECT id, nom_hotel, ville, categorie, adresse, contact, site_web, remarques, actif
         FROM catalogue_hotels WHERE id = ?1"
    )?;

    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
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
        Err(AppError::NotFound(format!("Hotel avec l'ID {} introuvable", id)))
    }
}

pub fn list(conn: &Connection, actif_seulement: Option<bool>) -> AppResult<Vec<HotelCatalogue>> {
    let mut query = String::from(
        "SELECT id, nom_hotel, ville, categorie, adresse, contact, site_web, remarques, actif
         FROM catalogue_hotels"
    );
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(actif) = actif_seulement {
        query.push_str(" WHERE actif = ?");
        params_vec.push(Box::new(actif));
    }

    query.push_str(" ORDER BY nom_hotel");

    let mut stmt = conn.prepare(&query)?;
    let final_params: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    let mut rows = stmt.query(&final_params[..])?;
    let mut hotels = Vec::new();

    while let Some(row) = rows.next()? {
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

pub fn update(conn: &Connection, id: i64, update_data: HotelCatalogueUpdate) -> AppResult<HotelCatalogue> {
    let mut sets = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(nom_hotel) = update_data.nom_hotel {
        sets.push("nom_hotel = ?");
        params_vec.push(Box::new(nom_hotel));
    }
    if let Some(ville) = update_data.ville {
        sets.push("ville = ?");
        params_vec.push(Box::new(ville));
    }
    if let Some(categorie) = update_data.categorie {
        sets.push("categorie = ?");
        params_vec.push(Box::new(categorie));
    }
    if let Some(adresse) = update_data.adresse {
        sets.push("adresse = ?");
        params_vec.push(Box::new(adresse));
    }
    if let Some(contact) = update_data.contact {
        sets.push("contact = ?");
        params_vec.push(Box::new(contact));
    }
    if let Some(site_web) = update_data.site_web {
        sets.push("site_web = ?");
        params_vec.push(Box::new(site_web));
    }
    if let Some(remarques) = update_data.remarques {
        sets.push("remarques = ?");
        params_vec.push(Box::new(remarques));
    }
    if let Some(actif) = update_data.actif {
        sets.push("actif = ?");
        params_vec.push(Box::new(actif));
    }

    if sets.is_empty() {
        return Err(AppError::Validation("Aucune donnee a mettre a jour".to_string()));
    }

    sets.push("updated_at = CURRENT_TIMESTAMP");
    let query = format!(
        "UPDATE catalogue_hotels SET {} WHERE id = ?",
        sets.join(", ")
    );

    let mut stmt = conn.prepare(&query)?;
    let mut final_params: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    final_params.push(&id);
    stmt.execute(&final_params[..])?;

    get_by_id(conn, id)
}

pub fn delete(conn: &Connection, id: i64) -> AppResult<()> {
    get_by_id(conn, id)?;
    conn.execute("DELETE FROM catalogue_hotels WHERE id = ?1", params![id])?;
    Ok(())
}
