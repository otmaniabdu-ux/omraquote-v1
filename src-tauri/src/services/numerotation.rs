use rusqlite::{params, Connection};
use chrono::{NaiveDate, Datelike};
use crate::error::{AppResult, AppError};

/// Génère le prochain numéro de devis pour un mois donné.
/// Met à jour le compteur dans la table compteurs_numerotation.
pub fn generer_numero_devis(conn: &Connection, date: NaiveDate) -> AppResult<String> {
    let year = date.year();
    let month = date.month();
    let cle = format!("DEVIS-{:04}-{:02}", year, month);

    // Transaction pour éviter les conflits
    let tx = conn.unchecked_transaction()?;

    // Lire le dernier numéro
    let last_num = match tx.query_row(
        "SELECT dernier_numero FROM compteurs_numerotation WHERE cle = ?1",
        &[&cle],
        |row| row.get(0),
    ) {
        Ok(val) => val,
        Err(_) => {
            tx.execute(
                "INSERT INTO compteurs_numerotation (cle, dernier_numero) VALUES (?1, 0)",
                &[&cle],
            )?;
            0
        }
    };

    let new_num = last_num + 1;
    tx.execute(
        "UPDATE compteurs_numerotation SET dernier_numero = ?1, updated_at = CURRENT_TIMESTAMP WHERE cle = ?2",
        params![new_num, cle],
    ).map_err(|e| AppError::Database(e))?;

    tx.commit()?;

    Ok(format!("{}-{:03}", cle, new_num))
}

#[cfg(test)]
mod tests {
    // Les tests nécessitent une base en mémoire, on les fera plus tard
}
