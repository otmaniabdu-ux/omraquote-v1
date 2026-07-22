use crate::models::passager::{Passager, PassagerCreate};
use crate::error::{AppResult, AppError};
use rusqlite::{params, Connection};

pub fn create(conn: &Connection, passager_data: PassagerCreate) -> AppResult<Passager> {
    let mut stmt = conn.prepare(
        "INSERT INTO passagers (devis_id, categorie, nom_complet, date_naissance, nationalite,
                                numero_passeport, date_expiration_passeport, lieu_delivrance, remarques)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
    )?;

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
    ])?;

    let id = conn.last_insert_rowid();
    get_by_id(conn, id)
}

pub fn get_by_id(conn: &Connection, id: i64) -> AppResult<Passager> {
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, categorie, nom_complet, date_naissance, nationalite,
                numero_passeport, date_expiration_passeport, lieu_delivrance, remarques
         FROM passagers WHERE id = ?1"
    )?;

    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
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
        Err(AppError::NotFound(format!("Passager avec l'ID {} introuvable", id)))
    }
}

pub fn list_by_devis(conn: &Connection, devis_id: i64) -> AppResult<Vec<Passager>> {
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, categorie, nom_complet, date_naissance, nationalite,
                numero_passeport, date_expiration_passeport, lieu_delivrance, remarques
         FROM passagers WHERE devis_id = ?1 ORDER BY id"
    )?;

    let mut rows = stmt.query(params![devis_id])?;
    let mut passagers = Vec::new();
    while let Some(row) = rows.next()? {
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

pub fn delete(conn: &Connection, id: i64) -> AppResult<()> {
    get_by_id(conn, id)?;
    conn.execute("DELETE FROM passagers WHERE id = ?1", params![id])?;
    Ok(())
}
