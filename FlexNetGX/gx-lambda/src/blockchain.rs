// gx-lambda/src/blockchain.rs
use sha2::{Sha256, Digest};
use anyhow::{Result, anyhow};
use solana_program::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signature},
    transaction::Transaction,
    sysvar::rent::Rent,
};
use anyhow::Result;

pub struct SolanaClient {
    client: RpcClient,
    program_id: Pubkey,
}

pub struct BlockchainService {
    // Add RPC client configuration here for MVP
}

impl BlockchainService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn hash_description(&self, description: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(description.as_bytes());
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result[..]);
        hash
    }
}

impl SolanaClient {
    pub fn new() -> Self {
        let endpoint = std::env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
            
        let program_id = std::env::var("SOLANA_PROGRAM_ID")
            .expect("SOLANA_PROGRAM_ID must be set");

        Self {
            client: RpcClient::new(endpoint),
            program_id: program_id.parse().expect("Invalid program ID"),
        }
    }

    pub async fn create_bounty(
        &self,
        creator_pubkey: &str,
        amount: u64,
    ) -> Result<String> {
        // Implement Solana transaction for bounty creation
        // This is a placeholder - actual implementation will depend on your Solana program
        Ok("transaction_signature".to_string())
    }

    pub async fn submit_work(
        &self,
        bounty_id: &str,
        hunter_pubkey: &str,
    ) -> Result<String> {
        // Implement Solana transaction for work submission
        // This is a placeholder - actual implementation will depend on your Solana program
        Ok("transaction_signature".to_string())
    }
}
