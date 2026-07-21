use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passager {
    pub id: Option<i64>,
    pub devis_id: i64,
    pub categorie: String, // 'adulte', 'enfant_avec_lit', 'enfant_sans_lit', 'bebe'
    pub nom_complet: String,
    pub date_naissance: NaiveDate,
    pub nationalite: Option<String>,
    pub numero_passeport: Option<String>,
    pub date_expiration_passeport: Option<NaiveDate>,
    pub lieu_delivrance: Option<String>,
    pub remarques: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassagerCreate {
    pub devis_id: i64,
    pub categorie: String,
    pub nom_complet: String,
    pub date_naissance: NaiveDate,
    pub nationalite: Option<String>,
    pub numero_passeport: Option<String>,
    pub date_expiration_passeport: Option<NaiveDate>,
    pub lieu_delivrance: Option<String>,
    pub remarques: Option<String>,
}