use std::fmt;
use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Erreur de base de données : {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Erreur de validation : {0}")]
    Validation(String),

    #[error("Ressource non trouvée : {0}")]
    NotFound(String),

    #[error("Entrée invalide : {0}")]
    InvalidInput(String),

    #[error("Conflit : {0}")]
    Conflict(String),

    #[error("Erreur interne : {0}")]
    Internal(String),
}

// Implémentation personnalisée de la sérialisation pour que Tauri puisse renvoyer l'erreur en String au frontend
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
