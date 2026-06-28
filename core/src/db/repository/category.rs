use crate::db::category::{Category, NewCategory, UpdateCategory};

pub fn get_all(conn: &rusqlite::Connection) -> Result<Vec<Category>, String> {
    fetch_all!(
        conn,
        "SELECT category_id, category_name, category_icon, created_at FROM categories ORDER BY category_name ASC",
        (),
        |row| Category {
            category_id: row.get(0)?,
            category_name: row.get(1)?,
            category_icon: row.get(2)?,
            created_at: row.get(3)?,
        }
    )
}

pub fn get_by_id(conn: &rusqlite::Connection, category_id: i64) -> Result<Option<Category>, String> {
    fetch_one!(
        conn,
        "SELECT category_id, category_name, category_icon, created_at FROM categories WHERE category_id = ?",
        [category_id],
        |row| Category {
            category_id: row.get(0)?,
            category_name: row.get(1)?,
            category_icon: row.get(2)?,
            created_at: row.get(3)?,
        }
    )
}

pub fn create(conn: &rusqlite::Connection, new_category: &NewCategory) -> Result<i64, String> {
    execute_insert!(
        conn,
        "INSERT INTO categories (category_name, category_icon) VALUES (?, ?)",
        (&new_category.category_name, &new_category.category_icon)
    )
}

pub fn update(conn: &rusqlite::Connection, update_category: &UpdateCategory) -> Result<(), String> {
    let mut update_parts = Vec::new();
    let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

    push_field!(update_parts, params, "category_name = ?", update_category.category_name);
    push_field!(update_parts, params, "category_icon = ?", update_category.category_icon);

    if update_parts.is_empty() {
        return Ok(());
    }

    params.push(&update_category.category_id);

    let sql = format!("UPDATE categories SET {} WHERE category_id = ?", update_parts.join(", "));

    execute_modify!(
        conn,
        &sql,
        params.as_slice(),
        "Category not found"
    )
}

pub fn delete(conn: &rusqlite::Connection, category_id: i64) -> Result<(), String> {
    execute_modify!(
        conn,
        "DELETE FROM categories WHERE category_id = ?",
        [category_id],
        "Category not found"
    )
}
