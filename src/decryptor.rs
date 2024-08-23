use aes_gcm::aead::{Aead, KeyInit, generic_array::GenericArray};
use aes_gcm::{Aes256Gcm, Nonce};
use hex;

pub struct Decryptor {
    cipher: Aes256Gcm,
}

impl Decryptor {
    pub fn new(key: &[u8]) -> Self {
        let key = GenericArray::from_slice(key);
        let cipher = Aes256Gcm::new(key);

        Decryptor { cipher }
    }

    pub fn decrypt(&self, encrypted_hex: &str) -> String {
        let encrypted_data = hex::decode(encrypted_hex).expect("Decoding failed");
        let (nonce, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce);

        let plaintext = self.cipher.decrypt(nonce, ciphertext)
            .expect("decryption failure!");

        String::from_utf8(plaintext).expect("invalid UTF-8")
    }
}

