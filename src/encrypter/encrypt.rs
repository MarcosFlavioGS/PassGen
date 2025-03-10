use crate::{config::Config, encrypter::derive::derive};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Error, Key, Nonce,
};
// use std::env;

///
/// Encrypts a given pass using a PASSGEN_KEY and returns a Vec with the data to insert into the file.
///
pub fn encrypt(pass: &str, config: &Config) -> Result<Vec<u8>, Box<Error>> {
    // let passgen_key = env::var("PASSGEN_KEY").expect("Unable to locate environment variable");
    let key = Key::from_slice(&derive(&config.encryption.passgen_key).unwrap()).to_owned();
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce: Nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let cipher_text = cipher.encrypt(&nonce, pass.as_bytes().as_ref())?;

    let mut result: Vec<u8> = Vec::with_capacity(nonce.len() + cipher_text.len());
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&cipher_text);

    Ok(result)
}

///
/// Decrypts a nonced cipher and returns a plain text, decrypted pass.
///
pub fn decrypt(
    cipher_text_with_nonce: Vec<u8>,
    config: &Config,
) -> Result<String, Box<dyn std::error::Error>> {
    // let passgen_key = env::var("PASSGEN_KEY").expect("Unable to locate environment variable");
    let key = Key::from_slice(&derive(&config.encryption.passgen_key).unwrap()).to_owned();

    let cipher = ChaCha20Poly1305::new(&key);

    if cipher_text_with_nonce.len() < 12 {
        return Err("Invalid cipher text".into());
    }

    let (nonce, cipher_text) = cipher_text_with_nonce.split_at(12);

    if let Ok(plain_text) = cipher.decrypt(nonce.into(), cipher_text) {
        let plain_text_str = String::from_utf8(plain_text)?;
        return Ok(plain_text_str);
    } else {
        return Err("Uncable to decrypt password...".into());
    }
}
