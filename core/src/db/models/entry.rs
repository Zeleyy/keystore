use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub entry_id: i64,
    pub entry_title: String,
    pub entry_url: Option<String>,
    pub entry_username: Option<String>,
    pub entry_password: String,
    pub entry_password_nonce: String,
    pub entry_notes: Option<String>,
    pub category_id: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewEntry {
    pub entry_title: String,
    pub entry_url: Option<String>,
    pub entry_username: Option<String>,
    pub entry_password: String,
    pub entry_password_nonce: String,
    pub entry_notes: Option<String>,
    pub category_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateEntry {
    pub entry_id: i64,
    pub entry_title: Option<String>,
    pub entry_url: Option<Option<String>>,
    pub entry_username: Option<Option<String>>,
    pub entry_password: Option<String>,
    pub entry_password_nonce: Option<String>,
    pub entry_notes: Option<Option<String>>,
    pub category_id: Option<Option<i64>>,
}
