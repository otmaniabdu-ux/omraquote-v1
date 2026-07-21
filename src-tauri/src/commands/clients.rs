use crate::db::DbState;
use crate::models::client::{Client, ClientCreate, ClientUpdate};
use chrono::NaiveDate;
use rusqlite::{params, Connection, Result};
use tauri::State;
use std::sync::Mutex;

// --- CRUD Client ---

#[tauri::command]
pub fn create_client(
    state: State<DbState>,
    client_data: ClientCreate,
) -> Result<Client, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "INSERT INTO clients (code_client, raison_sociale, nom_contact, telephone, email, adresse, pays, type_client, remarques)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
    ).map_err(|e| e.to_string())?;

    stmt.execute(params![
        client_data.code_client,
        client_data.raison_sociale,
        client_data.nom_contact,
        client_data.telephone,
        client_data.email,
        client_data.adresse,
        client_data.pays,
        client_data.type_client,
        client_data.remarques,
    ]).map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    get_client_by_id(state, id)
}

#[tauri::command]
pub fn get_client_by_id(state: State<DbState>, id: i64) -> Result<Client, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, code_client, raison_sociale, nom_contact, telephone, email, adresse, pays, type_client, remarques, created_at, updated_at
         FROM clients WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(Client {
            id: Some(row.get(0)?),
            code_client: row.get(1)?,
            raison_sociale: row.get(2)?,
            nom_contact: row.get(3)?,
            telephone: row.get(4)?,
            email: row.get(5)?,
            adresse: row.get(6)?,
            pays: row.get(7)?,
            type_client: row.get(8)?,
            remarques: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    } else {
        Err("Client non trouvé".to_string())
    }
}

#[tauri::command]
pub fn list_clients(state: State<DbState>) -> Result<Vec<Client>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, code_client, raison_sociale, nom_contact, telephone, email, adresse, pays, type_client, remarques, created_at, updated_at
         FROM clients ORDER BY id DESC"
    ).map_err(|e| e.to_string())?;

    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
    let mut clients = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        clients.push(Client {
            id: Some(row.get(0)?),
            code_client: row.get(1)?,
            raison_sociale: row.get(2)?,
            nom_contact: row.get(3)?,
            telephone: row.get(4)?,
            email: row.get(5)?,
            adresse: row.get(6)?,
            pays: row.get(7)?,
            type_client: row.get(8)?,
            remarques: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        });
    }
    Ok(clients)
}

#[tauri::command]
pub fn update_client(
    state: State<DbState>,
    id: i64,
    update_data: ClientUpdate,
) -> Result<Client, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut sets = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(raison_sociale) = update_data.raison_sociale {
        sets.push("raison_sociale = ?");
        params.push(Box::new(raison_sociale));
    }
    if let Some(nom_contact) = update_data.nom_contact {
        sets.push("nom_contact = ?");
        params.push(Box::new(nom_contact));
    }
    if let Some(telephone) = update_data.telephone {
        sets.push("telephone = ?");
        params.push(Box::new(telephone));
    }
    if let Some(email) = update_data.email {
        sets.push("email = ?");
        params.push(Box::new(email));
    }
    if let Some(adresse) = update_data.adresse {
        sets.push("adresse = ?");
        params.push(Box::new(adresse));
    }
    if let Some(pays) = update_data.pays {
        sets.push("pays = ?");
        params.push(Box::new(pays));
    }
    if let Some(type_client) = update_data.type_client {
        sets.push("type_client = ?");
        params.push(Box::new(type_client));
    }
    if let Some(remarques) = update_data.remarques {
        sets.push("remarques = ?");
        params.push(Box::new(remarques));
    }

    if sets.is_empty() {
        return Err("Aucune donnée à mettre à jour".to_string());
    }

    sets.push("updated_at = CURRENT_TIMESTAMP");
    let query = format!(
        "UPDATE clients SET {} WHERE id = ?",
        sets.join(", ")
    );

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let mut final_params: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    final_params.push(&id);
    stmt.execute(&final_params).map_err(|e| e.to_string())?;

    get_client_by_id(state, id)
}

#[tauri::command]
pub fn delete_client(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM clients WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// --- Générateur de code client ---

#[tauri::command]
pub fn generate_client_code(state: State<DbState>) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM clients",
        [],
        |row| row.get(0)
    ).map_err(|e| e.to_string())?;
    let next_num = count + 1;
    Ok(format!("CLT-{:04}", next_num))
}