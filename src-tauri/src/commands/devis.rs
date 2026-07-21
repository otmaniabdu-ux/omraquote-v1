use crate::db::DbState;
use crate::models::devis::{Devis, DevisCreate, DevisUpdate};
use chrono::NaiveDate;
use rusqlite::{params, Connection, Result};
use tauri::State;
use std::sync::Mutex;
use rust_decimal::Decimal;
use crate::services::calcul_prix::calculer_totaux_devis;
use crate::services::numerotation::generer_numero_devis;
use crate::services::validation::alerte_passeport;
use crate::services::validation::valider_dates_sejour;

/// Pour chaque devis, indique s'il a au moins un passager avec alerte passeport.
#[tauri::command]
pub fn get_alertes_tous_devis(state: State<DbState>) -> Result<Vec<serde_json::Value>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    // Récupérer tous les devis
    let devis_list = list_devis(state.clone())?;

    let mut result = Vec::new();
    for devis in devis_list {
        // Récupérer les passagers de ce devis
        let mut stmt = conn.prepare(
            "SELECT date_expiration_passeport FROM passagers WHERE devis_id = ?1"
        ).map_err(|e| e.to_string())?;
        let mut rows = stmt.query(&[&devis.id.unwrap()]).map_err(|e| e.to_string())?;

        let mut alerte = false;
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
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

/// Récupère la liste des passagers d'un devis avec leur alerte passeport.
#[tauri::command]
pub fn get_alertes_devis(state: State<DbState>, devis_id: i64) -> Result<Vec<serde_json::Value>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    // 1. Récupérer le devis pour avoir date_retour
    let devis = get_devis_by_id(state.clone(), devis_id)?;

    // 2. Récupérer les passagers
    let mut stmt = conn.prepare(
        "SELECT id, nom_complet, date_expiration_passeport FROM passagers WHERE devis_id = ?1"
    ).map_err(|e| e.to_string())?;
    let mut rows = stmt.query(&[&devis_id]).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let passager_id: i64 = row.get(0)?;
        let nom: String = row.get(1)?;
        let date_expiration: Option<NaiveDate> = row.get(2)?;

        let alerte = match date_expiration {
            Some(exp) => alerte_passeport(exp, devis.date_retour),
            None => false, // pas de passeport renseigné => pas d'alerte (ou on pourrait considérer comme alerte)
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

// --- CRUD Devis ---

#[tauri::command]
pub fn create_devis(
    state: State<DbState>,
    devis_data: DevisCreate,
) -> Result<Devis, String> {
    // Valider les dates
    valider_dates_sejour(devis_data.date_depart, devis_data.date_retour)?;

    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let numero_devis = generer_numero_devis(&conn, devis_data.date_depart)?;

    let mut stmt = conn.prepare(
        "INSERT INTO devis (
            numero_devis, client_id, date_depart, date_retour, type_visa,
            assurance_medicale, devise_achat, taux_sar_dzd, taux_usd_dzd, taux_eur_dzd,
            marge_type, marge_valeur, notes_internes
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)"
    ).map_err(|e| e.to_string())?;


    stmt.execute(params![
        numero_devis,
        devis_data.client_id,
        devis_data.date_depart,
        devis_data.date_retour,
        devis_data.type_visa,
        devis_data.assurance_medicale,
        devis_data.devise_achat,
        devis_data.taux_sar_dzd,
        devis_data.taux_usd_dzd,
        devis_data.taux_eur_dzd,
        devis_data.marge_type,
        devis_data.marge_valeur,
        devis_data.notes_internes,
    ]).map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    get_devis_by_id(state, id)
}

#[tauri::command]
pub fn get_devis_by_id(state: State<DbState>, id: i64) -> Result<Devis, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, numero_devis, client_id, date_creation, date_depart, date_retour,
                type_visa, assurance_medicale, devise_achat, taux_sar_dzd, taux_usd_dzd, taux_eur_dzd,
                marge_type, marge_valeur, cout_net_total, montant_marge, prix_vente_total,
                statut, remise, notes_internes
         FROM devis WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(Devis {
            id: Some(row.get(0)?),
            numero_devis: row.get(1)?,
            client_id: row.get(2)?,
            date_creation: row.get(3)?,
            date_depart: row.get(4)?,
            date_retour: row.get(5)?,
            type_visa: row.get(6)?,
            assurance_medicale: row.get(7)?,
            devise_achat: row.get(8)?,
            taux_sar_dzd: row.get(9)?,
            taux_usd_dzd: row.get(10)?,
            taux_eur_dzd: row.get(11)?,
            marge_type: row.get(12)?,
            marge_valeur: row.get(13)?,
            cout_net_total: row.get(14)?,
            montant_marge: row.get(15)?,
            prix_vente_total: row.get(16)?,
            statut: row.get(17)?,
            remise: row.get(18)?,
            notes_internes: row.get(19)?,
        })
    } else {
        Err("Devis non trouvé".to_string())
    }
}

#[tauri::command]
pub fn list_devis(state: State<DbState>) -> Result<Vec<Devis>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, numero_devis, client_id, date_creation, date_depart, date_retour,
                type_visa, assurance_medicale, devise_achat, taux_sar_dzd, taux_usd_dzd, taux_eur_dzd,
                marge_type, marge_valeur, cout_net_total, montant_marge, prix_vente_total,
                statut, remise, notes_internes
         FROM devis ORDER BY id DESC"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
    let mut devis_list = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
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
            taux_sar_dzd: row.get(9)?,
            taux_usd_dzd: row.get(10)?,
            taux_eur_dzd: row.get(11)?,
            marge_type: row.get(12)?,
            marge_valeur: row.get(13)?,
            cout_net_total: row.get(14)?,
            montant_marge: row.get(15)?,
            prix_vente_total: row.get(16)?,
            statut: row.get(17)?,
            remise: row.get(18)?,
            notes_internes: row.get(19)?,
        });
    }
    Ok(devis_list)
}

#[tauri::command]
pub fn update_devis(
    state: State<DbState>,
    id: i64,
    update_data: DevisUpdate,
) -> Result<Devis, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut sets = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let (Some(depart), Some(retour)) = (update_data.date_depart, update_data.date_retour) {
        valider_dates_sejour(depart, retour)?;
    }
    if let Some(date_retour) = update_data.date_retour {
        sets.push("date_retour = ?");
        params.push(Box::new(date_retour));
    }
    if let Some(type_visa) = update_data.type_visa {
        sets.push("type_visa = ?");
        params.push(Box::new(type_visa));
    }
    if let Some(assurance_medicale) = update_data.assurance_medicale {
        sets.push("assurance_medicale = ?");
        params.push(Box::new(assurance_medicale));
    }
    if let Some(devise_achat) = update_data.devise_achat {
        sets.push("devise_achat = ?");
        params.push(Box::new(devise_achat));
    }
    if let Some(taux_sar_dzd) = update_data.taux_sar_dzd {
        sets.push("taux_sar_dzd = ?");
        params.push(Box::new(taux_sar_dzd));
    }
    if let Some(taux_usd_dzd) = update_data.taux_usd_dzd {
        sets.push("taux_usd_dzd = ?");
        params.push(Box::new(taux_usd_dzd));
    }
    if let Some(taux_eur_dzd) = update_data.taux_eur_dzd {
        sets.push("taux_eur_dzd = ?");
        params.push(Box::new(taux_eur_dzd));
    }
    if let Some(marge_type) = update_data.marge_type {
        sets.push("marge_type = ?");
        params.push(Box::new(marge_type));
    }
    if let Some(marge_valeur) = update_data.marge_valeur {
        sets.push("marge_valeur = ?");
        params.push(Box::new(marge_valeur));
    }
    if let Some(statut) = update_data.statut {
        sets.push("statut = ?");
        params.push(Box::new(statut));
    }
    if let Some(remise) = update_data.remise {
        sets.push("remise = ?");
        params.push(Box::new(remise));
    }
    if let Some(notes_internes) = update_data.notes_internes {
        sets.push("notes_internes = ?");
        params.push(Box::new(notes_internes));
    }

    if sets.is_empty() {
        return Err("Aucune donnée à mettre à jour".to_string());
    }

    sets.push("updated_at = CURRENT_TIMESTAMP");
    let query = format!(
        "UPDATE devis SET {} WHERE id = ?",
        sets.join(", ")
    );

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let current = get_devis_by_id(state.clone(), id)?;
    let depart = update_data.date_depart.unwrap_or(current.date_depart);
    let retour = update_data.date_retour.unwrap_or(current.date_retour);
    valider_dates_sejour(depart, retour)?;
    let mut final_params: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    final_params.push(&id);
    stmt.execute(&final_params).map_err(|e| e.to_string())?;

    get_devis_by_id(state, id)
}

#[tauri::command]
pub fn delete_devis(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM devis WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// --- Commande pour le calcul de la marge (appelée par le frontend) ---

#[tauri::command]
pub fn calculate_totals(
    state: State<DbState>,
    devis_id: i64,
) -> Result<Devis, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    // 1. Récupérer le devis
    let devis = get_devis_by_id(state.clone(), devis_id)?;

    // 2. Récupérer les lignes associées
    // Vols
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, ordre, compagnie, numero_vol, classe, date_vol,
                aeroport_depart, aeroport_arrivee, heure_depart, heure_arrivee,
                prix_adulte, prix_enfant, prix_bebe, devise_prix, remarques
         FROM segments_vol WHERE devis_id = ?1"
    ).map_err(|e| e.to_string())?;
    let mut rows = stmt.query(&[&devis_id]).map_err(|e| e.to_string())?;
    let mut segments_vol = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        segments_vol.push(crate::models::segment_vol::SegmentVol {
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

    // Hébergements
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, ville, nom_hotel, type_chambre, formule_repas, vue,
                date_checkin, date_checkout, nb_nuitees, prix_par_nuit, devise_prix,
                taxes_incluses, remarques
         FROM hebergements WHERE devis_id = ?1"
    ).map_err(|e| e.to_string())?;
    let mut rows = stmt.query(&[&devis_id]).map_err(|e| e.to_string())?;
    let mut hebergements = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        hebergements.push(crate::models::hebergement::Hebergement {
            id: row.get(0)?,
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

    // Transferts
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, type_transfert, trajet, type_vehicule,
                date_transfert, heure_transfert, nombre_vehicules,
                prix_unitaire, devise_prix, remarques
         FROM transferts WHERE devis_id = ?1"
    ).map_err(|e| e.to_string())?;
    let mut rows = stmt.query(&[&devis_id]).map_err(|e| e.to_string())?;
    let mut transferts = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        transferts.push(crate::models::transfert::Transfert {
            id: row.get(0)?,
            devis_id: row.get(1)?,
            type_transfert: row.get(2)?,
            trajet: row.get(3)?,
            type_vehicule: row.get(4)?,
            date_transfert: row.get(5)?,
            heure_transfert: row.get(6)?,
            nombre_vehicules: row.get(7)?,
            prix_unitaire: row.get(8)?,
            devise_prix: row.get(9)?,
            remarques: row.get(10)?,
        });
    }

    // Prestations VIP
    let mut stmt = conn.prepare(
        "SELECT id, devis_id, type_prestation, description,
                prix_unitaire, quantite, devise_prix, remarques
         FROM prestations_vip WHERE devis_id = ?1"
    ).map_err(|e| e.to_string())?;
    let mut rows = stmt.query(&[&devis_id]).map_err(|e| e.to_string())?;
    let mut prestations = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        prestations.push(crate::models::prestation_vip::PrestationVip {
            id: row.get(0)?,
            devis_id: row.get(1)?,
            type_prestation: row.get(2)?,
            description: row.get(3)?,
            prix_unitaire: row.get(4)?,
            quantite: row.get(5)?,
            devise_prix: row.get(6)?,
            remarques: row.get(7)?,
        });
    }

    // 3. Appeler le service de calcul
    let (cout_net, marge, prix_vente) = calculer_totaux_devis(
        &devis,
        &segments_vol,
        &hebergements,
        &transferts,
        &prestations,
    )?;

    // 4. Mettre à jour le devis en base
    conn.execute(
        "UPDATE devis SET cout_net_total = ?1, montant_marge = ?2, prix_vente_total = ?3, updated_at = CURRENT_TIMESTAMP WHERE id = ?4",
        &[&cout_net, &marge, &prix_vente, &devis_id],
    ).map_err(|e| e.to_string())?;

    // 5. Retourner le devis mis à jour
    get_devis_by_id(state, devis_id)
}
