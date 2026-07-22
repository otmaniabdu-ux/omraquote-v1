use crate::models::transfert::{Transfert, TransfertCreate, TransfertUpdate};
use crate::error::{AppResult, AppError};
use rusqlite::{params, Connection};
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn create(conn: &Connection, transfert_data: TransfertCreate) -> AppResult<Transfert> {
    let mut stmt = conn.prepare(
        "INSERT INTO transferts (devis_id, type_transfert, trajet, type_vehicule,
                                   date_transfert, heure_transfert, nombre_vehicules,
                                   prix_unitaire, devise_prix, remarques)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
    )?;

    stmt.execute(params![
        transfert_data.devis_id,
        transfert_data.type_transfert,
        transfert_data.trajet,
        transfert_data.type_vehicule,
        transfert_data.date_transfert,
        transfert_data.heure_transfert,
        transfert_data.nombre_vehicules,
        transfert_data.prix_unitaire.to_string(),
        transfert_data.devise_prix,
        transfert_data.remarques,
    ])?;

    let id = conn.last_insert_rowid();
    get_by_id(conn, id)
}

pub fn get_by_id(conn: &Connection, id: i64) -> AppResult<Transfert> {
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, type_transfert, trajet, type_vehicule,
                date_transfert, heure_transfert, nombre_vehicules,
                prix_unitaire, devise_prix, remarques
         FROM transferts WHERE id = ?1"
    )?;

    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        let prix_unitaire_str: String = row.get(8)?;
        let prix_unitaire = Decimal::from_str(&prix_unitaire_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing prix_unitaire: {}", e)))?;

        Ok(Transfert {
            id: Some(row.get(0)?),
            devis_id: row.get(1)?,
            type_transfert: row.get(2)?,
            trajet: row.get(3)?,
            type_vehicule: row.get(4)?,
            date_transfert: row.get(5)?,
            heure_transfert: row.get(6)?,
            nombre_vehicules: row.get(7)?,
            prix_unitaire,
            devise_prix: row.get(9)?,
            remarques: row.get(10)?,
        })
    } else {
        Err(AppError::NotFound(format!("Transfert avec l'ID {} introuvable", id)))
    }
}

pub fn list_by_devis(conn: &Connection, devis_id: i64) -> AppResult<Vec<Transfert>> {
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, type_transfert, trajet, type_vehicule,
                date_transfert, heure_transfert, nombre_vehicules,
                prix_unitaire, devise_prix, remarques
         FROM transferts WHERE devis_id = ?1 ORDER BY id"
    )?;

    let mut rows = stmt.query(params![devis_id])?;
    let mut list = Vec::new();
    while let Some(row) = rows.next()? {
        let prix_unitaire_str: String = row.get(8)?;
        let prix_unitaire = Decimal::from_str(&prix_unitaire_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing prix_unitaire: {}", e)))?;

        list.push(Transfert {
            id: Some(row.get(0)?),
            devis_id: row.get(1)?,
            type_transfert: row.get(2)?,
            trajet: row.get(3)?,
            type_vehicule: row.get(4)?,
            date_transfert: row.get(5)?,
            heure_transfert: row.get(6)?,
            nombre_vehicules: row.get(7)?,
            prix_unitaire,
            devise_prix: row.get(9)?,
            remarques: row.get(10)?,
        });
    }
    Ok(list)
}

pub fn update(conn: &Connection, id: i64, update_data: TransfertUpdate) -> AppResult<Transfert> {
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
        params_vec.push(Box::new(prix_unitaire.to_string()));
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
        return Err(AppError::Validation("Aucune donnee a mettre a jour".to_string()));
    }

    sets.push("updated_at = CURRENT_TIMESTAMP");
    let query = format!(
        "UPDATE transferts SET {} WHERE id = ?",
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
    conn.execute("DELETE FROM transferts WHERE id = ?1", params![id])?;
    Ok(())
}
