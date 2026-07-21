use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestationVip {
    pub id: Option<i64>,
    pub devis_id: i64,
    pub type_prestation: String, // 'ziyarat', 'lounge', 'fast_track', 'bagages', 'zamzam', 'autre'
    pub description: String,
    pub prix_unitaire: Decimal,
    pub quantite: i32,
    pub devise_prix: String,
    pub remarques: Option<String>,
}

/// Struct de création de prestation VIP (exclut id et updated_at)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestationVipCreate {
    pub devis_id: i64,
    pub type_prestation: String,
    pub description: String,
    pub prix_unitaire: Decimal,
    pub quantite: i32,
    pub devise_prix: String,
    pub remarques: Option<String>,
}

/// Struct de mise à jour de prestation VIP (tous les champs optionnels)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestationVipUpdate {
    pub type_prestation: Option<String>,
    pub description: Option<String>,
    pub prix_unitaire: Option<Decimal>,
    pub quantite: Option<i32>,
    pub devise_prix: Option<String>,
    pub remarques: Option<String>,
}
