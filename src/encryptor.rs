use aes_gcm::aead::{Aead, KeyInit, OsRng, generic_array::GenericArray};
use aes_gcm::{Aes256Gcm, Nonce};
use rand::RngCore;

pub struct Encryptor {
    cipher: Aes256Gcm,
}

impl Encryptor {
    pub fn new(key: &[u8]) -> Self {
        let key = GenericArray::from_slice(key);
        let cipher = Aes256Gcm::new(key);

        Encryptor { cipher }
    }

    pub fn encrypt(&self, plaintext: &str) -> Vec<u8> {
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        let nonce = Nonce::from_slice(&nonce);

        let ciphertext = self.cipher.encrypt(nonce, plaintext.as_ref())
            .expect("encryption failure!");

        [nonce.as_slice(), &ciphertext].concat()
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> String {
        let (nonce, ciphertext) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce);

        let plaintext = self.cipher.decrypt(nonce, ciphertext)
            .expect("decryption failure!");

        String::from_utf8(plaintext).expect("invalid UTF-8")
    }
}

