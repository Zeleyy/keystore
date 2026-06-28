use keyring::Entry;

use crate::{SERVICE_NAME, constants::METADATA_KEY};

pub fn get_entry(key: &str) -> Result<Entry, String> {
    Entry::new(SERVICE_NAME, key).map_err(|e| format!("Keyring error: {}", e))
}

pub fn load_metadata() -> Result<Option<String>, String> {
    let entry = get_entry(METADATA_KEY)?;

    match entry.get_password() {
        Ok(json_string) => Ok(Some(json_string)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Failed to read from keyring: {}", e)),
    }
}

pub fn save_metadata(json_string: &str) -> Result<(), String> {
    let entry = get_entry(METADATA_KEY)?;

    entry
        .set_password(json_string)
        .map_err(|e| format!("Failed to save to keyring: {}", e))
}

pub fn delete_metadata() -> Result<(), String> {
    let entry = get_entry(METADATA_KEY)?;

    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Failed to delete from keyring: {}", e)),
    }
}
