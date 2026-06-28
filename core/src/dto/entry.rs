use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewEntryDto {
    pub entry_title: String,
    pub entry_url: Option<String>,
    pub entry_username: Option<String>,
    pub entry_password: String,
    pub entry_notes: Option<String>,
    pub category_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEntryDto {
    pub entry_id: i64,
    pub entry_title: Option<String>,
    pub entry_url: Option<Option<String>>,
    pub entry_username: Option<Option<String>>,
    pub entry_password: Option<String>,
    pub entry_notes: Option<Option<String>>,
    pub category_id: Option<Option<i64>>,
}