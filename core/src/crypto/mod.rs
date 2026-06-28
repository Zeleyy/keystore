use serde::{Deserialize, Serialize};

use crate::constants::DB_KEY_LEN;

pub mod cipher;
pub mod kdf;
pub mod keyring;
mod manager;

pub use manager::{change_master_password, get_or_create_database_key};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct KeyringConfig {
    pub encrypted: bool,
    pub database_key_hex: String,
    pub argon_salt_hex: Option<String>,
    pub master_iv_hex: Option<String>,
}

impl KeyringConfig {
    fn create(db_key: &[u8; DB_KEY_LEN], master_password: Option<&str>) -> Result<Self, String> {
        if let Some(password) = master_password {
            let salt = kdf::generate_salt();
            let kek = kdf::derive_key(password, &salt)?;
            let (encrypted_bytes, nonce) = cipher::encrypt_database_key(db_key, &kek)?;

            Ok(Self {
                encrypted: true,
                database_key_hex: hex::encode(encrypted_bytes),
                argon_salt_hex: Some(hex::encode(salt)),
                master_iv_hex: Some(hex::encode(nonce)),
            })
        } else {
            Ok(Self {
                encrypted: false,
                database_key_hex: hex::encode(db_path_key_bytes(db_key)),
                argon_salt_hex: None,
                master_iv_hex: None,
            })
        }
    }
}

fn db_path_key_bytes(key: &[u8; DB_KEY_LEN]) -> &[u8] { key }
