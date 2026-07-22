use crate::models::devis::{Devis, DevisCreate, DevisUpdate};
use crate::error::{AppResult, AppError};
use crate::services::numerotation::generer_numero_devis;
use crate::services::validation::{alerte_passeport, valider_dates_sejour};
use crate::services::calcul_prix::calculer_totaux_devis;
use crate::services::db::{vols, hebergements, transferts, prestations};
use rusqlite::{params, Connection, OptionalExtension};
use rust_decimal::Decimal;
use std::str::FromStr;
use chrono::{NaiveDate, NaiveDateTime};

fn parse_decimal(s: &str, field_name: &str) -> AppResult<Decimal> {
    Decimal::from_str(s).map_err(|e| AppError::Internal(format!("Erreur parsing {}: {}", field_name, e)))
}

fn parse_opt_decimal(s: Option<String>, field_name: &str) -> AppResult<Option<Decimal>> {
    match s {
        Some(val) => Ok(Some(parse_decimal(&val, field_name)?)),
        None => Ok(None),
    }
}

pub fn get_alertes_tous_devis(conn: &Connection) -> AppResult<Vec<serde_json::Value>> {
    let devis_list = list(conn)?;
    let mut result = Vec::new();
    for devis in devis_list {
        let mut stmt = conn.prepare(
            "SELECT date_expiration_passeport FROM passagers WHERE devis_id = ?1"
        )?;
        let devis_id = devis.id.ok_or(AppError::NotFound("L'ID du devis est manquant".to_string()))?;
        let mut rows = stmt.query(&[&devis_id])?;

        let mut alerte = false;
        while let Some(row) = rows.next()? {
            let date_exp: Option<NaiveDate> = row.get(0)?;
            if let Some(exp) = date_exp {
                if alerte_passeport(exp, devis.date_retour) {
                    alerte = true;
                    break;
                }
            }
        }

        result.push(serde_json::json!({
            "devis_id": devis.id,
            "numero_devis": devis.numero_devis,
            "alerte": alerte,
        }));
    }
    Ok(result)
}

pub fn get_alertes_devis(conn: &Connection, devis_id: i64) -> AppResult<Vec<serde_json::Value>> {
    let devis = get_by_id(conn, devis_id)?;
    let mut stmt = conn.prepare(
        "SELECT id, nom_complet, date_expiration_passeport FROM passagers WHERE devis_id = ?1"
    )?;
    let mut rows = stmt.query(&[&devis_id])?;

    let mut result = Vec::new();
    while let Some(row) = rows.next()? {
        let passager_id: i64 = row.get(0)?;
        let nom: String = row.get(1)?;
        let date_expiration: Option<NaiveDate> = row.get(2)?;

        let alerte = match date_expiration {
            Some(exp) => alerte_passeport(exp, devis.date_retour),
            None => false,
        };

        result.push(serde_json::json!({
            "passager_id": passager_id,
            "nom": nom,
            "date_expiration": date_expiration,
            "alerte": alerte,
        }));
    }
    Ok(result)
}

pub fn create(conn: &Connection, devis_data: DevisCreate) -> AppResult<Devis> {
    valider_dates_sejour(devis_data.date_depart, devis_data.date_retour)?;
    let numero_devis = generer_numero_devis(conn, devis_data.date_depart)?;

    let mut stmt = conn.prepare(
        "INSERT INTO devis (
            numero_devis, client_id, date_depart, date_retour, type_visa,
            assurance_medicale, devise_achat, taux_sar_dzd, taux_usd_dzd, taux_eur_dzd,
            marge_type, marge_valeur, notes_internes
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)"
    )?;

    stmt.execute(params![
        numero_devis,
        devis_data.client_id,
        devis_data.date_depart,
        devis_data.date_retour,
        devis_data.type_visa,
        devis_data.assurance_medicale,
        devis_data.devise_achat,
        devis_data.taux_sar_dzd.to_string(),
        devis_data.taux_usd_dzd.to_string(),
        devis_data.taux_eur_dzd.to_string(),
        devis_data.marge_type,
        devis_data.marge_valeur.to_string(),
        devis_data.notes_internes,
    ])?;

    let id = conn.last_insert_rowid();
    get_by_id(conn, id)
}

pub fn get_by_id(conn: &Connection, id: i64) -> AppResult<Devis> {
    let mut stmt = conn.prepare(
        "SELECT id, numero_devis, client_id, date_creation, date_depart, date_retour,
                type_visa, assurance_medicale, devise_achat, taux_sar_dzd, taux_usd_dzd, taux_eur_dzd,
                marge_type, marge_valeur, cout_net_total, montant_marge, prix_vente_total,
                statut, remise, notes_internes, updated_at
         FROM devis WHERE id = ?1"
    )?;

    let devis = stmt.query_row(params![id], |row| {
        let taux_sar: String = row.get(9)?;
        let taux_usd: String = row.get(10)?;
        let taux_eur: String = row.get(11)?;
        let marge_val: String = row.get(13)?;
        let cout_net_str: Option<String> = row.get(14)?;
        let marge_tot_str: Option<String> = row.get(15)?;
        let prix_vente_str: Option<String> = row.get(16)?;
        let remise_str: Option<String> = row.get(18)?;

        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
            taux_sar,
            taux_usd,
            taux_eur,
            row.get(12)?,
            marge_val,
            cout_net_str,
            marge_tot_str,
            prix_vente_str,
            row.get(17)?,
            remise_str,
            row.get(19)?,
            row.get(20)?,
        ))
    }).optional()?;

    let (
        id, num, client_id, date_creation, date_depart, date_retour,
        type_visa, assurance_medicale, devise_achat, taux_sar, taux_usd, taux_eur,
        marge_type, marge_val, cout_net_str, marge_tot_str, prix_vente_str,
        statut, remise_str, notes_internes, updated_at
    ) = devis.ok_or_else(|| AppError::NotFound(format!("Devis avec l'ID {} introuvable", id)))?;

    Ok(Devis {
        id: Some(id),
        numero_devis: num,
        client_id,
        date_creation,
        date_depart,
        date_retour,
        type_visa,
        assurance_medicale,
        devise_achat,
        taux_sar_dzd: parse_decimal(&taux_sar, "taux_sar_dzd")?,
        taux_usd_dzd: parse_decimal(&taux_usd, "taux_usd_dzd")?,
        taux_eur_dzd: parse_decimal(&taux_eur, "taux_eur_dzd")?,
        marge_type,
        marge_valeur: parse_decimal(&marge_val, "marge_valeur")?,
        cout_net_total: parse_opt_decimal(cout_net_str, "cout_net_total")?,
        montant_marge: parse_opt_decimal(marge_tot_str, "montant_marge")?,
        prix_vente_total: parse_opt_decimal(prix_vente_str, "prix_vente_total")?,
        statut,
        remise: parse_opt_decimal(remise_str, "remise")?,
        notes_internes,
        updated_at,
    })
}

pub fn list(conn: &Connection) -> AppResult<Vec<Devis>> {
    let mut stmt = conn.prepare(
        "SELECT id, numero_devis, client_id, date_creation, date_depart, date_retour,
                type_visa, assurance_medicale, devise_achat, taux_sar_dzd, taux_usd_dzd, taux_eur_dzd,
                marge_type, marge_valeur, cout_net_total, montant_marge, prix_vente_total,
                statut, remise, notes_internes, updated_at
         FROM devis ORDER BY id DESC"
    )?;

    let mut rows = stmt.query([])?;
    let mut devis_list = Vec::new();
    while let Some(row) = rows.next()? {
        let taux_sar: String = row.get(9)?;
        let taux_usd: String = row.get(10)?;
        let taux_eur: String = row.get(11)?;
        let marge_val: String = row.get(13)?;
        let cout_net_str: Option<String> = row.get(14)?;
        let marge_tot_str: Option<String> = row.get(15)?;
        let prix_vente_str: Option<String> = row.get(16)?;
        let remise_str: Option<String> = row.get(18)?;

        devis_list.push(Devis {
            id: Some(row.get(0)?),
            numero_devis: row.get(1)?,
            client_id: row.get(2)?,
            date_creation: row.get(3)?,
            date_depart: row.get(4)?,
            date_retour: row.get(5)?,
            type_visa: row.get(6)?,
            assurance_medicale: row.get(7)?,
            devise_achat: row.get(8)?,
            taux_sar_dzd: parse_decimal(&taux_sar, "taux_sar_dzd")?,
            taux_usd_dzd: parse_decimal(&taux_usd, "taux_usd_dzd")?,
            taux_eur_dzd: parse_decimal(&taux_eur, "taux_eur_dzd")?,
            marge_type: row.get(12)?,
            marge_valeur: parse_decimal(&marge_val, "marge_valeur")?,
            cout_net_total: parse_opt_decimal(cout_net_str, "cout_net_total")?,
            montant_marge: parse_opt_decimal(marge_tot_str, "montant_marge")?,
            prix_vente_total: parse_opt_decimal(prix_vente_str, "prix_vente_total")?,
            statut: row.get(17)?,
            remise: parse_opt_decimal(remise_str, "remise")?,
            notes_internes: row.get(19)?,
            updated_at: row.get(20)?,
        });
    }
    Ok(devis_list)
}

pub fn update(conn: &Connection, id: i64, update_data: DevisUpdate) -> AppResult<Devis> {
    if let (Some(depart), Some(retour)) = (update_data.date_depart, update_data.date_retour) {
        valider_dates_sejour(depart, retour)?;
    }

    let mut sets = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(date_retour) = update_data.date_retour {
        sets.push("date_retour = ?");
        params_vec.push(Box::new(date_retour));
    }
    if let Some(type_visa) = update_data.type_visa {
        sets.push("type_visa = ?");
        params_vec.push(Box::new(type_visa));
    }
    if let Some(assurance_medicale) = update_data.assurance_medicale {
        sets.push("assurance_medicale = ?");
        params_vec.push(Box::new(assurance_medicale));
    }
    if let Some(devise_achat) = update_data.devise_achat {
        sets.push("devise_achat = ?");
        params_vec.push(Box::new(devise_achat));
    }
    if let Some(taux_sar_dzd) = update_data.taux_sar_dzd {
        sets.push("taux_sar_dzd = ?");
        params_vec.push(Box::new(taux_sar_dzd.to_string()));
    }
    if let Some(taux_usd_dzd) = update_data.taux_usd_dzd {
        sets.push("taux_usd_dzd = ?");
        params_vec.push(Box::new(taux_usd_dzd.to_string()));
    }
    if let Some(taux_eur_dzd) = update_data.taux_eur_dzd {
        sets.push("taux_eur_dzd = ?");
        params_vec.push(Box::new(taux_eur_dzd.to_string()));
    }
    if let Some(marge_type) = update_data.marge_type {
        sets.push("marge_type = ?");
        params_vec.push(Box::new(marge_type));
    }
    if let Some(marge_valeur) = update_data.marge_valeur {
        sets.push("marge_valeur = ?");
        params_vec.push(Box::new(marge_valeur.to_string()));
    }
    if let Some(statut) = update_data.statut {
        sets.push("statut = ?");
        params_vec.push(Box::new(statut));
    }
    if let Some(remise) = update_data.remise {
        sets.push("remise = ?");
        params_vec.push(Box::new(remise.to_string()));
    }
    if let Some(notes_internes) = update_data.notes_internes {
        sets.push("notes_internes = ?");
        params_vec.push(Box::new(notes_internes));
    }

    if sets.is_empty() {
        return Err(AppError::Validation("Aucune donnée à mettre à jour".to_string()));
    }

    sets.push("updated_at = CURRENT_TIMESTAMP");
    let query = format!(
        "UPDATE devis SET {} WHERE id = ?",
        sets.join(", ")
    );

    let mut stmt = conn.prepare(&query)?;
    let current = get_by_id(conn, id)?;
    let depart = update_data.date_depart.unwrap_or(current.date_depart);
    let retour = update_data.date_retour.unwrap_or(current.date_retour);
    valider_dates_sejour(depart, retour)?;

    let mut final_params: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    final_params.push(&id);
    stmt.execute(&final_params[..])?;

    get_by_id(conn, id)
}

pub fn delete(conn: &Connection, id: i64) -> AppResult<()> {
    get_by_id(conn, id)?;
    conn.execute("DELETE FROM devis WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn calculate_totals(conn: &Connection, devis_id: i64) -> AppResult<Devis> {
    let devis = get_by_id(conn, devis_id)?;
    let segments_vol = vols::list_by_devis(conn, devis_id)?;
    let hebergements = hebergements::list_hebergements_by_devis(conn, devis_id)?;
    let transferts = transferts::list_by_devis(conn, devis_id)?;
    let prestations_vip = prestations::list_by_devis(conn, devis_id)?;

    let (cout_net, marge, prix_vente) = calculer_totaux_devis(
        &devis,
        &segments_vol,
        &hebergements,
        &transferts,
        &prestations_vip,
    )?;

    conn.execute(
        "UPDATE devis SET cout_net_total = ?1, montant_marge = ?2, prix_vente_total = ?3, updated_at = CURRENT_TIMESTAMP WHERE id = ?4",
        &[&cout_net.to_string(), &marge.to_string(), &prix_vente.to_string(), &devis_id.to_string()],
    )?;

    get_by_id(conn, devis_id)
}
