use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key
};
use argon2::{password_hash::SaltString, Argon2};

use crate::files::read_file_bytes;

pub fn generate_encryption_key(password: &str) -> Result<[u8; 32], argon2::Error> {
    let mut key_buffer = [0u8; 32];
    let salt = SaltString::generate(&mut OsRng);
    match Argon2::default().hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut key_buffer) {
        Ok(_) => return Ok(key_buffer),
        Err(err) => return Err(err),
    }
}

pub fn encrypt_file(filepath: &str, hashed_password: [u8; 32]) -> Result<Vec<u8>, aes_gcm::Error> {
    // Generate random encryption key
    let encryption_key = Key::<Aes256Gcm>::from_slice(&hashed_password);

    // Read file data
    let file_data = read_file_bytes(filepath).expect("Could not read file");

    // Encrypt data
    let cipher = Aes256Gcm::new(encryption_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    match cipher.encrypt(&nonce, file_data.as_ref()) {
        Ok(ciphertext) => return Ok(ciphertext),
        Err(e) => Err(e),
    }
}