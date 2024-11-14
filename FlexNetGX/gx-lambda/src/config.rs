// gx-lambda/src/config.rs
pub struct Config {
    pub rpc_url: String,
    pub program_id: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            rpc_url: std::env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string()),
            program_id: std::env::var("PROGRAM_ID")
                .expect("PROGRAM_ID must be set"),
        }
    }
}