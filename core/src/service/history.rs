use crate::{constants::NONCE_LEN, db::{DbConn, history}, dto::PasswordHistoryDto, service::CryptoService};

pub struct HistoryService {
    crypto: CryptoService,
}

impl HistoryService {
    pub fn new(crypto: CryptoService) -> Self {
        Self { crypto }
    }

    pub fn get_all(&self, conn: &DbConn, entry_id: i64) -> Result<Vec<PasswordHistoryDto>, String> {
        let encrypted_histories = history::get_all(conn, entry_id)?;
        let mut decrypted_dtos = Vec::new();

        for hist in encrypted_histories {
            let nonce_bytes: [u8; NONCE_LEN] = hex::decode(&hist.password_nonce)
                .map_err(|e| format!("Invalid history nonce hex format: {}", e))?
                .try_into()
                .map_err(|_| "Invalid history nonce length")?;

            let decrypted_password = self.crypto.decrypt(&hist.password, &nonce_bytes)?;

            decrypted_dtos.push(PasswordHistoryDto {
                password_history_id: hist.password_history_id,
                entry_id: hist.entry_id,
                password: decrypted_password,
                created_at: hist.created_at,
            });
        }

        Ok(decrypted_dtos)
    }

    pub fn delete(conn: &DbConn, password_history_id: i64) -> Result<(), String> {
        history::delete(conn, password_history_id)
    }

    pub fn clear(conn: &DbConn, entry_id: i64) -> Result<(), String> {
        history::clear(conn, entry_id)
    }
}