use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use argon2::{password_hash::SaltString, Argon2};

use crate::files::read_file_bytes;

pub fn generate_salt() -> String {
    let salt = SaltString::generate(&mut OsRng).as_str().to_string();
    return salt;
}

pub fn generate_encryption_key(password: &str, salt: &str) -> Result<[u8; 32], argon2::Error> {
    let mut key_buffer = [0u8; 32];
    match Argon2::default().hash_password_into(password.as_bytes(), salt.as_bytes(), &mut key_buffer) {
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
        Ok(mut ciphertext) => {
            ciphertext.append(&mut nonce.to_vec());
            return Ok(ciphertext)
        },
        Err(e) => Err(e),
    }
}

pub fn decrypt_file(filepath: &str, hashed_password: [u8; 32]) -> Result<Vec<u8>, aes_gcm::Error> {
    // Read file data
    let mut file_data = read_file_bytes(filepath).expect("Could not read file");
    // Get 12 byte nonce from file
    let file_length = file_data.len().saturating_sub(12);
    let nonce_bytes = file_data.split_off(file_length);
    let nonce = Nonce::from_slice(&nonce_bytes);
    // Decrypt file
    let encryption_key = Key::<Aes256Gcm>::from_slice(&hashed_password);
    let cipher = Aes256Gcm::new(&encryption_key);
    match cipher.decrypt(nonce, file_data.as_ref()) {
        Ok(plaintext) => { return Ok(plaintext)},
        Err(e) => Err(e),
    }
}