use crate::{constants::NONCE_LEN, db::{DbConn, entry::{self, Entry, NewEntry, UpdateEntry}, history::{self, NewPasswordHistory}}, dto::{NewEntryDto, UpdateEntryDto}, service::CryptoService};

pub struct EntryService {
    crypto: CryptoService,
}

impl EntryService {
    pub fn new(crypto: CryptoService) -> Self {
        Self { crypto }
    }

    pub fn create(&self, conn: &DbConn, dto: NewEntryDto) -> Result<i64, String> {
        let mut new_entry = NewEntry {
            entry_title: dto.entry_title,
            entry_url: dto.entry_url,
            entry_username: dto.entry_username,
            entry_password: dto.entry_password,
            entry_password_nonce: String::new(),
            entry_notes: dto.entry_notes,
            category_id: dto.category_id,
        };

        let nonce_bytes = self.crypto.generate_nonce();
        let encrypted_password = self.crypto.encrypt(&new_entry.entry_password, &nonce_bytes)?;

        new_entry.entry_password = encrypted_password;
        new_entry.entry_password_nonce = hex::encode(nonce_bytes);

        entry::create(conn, &new_entry)
    }

    pub fn update(&self, conn: &mut DbConn, dto: UpdateEntryDto) -> Result<(), String> {
        let tx = conn
            .transaction()
            .map_err(|e| format!("Failed to start transaction: {}", e))?;

        let mut update_entry = UpdateEntry {
            entry_id: dto.entry_id,
            entry_title: dto.entry_title,
            entry_url: dto.entry_url,
            entry_username: dto.entry_username,
            entry_notes: dto.entry_notes,
            category_id: dto.category_id,
            entry_password: None,
            entry_password_nonce: None,
        };

        if let Some(new_password) = &dto.entry_password {
            if let Some(old_entry) = entry::get_by_id(&tx, update_entry.entry_id)? {
                let history_record = NewPasswordHistory {
                    entry_id: update_entry.entry_id,
                    password: old_entry.entry_password,
                    password_nonce: Some(old_entry.entry_password_nonce)
                };

                history::create(&tx, &history_record)?;
            }

            let nonce_bytes = self.crypto.generate_nonce();
            let encrypted_password = self.crypto.encrypt(new_password, &nonce_bytes)?;

            update_entry.entry_password = Some(encrypted_password);
            update_entry.entry_password_nonce = Some(hex::encode(nonce_bytes));
        }

        entry::update(&tx, &update_entry)?;

        tx.commit()
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;

        Ok(())
    }

    pub fn get_all(conn: &DbConn, category_id: Option<i64>) -> Result<Vec<Entry>, String> {
        entry::get_all(conn, category_id)
    }

    pub fn get_by_id(&self, conn: &DbConn, entry_id: i64) -> Result<Option<Entry>, String> {
        entry::get_by_id(conn, entry_id)?.map(|mut entry| {
            let nonce_bytes: [u8; NONCE_LEN] = hex::decode(&entry.entry_password_nonce)
                .map_err(|e| format!("Invalid nonce hex format: {}", e))?
                .try_into()
                .map_err(|_| "Invalid nonce length")?;

            entry.entry_password = self.crypto.decrypt(&entry.entry_password, &nonce_bytes)?;
            Ok(entry)
        }).transpose()
    }

    pub fn delete(conn: &DbConn, entry_id: i64) -> Result<(), String> {
        entry::delete(conn, entry_id)
    }
}