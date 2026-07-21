use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: Option<i64>,
    pub code_client: String,
    pub raison_sociale: Option<String>,
    pub nom_contact: Option<String>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub adresse: Option<String>,
    pub pays: Option<String>,
    pub type_client: String, // 'particulier' ou 'agence'
    pub remarques: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCreate {
    pub code_client: String,
    pub raison_sociale: Option<String>,
    pub nom_contact: Option<String>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub adresse: Option<String>,
    pub pays: Option<String>,
    pub type_client: String,
    pub remarques: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientUpdate {
    pub raison_sociale: Option<String>,
    pub nom_contact: Option<String>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub adresse: Option<String>,
    pub pays: Option<String>,
    pub type_client: Option<String>,
    pub remarques: Option<String>,
}