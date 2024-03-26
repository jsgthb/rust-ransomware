use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm
};

use crate::files::read_file_bytes;

pub fn encrypt_file(filepath: &str) -> Result<Vec<u8>, aes_gcm::Error> {
    // Generate random encryption key
    let key = Aes256Gcm::generate_key(OsRng);

    // Read file data
    let file_data = read_file_bytes(filepath).expect("Could not read file");

    // Encrypt data
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    match cipher.encrypt(&nonce, file_data.as_ref()) {
        Ok(ciphertext) => return Ok(ciphertext),
        Err(e) => Err(e),
    }
}