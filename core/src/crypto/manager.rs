use crate::{constants::{DB_KEY_LEN, NONCE_LEN}, crypto::{KeyringConfig, cipher, kdf, keyring}};

pub fn get_or_create_database_key(master_password: Option<&str>) -> Result<[u8; DB_KEY_LEN], String> {
    match keyring::load_metadata()? {
        Some(json_string) => decode_existing_key(&json_string, master_password),
        None => generate_and_save_new_key(master_password),
    }
}

fn decode_existing_key(json_string: &str, master_password: Option<&str>) -> Result<[u8; DB_KEY_LEN], String> {
    let config: KeyringConfig = serde_json::from_str(&json_string)
        .map_err(|e| format!("Failed to parse keyring config: {}", e))?;

    if config.encrypted {
        let password = master_password.ok_or("Master password is required but not provided")?;

        let salt_hex = config.argon_salt_hex.ok_or("Missing crypto salt in config")?;
        let iv_hex = config.master_iv_hex.ok_or("Missing crypto IV in config")?;

        let salt = hex::decode(salt_hex).map_err(|e| format!("Invalid salt hex: {}", e))?;
        let nonce_bytes = hex::decode(iv_hex).map_err(|e| format!("Invalid IV hex: {}", e))?;
        let encrypted_db_key = hex::decode(config.database_key_hex).map_err(|e| format!("Invalid database key hex: {}", e))?;

        let nonce: [u8; NONCE_LEN] = nonce_bytes
            .try_into()
            .map_err(|_| "Invalid IV length (expected 12 bytes)")?;

        let kek = kdf::derive_key(password, &salt)?;
        let db_key = cipher::decrypt_database_key(&encrypted_db_key, &kek, &nonce)?;

        Ok(db_key)
    } else {
        let db_key_bytes = hex::decode(config.database_key_hex)
            .map_err(|e| format!("Invalid database key hex: {}", e))?;
        
        let db_key: [u8; DB_KEY_LEN] = db_key_bytes
            .try_into()
            .map_err(|_| "Invalid database key length (expected 32 bytes)")?;
        Ok(db_key)
    }
}

fn generate_and_save_new_key(master_password: Option<&str>) -> Result<[u8; DB_KEY_LEN], String> {
    let new_db_key = cipher::generate_random_key();
    let config = KeyringConfig::create(&new_db_key, master_password)?;

    let json_string = serde_json::to_string(&config)
        .map_err(|e| format!("Failed to serialize keyring config: {}", e))?;
    keyring::save_metadata(&json_string)?;

    Ok(new_db_key)
}

pub fn change_master_password(
    current_password: Option<&str>,
    new_password: Option<&str>,
) -> Result<(), String> {
    let db_key = get_or_create_database_key(current_password)?;

    let new_config = KeyringConfig::create(&db_key, new_password)?;

    let json_string = serde_json::to_string(&new_config)
        .map_err(|e| format!("Failed to serialize keyring config: {}", e))?;
    keyring::save_metadata(&json_string)?;

    Ok(())
}