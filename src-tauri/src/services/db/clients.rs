use crate::models::client::{Client, ClientCreate, ClientUpdate};
use crate::error::AppResult;
use rusqlite::{params, Connection};

pub fn create(conn: &Connection, client_data: ClientCreate) -> AppResult<Client> {
    let mut stmt = conn.prepare(
        "INSERT INTO clients (code_client, raison_sociale, nom_contact, telephone, email, adresse, pays, type_client, remarques)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
    )?;

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
    ])?;

    let id = conn.last_insert_rowid();
    get_by_id(conn, id)
}

pub fn get_by_id(conn: &Connection, id: i64) -> AppResult<Client> {
    let mut stmt = conn.prepare(
        "SELECT id, code_client, raison_sociale, nom_contact, telephone, email, adresse, pays, type_client, remarques, created_at, updated_at
         FROM clients WHERE id = ?1"
    )?;

    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
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
        Err(crate::error::AppError::NotFound("Client non trouvé".to_string()))
    }
}

pub fn list(conn: &Connection) -> AppResult<Vec<Client>> {
    let mut stmt = conn.prepare(
        "SELECT id, code_client, raison_sociale, nom_contact, telephone, email, adresse, pays, type_client, remarques, created_at, updated_at
         FROM clients ORDER BY id DESC"
    )?;

    let mut rows = stmt.query([])?;
    let mut clients = Vec::new();
    while let Some(row) = rows.next()? {
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

pub fn update(conn: &Connection, id: i64, update_data: ClientUpdate) -> AppResult<Client> {
    let mut fields = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(raison_sociale) = update_data.raison_sociale {
        fields.push("raison_sociale");
        params.push(Box::new(raison_sociale));
    }
    if let Some(nom_contact) = update_data.nom_contact {
        fields.push("nom_contact");
        params.push(Box::new(nom_contact));
    }
    if let Some(telephone) = update_data.telephone {
        fields.push("telephone");
        params.push(Box::new(telephone));
    }
    if let Some(email) = update_data.email {
        fields.push("email");
        params.push(Box::new(email));
    }
    if let Some(adresse) = update_data.adresse {
        fields.push("adresse");
        params.push(Box::new(adresse));
    }
    if let Some(pays) = update_data.pays {
        fields.push("pays");
        params.push(Box::new(pays));
    }
    if let Some(type_client) = update_data.type_client {
        fields.push("type_client");
        params.push(Box::new(type_client));
    }
    if let Some(remarques) = update_data.remarques {
        fields.push("remarques");
        params.push(Box::new(remarques));
    }

    if fields.is_empty() {
        return Err(crate::error::AppError::Validation("Aucune donnée à mettre à jour".to_string()));
    }

    let query = crate::utils::query_builder::build_update_query("clients", &fields, "id");

    let mut stmt = conn.prepare(&query)?;
    let mut final_params: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    final_params.push(&id);
    stmt.execute(&final_params[..])?;

    get_by_id(conn, id)
}

pub fn delete(conn: &Connection, id: i64) -> AppResult<()> {
    conn.execute("DELETE FROM clients WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn generate_code(conn: &Connection) -> AppResult<String> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM clients",
        [],
        |row| row.get(0)
    )?;
    let next_num = count + 1;
    Ok(format!("CLT-{:04}", next_num))
}
