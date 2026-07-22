use rusqlite::{Connection, Result};
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;
use std::fs;

/// Obtient le chemin du fichier SQLite (dans le répertoire de données de l'application)
pub fn get_db_path(app_handle: &AppHandle) -> PathBuf {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Impossible d'obtenir le répertoire de données");
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).expect("Impossible de créer le répertoire de données");
    }
    app_dir.join("omravip.db")
}

/// Établit la connexion à la base de données, crée le fichier si absent
pub fn init_db(app_handle: &AppHandle) -> Result<Connection> {
    let db_path = get_db_path(app_handle);
    let conn = Connection::open(&db_path)?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    Ok(conn)
}

/// Exécute les migrations (création des tables si elles n'existent pas)
pub fn run_migrations(conn: &Connection) -> Result<()> {
    // On utilise le schéma SQL fourni dans database/schema.sql
    // Pour simplifier, on l'exécute directement via include_str!
    let schema = include_str!("../../../database/schema.sql");
    conn.execute_batch(schema)?;
    Ok(())
}