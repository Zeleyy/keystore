use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub tag_id: i64,
    pub tag_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewTag {
    pub tag_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntryTag {
    pub entry_id: i64,
    pub tag_id: i64,
}