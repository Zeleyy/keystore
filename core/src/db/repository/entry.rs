use crate::db::entry::{Entry, NewEntry, UpdateEntry};

pub fn get_all(conn: &rusqlite::Connection, category_id: Option<i64>) -> Result<Vec<Entry>, String> {
    fetch_all!(
        conn,
        "SELECT entry_id, entry_title, entry_url, entry_username, entry_notes, category_id, created_at 
        FROM entries 
        WHERE (?1 IS NULL OR category_id = ?1)
        ORDER BY entry_title ASC",
        [category_id],
        |row| Entry {
            entry_id: row.get(0)?,
            entry_title: row.get(1)?,
            entry_url: row.get(2)?,
            entry_username: row.get(3)?,
            entry_password: String::new(),
            entry_password_nonce: String::new(),
            entry_notes: row.get(4)?,
            category_id: row.get(5)?,
            created_at: row.get(6)?,
        }
    )
}

pub fn get_by_id(conn: &rusqlite::Connection, entry_id: i64) -> Result<Option<Entry>, String> {
    fetch_one!(
        conn,
        "SELECT entry_id, entry_title, entry_url, entry_username, entry_password, entry_password_nonce, entry_notes, category_id, created_at 
        FROM entries 
        WHERE entry_id = ?",
        [entry_id],
        |row| Entry {
            entry_id: row.get(0)?,
            entry_title: row.get(1)?,
            entry_url: row.get(2)?,
            entry_username: row.get(3)?,
            entry_password: row.get(4)?,
            entry_password_nonce: row.get(5)?,
            entry_notes: row.get(6)?,
            category_id: row.get(7)?,
            created_at: row.get(8)?,
        }
    )
}

pub fn create(conn: &rusqlite::Connection, new_entry: &NewEntry) -> Result<i64, String> {
    execute_insert!(
        conn,
        "INSERT INTO entries (entry_title, entry_url, entry_username, entry_password, entry_password_nonce, entry_notes, category_id) 
        VALUES (?, ?, ?, ?, ?, ?, ?)",
        (
            &new_entry.entry_title,
            &new_entry.entry_url,
            &new_entry.entry_username,
            &new_entry.entry_password,
            &new_entry.entry_password_nonce,
            &new_entry.entry_notes,
            &new_entry.category_id,
        )
    )
}

pub fn update(conn: &rusqlite::Connection, update_entry: &UpdateEntry) -> Result<(), String> {
    let mut update_parts = Vec::new();
    let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

    push_field!(update_parts, params, "entry_title = ?", update_entry.entry_title);
    push_field!(update_parts, params, "entry_url = ?", update_entry.entry_url);
    push_field!(update_parts, params, "entry_username = ?", update_entry.entry_username);
    push_field!(update_parts, params, "entry_password = ?", update_entry.entry_password);
    push_field!(update_parts, params, "entry_password_nonce = ?", update_entry.entry_password_nonce);
    push_field!(update_parts, params, "entry_notes = ?", update_entry.entry_notes);
    push_field!(update_parts, params, "category_id = ?", update_entry.category_id);

    if update_parts.is_empty() {
        return Ok(());
    }

    params.push(&update_entry.entry_id);

    let sql = format!("UPDATE entries SET {} WHERE entry_id = ?", update_parts.join(", "));

    execute_modify!(
        conn,
        &sql,
        params.as_slice(),
        "Entry not found"
    )
}

pub fn delete(conn: &rusqlite::Connection, entry_id: i64) -> Result<(), String> {
    execute_modify!(
        conn,
        "DELETE FROM entries WHERE entry_id = ?",
        [entry_id],
        "Entry not found"
    )
}
