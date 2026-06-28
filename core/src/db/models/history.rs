use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordHistory {
    pub password_history_id: i64,
    pub entry_id: i64,
    pub password: String,
    pub password_nonce: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewPasswordHistory {
    pub entry_id: i64,
    pub password: String,
    pub password_nonce: Option<String>,
}
