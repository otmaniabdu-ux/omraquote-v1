use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Devis {
    pub id: Option<i64>,
    pub numero_devis: String,
    pub client_id: i64,
    pub date_creation: NaiveDateTime,
    pub date_depart: NaiveDate,
    pub date_retour: NaiveDate,
    pub type_visa: String, // 'omra_standard', 'touristique', 'hadj'
    pub assurance_medicale: bool,
    pub devise_achat: String,
    pub taux_sar_dzd: Decimal,
    pub taux_usd_dzd: Decimal,
    pub taux_eur_dzd: Decimal,
    pub marge_type: String, // 'pourcentage' ou 'montant_fixe'
    pub marge_valeur: Decimal,
    pub cout_net_total: Option<Decimal>,
    pub montant_marge: Option<Decimal>,
    pub prix_vente_total: Option<Decimal>,
    pub statut: String, // 'brouillon', 'finalise', 'envoye', 'accepte', 'perdu'
    pub remise: Option<Decimal>,
    pub notes_internes: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevisCreate {
    pub client_id: i64,
    pub date_depart: NaiveDate,
    pub date_retour: NaiveDate,
    pub type_visa: String,
    pub assurance_medicale: bool,
    pub devise_achat: String,
    pub taux_sar_dzd: Decimal,
    pub taux_usd_dzd: Decimal,
    pub taux_eur_dzd: Decimal,
    pub marge_type: String,
    pub marge_valeur: Decimal,
    pub notes_internes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevisUpdate {
    pub date_depart: Option<NaiveDate>,
    pub date_retour: Option<NaiveDate>,
    pub type_visa: Option<String>,
    pub assurance_medicale: Option<bool>,
    pub devise_achat: Option<String>,
    pub taux_sar_dzd: Option<Decimal>,
    pub taux_usd_dzd: Option<Decimal>,
    pub taux_eur_dzd: Option<Decimal>,
    pub marge_type: Option<String>,
    pub marge_valeur: Option<Decimal>,
    pub statut: Option<String>,
    pub remise: Option<Decimal>,
    pub notes_internes: Option<String>,
}