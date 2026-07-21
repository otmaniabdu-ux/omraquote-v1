use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotelCatalogue {
    pub id: Option<i64>,
    pub nom_hotel: String,
    pub ville: String, // 'Makkah' ou 'Medine'
    pub categorie: Option<String>, // '5_etoiles', '4_etoiles', etc.
    pub adresse: Option<String>,
    pub contact: Option<String>,
    pub site_web: Option<String>,
    pub remarques: Option<String>,
    pub actif: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotelCatalogueCreate {
    pub nom_hotel: String,
    pub ville: String,
    pub categorie: Option<String>,
    pub adresse: Option<String>,
    pub contact: Option<String>,
    pub site_web: Option<String>,
    pub remarques: Option<String>,
    pub actif: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotelCatalogueUpdate {
    pub nom_hotel: Option<String>,
    pub ville: Option<String>,
    pub categorie: Option<String>,
    pub adresse: Option<String>,
    pub contact: Option<String>,
    pub site_web: Option<String>,
    pub remarques: Option<String>,
    pub actif: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompagnieCatalogue {
    pub id: Option<i64>,
    pub code_iata: Option<String>,
    pub nom_compagnie: String,
    pub pays: Option<String>,
    pub site_web: Option<String>,
    pub actif: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompagnieCatalogueCreate {
    pub code_iata: Option<String>,
    pub nom_compagnie: String,
    pub pays: Option<String>,
    pub site_web: Option<String>,
    pub actif: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompagnieCatalogueUpdate {
    pub code_iata: Option<String>,
    pub nom_compagnie: Option<String>,
    pub pays: Option<String>,
    pub site_web: Option<String>,
    pub actif: Option<bool>,
}