use chrono::{NaiveDate, Datelike};
use rust_decimal::Decimal;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::{AppResult, AppError};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MargeParMois {
    pub mois: String,
    pub nb_devis: i64,
    pub cout_net_total: Decimal,
    pub marge_total: Decimal,
    pub prix_vente_total: Decimal,
    pub marge_moyenne_pourcentage: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatistiquesMarges {
    pub total_devis: i64,
    pub total_cout_net: Decimal,
    pub total_marge: Decimal,
    pub total_prix_vente: Decimal,
    pub marge_moyenne_pourcentage: Decimal,
    pub meilleur_devis: Option<MeilleurDevis>,
    pub par_mois: Vec<MargeParMois>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeilleurDevis {
    pub numero_devis: String,
    pub marge: Decimal,
    pub marge_pourcentage: Decimal,
}

pub fn get_statistiques_marges(
    conn: &Connection,
    date_debut: NaiveDate,
    date_fin: NaiveDate,
) -> AppResult<StatistiquesMarges> {
    let mut stmt = conn.prepare(
        "SELECT id, numero_devis, cout_net_total, montant_marge, prix_vente_total, marge_type, marge_valeur
         FROM devis
         WHERE date_creation BETWEEN ?1 AND ?2
         AND statut IN ('finalise', 'envoye', 'accepte')
         AND cout_net_total IS NOT NULL
         AND montant_marge IS NOT NULL
         AND prix_vente_total IS NOT NULL"
    )?;

    let mut rows = stmt.query(&[&date_debut, &date_fin])?;

    let mut total_cout_net = Decimal::ZERO;
    let mut total_marge = Decimal::ZERO;
    let mut total_prix_vente = Decimal::ZERO;
    let mut nb_devis = 0;
    let mut meilleur_devis: Option<MeilleurDevis> = None;

    while let Some(row) = rows.next()? {
        let _id: i64 = row.get(0)?;
        let numero_devis: String = row.get(1)?;
        
        let cout_net_str: String = row.get(2)?;
        let marge_str: String = row.get(3)?;
        let prix_vente_str: String = row.get(4)?;
        let marge_type: String = row.get(5)?;
        let marge_valeur_str: String = row.get(6)?;

        let cout_net = Decimal::from_str(&cout_net_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing cout_net : {}", e)))?;
        let marge = Decimal::from_str(&marge_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing marge : {}", e)))?;
        let prix_vente = Decimal::from_str(&prix_vente_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing prix_vente : {}", e)))?;
        let marge_valeur = Decimal::from_str(&marge_valeur_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing marge_valeur : {}", e)))?;

        let marge_pourcentage = if marge_type == "pourcentage" {
            marge_valeur
        } else {
            if cout_net != Decimal::ZERO {
                (marge / cout_net) * Decimal::from(100)
            } else {
                Decimal::ZERO
            }
        };

        total_cout_net += cout_net;
        total_marge += marge;
        total_prix_vente += prix_vente;
        nb_devis += 1;

        if let Some(ref best) = meilleur_devis {
            if marge > best.marge {
                meilleur_devis = Some(MeilleurDevis {
                    numero_devis,
                    marge,
                    marge_pourcentage,
                });
            }
        } else {
            meilleur_devis = Some(MeilleurDevis {
                numero_devis,
                marge,
                marge_pourcentage,
            });
        }
    }

    let mut stmt = conn.prepare(
        "SELECT strftime('%Y-%m', date_creation) as mois,
                 COUNT(*) as nb,
                 SUM(cout_net_total) as cout,
                 SUM(montant_marge) as marge,
                 SUM(prix_vente_total) as prix
          FROM devis
          WHERE date_creation BETWEEN ?1 AND ?2
          AND statut IN ('finalise', 'envoye', 'accepte')
          AND cout_net_total IS NOT NULL
          AND montant_marge IS NOT NULL
          AND prix_vente_total IS NOT NULL
          GROUP BY mois
          ORDER BY mois"
    )?;

    let mut rows = stmt.query(&[&date_debut, &date_fin])?;
    let mut par_mois = Vec::new();

    while let Some(row) = rows.next()? {
        let mois: String = row.get(0)?;
        let nb: i64 = row.get(1)?;
        
        let cout_str: String = row.get(2)?;
        let marge_str: String = row.get(3)?;
        let prix_str: String = row.get(4)?;

        let cout = Decimal::from_str(&cout_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing cout mensuel : {}", e)))?;
        let marge = Decimal::from_str(&marge_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing marge mensuelle : {}", e)))?;
        let prix = Decimal::from_str(&prix_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing prix mensuel : {}", e)))?;

        let marge_pourcentage = if cout != Decimal::ZERO {
            (marge / cout) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        par_mois.push(MargeParMois {
            mois,
            nb_devis: nb,
            cout_net_total: cout,
            marge_total: marge,
            prix_vente_total: prix,
            marge_moyenne_pourcentage: marge_pourcentage,
        });
    }

    let marge_moyenne_pourcentage = if total_cout_net != Decimal::ZERO {
        (total_marge / total_cout_net) * Decimal::from(100)
    } else {
        Decimal::ZERO
    };

    Ok(StatistiquesMarges {
        total_devis: nb_devis,
        total_cout_net,
        total_marge,
        total_prix_vente,
        marge_moyenne_pourcentage,
        meilleur_devis,
        par_mois,
    })
}

pub fn get_marges_par_client(
    conn: &Connection,
    date_debut: NaiveDate,
    date_fin: NaiveDate,
    limit: Option<i64>,
) -> AppResult<Vec<(String, Decimal, Decimal, i64)>> {
    let limit_clause = match limit {
        Some(l) => format!("LIMIT {}", l),
        None => "".to_string(),
    };

    let query = format!(
        "SELECT c.nom_contact, c.code_client,
                SUM(d.montant_marge) as total_marge,
                COUNT(d.id) as nb_devis
         FROM devis d
         JOIN clients c ON d.client_id = c.id
         WHERE d.date_creation BETWEEN ?1 AND ?2
         AND d.statut IN ('finalise', 'envoye', 'accepte')
         AND d.montant_marge IS NOT NULL
         GROUP BY c.id
         ORDER BY total_marge DESC
         {}",
        limit_clause
    );

    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(&[&date_debut, &date_fin])?;
    let mut result = Vec::new();

    while let Some(row) = rows.next()? {
        let nom: String = row.get(0)?;
        let code: String = row.get(1)?;
        let marge_str: String = row.get(2)?;
        let nb: i64 = row.get(3)?;

        let marge = Decimal::from_str(&marge_str)
            .map_err(|e| AppError::Internal(format!("Erreur parsing marge client : {}", e)))?;

        result.push((format!("{} ({})", nom, code), marge, Decimal::ZERO, nb));
    }

    Ok(result)
}
