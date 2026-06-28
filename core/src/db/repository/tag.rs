use crate::db::tag::{Tag, NewTag, EntryTag};

pub fn get_all(conn: &rusqlite::Connection) -> Result<Vec<Tag>, String> {
    fetch_all!(
        conn, 
        "SELECT tag_id, tag_name FROM tags ORDER BY tag_name ASC", 
        (),
        |row| Tag {
            tag_id: row.get(0)?,
            tag_name: row.get(1)?,
        }
    )
}

pub fn create(conn: &rusqlite::Connection, new_tag: &NewTag) -> Result<i64, String> {
    execute_insert!(
        conn,
        "INSERT INTO tags (tag_name) VALUES (?)",
        [&new_tag.tag_name]
    )
}

pub fn delete(conn: &rusqlite::Connection, tag_id: i64) -> Result<(), String> {
    execute_modify!(
        conn,
        "DELETE FROM tags WHERE tag_id = ?",
        [tag_id],
        "Tag not found"
    )
}

pub fn add_to_entry(conn: &rusqlite::Connection, entry_tag: &EntryTag) -> Result<i64, String> {
    execute_insert!(
        conn,
        "INSERT INTO entry_tags (entry_id, tag_id) VALUES (?, ?)",
        (&entry_tag.entry_id, &entry_tag.tag_id)
    )
}

pub fn remove_from_entry(conn: &rusqlite::Connection, entry_tag: &EntryTag) -> Result<(), String> {
    execute_modify!(
        conn,
        "DELETE FROM entry_tags WHERE entry_id = ? AND tag_id = ?",
        (&entry_tag.entry_id, &entry_tag.tag_id),
        "Relation between entry and tag not found"
    )
}
