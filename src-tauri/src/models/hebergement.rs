use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hebergement {
    pub id: Option<i64>,
    pub devis_id: i64,
    pub ville: String, // 'Makkah', 'Medine'
    pub nom_hotel: String,
    pub type_chambre: String, // 'single', 'double', 'triple', 'quadruple'
    pub formule_repas: Option<String>,
    pub vue: Option<String>, // 'Kaaba', 'Haram', 'City'
    pub date_checkin: NaiveDate,
    pub date_checkout: NaiveDate,
    pub nb_nuitees: Option<i64>, // calculé automatiquement
    pub prix_par_nuit: Decimal,
    pub devise_prix: String,
    pub taxes_incluses: bool,
    pub remarques: Option<String>,
}

/// Struct de création d'hébergement (exclut id et updated_at)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HebergementCreate {
    pub devis_id: i64,
    pub ville: String,
    pub nom_hotel: String,
    pub type_chambre: String,
    pub formule_repas: Option<String>,
    pub vue: Option<String>,
    pub date_checkin: NaiveDate,
    pub date_checkout: NaiveDate,
    pub prix_par_nuit: Decimal,
    pub devise_prix: String,
    pub taxes_incluses: bool,
    pub remarques: Option<String>,
}
