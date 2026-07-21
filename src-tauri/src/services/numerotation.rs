use rusqlite::Connection;
use chrono::NaiveDate;

/// Génère le prochain numéro de devis pour un mois donné.
/// Met à jour le compteur dans la table compteurs_numerotation.
pub fn generer_numero_devis(conn: &Connection, date: NaiveDate) -> Result<String, String> {
    let year = date.year();
    let month = date.month();
    let cle = format!("DEVIS-{:04}-{:02}", year, month);

    // Transaction pour éviter les conflits
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Lire le dernier numéro
    let last_num: i64 = tx.query_row(
        "SELECT dernier_numero FROM compteurs_numerotation WHERE cle = ?1",
        &[&cle],
        |row| row.get(0),
    ).or_else(|_| {
        // Si la clé n'existe pas, on l'insère avec 0
        tx.execute(
            "INSERT INTO compteurs_numerotation (cle, dernier_numero) VALUES (?1, 0)",
            &[&cle],
        ).map_err(|e| e.to_string())?;
        Ok(0)
    })?;

    let new_num = last_num + 1;
    tx.execute(
        "UPDATE compteurs_numerotation SET dernier_numero = ?1, updated_at = CURRENT_TIMESTAMP WHERE cle = ?2",
        &[&new_num, &cle],
    ).map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok(format!("{}-{:03}", cle, new_num))
}

#[cfg(test)]
mod tests {
    // Les tests nécessitent une base en mémoire, on les fera plus tard
}