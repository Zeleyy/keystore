use crate::db::history::{PasswordHistory, NewPasswordHistory};

pub fn get_all(conn: &rusqlite::Connection, entry_id: i64) -> Result<Vec<PasswordHistory>, String> {
    fetch_all!(
        conn,
        "SELECT password_history_id, entry_id, password, password_nonce, created_at FROM password_history WHERE entry_id = ?",
        [entry_id],
        |row| PasswordHistory {
            password_history_id: row.get(0)?,
            entry_id: row.get(1)?,
            password: row.get(2)?,
            password_nonce: row.get(3)?,
            created_at: row.get(4)?,
        }
    )
}

pub fn create(conn: &rusqlite::Connection, new_history: &NewPasswordHistory) -> Result<i64, String> {
    execute_insert!(
        conn,
        "INSERT INTO password_history (entry_id, password, password_nonce) VALUES (?, ?, ?)",
        (&new_history.entry_id, &new_history.password, &new_history.password_nonce)
    )
}

pub fn delete(conn: &rusqlite::Connection, password_history_id: i64) -> Result<(), String> {
    execute_modify!(
        conn,
        "DELETE FROM password_history WHERE password_history_id = ?",
        [password_history_id],
        "Password history not found"
    )
}

pub fn clear(conn: &rusqlite::Connection, entry_id: i64) -> Result<(), String> {
    conn.execute(
        "DELETE FROM password_history WHERE entry_id = ?",
        [entry_id],
    )
    .map_err(|e| format!("Clear history error: {}", e))?;

    Ok(())
}
