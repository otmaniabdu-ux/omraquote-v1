use crate::db::DbState;
use crate::models::passager::Passager;
use crate::services::validation::{valider_dates_devis, valider_hebergement, verifier_alertes_passeports};
use chrono::NaiveDate;
use rusqlite::params;
use tauri::State;
use std::sync::Mutex;

// Validation des dates d'un devis
#[tauri::command]
pub fn valider_dates_devis_command(
    date_depart: NaiveDate,
    date_retour: NaiveDate,
) -> Result<(), String> {
    valider_dates_devis(date_depart, date_retour)
}

// Validation des dates d'un hébergement
#[tauri::command]
pub fn valider_hebergement_command(
    checkin: NaiveDate,
    checkout: NaiveDate,
    date_depart: NaiveDate,
    date_retour: NaiveDate,
) -> Result<(), String> {
    valider_hebergement(checkin, checkout, date_depart, date_retour)
}

// Obtenir les alertes passeport pour un devis donné
#[tauri::command]
pub fn get_passeport_alertes(
    state: State<DbState>,
    devis_id: i64,
) -> Result<Vec<(i64, String)>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    // Récupérer la date de retour du devis
    let date_retour: NaiveDate = conn.query_row(
        "SELECT date_retour FROM devis WHERE id = ?1",
        params![devis_id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    // Récupérer tous les passagers du devis
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, categorie, nom_complet, date_naissance, nationalite,
                numero_passeport, date_expiration_passeport, lieu_delivrance, remarques
         FROM passagers WHERE devis_id = ?1"
    ).map_err(|e| e.to_string())?;
    let mut rows = stmt.query(params![devis_id]).map_err(|e| e.to_string())?;
    let mut passagers = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        passagers.push(Passager {
            id: row.get(0)?,
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

    Ok(verifier_alertes_passeports(&passagers, date_retour))
}

// Vérifier un seul passager (utile lors de la saisie)
#[tauri::command]
pub fn check_passager_passeport(
    passager: Passager,
    date_retour: NaiveDate,
) -> Result<(bool, Option<String>), String> {
    use crate::services::validation::verifier_alerte_passeport_passager;
    Ok(verifier_alerte_passeport_passager(&passager, date_retour))
}