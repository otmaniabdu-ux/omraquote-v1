use chrono::{NaiveDate, Datelike};
use rust_decimal::Decimal;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MargeParMois {
    pub mois: String, // "YYYY-MM"
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

/// Récupère les statistiques de marges pour une période donnée.
pub fn get_statistiques_marges(
    conn: &Connection,
    date_debut: NaiveDate,
    date_fin: NaiveDate,
) -> Result<StatistiquesMarges, String> {
    // 1. Récupérer tous les devis finalisés ou envoyés dans la période
    let mut stmt = conn.prepare(
        "SELECT id, numero_devis, cout_net_total, montant_marge, prix_vente_total, marge_type, marge_valeur
         FROM devis
         WHERE date_creation BETWEEN ?1 AND ?2
         AND statut IN ('finalise', 'envoye', 'accepte')
         AND cout_net_total IS NOT NULL
         AND montant_marge IS NOT NULL
         AND prix_vente_total IS NOT NULL"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(&[&date_debut, &date_fin]).map_err(|e| e.to_string())?;

    let mut total_cout_net = Decimal::ZERO;
    let mut total_marge = Decimal::ZERO;
    let mut total_prix_vente = Decimal::ZERO;
    let mut nb_devis = 0;
    let mut meilleur_devis: Option<MeilleurDevis> = None;
    let mut par_mois_map: HashMap<String, (i64, Decimal, Decimal, Decimal)> = HashMap::new();

    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let _id: i64 = row.get(0)?;
        let numero_devis: String = row.get(1)?;
        let cout_net: Decimal = row.get(2)?;
        let marge: Decimal = row.get(3)?;
        let prix_vente: Decimal = row.get(4)?;
        let marge_type: String = row.get(5)?;
        let marge_valeur: Decimal = row.get(6)?;

        // Calcul du pourcentage de marge
        let marge_pourcentage = if marge_type == "pourcentage" {
            marge_valeur
        } else {
            if cout_net != Decimal::ZERO {
                (marge / cout_net) * Decimal::from(100)
            } else {
                Decimal::ZERO
            }
        };

        // Mettre à jour les totaux
        total_cout_net += cout_net;
        total_marge += marge;
        total_prix_vente += prix_vente;
        nb_devis += 1;

        // Meilleur devis (par marge en montant)
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

        // Aggrégation par mois (on récupère la date de création du devis)
        // On a besoin de la date_creation, donc on refait une requête ou on la passe en paramètre
        // Pour simplifier, on va faire une deuxième requête groupée par mois
    }

    // 2. Requête groupée par mois pour les statistiques mensuelles
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
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(&[&date_debut, &date_fin]).map_err(|e| e.to_string())?;
    let mut par_mois = Vec::new();

    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let mois: String = row.get(0)?;
        let nb: i64 = row.get(1)?;
        let cout: Decimal = row.get(2)?;
        let marge: Decimal = row.get(3)?;
        let prix: Decimal = row.get(4)?;

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

    // 3. Statistiques globales
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

/// Récupère les marges par client (top clients)
pub fn get_marges_par_client(
    conn: &Connection,
    date_debut: NaiveDate,
    date_fin: NaiveDate,
    limit: Option<i64>,
) -> Result<Vec<(String, Decimal, Decimal, i64)>, String> {
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

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let mut rows = stmt.query(&[&date_debut, &date_fin]).map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let nom: String = row.get(0)?;
        let code: String = row.get(1)?;
        let marge: Decimal = row.get(2)?;
        let nb: i64 = row.get(3)?;
        result.push((format!("{} ({})", nom, code), marge, Decimal::ZERO, nb));
    }

    Ok(result)
}