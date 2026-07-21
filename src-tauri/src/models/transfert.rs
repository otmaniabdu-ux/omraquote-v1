use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfert {
    pub id: Option<i64>,
    pub devis_id: i64,
    pub type_transfert: String, // 'obligatoire' ou 'optionnel'
    pub trajet: String,
    pub type_vehicule: String, // 'GMC_Yukon', 'Mercedes_Classe_E', 'Bus_VIP_prive'
    pub date_transfert: Option<NaiveDate>,
    pub heure_transfert: Option<String>,
    pub nombre_vehicules: i32,
    pub prix_unitaire: Decimal,
    pub devise_prix: String,
    pub remarques: Option<String>,
}

/// Struct de création de transfert (exclut id et updated_at)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransfertCreate {
    pub devis_id: i64,
    pub type_transfert: String,
    pub trajet: String,
    pub type_vehicule: String,
    pub date_transfert: Option<NaiveDate>,
    pub heure_transfert: Option<String>,
    pub nombre_vehicules: i32,
    pub prix_unitaire: Decimal,
    pub devise_prix: String,
    pub remarques: Option<String>,
}

/// Struct de mise à jour de transfert (tous les champs optionnels)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransfertUpdate {
    pub type_transfert: Option<String>,
    pub trajet: Option<String>,
    pub type_vehicule: Option<String>,
    pub date_transfert: Option<NaiveDate>,
    pub heure_transfert: Option<String>,
    pub nombre_vehicules: Option<i32>,
    pub prix_unitaire: Option<Decimal>,
    pub devise_prix: Option<String>,
    pub remarques: Option<String>,
}
