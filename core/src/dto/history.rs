use serde::Serialize;

#[derive(Serialize)]
pub struct PasswordHistoryDto {
    pub password_history_id: i64,
    pub entry_id: i64,
    pub password: String,
    pub created_at: i64,
}