use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub category_id: i64,
    pub category_name: String,
    pub category_icon: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewCategory {
    pub category_name: String,
    pub category_icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateCategory {
    pub category_id: i64,
    pub category_name: Option<String>,
    pub category_icon: Option<Option<String>>,
}
