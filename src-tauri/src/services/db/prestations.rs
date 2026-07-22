use crate::models::prestation_vip::{PrestationVip, PrestationVipCreate, PrestationVipUpdate};
use crate::error::{AppResult, AppError};
use rusqlite::{params, Connection};
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn create(conn: &Connection, prestation_data: PrestationVipCreate) -> AppResult<PrestationVip> {
    let mut stmt = conn.prepare(
        "INSERT INTO prestations_vip (devis_id, type_prestation, description,
                                       prix_unitaire, quantite, devise_prix, remarques)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
    )?;

    stmt.execute(params![
        prestation_data.devis_id,
        prestation_data.type_prestation,
        prestation_data.description,
        prestation_data.prix_unitaire.to_string(),
        prestation_data.quantite,
        prestation_data.devise_prix,
        prestation_data.remarques,
    ])?;

    let id = conn.last_insert_rowid();
    get_by_id(conn, id)
}

pub fn get_by_id(conn: &Connection, id: i64) -> AppResult<PrestationVip> {
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, type_prestation, description,
                prix_unitaire, quantite, devise_prix, remarques
         FROM prestations_vip WHERE id = ?1"
    )?;

    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        let prix_unitaire_str: String = row.get(4)?;
        let prix_unitaire = Decimal::from_str(&prix_unitaire_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing prix_unitaire: {}", e)))?;

        Ok(PrestationVip {
            id: Some(row.get(0)?),
            devis_id: row.get(1)?,
            type_prestation: row.get(2)?,
            description: row.get(3)?,
            prix_unitaire,
            quantite: row.get(5)?,
            devise_prix: row.get(6)?,
            remarques: row.get(7)?,
        })
    } else {
        Err(AppError::NotFound(format!("Prestation VIP avec l'ID {} introuvable", id)))
    }
}

pub fn list_by_devis(conn: &Connection, devis_id: i64) -> AppResult<Vec<PrestationVip>> {
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, type_prestation, description,
                prix_unitaire, quantite, devise_prix, remarques
         FROM prestations_vip WHERE devis_id = ?1 ORDER BY id"
    )?;

    let mut rows = stmt.query(params![devis_id])?;
    let mut list = Vec::new();
    while let Some(row) = rows.next()? {
        let prix_unitaire_str: String = row.get(4)?;
        let prix_unitaire = Decimal::from_str(&prix_unitaire_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing prix_unitaire: {}", e)))?;

        list.push(PrestationVip {
            id: Some(row.get(0)?),
            devis_id: row.get(1)?,
            type_prestation: row.get(2)?,
            description: row.get(3)?,
            prix_unitaire,
            quantite: row.get(5)?,
            devise_prix: row.get(6)?,
            remarques: row.get(7)?,
        });
    }
    Ok(list)
}

pub fn update(conn: &Connection, id: i64, update_data: PrestationVipUpdate) -> AppResult<PrestationVip> {
    let mut fields = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(type_prestation) = update_data.type_prestation {
        fields.push("type_prestation");
        params_vec.push(Box::new(type_prestation));
    }
    if let Some(description) = update_data.description {
        fields.push("description");
        params_vec.push(Box::new(description));
    }
    if let Some(prix_unitaire) = update_data.prix_unitaire {
        fields.push("prix_unitaire");
        params_vec.push(Box::new(prix_unitaire.to_string()));
    }
    if let Some(quantite) = update_data.quantite {
        fields.push("quantite");
        params_vec.push(Box::new(quantite));
    }
    if let Some(devise_prix) = update_data.devise_prix {
        fields.push("devise_prix");
        params_vec.push(Box::new(devise_prix));
    }
    if let Some(remarques) = update_data.remarques {
        fields.push("remarques");
        params_vec.push(Box::new(remarques));
    }

    if fields.is_empty() {
        return Err(AppError::Validation("Aucune donnee a mettre a jour".to_string()));
    }

    let query = crate::utils::query_builder::build_update_query("prestations_vip", &fields, "id");

    let mut stmt = conn.prepare(&query)?;
    let mut final_params: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    final_params.push(&id);
    stmt.execute(&final_params[..])?;

    get_by_id(conn, id)
}

pub fn delete(conn: &Connection, id: i64) -> AppResult<()> {
    get_by_id(conn, id)?;
    conn.execute("DELETE FROM prestations_vip WHERE id = ?1", params![id])?;
    Ok(())
}
