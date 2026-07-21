use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametresAgence {
    pub id: i64,
    pub nom_agence_fr: String,
    pub nom_agence_ar: String,
    pub adresse: Option<String>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub site_web: Option<String>,
    pub numero_agrement: Option<String>,
    pub logo_path: Option<String>,
    pub devise_defaut: String,
    pub taux_tva: Option<Decimal>,
}