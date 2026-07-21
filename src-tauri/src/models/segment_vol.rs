use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentVol {
    pub id: Option<i64>,
    pub devis_id: i64,
    pub ordre: i32,
    pub compagnie: String,
    pub numero_vol: Option<String>,
    pub classe: String, // 'economique', 'affaires', 'premiere'
    pub date_vol: NaiveDate,
    pub aeroport_depart: String,
    pub aeroport_arrivee: String,
    pub heure_depart: Option<String>, // HH:MM
    pub heure_arrivee: Option<String>,
    pub prix_adulte: Decimal,
    pub prix_enfant: Decimal,
    pub prix_bebe: Decimal,
    pub devise_prix: String,
    pub remarques: Option<String>,
}