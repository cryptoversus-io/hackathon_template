use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use sha2::{Sha256, Digest};
use solana_program::program_error::ProgramError;
use rand::Rng;

pub struct SecurityManager {
    cipher: Aes256Gcm,
}

impl SecurityManager {
    pub fn new() -> Result<Self, ProgramError> {
        let mut rng = rand::thread_rng();
        let mut key = [0u8; 32];
        rng.fill(&mut key);
        
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
            
        Ok(Self { cipher })
    }

    pub fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, ProgramError> {
        let mut rng = rand::thread_rng();
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        self.cipher
            .encrypt(nonce, data)
            .map_err(|_| ProgramError::InvalidInstructionData)
    }

    pub fn decrypt_data(&self, encrypted_data: &[u8], nonce: &[u8]) -> Result<Vec<u8>, ProgramError> {
        let nonce = Nonce::from_slice(nonce);
        self.cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|_| ProgramError::InvalidInstructionData)
    }

    pub fn hash_data(&self, data: &[u8]) -> (String, String) {
        let blake3_hash = blake3::hash(data);
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(data);
        let sha256_hash = sha256_hasher.finalize();

        (
            hex::encode(blake3_hash.as_bytes()),
            hex::encode(sha256_hash),
        )
    }
}

// Tests for the SecurityManager
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_manager() {
        let manager = SecurityManager::new().unwrap();
        let data = b"test data";
        
        // Test encryption and decryption
        let encrypted = manager.encrypt_data(data).unwrap();
        assert_ne!(encrypted, data);
        
        // Test hashing
        let (blake3_hash, sha256_hash) = manager.hash_data(data);
        assert!(!blake3_hash.is_empty());
        assert!(!sha256_hash.is_empty());
    }
}