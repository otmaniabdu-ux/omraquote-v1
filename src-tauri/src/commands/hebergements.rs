use crate::db::DbState;
use crate::models::hebergement::{Hebergement, HebergementCreate};
use crate::services::validation::valider_dates_hebergement;
use rusqlite::params;
use tauri::State;

/// Cree un nouvel hebergement lie a un devis.
#[tauri::command]
pub fn create_hebergement(
    state: State<DbState>,
    hebergement_data: HebergementCreate,
) -> Result<Hebergement, String> {
    // Valider les dates
    valider_dates_hebergement(hebergement_data.date_checkin, hebergement_data.date_checkout)?;

    let conn = state.0.lock().map_err(|e| e.to_string())?;
    // Calculer automatiquement les nuitees
    use crate::services::calcul_nuitees::calculer_nuitees;
    let nb_nuitees = calculer_nuitees(hebergement_data.date_checkin, hebergement_data.date_checkout)?;

    let mut stmt = conn.prepare(
        "INSERT INTO hebergements (devis_id, ville, nom_hotel, type_chambre, formule_repas, vue,
                                   date_checkin, date_checkout, nb_nuitees, prix_par_nuit, devise_prix,
                                   taxes_incluses, remarques)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)"
    ).map_err(|e| e.to_string())?;

    stmt.execute(params![
        hebergement_data.devis_id,
        hebergement_data.ville,
        hebergement_data.nom_hotel,
        hebergement_data.type_chambre,
        hebergement_data.formule_repas,
        hebergement_data.vue,
        hebergement_data.date_checkin,
        hebergement_data.date_checkout,
        nb_nuitees,
        hebergement_data.prix_par_nuit,
        hebergement_data.devise_prix,
        hebergement_data.taxes_incluses,
        hebergement_data.remarques,
    ]).map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    get_hebergement_by_id(state, id)
}

/// Retourne un hebergement par son identifiant.
#[tauri::command]
pub fn get_hebergement_by_id(
    state: State<DbState>,
    id: i64,
) -> Result<Hebergement, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, ville, nom_hotel, type_chambre, formule_repas, vue,
                date_checkin, date_checkout, nb_nuitees, prix_par_nuit, devise_prix,
                taxes_incluses, remarques
         FROM hebergements WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(Hebergement {
            id: Some(row.get(0)?),
            devis_id: row.get(1)?,
            ville: row.get(2)?,
            nom_hotel: row.get(3)?,
            type_chambre: row.get(4)?,
            formule_repas: row.get(5)?,
            vue: row.get(6)?,
            date_checkin: row.get(7)?,
            date_checkout: row.get(8)?,
            nb_nuitees: row.get(9)?,
            prix_par_nuit: row.get(10)?,
            devise_prix: row.get(11)?,
            taxes_incluses: row.get(12)?,
            remarques: row.get(13)?,
        })
    } else {
        Err("Hebergement non trouve".to_string())
    }
}

/// Retourne la liste des hebergements lies a un devis donne.
#[tauri::command]
pub fn list_hebergements_by_devis(
    state: State<DbState>,
    devis_id: i64,
) -> Result<Vec<Hebergement>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, ville, nom_hotel, type_chambre, formule_repas, vue,
                date_checkin, date_checkout, nb_nuitees, prix_par_nuit, devise_prix,
                taxes_incluses, remarques
         FROM hebergements WHERE devis_id = ?1 ORDER BY id"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(params![devis_id]).map_err(|e| e.to_string())?;
    let mut list = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        list.push(Hebergement {
            id: Some(row.get(0)?),
            devis_id: row.get(1)?,
            ville: row.get(2)?,
            nom_hotel: row.get(3)?,
            type_chambre: row.get(4)?,
            formule_repas: row.get(5)?,
            vue: row.get(6)?,
            date_checkin: row.get(7)?,
            date_checkout: row.get(8)?,
            nb_nuitees: row.get(9)?,
            prix_par_nuit: row.get(10)?,
            devise_prix: row.get(11)?,
            taxes_incluses: row.get(12)?,
            remarques: row.get(13)?,
        });
    }
    Ok(list)
}

/// Met a jour un hebergement existant.
#[tauri::command]
pub fn update_hebergement(
    state: State<DbState>,
    id: i64,
    update_data: HebergementCreate,
) -> Result<Hebergement, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    // Recupere l'hebergement existant pour les calculs de nuitees
    let existing = get_hebergement_by_id(state.clone(), id)?;

    // Valider les dates si elles ont change
    if update_data.date_checkin != existing.date_checkin || update_data.date_checkout != existing.date_checkout {
        valider_dates_hebergement(update_data.date_checkin, update_data.date_checkout)?;
    }

    let nb_nuitees = crate::services::calcul_nuitees::calculer_nuitees(
        update_data.date_checkin, update_data.date_checkout
    )?;

    let mut stmt = conn.prepare(
        "UPDATE hebergements SET
            ville = ?1, nom_hotel = ?2, type_chambre = ?3, formule_repas = ?4, vue = ?5,
            date_checkin = ?6, date_checkout = ?7, nb_nuitees = ?8, prix_par_nuit = ?9,
            devise_prix = ?10, taxes_incluses = ?11, remarques = ?12,
            updated_at = CURRENT_TIMESTAMP
         WHERE id = ?13"
    ).map_err(|e| e.to_string())?;

    stmt.execute(params![
        update_data.ville,
        update_data.nom_hotel,
        update_data.type_chambre,
        update_data.formule_repas,
        update_data.vue,
        update_data.date_checkin,
        update_data.date_checkout,
        nb_nuitees,
        update_data.prix_par_nuit,
        update_data.devise_prix,
        update_data.taxes_incluses,
        update_data.remarques,
        id,
    ]).map_err(|e| e.to_string())?;

    get_hebergement_by_id(state, id)
}

/// Supprime un hebergement par son identifiant.
#[tauri::command]
pub fn delete_hebergement(
    state: State<DbState>,
    id: i64,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM hebergements WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
