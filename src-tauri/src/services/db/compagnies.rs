use crate::models::catalogue::{CompagnieCatalogue, CompagnieCatalogueCreate, CompagnieCatalogueUpdate};
use crate::error::{AppResult, AppError};
use rusqlite::{params, Connection, OptionalExtension};

pub fn create_compagnie(
    conn: &Connection,
    compagnie_data: CompagnieCatalogueCreate,
) -> AppResult<CompagnieCatalogue> {
    let actif = compagnie_data.actif.unwrap_or(true);

    let mut stmt = conn.prepare(
        "INSERT INTO catalogue_compagnies (code_iata, nom_compagnie, pays, site_web, actif)
         VALUES (?1, ?2, ?3, ?4, ?5)"
    )?;

    stmt.execute(params![
        compagnie_data.code_iata,
        compagnie_data.nom_compagnie,
        compagnie_data.pays,
        compagnie_data.site_web,
        actif,
    ])?;

    let id = conn.last_insert_rowid();
    get_compagnie_by_id(conn, id)
}

pub fn get_compagnie_by_id(conn: &Connection, id: i64) -> AppResult<CompagnieCatalogue> {
    let mut stmt = conn.prepare(
        "SELECT id, code_iata, nom_compagnie, pays, site_web, actif
         FROM catalogue_compagnies WHERE id = ?1"
    )?;

    let compagnie = stmt.query_row(params![id], |row| {
        Ok(CompagnieCatalogue {
            id: Some(row.get(0)?),
            code_iata: row.get(1)?,
            nom_compagnie: row.get(2)?,
            pays: row.get(3)?,
            site_web: row.get(4)?,
            actif: row.get(5)?,
        })
    }).optional()?;

    compagnie.ok_or_else(|| AppError::NotFound("Compagnie non trouvée".to_string()))
}

pub fn list_compagnies(conn: &Connection, actif_seulement: Option<bool>) -> AppResult<Vec<CompagnieCatalogue>> {
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

    let mut stmt = conn.prepare(&query)?;
    let final_params: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let compagnie_iter = stmt.query_map(&final_params[..], |row| {
        Ok(CompagnieCatalogue {
            id: Some(row.get(0)?),
            code_iata: row.get(1)?,
            nom_compagnie: row.get(2)?,
            pays: row.get(3)?,
            site_web: row.get(4)?,
            actif: row.get(5)?,
        })
    })?;

    let mut compagnies = Vec::new();
    for compagnie in compagnie_iter {
        compagnies.push(compagnie?);
    }
    Ok(compagnies)
}

pub fn update_compagnie(
    conn: &Connection,
    id: i64,
    update_data: CompagnieCatalogueUpdate,
) -> AppResult<CompagnieCatalogue> {
    let mut fields = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(code_iata) = update_data.code_iata {
        fields.push("code_iata");
        params.push(Box::new(code_iata));
    }
    if let Some(nom_compagnie) = update_data.nom_compagnie {
        fields.push("nom_compagnie");
        params.push(Box::new(nom_compagnie));
    }
    if let Some(pays) = update_data.pays {
        fields.push("pays");
        params.push(Box::new(pays));
    }
    if let Some(site_web) = update_data.site_web {
        fields.push("site_web");
        params.push(Box::new(site_web));
    }
    if let Some(actif) = update_data.actif {
        fields.push("actif");
        params.push(Box::new(actif));
    }

    if fields.is_empty() {
        return Err(AppError::Validation("Aucune donnée à mettre à jour".to_string()));
    }

    let query = crate::utils::query_builder::build_update_query("catalogue_compagnies", &fields, "id");

    let mut stmt = conn.prepare(&query)?;
    let mut final_params: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    final_params.push(&id);
    stmt.execute(&final_params[..])?;

    get_compagnie_by_id(conn, id)
}

pub fn delete_compagnie(conn: &Connection, id: i64) -> AppResult<()> {
    conn.execute("DELETE FROM catalogue_compagnies WHERE id = ?1", params![id])?;
    Ok(())
}
