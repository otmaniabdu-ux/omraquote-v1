/// Construit dynamiquement une requête SQL UPDATE sécurisée.
///
/// Retourne une chaîne du type :
/// `UPDATE {table} SET {field1} = ?, {field2} = ?, updated_at = CURRENT_TIMESTAMP WHERE {id_field} = ?`
pub fn build_update_query(table: &str, fields: &[&str], id_field: &str) -> String {
    let set_clause: Vec<String> = fields.iter().map(|f| format!("{} = ?", f)).collect();
    format!(
        "UPDATE {} SET {}, updated_at = CURRENT_TIMESTAMP WHERE {} = ?",
        table,
        set_clause.join(", "),
        id_field
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_update_query() {
        let fields = vec!["nom", "email", "age"];
        let sql = build_update_query("clients", &fields, "id");
        assert_eq!(
            sql,
            "UPDATE clients SET nom = ?, email = ?, age = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
        );
    }
}
