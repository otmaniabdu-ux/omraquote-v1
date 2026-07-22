use crate::models::segment_vol::SegmentVol;
use crate::error::{AppResult, AppError};
use rusqlite::{params, Connection};
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn list_by_devis(conn: &Connection, devis_id: i64) -> AppResult<Vec<SegmentVol>> {
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, ordre, compagnie, numero_vol, classe, date_vol,
                aeroport_depart, aeroport_arrivee, heure_depart, heure_arrivee,
                prix_adulte, prix_enfant, prix_bebe, devise_prix, remarques
         FROM segments_vol WHERE devis_id = ?1"
    )?;
    let mut rows = stmt.query(params![devis_id])?;
    let mut list = Vec::new();
    while let Some(row) = rows.next()? {
        let prix_adulte_str: String = row.get(11)?;
        let prix_enfant_str: String = row.get(12)?;
        let prix_bebe_str: String = row.get(13)?;

        let prix_adulte = Decimal::from_str(&prix_adulte_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing prix_adulte: {}", e)))?;
        let prix_enfant = Decimal::from_str(&prix_enfant_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing prix_enfant: {}", e)))?;
        let prix_bebe = Decimal::from_str(&prix_bebe_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing prix_bebe: {}", e)))?;

        list.push(SegmentVol {
            id: row.get(0)?,
            devis_id: row.get(1)?,
            ordre: row.get(2)?,
            compagnie: row.get(3)?,
            numero_vol: row.get(4)?,
            classe: row.get(5)?,
            date_vol: row.get(6)?,
            aeroport_depart: row.get(7)?,
            aeroport_arrivee: row.get(8)?,
            heure_depart: row.get(9)?,
            heure_arrivee: row.get(10)?,
            prix_adulte,
            prix_enfant,
            prix_bebe,
            devise_prix: row.get(14)?,
            remarques: row.get(15)?,
        });
    }
    Ok(list)
}
