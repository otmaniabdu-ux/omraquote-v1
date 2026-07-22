use crate::models::hebergement::{Hebergement, HebergementCreate};
use crate::error::{AppResult, AppError};
use crate::services::validation::valider_dates_hebergement;
use rusqlite::{params, Connection, OptionalExtension};
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn create_hebergement(
    conn: &Connection,
    hebergement_data: HebergementCreate,
) -> AppResult<Hebergement> {
    valider_dates_hebergement(hebergement_data.date_checkin, hebergement_data.date_checkout)?;

    use crate::services::calcul_nuitees::calculer_nuitees;
    let nb_nuitees = calculer_nuitees(hebergement_data.date_checkin, hebergement_data.date_checkout)?;

    let mut stmt = conn.prepare(
        "INSERT INTO hebergements (devis_id, ville, nom_hotel, type_chambre, formule_repas, vue,
                                   date_checkin, date_checkout, nb_nuitees, prix_par_nuit, devise_prix,
                                   taxes_incluses, remarques)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)"
    )?;

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
        hebergement_data.prix_par_nuit.to_string(),
        hebergement_data.devise_prix,
        hebergement_data.taxes_incluses,
        hebergement_data.remarques,
    ])?;

    let id = conn.last_insert_rowid();
    get_hebergement_by_id(conn, id)
}

pub fn get_hebergement_by_id(conn: &Connection, id: i64) -> AppResult<Hebergement> {
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, ville, nom_hotel, type_chambre, formule_repas, vue,
                date_checkin, date_checkout, nb_nuitees, prix_par_nuit, devise_prix,
                taxes_incluses, remarques
         FROM hebergements WHERE id = ?1"
    )?;

    let hebergement = stmt.query_row(params![id], |row| {
        let prix_par_nuit_str: String = row.get(10)?;
        let prix_par_nuit = Decimal::from_str(&prix_par_nuit_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(10, rusqlite::types::Type::Text, Box::new(e)))?;

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
            prix_par_nuit,
            devise_prix: row.get(11)?,
            taxes_incluses: row.get(12)?,
            remarques: row.get(13)?,
        })
    }).optional()?;

    hebergement.ok_or_else(|| AppError::NotFound("Hebergement non trouve".to_string()))
}

pub fn list_hebergements_by_devis(conn: &Connection, devis_id: i64) -> AppResult<Vec<Hebergement>> {
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, ville, nom_hotel, type_chambre, formule_repas, vue,
                date_checkin, date_checkout, nb_nuitees, prix_par_nuit, devise_prix,
                taxes_incluses, remarques
         FROM hebergements WHERE devis_id = ?1 ORDER BY id"
    )?;

    let hebergement_iter = stmt.query_map(params![devis_id], |row| {
        let prix_par_nuit_str: String = row.get(10)?;
        let prix_par_nuit = Decimal::from_str(&prix_par_nuit_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(10, rusqlite::types::Type::Text, Box::new(e)))?;

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
            prix_par_nuit,
            devise_prix: row.get(11)?,
            taxes_incluses: row.get(12)?,
            remarques: row.get(13)?,
        })
    })?;

    let mut list = Vec::new();
    for hebergement in hebergement_iter {
        list.push(hebergement?);
    }
    Ok(list)
}

pub fn update_hebergement(
    conn: &Connection,
    id: i64,
    update_data: HebergementCreate,
) -> AppResult<Hebergement> {
    let existing = get_hebergement_by_id(conn, id)?;

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
    )?;

    stmt.execute(params![
        update_data.ville,
        update_data.nom_hotel,
        update_data.type_chambre,
        update_data.formule_repas,
        update_data.vue,
        update_data.date_checkin,
        update_data.date_checkout,
        nb_nuitees,
        update_data.prix_par_nuit.to_string(),
        update_data.devise_prix,
        update_data.taxes_incluses,
        update_data.remarques,
        id,
    ])?;

    get_hebergement_by_id(conn, id)
}

pub fn delete_hebergement(conn: &Connection, id: i64) -> AppResult<()> {
    conn.execute("DELETE FROM hebergements WHERE id = ?1", params![id])?;
    Ok(())
}
