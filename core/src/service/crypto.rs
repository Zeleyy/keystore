use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use rand::{Rng, rng};

use crate::constants::NONCE_LEN;

pub struct CryptoService {
    cipher: Aes256Gcm,
}

impl CryptoService {
    pub fn new(fields_key: &[u8; 32]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(fields_key);
        let cipher = Aes256Gcm::new(key);
        Self { cipher }
    }

    pub fn generate_nonce(&self) -> [u8; NONCE_LEN] {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        rng().fill_bytes(&mut nonce_bytes);
        nonce_bytes
    }

    pub fn encrypt(&self, plain_text: &str, nonce_bytes: &[u8; NONCE_LEN]) -> Result<String, String> {
        let nonce = Nonce::from_slice(nonce_bytes);
        let cipher_text = self.cipher.encrypt(nonce, plain_text.as_bytes())
            .map_err(|e| format!("Encryption error: {}", e))?;
        
        Ok(hex::encode(cipher_text))
    }

    pub fn decrypt(&self, cipher_text_hex: &str, nonce_bytes: &[u8; NONCE_LEN]) -> Result<String, String> {
        let cipher_text = hex::decode(cipher_text_hex)
            .map_err(|e| format!("Invalid ciphertext hex: {}", e))?;

        let nonce = Nonce::from_slice(nonce_bytes);
        let decrypted_bytes = self.cipher.decrypt(nonce, cipher_text.as_slice())
            .map_err(|e| format!("Decryption error: {}", e))?;

        String::from_utf8(decrypted_bytes)
            .map_err(|e| format!("UTF-8 decoding error: {}", e))
    }
}
