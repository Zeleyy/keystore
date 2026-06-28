use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use rand::Rng;

use crate::constants::{DB_KEY_LEN, NONCE_LEN};

pub fn generate_random_key() -> [u8; DB_KEY_LEN] {
    let mut key = [0u8; DB_KEY_LEN];
    rand::rng().fill_bytes(&mut key);
    key
}

pub fn encrypt_database_key(db_key: &[u8], kek: &[u8]) -> Result<(Vec<u8>, [u8; NONCE_LEN]), String> {
    let cipher = Aes256Gcm::new_from_slice(kek)
        .map_err(|e| format!("Cipher initialization error: {}", e))?;

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let encrypted_bytes = cipher
        .encrypt(nonce, db_key)
        .map_err(|e| format!("Encryption error: {}", e))?;

    Ok((encrypted_bytes, nonce_bytes))
}

pub fn decrypt_database_key(encrypted_db_key: &[u8], kek: &[u8], nonce_bytes: &[u8; NONCE_LEN]) -> Result<[u8; DB_KEY_LEN], String> {
    let cipher = Aes256Gcm::new_from_slice(kek)
        .map_err(|e| format!("Cipher initialization error: {}", e))?;

    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted_bytes = cipher
        .decrypt(nonce, encrypted_db_key)
        .map_err(|e| format!("Decryption error: Wrong password or ({})", e))?;

    let db_key: [u8; DB_KEY_LEN] = decrypted_bytes
        .try_into()
        .map_err(|_| "Decrypted key has invalid length".to_string())?;

    Ok(db_key)
}
