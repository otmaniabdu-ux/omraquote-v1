use crate::db::DbState;
use crate::models::segment_vol::SegmentVol;
use rusqlite::params;
use tauri::State;

pub fn list_segments_by_devis(state: State<DbState>, devis_id: i64) -> Result<Vec<SegmentVol>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, ordre, compagnie, numero_vol, classe, date_vol,
                aeroport_depart, aeroport_arrivee, heure_depart, heure_arrivee,
                prix_adulte, prix_enfant, prix_bebe, devise_prix, remarques
         FROM segments_vol WHERE devis_id = ?1"
    ).map_err(|e| e.to_string())?;
    let mut rows = stmt.query(params![devis_id]).map_err(|e| e.to_string())?;
    let mut list = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
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
            prix_adulte: row.get(11)?,
            prix_enfant: row.get(12)?,
            prix_bebe: row.get(13)?,
            devise_prix: row.get(14)?,
            remarques: row.get(15)?,
        });
    }
    Ok(list)
}