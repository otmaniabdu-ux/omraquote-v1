pub mod connection;
pub mod migrations;

use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

// On stocke la connexion unique dans l'état Tauri
pub struct DbState(pub Mutex<Connection>);