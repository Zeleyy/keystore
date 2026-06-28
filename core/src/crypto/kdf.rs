use argon2::{Argon2, Params, Version};
use rand::Rng;
use sha2::{Digest, Sha256};

use crate::constants::{KEK_LEN, SALT_LEN};

pub fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    rand::rng().fill_bytes(&mut salt);
    salt
}

pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; KEK_LEN], String> {
    let params = Params::new(65536, 3, 4, Some(KEK_LEN))
        .map_err(|e| format!("KDF params error: {}", e))?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, params);

    let mut kek = [0u8; KEK_LEN];

    argon2
        .hash_password_into(password.as_bytes(), salt, &mut kek)
        .map_err(|e| format!("Failed to derive key via Argon2id: {}", e))?;

    Ok(kek)
}

pub fn derive_working_keys(master_key: &[u8; 32]) -> (String, [u8; 32]) {
    let mut db_hasher = Sha256::new();
    db_hasher.update(master_key);
    db_hasher.update(b"keystore_database_salt_v1");
    let db_key_hex = hex::encode(db_hasher.finalize());

    let mut fields_hasher = Sha256::new();
    fields_hasher.update(master_key);
    fields_hasher.update(b"keystore_fields_salt_v1");
    let mut fields_key = [0u8; 32];
    fields_key.copy_from_slice(&fields_hasher.finalize());

    (db_key_hex, fields_key)
}
